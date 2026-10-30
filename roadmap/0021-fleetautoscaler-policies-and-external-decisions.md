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


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.




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


These examples explain reading order and borrowing. Actual APIs must come from the selected pinned dependencies. The owning CGP provider supplies the collections; derive-deftly supplies repeated declarations/projections. Preserve Agones-required ordering and avoid redundant clones or a comprehension around complicated effects.


## Package/module and file instructions


## Package `agones:pkg/fleetautoscalers` -> module `pkg::fleetautoscalers`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-bb8d485bbcbfdcd163.** [agones/pkg/fleetautoscalers/doc.go, lines 15–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleetautoscalers/doc.go#L15-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4d740ad1b39f9f41efa232895b02fd73b93c7af8724476c068c49d0dd4309ab8`.

> // Package fleetautoscalers handles management of the
> // Fleet Custom Resource Definition
> package fleetautoscalers


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones scaler policies, bounded counts, webhook decisions, schedules and failure outcomes. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 24 lexically identified declarations, including `fasState` (L69), `fasThread` (L75), `close` (L82), `Extensions` (L93), `FasLogger` (L99), `Controller` (L109), `NewController` (L129), `NewExtensions` (L188), `Run` (L205), `loggerForFleetAutoscalerKey` (L230), `loggerForFleetAutoscaler` (L234), `loggerForFleetAutoscalerKey` (L242), `loggerForFleetAutoscaler` (L246), `mutationHandler` (L252), `validationHandler` (L288), `syncFleetAutoscaler` (L313), `getFleetAutoscalerByKey` (L397), `scaleFleet` (L417), `updateStatus` (L436), `updateStatusUnableToScale` (L475), `addFasThread` (L496), `updateFasThread` (L535), `deleteFasThread` (L558), `cleanFasThreads` (L574). The complete list remains in `agones:pkg/fleetautoscalers/controller.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


10. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones scaler policies, bounded counts, webhook decisions, schedules and failure outcomes. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones scaler policies, bounded counts, webhook decisions, schedules and failure outcomes. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 23 lexically identified declarations, including `errs` (L54), `InactiveScheduleError` (L57), `Error` (L59), `computeDesiredFleetSize` (L64), `applyWasmPolicy` (L101), `buildURLFromConfiguration` (L210), `createURL` (L266), `setCABundle` (L279), `applyWebhookPolicy` (L288), `applyBufferPolicy` (L379), `applyCounterOrListPolicyWrapper` (L419), `applyCounterOrListPolicy` (L442), `applySchedulePolicy` (L563), `applyChainPolicy` (L580), `isScheduleActive` (L651), `getSortedDeletableGameServers` (L706), `isLimited` (L727), `scaleUpLimited` (L738), `scaleDownLimited` (L750), `scaleLimited` (L783), `scaleUp` (L801), `scaleDown` (L822), `emitChainPolicyEvent` (L886). The complete list remains in `agones:pkg/fleetautoscalers/fleetautoscalers.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

