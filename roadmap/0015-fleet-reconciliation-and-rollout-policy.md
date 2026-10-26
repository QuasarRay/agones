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


## Package `agones:pkg/fleets` -> module `pkg::fleets`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-e52912bf54a3c4a5fc.** [agones/pkg/fleets/doc.go, lines 15–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/fleets/doc.go#L15-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `60faf5290a33b2d57177ddce349d06d59a0ff4f650405486605b2bb2995eebb0`.

> // Package fleets handles management of the
> // Fleet Custom Resource Definition
> package fleets


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 31 lexically identified declarations, including `Extensions` (L59), `Controller` (L66), `NewController` (L83), `NewExtensions` (L168), `creationMutationHandler` (L184), `creationValidationHandler` (L226), `Run` (L252), `loggerForFleetKey` (L269), `loggerForFleet` (L273), `gameServerSetEventHandler` (L283), `syncFleet` (L305), `upsertGameServerSet` (L361), `applyDeploymentStrategy` (L417), `deleteEmptyGameServerSets` (L447), `recreateDeployment` (L465), `rollingUpdateDeployment` (L488), `rollingUpdateActive` (L501), `rollingUpdateRestFixedOnReady` (L555), `rollingUpdateRest` (L563), `updateFleetStatus` (L569), `filterGameServerSetByActive` (L633), `flushAllocations` (L659), `allocTracker` (L675), `newAllocTracker` (L680). The full declaration list is recorded under `agones:pkg/fleets/controller.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/util/crd`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/webhooks`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 2 lexically identified declarations, including `cleanupUnhealthyReplicasRollingUpdateFix` (L14), `rollingUpdateRestFixedOnReadyRollingUpdateFix` (L54). The full declaration list is recorded under `agones:pkg/fleets/controller_rollingupdatefix.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 3 lexically identified declarations, including `errs` (L27), `ListGameServerSetsByFleetOwner` (L31), `ListGameServersByFleetOwner` (L49). The full declaration list is recorded under `agones:pkg/fleets/fleets.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/util/errors`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
