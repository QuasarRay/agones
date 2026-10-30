<a id="m0015"></a>


# M0015 — Fleet reconciliation and rollout policy


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0015-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-14df81f1919fe3b881.** [agones/site/content/en/docs/Reference/fleet.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/fleet.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `de8f4305c2d2202143571ac4c41dfed4044fa4184a835239cc6ce2b4673e2c1c`.

> 
> To allocate a `GameServer` from a `Fleet`, use a `GameServerAllocation`.
> 
> Like any other Kubernetes resource you describe a `Fleet`'s desired state via a specification written in YAML or JSON to the Kubernetes API. The Agones controller will then change the actual state to the desired state.
> 
> A full `Fleet` specification is available below and in the {{< ghlink href="examples/fleet.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: Fleet
> # Fleet Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Port Fleet replica helpers, GameServerSet creation/selection, rolling update and recreate behavior. Preserve maxSurge/maxUnavailable rounding, percentage and absolute forms, allocation overflow and status fields.

2. Keep generation/resource-version conflict semantics and source handling of allocated servers. Record exact source helper dependencies and arithmetic domains.

3. CGP owns rollout/retry/lifecycle policy; derive-deftly owns repeated declarative wiring and projections. Name each transformation so a Python developer can follow desired state to decisions to mutations.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.




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


These examples explain reading order and borrowing. Actual APIs must come from the selected pinned dependencies. The owning CGP provider supplies the collections; derive-deftly supplies repeated declarations/projections. Preserve Agones-required ordering and avoid redundant clones or a comprehension around complicated effects.


## Package/module and file instructions


## Package `agones:pkg/fleets` -> module `pkg::fleets`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-e52912bf54a3c4a5fc.** [agones/pkg/fleets/doc.go, lines 15–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleets/doc.go#L15-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `60faf5290a33b2d57177ddce349d06d59a0ff4f650405486605b2bb2995eebb0`.

> // Package fleets handles management of the
> // Fleet Custom Resource Definition
> package fleets


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-da3153c9de36746a"></a>


### File `pkg/fleets/controller.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `627e2ef9a63266cabbba778bd88dac926999746b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/fleets/controller.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-aaff05baa00933f900.** [agones/pkg/fleets/controller.go, lines 57–71](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleets/controller.go#L57-L71). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a2b5995f0709f93f0ed74ef1fc0cd97ba0c0e360091543e1ab4cfb5273faa76c`.

> 
> // Extensions struct contains what is needed to bind webhook handlers
> type Extensions struct {
> 	baseLogger *logrus.Entry
> 	apiHooks   agonesv1.APIHooks
> 	errs       *errors.Errors
> }
> 
> // Controller is a the GameServerSet controller
> type Controller struct {
> 	baseLogger          *logrus.Entry
> 	errs                *errors.Errors
> 	crdGetter           apiextclientv1.CustomResourceDefinitionInterface
> 	gameServerSynced    cache.InformerSynced
> 	gameServerSetGetter getterv1.GameServerSetsGetter


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones Fleet rollout, replica and surge/unavailability arithmetic, resource versions and allocated-server handling. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 31 lexically identified declarations, including `Extensions` (L59), `Controller` (L66), `NewController` (L83), `NewExtensions` (L168), `creationMutationHandler` (L184), `creationValidationHandler` (L226), `Run` (L252), `loggerForFleetKey` (L269), `loggerForFleet` (L273), `gameServerSetEventHandler` (L283), `syncFleet` (L305), `upsertGameServerSet` (L361), `applyDeploymentStrategy` (L417), `deleteEmptyGameServerSets` (L447), `recreateDeployment` (L465), `rollingUpdateDeployment` (L488), `rollingUpdateActive` (L501), `rollingUpdateRestFixedOnReady` (L555), `rollingUpdateRest` (L563), `updateFleetStatus` (L569), `filterGameServerSetByActive` (L633), `flushAllocations` (L659), `allocTracker` (L675), `newAllocTracker` (L680). The complete list remains in `agones:pkg/fleets/controller.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-11ddb143f5644160"></a>


### File `pkg/fleets/controller_rollingupdatefix.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `289bad98fa83b5230026e11afc20b39f8a86abb1`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/fleets/controller_rollingupdatefix.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-dd1ff925194567819a.** [agones/pkg/fleets/controller_rollingupdatefix.go, lines 13–27](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleets/controller_rollingupdatefix.go#L13-L27). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e5e6719009f5bee6c5f2726ffbde2111aa7f6cc06b83b41875b31a939b76cc3d`.

> 
> func (c *Controller) cleanupUnhealthyReplicasRollingUpdateFix(ctx context.Context, rest []*agonesv1.GameServerSet,
> 	fleet *agonesv1.Fleet, maxCleanupCount int32) ([]*agonesv1.GameServerSet, int32, error) {
> 
> 	// Safely scale down all old GameServerSets with unhealthy replicas.
> 	totalScaledDown := int32(0)
> 	for i, gsSet := range rest {
> 		healthy := gsSet.Status.ReadyReplicas + gsSet.Status.AllocatedReplicas + gsSet.Status.ReservedReplicas
> 
> 		if totalScaledDown >= maxCleanupCount {
> 			break
> 		}
> 		if gsSet.Spec.Replicas == 0 {
> 			// cannot scale down this replica set.
> 			continue


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones Fleet rollout, replica and surge/unavailability arithmetic, resource versions and allocated-server handling. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 2 lexically identified declarations, including `cleanupUnhealthyReplicasRollingUpdateFix` (L14), `rollingUpdateRestFixedOnReadyRollingUpdateFix` (L54). The complete list remains in `agones:pkg/fleets/controller_rollingupdatefix.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-a904f55aefbcae5e"></a>


### File `pkg/fleets/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0f8c608a20d74a5e3e017c4e0bd96ba5b2504fe1`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/fleets/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-25dee8811f3307985a.** [agones/pkg/fleets/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleets/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `60faf5290a33b2d57177ddce349d06d59a0ff4f650405486605b2bb2995eebb0`.

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
> // Package fleets handles management of the


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones Fleet rollout, replica and surge/unavailability arithmetic, resource versions and allocated-server handling. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-bf14cf44a7ade13a"></a>


### File `pkg/fleets/fleets.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `812b33c824103fd94da6ebefd70904599433ef42`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/fleets/fleets.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-885beb12257c592eb1.** [agones/pkg/fleets/fleets.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleets/fleets.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a2db023f36e2618f7728f94dfb2e7a32b54dee5dae8b40235af98ced74bd407b`.

> 
> // ListGameServerSetsByFleetOwner lists all the GameServerSets for a given
> // Fleet
> func ListGameServerSetsByFleetOwner(gameServerSetNamespacedLister listerv1.GameServerSetNamespaceLister, f *agonesv1.Fleet) ([]*agonesv1.GameServerSet, error) {
> 	list, err := gameServerSetNamespacedLister.List(labels.SelectorFromSet(labels.Set{agonesv1.FleetNameLabel: f.ObjectMeta.Name}))
> 	if err != nil {
> 		return list, errs.Wrapf(err, "error listing gameserversets for fleet %s", f.ObjectMeta.Name)
> 	}
> 
> 	var result []*agonesv1.GameServerSet
> 	for _, gsSet := range list {
> 		if metav1.IsControlledBy(gsSet, f) {
> 			result = append(result, gsSet)
> 		}
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones Fleet rollout, replica and surge/unavailability arithmetic, resource versions and allocated-server handling. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 3 lexically identified declarations, including `errs` (L27), `ListGameServerSetsByFleetOwner` (L31), `ListGameServersByFleetOwner` (L49). The complete list remains in `agones:pkg/fleets/fleets.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

