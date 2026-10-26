<a id="m0019"></a>


# M0019 — GameServer and GameServerSet lifecycle


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0019-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0013](0013-cloud-hooks-through-the-permitted-rusternetes-boundary.md), [M0017](0017-telemetry-and-metric-semantics.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0011](0011-port-ownership-and-allocation-policy.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-065412871f52a2cfa1.** [agones/site/content/en/docs/Advanced/scheduling-and-autoscaling.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/scheduling-and-autoscaling.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c49c4434a7b6b09cdc55409e9d8a3818562323a54c08b1c42e8005f54fdb10bd`.

> 
> ## Cluster Autoscaler
> 
> Kubernetes has a [cluster node autoscaler that works with a wide variety of cloud providers](https://github.com/kubernetes/autoscaler/tree/master/cluster-autoscaler).
> 
> The default scheduling strategy (`Packed`) is designed to work with the Kubernetes autoscaler out of the box.
> 
> The autoscaler will automatically add Nodes to the cluster when `GameServers` don't have room to be scheduled on the
> clusters, and then scale down when there are empty Nodes with no `GameServers` running on them.
> 
> This means that scaling `Fleets` up and down can be used to control the size of the cluster, as the cluster autoscaler
> will adjust the size of the cluster to match the resource needs of one or more `Fleets` running on it.


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Implement GameServer reconciliation, pod/workload construction, sidecar settings, health/state handling, finalizers and admission hooks from the exact source bodies.

2. Retain scheduling, port, resource, service-account, feature-gate, deletion and source failure behavior. A source transition and its persisted mutation/event must remain connected to the real production path.

3. Express controllers and services as CGP providers with derive-deftly-generated ownership/wiring. Keep initialization, watch, queue, readiness, cancellation, drain and cleanup policies in the same lifecycle tree.

4. Port replica calculation, GameServer selection, create/delete decisions, allocation-aware scaling and source status updates for GameServerSets.

5. Preserve Packed/Distributed policy, Ready/Allocated accounting, deletion candidates, overflow metadata, ordering and conflict retries. Do not hide unfinished update paths behind a state-only controller.

6. Implement decisions as small CGP provider functions; derive repeated state/resource projections. Comprehensions must preserve the source order or explicitly retain its allowed unordered outcomes.


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


## Package `agones:pkg/gameservers` -> module `pkg::gameservers`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-284bab084a422498d3.** [agones/pkg/gameservers/doc.go, lines 15–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/doc.go#L15-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7bc39bff8f4e47d718f688d4bb9cd6ff78b36abdeb14002dc734fe0a01f5063d`.

> // Package gameservers handles management of the
> // GameServer Custom Resource Definition
> package gameservers


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-f57ba351e86b3903"></a>


### File `pkg/gameservers/controller.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `84217f84969e55a5d2c71a1dd6e4f4f3c495c9ff`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameservers/controller.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b90252eeeb6c61e911.** [agones/pkg/gameservers/controller.go, lines 70–84](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/controller.go#L70-L84). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `967cc16abde41b47212cd2da4052b8007e629fe43e149fbb550044f0fe89f624`.

> 
> // Extensions struct contains what is needed to bind webhook handlers
> type Extensions struct {
> 	baseLogger *logrus.Entry
> 	apiHooks   agonesv1.APIHooks
> 	errs       *errors.Errors
> }
> 
> // Controller is a the main GameServer crd controller
> //
> //nolint:govet // ignore fieldalignment, singleton
> type Controller struct {
> 	baseLogger               *logrus.Entry
> 	errs                     *errors.Errors
> 	controllerHooks          cloudproduct.ControllerHooksInterface


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 32 lexically identified declarations, including `Extensions` (L72), `Controller` (L81), `NewController` (L116), `NewExtensions` (L239), `enqueueGameServerBasedOnState` (L252), `fastRateLimiter` (L269), `creationMutationHandler` (L280), `loggerForGameServerKey` (L316), `loggerForGameServer` (L320), `creationValidationHandler` (L330), `creationMutationHandlerPod` (L355), `Run` (L416), `syncGameServer` (L480), `syncGameServerDeletionTimestamp` (L532), `syncGameServerPortAllocationState` (L574), `syncGameServerCreatingState` (L598), `syncDevelopmentGameServer` (L649), `createGameServerPod` (L692), `sidecar` (L746), `DefaultSidecarSecurityContext` (L830), `addGameServerHealthCheck` (L841), `addSDKServerEnvVars` (L876), `addEnvVarsToContainer` (L901), `reservedEnvironmentVariableName` (L914). The full declaration list is recorded under `agones:pkg/gameservers/controller.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/cloudproduct`, `agones.dev/agones/pkg/portallocator`, `agones.dev/agones/pkg/util/crd`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/webhooks`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-db699f78a9bbc226"></a>


### File `pkg/gameservers/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a27c3bc5f37b665260adfd68ad2ad820a70541a4`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameservers/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e3bbef1d5a475caed0.** [agones/pkg/gameservers/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7bc39bff8f4e47d718f688d4bb9cd6ff78b36abdeb14002dc734fe0a01f5063d`.

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
> // Package gameservers handles management of the


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-880fef94776aebd7"></a>


### File `pkg/gameservers/gameservers.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `13f744e7965f2640934f65cc44600441287b2dff`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameservers/gameservers.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-2063bdc9ff7474267e.** [agones/pkg/gameservers/gameservers.go, lines 36–54](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/gameservers.go#L36-L54). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0501e2f0ab9479bbf5a3b4ef08d5938660803b9f62203825ca3a1b0fea9eee0e`.

> 
> // newStateUpdateLimiter returns a limiter for the controllers that move a GameServer to a terminal
> // state (SucceededController, MissingPodController and HealthController), to be waited on just
> // before that Update, so a large wave of completed, missing or failed Pods doesn't swamp the API
> // server.
> //
> // Those controllers' informer handlers fire on every informer resync, so they enqueue with
> // EnqueueImmediately rather than Enqueue: a rate-limited enqueue spends a token from the queue's
> // shared bucket on every call, even when the key is already queued, which lets the bucket's backlog
> // grow without bound under load. Throttling the write instead only spends tokens on real updates,
> // and failed syncs still retry with backoff through the workerqueue.
> func newStateUpdateLimiter() *rate.Limiter {
> 	return rate.NewLimiter(stateUpdateRate, stateUpdateBurst)
> }
> 
> // isGameServerPod returns if this Pod is a Pod that comes from a GameServer
> func isGameServerPod(pod *corev1.Pod) bool {
> 	// add some defense in case this happens
> 	if pod == nil {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `errs` (L28), `newStateUpdateLimiter` (L47), `isGameServerPod` (L52), `address` (L74), `applyGameServerAddressAndPort` (L110), `applyGameServerPodIPs` (L143), `isBeforePodCreated` (L169). The full declaration list is recorded under `agones:pkg/gameservers/gameservers.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/util/errors`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a37562ee9d9a2d3f"></a>


### File `pkg/gameservers/health.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1aae8cca1fde92cb0b9edbf03ea25e4fd304c122`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameservers/health.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-2fefebe62a5133aee7.** [agones/pkg/gameservers/health.go, lines 45–59](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/health.go#L45-L59). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3efbba0165460259ca2b419b529dd8f0aece9af65c86201ce486aa8a884877a2`.

> 
> // HealthController watches Pods, and applies
> // an Unhealthy state if certain pods crash, or can't be assigned a port, and other
> // similar type conditions.
> type HealthController struct {
> 	baseLogger       *logrus.Entry
> 	errs             *errors.Errors
> 	podSynced        cache.InformerSynced
> 	podLister        corelisterv1.PodLister
> 	gameServerSynced cache.InformerSynced
> 	gameServerGetter getterv1.GameServersGetter
> 	gameServerLister listerv1.GameServerLister
> 	workerqueue      *workerqueue.WorkerQueue
> 	unhealthyLimiter *rate.Limiter
> 	recorder         record.EventRecorder


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 12 lexically identified declarations, including `HealthController` (L49), `NewHealthController` (L64), `isUnhealthy` (L115), `unschedulableWithNoFreePorts` (L121), `failedPod` (L143), `evictedPod` (L154), `failedContainer` (L164), `Run` (L209), `loggerForGameServerKey` (L220), `loggerForGameServer` (L224), `syncGameServer` (L233), `skipUnhealthyGameContainer` (L305). The full declaration list is recorded under `agones:pkg/gameservers/health.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f66c4567b874eace"></a>


### File `pkg/gameservers/migration.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `319bdb1e61b5f97746a46d13985e707bf92f8b90`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameservers/migration.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-84c2532f737cfc198b.** [agones/pkg/gameservers/migration.go, lines 44–58](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/migration.go#L44-L58). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a24081f90af1f1838fda60bc17cbaa9a21439a594b285664ac8f971818570038`.

> 
> // MigrationController watches for if a Pod is migrated/a maintenance
> // event happens on a node, and a Pod is recreated with a new Address for a
> // GameServer
> type MigrationController struct {
> 	baseLogger               *logrus.Entry
> 	podSynced                cache.InformerSynced
> 	podLister                corelisterv1.PodLister
> 	gameServerSynced         cache.InformerSynced
> 	gameServerGetter         getterv1.GameServersGetter
> 	gameServerLister         listerv1.GameServerLister
> 	nodeLister               corelisterv1.NodeLister
> 	nodeSynced               cache.InformerSynced
> 	workerqueue              *workerqueue.WorkerQueue
> 	recorder                 record.EventRecorder


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `MigrationController` (L48), `NewMigrationController` (L64), `Run` (L114), `loggerForGameServerKey` (L124), `loggerForGameServer` (L128), `isMigratingGameServerPod` (L136), `syncGameServer` (L187), `anyAddressMatch` (L235). The full declaration list is recorded under `agones:pkg/gameservers/migration.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8522bc95fc7c2da2"></a>


### File `pkg/gameservers/missing.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6a7c52f7cd81b45ae477e6f8a88b862369b3e7e2`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameservers/missing.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f0168be392759b8c9e.** [agones/pkg/gameservers/missing.go, lines 44–63](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/missing.go#L44-L63). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `87f272685bf3af3a98d7e59ca50ea79e8f6d314845b503d87646f230f338aebf`.

> 
> // MissingPodController makes sure that any GameServer that isn't in a
> // Scheduled or Unhealthy state and is either missing a Pod, or has a Pod in a
> // Failed state, is moved to Unhealthy.
> //
> // It's possible that a GameServer is missing its associated pod due to
> // unexpected controller downtime or if the Pod is deleted with no subsequent Delete event.
> // Similarly, a Pod can reach the Failed phase (e.g. non-zero exit with SidecarContainers
> // enabled) without a corresponding Unhealthy transition being triggered.
> //
> // Since resync on the controller is every 30 seconds, even if there is some time in which a GameServer
> // is in a broken state, it will eventually move to Unhealthy, and get replaced (if in a Fleet).
> type MissingPodController struct {
> 	baseLogger       *logrus.Entry
> 	podSynced        cache.InformerSynced
> 	podLister        corelisterv1.PodLister
> 	gameServerSynced cache.InformerSynced
> 	gameServerGetter getterv1.GameServersGetter
> 	gameServerLister listerv1.GameServerLister
> 	workerqueue      *workerqueue.WorkerQueue


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 5 lexically identified declarations, including `MissingPodController` (L56), `NewMissingPodController` (L70), `Run` (L119), `loggerForGameServerKey` (L129), `syncGameServer` (L135). The full declaration list is recorded under `agones:pkg/gameservers/missing.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/scheme`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a6074bd24f7af4c2"></a>


### File `pkg/gameservers/pernodecounter.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `40e56ad96828d6fb71480199ad768dcae02a05ad`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameservers/pernodecounter.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-2445e6dbaefed62f16.** [agones/pkg/gameservers/pernodecounter.go, lines 33–47](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/pernodecounter.go#L33-L47). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d5af70cf55595d141d1fcdf12ca9e65409631009b2dd3c998df4cef0436ed263`.

> 
> // PerNodeCounter counts how many Allocated and
> // Ready GameServers currently exist on each node.
> // This is useful for scheduling allocations, fleet management
> // mostly under a Packed strategy
> //
> //nolint:govet // ignore fieldalignment, singleton
> type PerNodeCounter struct {
> 	logger           *logrus.Entry
> 	errs             *errors.Errors
> 	gameServerSynced cache.InformerSynced
> 	gameServerLister listerv1.GameServerLister
> 	countMutex       sync.RWMutex
> 	counts           map[string]*NodeCount
> 	processed        map[types.UID]processed


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 10 lexically identified declarations, including `PerNodeCounter` (L40), `processed` (L51), `NodeCount` (L59), `NewPerNodeCounter` (L67), `Run` (L202), `Counts` (L243), `inc` (L259), `calculateStateTransition` (L281), `isAlreadyProcessed` (L299), `updateProcessed` (L310). The full declaration list is recorded under `agones:pkg/gameservers/pernodecounter.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-798df576b0a04b8e"></a>


### File `pkg/gameservers/succeeded.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `745a29e39afd16222cefc7814f9749fefe6930aa`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameservers/succeeded.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c219db10717c292810.** [agones/pkg/gameservers/succeeded.go, lines 44–58](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameservers/succeeded.go#L44-L58). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `aca74fc22fecd57c3b47db2353a89c5eca3fe1ed25dfe3c29791313ae3b14b52`.

> 
> // SucceededController changes the state of a GameServer to Shutdown when its Pod has
> // completed: either the Pod reached Succeeded, or the game server container exited cleanly.
> type SucceededController struct {
> 	baseLogger       *logrus.Entry
> 	podSynced        cache.InformerSynced
> 	podLister        corelisterv1.PodLister
> 	gameServerSynced cache.InformerSynced
> 	gameServerGetter getterv1.GameServersGetter
> 	gameServerLister listerv1.GameServerLister
> 	workerqueue      *workerqueue.WorkerQueue
> 	shutdownLimiter  *rate.Limiter
> 	recorder         record.EventRecorder
> 	errs             *errors.Errors
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `SucceededController` (L47), `NewSucceededController` (L61), `Run` (L126), `loggerForGameServerKey` (L136), `syncGameServer` (L141), `podCompleted` (L202), `gameServerContainerCompleted` (L209). The full declaration list is recorded under `agones:pkg/gameservers/succeeded.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/scheme`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/gameserversets` -> module `pkg::gameserversets`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-fe00d1af2be4ae213e.** [agones/pkg/gameserversets/doc.go, lines 15–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserversets/doc.go#L15-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7ea0a88012e8214650aaad13c96144559c5f96c8994762c52f270cf00f2414a5`.

> // Package gameserversets handles management of the
> // GameServerSet Custom Resource Definition
> package gameserversets


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-85befa2749abccad"></a>


### File `pkg/gameserversets/allocation_overflow.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b46c16a2c71539d7c5d85aeefd03bacc21e5f9fd`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserversets/allocation_overflow.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-71a287086bc78f8b2c.** [agones/pkg/gameserversets/allocation_overflow.go, lines 38–52](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserversets/allocation_overflow.go#L38-L52). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `818f23fffe677cb709da1fe29ec4c297dc40bc92e7de28f190bef7e28b413838`.

> 
> // AllocationOverflowController watches `GameServerSets`, and those with configured
> // AllocationOverflow settings, will the relevant labels and annotations to `GameServers` attached to the given
> // `GameServerSet`
> type AllocationOverflowController struct {
> 	baseLogger          *logrus.Entry
> 	counter             *gameservers.PerNodeCounter
> 	gameServerSynced    cache.InformerSynced
> 	gameServerGetter    getterv1.GameServersGetter
> 	gameServerLister    listerv1.GameServerLister
> 	gameServerSetSynced cache.InformerSynced
> 	gameServerSetLister listerv1.GameServerSetLister
> 	workerqueue         *workerqueue.WorkerQueue
> 	errs                *errors.Errors
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 4 lexically identified declarations, including `AllocationOverflowController` (L42), `NewAllocatorOverflowController` (L55), `Run` (L99), `syncGameServerSet` (L111). The full declaration list is recorded under `agones:pkg/gameserversets/allocation_overflow.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c81ce0a753a01d89"></a>


### File `pkg/gameserversets/controller.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `96b53fffd032e1066b003687383e36d9c41911ed`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserversets/controller.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ecb8ffac56afe0237a.** [agones/pkg/gameserversets/controller.go, lines 66–80](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserversets/controller.go#L66-L80). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `981e9038f9fe73bdada1faa8a1b70019192ea6a0a764744aeddb675666c6ed93`.

> 
> // Extensions struct contains what is needed to bind webhook handlers
> type Extensions struct {
> 	baseLogger *logrus.Entry
> 	apiHooks   agonesv1.APIHooks
> 	errs       *errors.Errors
> }
> 
> func init() {
> 	registerViews()
> }
> 
> // Controller is a GameServerSet controller
> type Controller struct {
> 	baseLogger                     *logrus.Entry


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 25 lexically identified declarations, including `Extensions` (L68), `init` (L74), `Controller` (L79), `NewController` (L102), `NewExtensions` (L180), `Run` (L194), `updateValidationHandler` (L217), `creationValidationHandler` (L249), `gameServerEventHandler` (L273), `syncGameServerSet` (L298), `computeReconciliationAction` (L380), `shouldDeleteErroredGameServer` (L503), `addMoreGameServers` (L522), `deleteGameServers` (L547), `newGameServersChannel` (L565), `gameServerListToChannel` (L578), `parallelize` (L593), `syncGameServerSetStatus` (L623), `updateStatusIfChanged` (L628), `computeStatus` (L641), `createInitialListStatus` (L692), `createInitialCounterStatus` (L700), `aggregateCounters` (L709), `aggregateLists` (L751). The full declaration list is recorded under `agones:pkg/gameserversets/controller.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/util/crd`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/webhooks`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.

7. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

8. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1560b78e258eef42"></a>


### File `pkg/gameserversets/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `805db0bd869521653af3fad77259657712334cd8`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserversets/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-09a35d3de6e316ae17.** [agones/pkg/gameserversets/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserversets/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7ea0a88012e8214650aaad13c96144559c5f96c8994762c52f270cf00f2414a5`.

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
> // Package gameserversets handles management of the


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ed6466b6ab3938e8"></a>


### File `pkg/gameserversets/gameserver_state_cache.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9a6f7e3b19cf32f19a32278f4ff8175de172ed70`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserversets/gameserver_state_cache.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-657251394e5540a8dd.** [agones/pkg/gameserversets/gameserver_state_cache.go, lines 23–37](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserversets/gameserver_state_cache.go#L23-L37). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `045dc7de48b361f1447111ca9609b003f956a771a0bc0ece7e9cf264769e198e`.

> 
> // gameServerSetCacheEntry manages a list of items created and deleted locally for a single game server set.
> type gameServerSetCacheEntry struct {
> 	pendingCreation map[string]*agonesv1.GameServer
> 	pendingDeletion map[string]*agonesv1.GameServer
> 	mu              sync.Mutex
> }
> 
> func (e *gameServerSetCacheEntry) created(gs *agonesv1.GameServer) {
> 	e.mu.Lock()
> 	defer e.mu.Unlock()
> 	if e.pendingCreation == nil {
> 		e.pendingCreation = map[string]*agonesv1.GameServer{}
> 	}
> 	e.pendingCreation[gs.Name] = gs.DeepCopy()


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `gameServerSetCacheEntry` (L25), `created` (L31), `deleted` (L40), `reconcileWithUpdatedServerList` (L59), `gameServerStateCache` (L107), `forGameServerSet` (L111), `deleteGameServerSet` (L116). The full declaration list is recorded under `agones:pkg/gameserversets/gameserver_state_cache.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-94289c663d37896c"></a>


### File `pkg/gameserversets/gameserversets.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5dfe518eb4cabbbbda4c161e52713967bd367bdb`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserversets/gameserversets.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-550ade878f3b540484.** [agones/pkg/gameserversets/gameserversets.go, lines 32–46](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserversets/gameserversets.go#L32-L46). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `dae4e403864780f2bb027f5360363da20f57f460d2ef819de68aca30802d6c3f`.

> 
> func loggerForGameServerSetKey(log *logrus.Entry, key string) *logrus.Entry {
> 	return logfields.AugmentLogEntry(log, logfields.GameServerSetKey, key)
> }
> 
> func loggerForGameServerSet(log *logrus.Entry, gsSet *agonesv1.GameServerSet) *logrus.Entry {
> 	gsSetName := "NilGameServerSet"
> 	if gsSet != nil {
> 		gsSetName = gsSet.Namespace + "/" + gsSet.Name
> 	}
> 	return loggerForGameServerSetKey(log, gsSetName).WithField("gss", gsSet)
> }
> 
> // SortGameServersByStrategy will sort by least full nodes when Packed, and newest first when Distributed
> func SortGameServersByStrategy(strategy apis.SchedulingStrategy, list []*agonesv1.GameServer, counts map[string]gameservers.NodeCount, priorities []agonesv1.Priority) []*agonesv1.GameServer {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `errs` (L31), `loggerForGameServerSetKey` (L33), `loggerForGameServerSet` (L37), `SortGameServersByStrategy` (L46), `sortGameServersByPackedStrategy` (L55), `sortGameServersByDistributedStrategy` (L106), `ListGameServersByGameServerSetOwner` (L122). The full declaration list is recorded under `agones:pkg/gameserversets/gameserversets.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7111a218778b0213"></a>


### File `pkg/gameserversets/metrics.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d160e3687cfd9e2f855394348942668ec0184948`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserversets/metrics.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c888bb71b941ee5d40.** [agones/pkg/gameserversets/metrics.go, lines 51–65](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserversets/metrics.go#L51-L65). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f5894901e820b2ef46c15a74790cb0bd38b463d807a237ec7c3bf4973e545fcb`.

> 
> // register all our state views to OpenCensus
> func registerViews() {
> 	for _, v := range stateViews {
> 		if err := view.Register(v); err != nil {
> 			logger.WithError(err).Error("could not register view")
> 		}
> 	}
> }
> 
> // unregister views, this is only useful for tests as it trigger reporting.
> func unRegisterViews() {
> 	for _, v := range stateViews {
> 		view.Unregister(v)
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `registerViews` (L53), `unRegisterViews` (L62), `latencyTags` (L69), `metrics` (L75), `record` (L83), `mutate` (L88), `setError` (L97), `setRequest` (L102). The full declaration list is recorded under `agones:pkg/gameserversets/metrics.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/metrics`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

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

## Upstream platform contract reference

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-288b74c7e34d9adf8b.** [kubernetes-website/content/en/docs/concepts/architecture/controller.md, lines 9–22](https://github.com/kubernetes/website/blob/77904cb3ec33988ae7416fbf4b5f35baba407dc9/content/en/docs/concepts/architecture/controller.md#L9-L22). Revision `77904cb3ec33988ae7416fbf4b5f35baba407dc9`; source SHA-256 `dc74e2d9f6debb9e903bed3041b132534ed757fff0fd54677f502c80a987f323`.

> In robotics and automation, a _control loop_ is
> a non-terminating loop that regulates the state of a system.
> 
> Here is one example of a control loop: a thermostat in a room.
> 
> When you set the temperature, that's telling the thermostat
> about your *desired state*. The actual room temperature is the
> *current state*. The thermostat acts to bring the current state
> closer to the desired state, by turning equipment on or off.
> 
> {{< glossary_definition term_id="controller" length="short">}}
> 
> 

**Derived requirement.** Preserve resource/watch/reconciliation observations at the Rusternetes capability boundary. This source describes Kubernetes contracts; it does not authorize a second orchestrator integration or a pre-gate probe.
