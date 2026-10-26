<a id="m0021"></a>


# M0021 — FleetAutoscaler policies and external decisions


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0021-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0015](0015-fleet-reconciliation-and-rollout-policy.md), [M0019](0019-gameserver-and-gameserverset-lifecycle.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-b7dff160cee9345833.** [agones/site/content/en/docs/Reference/fleetautoscaler.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/fleetautoscaler.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `041941ac43734e13a409c2af21cbae5ab257f73fdfdfcaaed9474cdf1287629d`.

> 
> A full `FleetAutoscaler` specification is available below and in the
> {{< ghlink href="examples/fleetautoscaler.yaml" >}}example folder{{< /ghlink >}} for reference, but here are several
> examples that show different autoscaling policies.
> 
> ## Ready Buffer Autoscaling
> 
> Fleet autoscaling with a buffer can be used to maintain a configured number of game server instances ready to serve
> players based on number of allocated instances in a Fleet. The buffer size can be specified as an absolute number or a
> percentage of the desired number of Ready game server instances over the Allocated count.
> 
> ```yaml
> apiVersion: "autoscaling.agones.dev/v1"


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Implement source buffer/counter/list/webhook/scheduled and other discovered scaler policies, status and minimum/maximum/percentage behavior without narrowing the selected baseline.

2. Preserve webhook request/response, time-zone/timer behavior, retries and failure outcomes. Existing Wasm policy support does not authorize a plugin system; retain its exact boundary and surface genuine contract conflicts.

3. Compose policy selection statically through CGP and generate repeated policy/configuration structure with derive-deftly. No runtime service locator, alternate orchestrator or generic extension loader.


## Architecture, readability and authored outputs


1. Own 3 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:pkg/fleetautoscalers` -> module `pkg::fleetautoscalers`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-bb8d485bbcbfdcd163.** [agones/pkg/fleetautoscalers/doc.go, lines 15–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleetautoscalers/doc.go#L15-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4d740ad1b39f9f41efa232895b02fd73b93c7af8724476c068c49d0dd4309ab8`.

> // Package fleetautoscalers handles management of the
> // Fleet Custom Resource Definition
> package fleetautoscalers


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ed445d78711cd31f"></a>


### File `pkg/fleetautoscalers/controller.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `bb4c8b4e4a2664707902d1157f90b19867bd9134`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/fleetautoscalers/controller.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f9a71bd79e4e338d01.** [agones/pkg/fleetautoscalers/controller.go, lines 65–79](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleetautoscalers/controller.go#L65-L79). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `22e76f55eb336d961302e07863c3fd870b71f0658f80ded0f687e47632fcb797`.

> 
> // fasState is the state for each Fleet Autoscaler instance
> // expand this structure if extra state is needed for other autoscaler
> // strategies.
> type fasState struct {
> 	wasmPlugin *extism.Plugin
> 	httpClient *http.Client
> }
> 
> // fasThread is used for tracking each Fleet's autoscaling jobs
> type fasThread struct {
> 	cancel     context.CancelFunc
> 	state      *fasState
> 	generation int64
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 24 lexically identified declarations, including `fasState` (L69), `fasThread` (L75), `close` (L82), `Extensions` (L93), `FasLogger` (L99), `Controller` (L109), `NewController` (L129), `NewExtensions` (L188), `Run` (L205), `loggerForFleetAutoscalerKey` (L230), `loggerForFleetAutoscaler` (L234), `loggerForFleetAutoscalerKey` (L242), `loggerForFleetAutoscaler` (L246), `mutationHandler` (L252), `validationHandler` (L288), `syncFleetAutoscaler` (L313), `getFleetAutoscalerByKey` (L397), `scaleFleet` (L417), `updateStatus` (L436), `updateStatusUnableToScale` (L475), `addFasThread` (L496), `updateFasThread` (L535), `deleteFasThread` (L558), `cleanFasThreads` (L574). The full declaration list is recorded under `agones:pkg/fleetautoscalers/controller.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/apis/autoscaling`, `agones.dev/agones/pkg/apis/autoscaling/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned/typed/autoscaling/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/client/listers/autoscaling/v1`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/util/crd`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/webhooks`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

8. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7abad498f0d735a4"></a>


### File `pkg/fleetautoscalers/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c6867bc2fdcb186d4889d3c75c4ed5ccb74cfc34`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/fleetautoscalers/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b53995c9173f168fad.** [agones/pkg/fleetautoscalers/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleetautoscalers/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4d740ad1b39f9f41efa232895b02fd73b93c7af8724476c068c49d0dd4309ab8`.

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
> // Package fleetautoscalers handles management of the


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c503b35cee81c391"></a>


### File `pkg/fleetautoscalers/fleetautoscalers.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c03faea9a51ff195dd0edcad8f06827397e9d597`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/fleetautoscalers/fleetautoscalers.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-7b3686150001cd33f4.** [agones/pkg/fleetautoscalers/fleetautoscalers.go, lines 55–69](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleetautoscalers/fleetautoscalers.go#L55-L69). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `83dcdc14e35e85e50372b1d3734b11d955958aec5019a1800f7214f7cfc76357`.

> 
> // InactiveScheduleError denotes an error for schedules that are not currently active.
> type InactiveScheduleError struct{}
> 
> func (InactiveScheduleError) Error() string {
> 	return "inactive schedule, policy not applicable"
> }
> 
> // computeDesiredFleetSize computes the new desired size of the given fleet
> func computeDesiredFleetSize(ctx context.Context, state *fasState, pol autoscalingv1.FleetAutoscalerPolicy, f *agonesv1.Fleet,
> 	gameServerNamespacedLister listeragonesv1.GameServerNamespaceLister, nodeCounts map[string]gameservers.NodeCount, fasLog *FasLogger) (int32, bool, error) {
> 
> 	var (
> 		replicas int32
> 		limited  bool


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 23 lexically identified declarations, including `errs` (L54), `InactiveScheduleError` (L57), `Error` (L59), `computeDesiredFleetSize` (L64), `applyWasmPolicy` (L101), `buildURLFromConfiguration` (L210), `createURL` (L266), `setCABundle` (L279), `applyWebhookPolicy` (L288), `applyBufferPolicy` (L379), `applyCounterOrListPolicyWrapper` (L419), `applyCounterOrListPolicy` (L442), `applySchedulePolicy` (L563), `applyChainPolicy` (L580), `isScheduleActive` (L651), `getSortedDeletableGameServers` (L706), `isLimited` (L727), `scaleUpLimited` (L738), `scaleDownLimited` (L750), `scaleLimited` (L783), `scaleUp` (L801), `scaleDown` (L822), `emitChainPolicyEvent` (L886). The full declaration list is recorded under `agones:pkg/fleetautoscalers/fleetautoscalers.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/apis/autoscaling/v1`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/fleets`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/gameserversets`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Preserve source floating width, narrowing, exceptional values and relevant bits.

8. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
