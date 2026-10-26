<a id="m0023"></a>


# M0023 — Allocation processing and multi-cluster contracts


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0023-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0019](0019-gameserver-and-gameserverset-lifecycle.md), [M0017](0017-telemetry-and-metric-semantics.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-dd2032cbbfdf6faa47.** [agones/site/content/en/docs/Reference/gameserverallocation.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserverallocation.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `da063bba20d5be64ccf6be127fbd80b7fdfaa95bbbb2b8ab119a3eb72dda4ab8`.

> 
> 
> Allocation is the process of selecting the optimal `GameServer` that matches the filters defined in the `GameServerAllocation` specification below, and returning its details.
> 
> A successful Alloction moves the `GameServer` to the `Allocated` state, which indicates that it is currently active, likely with players on it, and should not be removed until SDK.Shutdown() is called, or it is explicitly manually deleted.
> 
> A full `GameServerAllocation` specification is available below and in the
> {{< ghlink href="/examples/gameserverallocation.yaml" >}}example folder{{< /ghlink >}} for reference:
> 
> 
> {{< tabpane persist="header" >}}
>   {{< tab header="selectors" lang="yaml" >}}
> apiVersion: "allocation.agones.dev/v1"


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Implement source allocation processing, converters, local selection, required/preferred selectors, priorities, counts/lists, patching and contention handling. Keep processor/converter dependencies in this cohesive milestone.

2. Retain multi-cluster policy and remote Agones protocol behavior, batching, timeouts, retries, response states and de-duplication. Remote allocation is not permission for another cluster orchestrator.

3. Bind allocation state changes and persistence to CGP providers with one resource owner. Derive repeatable request/projection/wiring structure; do not regenerate semantic policy from test outcomes.


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


## Package `agones:pkg/allocation/converters` -> module `pkg::allocation::converters`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-dd2032cbbfdf6faa47.** [agones/site/content/en/docs/Reference/gameserverallocation.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserverallocation.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `da063bba20d5be64ccf6be127fbd80b7fdfaa95bbbb2b8ab119a3eb72dda4ab8`.

> 
> 
> Allocation is the process of selecting the optimal `GameServer` that matches the filters defined in the `GameServerAllocation` specification below, and returning its details.
> 
> A successful Alloction moves the `GameServer` to the `Allocated` state, which indicates that it is currently active, likely with players on it, and should not be removed until SDK.Shutdown() is called, or it is explicitly manually deleted.
> 
> A full `GameServerAllocation` specification is available below and in the
> {{< ghlink href="/examples/gameserverallocation.yaml" >}}example folder{{< /ghlink >}} for reference:
> 
> 
> {{< tabpane persist="header" >}}
>   {{< tab header="selectors" lang="yaml" >}}
> apiVersion: "allocation.agones.dev/v1"


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-6f7b1bac69179e35"></a>


### File `pkg/allocation/converters/converter.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3e980606a545c12859e08e26409496f308b00147`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/allocation/converters/converter.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d704bc8f8b4534161f.** [agones/pkg/allocation/converters/converter.go, lines 30–44](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/allocation/converters/converter.go#L30-L44). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f6c54224721c07b5140d754c42adcab0fbe9cc0c3a4f17876afaf5014a5ccbce`.

> 
> // ConvertAllocationRequestToGSA converts AllocationRequest to GameServerAllocation V1 (GSA)
> func ConvertAllocationRequestToGSA(in *pb.AllocationRequest) *allocationv1.GameServerAllocation {
> 	if in == nil {
> 		return nil
> 	}
> 
> 	gsa := &allocationv1.GameServerAllocation{
> 		ObjectMeta: metav1.ObjectMeta{
> 			Namespace: in.GetNamespace(),
> 		},
> 		Spec: allocationv1.GameServerAllocationSpec{
> 			//nolint:staticcheck
> 			Preferred:  convertGameServerSelectorsToInternalGameServerSelectors(in.GetPreferredGameServerSelectors()),
> 			Selectors:  convertGameServerSelectorsToInternalGameServerSelectors(in.GetGameServerSelectors()),


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 31 lexically identified declarations, including `ConvertAllocationRequestToGSA` (L32), `ConvertGSAToAllocationRequest` (L91), `convertAllocationSchedulingToGSASchedulingStrategy` (L144), `convertGSASchedulingStrategyToAllocationScheduling` (L155), `convertLabelSelectorToInternalLabelSelector` (L165), `convertMatchExpressionsToInternal` (L172), `convertInternalMatchExpressions` (L194), `convertGameServerSelectorToInternalGameServerSelector` (L216), `convertInternalGameServerSelectorToGameServer` (L261), `convertInternalLabelSelectorToLabelSelector` (L304), `convertInternalLabelSelectorsToLabelSelectors` (L311), `convertGameServerSelectorsToInternalGameServerSelectors` (L320), `ConvertGSAToAllocationResponse` (L331), `convertGSACountersToAllocationCounters` (L361), `convertGSAListsToAllocationLists` (L373), `ConvertAllocationResponseToGSA` (L385), `convertGSAAddressesToAllocationAddresses` (L416), `convertAllocationAddressesToGSAAddresses` (L428), `convertGSAAgonesPortsToAllocationPorts` (L440), `convertAllocationPortsToGSAAgonesPorts` (L453), `convertGSAMetadataToAllocationMetadata` (L465), `convertAllocationMetadataToGSAMetadata` (L475), `convertAllocationCountersToGSACounters` (L485), `convertAllocationListsToGSALists` (L496). The full declaration list is recorded under `agones:pkg/allocation/converters/converter.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/allocation/go`, `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/apis/allocation/v1`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/gameserverallocations` -> module `pkg::gameserverallocations`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-23de89694ab2272646.** [agones/pkg/gameserverallocations/doc.go, lines 15–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/doc.go#L15-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `65fa1df337567438c1d03a3d642487567b03f0e96e18ca9e14f8bf8f17a720d2`.

> // Package gameserverallocations handles management of
> // allocating Ready GameServers via selectors
> package gameserverallocations


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-9e519b7078ca4764"></a>


### File `pkg/gameserverallocations/allocation_cache.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `34bf898ac68ba5bd47a16449a97378db5485404b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/allocation_cache.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-421f97c13a9516e92d.** [agones/pkg/gameserverallocations/allocation_cache.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/allocation_cache.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `23e4eb2cda7e47e19cd6b07b13105d1564ad9aa90a92b16b41d3dcd9e0be11ff`.

> 
> type matcher func(*agonesv1.GameServer) bool
> 
> // readyOrAllocatedGameServerMatcher returns true when a GameServer is in a Ready or Allocated state.
> func readyOrAllocatedGameServerMatcher(gs *agonesv1.GameServer) bool {
> 	return gs.Status.State == agonesv1.GameServerStateReady || gs.Status.State == agonesv1.GameServerStateAllocated
> }
> 
> // AllocationCache maintains a cache of GameServers that could potentially be allocated.
> type AllocationCache struct {
> 	baseLogger       *logrus.Entry
> 	cache            gameServerCache
> 	gameServerLister listerv1.GameServerLister
> 	gameServerSynced cache.InformerSynced
> 	workerqueue      *workerqueue.WorkerQueue


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 16 lexically identified declarations, including `matcher` (L37), `readyOrAllocatedGameServerMatcher` (L40), `AllocationCache` (L45), `NewAllocationCache` (L57), `loggerForGameServerKey` (L106), `RemoveGameServer` (L111), `Sync` (L120), `Resync` (L131), `Run` (L137), `AddGameServer` (L148), `getGameServers` (L155), `ListSortedGameServers` (L171), `ListSortedGameServersPriorities` (L229), `SyncGameServers` (L254), `syncCache` (L261), `getKey` (L306). The full declaration list is recorded under `agones:pkg/gameserverallocations/allocation_cache.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/apis/allocation/v1`, `agones.dev/agones/pkg/client/informers/externalversions/agones/v1`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0342e76f4982647d"></a>


### File `pkg/gameserverallocations/allocator.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `79175c078c40522d1f983ff2640cf7883c56d794`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/allocator.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-baa53b02454d224f12.** [agones/pkg/gameserverallocations/allocator.go, lines 100–114](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/allocator.go#L100-L114). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `335148f5dbd4da4c9eb964e1341239239cf2534373a5740f18f7fd8ccfeed43d`.

> 
> // Allocator handles game server allocation
> //
> //nolint:govet // fieldalignment: struct alignment is not critical for our use case
> type Allocator struct {
> 	baseLogger                   *logrus.Entry
> 	allocationPolicyLister       multiclusterlisterv1.GameServerAllocationPolicyLister
> 	allocationPolicySynced       cache.InformerSynced
> 	secretLister                 corev1lister.SecretLister
> 	secretSynced                 cache.InformerSynced
> 	gameServerGetter             getterv1.GameServersGetter
> 	recorder                     record.EventRecorder
> 	pendingRequests              chan request
> 	allocationCache              *AllocationCache
> 	remoteAllocationCallback     func(context.Context, string, grpc.DialOption, *pb.AllocationRequest) (*pb.AllocationResponse, error)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 24 lexically identified declarations, including `allocationRetry` (L88), `remoteAllocationRetry` (L95), `Allocator` (L104), `request` (L123), `response` (L130), `NewAllocator` (L137), `Run` (L177), `Sync` (L193), `Allocate` (L202), `loggerForGameServerAllocationKey` (L250), `loggerForGameServerAllocation` (L254), `allocateFromLocalCluster` (L264), `applyMultiClusterAllocation` (L316), `allocateFromRemoteCluster` (L364), `createRemoteClusterDialOption` (L415), `getClientCertificates` (L449), `allocate` (L467), `ListenAndAllocate` (L491), `allocationUpdateWorkers` (L607), `tryRespondWithError` (L643), `applyAllocationToGameServer` (L652), `Retry` (L708), `newMetrics` (L737), `addPort` (L750). The full declaration list is recorded under `agones:pkg/gameserverallocations/allocator.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/allocation/converters`, `agones.dev/agones/pkg/allocation/go`, `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/apis/allocation/v1`, `agones.dev/agones/pkg/apis/multicluster/v1`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions/multicluster/v1`, `agones.dev/agones/pkg/client/listers/multicluster/v1`, `agones.dev/agones/pkg/util/apiserver`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.

7. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

8. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-40f6160c05749b65"></a>


### File `pkg/gameserverallocations/cache.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ca20f665e1d7bc22f68e38db494813fcf8464878`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/cache.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a0cbe1988be7518478.** [agones/pkg/gameserverallocations/cache.go, lines 22–36](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/cache.go#L22-L36). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6b562bc0b79c51c0bafda5f5895d0afffb54dbb6cfc3a0f37f538bdf9b27d18d`.

> 
> // gameserver cache to keep the Ready state gameserver.
> //
> //nolint:govet // ignore fieldalignment, singleton embedded in AllocationCache
> type gameServerCache struct {
> 	mu    sync.RWMutex
> 	cache map[string]*agonesv1.GameServer
> }
> 
> // Store saves the data in the cache.
> // If there is already a value for this key, the incoming gs object is ignored
> // if its Generation metadata is less than the existing one, or if its Generation
> // is equal and its ResourceVersion is the same as the existing one.
> // Otherwise, the incoming gs object replaces the existing one.
> func (e *gameServerCache) Store(key string, gs *agonesv1.GameServer) {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `gameServerCache` (L26), `Store` (L36), `Delete` (L56), `Load` (L71), `Range` (L80), `Len` (L91). The full declaration list is recorded under `agones:pkg/gameserverallocations/cache.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0c64a72221440b66"></a>


### File `pkg/gameserverallocations/controller.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `329c6a38733bcb639a12c3f92ef138442d8ee17f`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/controller.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8505c4a17188d52530.** [agones/pkg/gameserverallocations/controller.go, lines 52–66](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/controller.go#L52-L66). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2ecca2b63caa4ed9e4204ebc7a4c4ce3e2ca9b1b44a9ed865df9c73959012727`.

> 
> func init() {
> 	registerViews()
> }
> 
> // Extensions is a GameServerAllocation controller within the Extensions service
> type Extensions struct {
> 	api             *apiserver.APIServer
> 	baseLogger      *logrus.Entry
> 	recorder        record.EventRecorder
> 	allocator       *Allocator
> 	processorClient processor.Client
> 	errs            *errors.Errors
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 11 lexically identified declarations, including `init` (L53), `Extensions` (L58), `NewExtensions` (L68), `NewProcessorExtensions` (L107), `registerAPIResource` (L125), `Run` (L143), `processAllocationRequest` (L155), `allocationDeserialization` (L215), `serialisation` (L255), `convertProcessorError` (L271), `convertProcessorResponse` (L300). The full declaration list is recorded under `agones:pkg/gameserverallocations/controller.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/allocation/converters`, `agones.dev/agones/pkg/allocation/go`, `agones.dev/agones/pkg/apis/allocation/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/gameserverallocations/processor`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/util/apiserver`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/https`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-25c8fcc80240dcf6"></a>


### File `pkg/gameserverallocations/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `aa7c40a5fd447737ff6e10d047d02602dc9cd83b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c49956acdaa9531f4e.** [agones/pkg/gameserverallocations/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `65fa1df337567438c1d03a3d642487567b03f0e96e18ca9e14f8bf8f17a720d2`.

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
> // Package gameserverallocations handles management of


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9408441aaa90bd84"></a>


### File `pkg/gameserverallocations/find.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0c5b96f1b5239b88a101610949426e70f3be9bd5`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/find.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ffd5604c530066a073.** [agones/pkg/gameserverallocations/find.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/find.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a9dd1c3886f6a1ccce1ed98eb9aba80c0f4c281b415960bd86c5226b177c6111`.

> 
> // findGameServerForAllocation finds an optimal gameserver, given the
> // set of preferred and required selectors on the GameServerAllocation. This also returns the index
> // that the gameserver was found at in `list`, in case you want to remove it from the list
> // Packed: will search list from start to finish
> // Distributed: will search in a random order through the list
> // It is assumed that all gameservers passed in, are Ready and not being deleted, and are sorted in Packed priority order
> func findGameServerForAllocation(gsa *allocationv1.GameServerAllocation, list []*agonesv1.GameServer) (*agonesv1.GameServer, int, error) {
> 	type result struct {
> 		gs    *agonesv1.GameServer
> 		index int
> 	}
> 
> 	selectors := make([]*result, len(gsa.Spec.Selectors))


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `findGameServerForAllocation` (L31). The full declaration list is recorded under `agones:pkg/gameserverallocations/find.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/apis/allocation/v1`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e903f3cc4e372875"></a>


### File `pkg/gameserverallocations/gameserverallocations.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4564ba538accaebaf678effeae37f1876171fe8f`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/gameserverallocations.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-36b166ea41cae9fc6b.** [agones/pkg/gameserverallocations/gameserverallocations.go, lines 18–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/gameserverallocations.go#L18-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7037de799377f7cd50e30185c2d6684d9f95608bb1a4f0648082cd81272cb7f2`.

> 
> var errs = errors.FromPackage()


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `errs` (L19). The full declaration list is recorded under `agones:pkg/gameserverallocations/gameserverallocations.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1c76fae58c62ce38"></a>


### File `pkg/gameserverallocations/metrics.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `828d976b68a198ff7bd89130e1f19715d400b4b5`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/metrics.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b49df552e41e342b3c.** [agones/pkg/gameserverallocations/metrics.go, lines 63–77](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/metrics.go#L63-L77). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b5378a47eeff9d58b6f3f69dbadf0582e214cb6887637264d11b59dbbbeac489`.

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

2. Account for 11 lexically identified declarations, including `registerViews` (L65), `unRegisterViews` (L74), `latencyTags` (L81), `metrics` (L89), `mutate` (L97), `setStatus` (L106), `setError` (L111), `setRequest` (L116), `setResponse` (L126), `record` (L149), `recordAllocationRetrySuccess` (L154). The full declaration list is recorded under `agones:pkg/gameserverallocations/metrics.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/apis/allocation/v1`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/metrics`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Preserve source floating width, narrowing, exceptional values and relevant bits.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/gameserverallocations/processor` -> module `pkg::gameserverallocations::processor`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-0be8162c0b7761757c.** [agones/pkg/gameserverallocations/processor/doc.go, lines 15–26](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/processor/doc.go#L15-L26). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4dcde4035723da2ada0dd43ccf6c7b2abb456dfca56514bd71b0ff8ca3de2373`.

> // Package processor provides client functionality for the Agones allocation processor system
> //
> // The allocation processor system enables batched game server allocation requests
> // This package contains the client implementation that connects to the processor service
> // batches allocation requests, and handles the stream-based communication protocol
> //
> // Key components:
> // - Client: Manages connection lifecycle and request batching
> // - Config: Configuration for processor client behavior
> // - Batch handling: Accumulates requests and sends them in batches to the processor
> //
> // The client establishes a bidirectional gRPC stream with the processor service,


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-f2450964992fb484"></a>


### File `pkg/gameserverallocations/processor/client.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0cc62978a1b92e787c80c81e973cfea1203956e9`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/processor/client.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-bc9534ae00de31be43.** [agones/pkg/gameserverallocations/processor/client.go, lines 49–63](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/processor/client.go#L49-L63). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7878f6a8d9a27357e2ef7d5ea0b687bbe17d0881f3b5600b6db241afc894e7e1`.

> 
> // Config holds the processor client configuration
> type Config struct {
> 	// ClientID is a unique identifier for this processor client instance
> 	ClientID string
> 
> 	// ProcessorAddress specifies the address of the processor service to connect to
> 	ProcessorAddress string
> 
> 	// MaxBatchSize determines the maximum number of allocation requests to batch together
> 	MaxBatchSize int
> 
> 	// AllocationTimeout is the maximum duration to wait for an allocation response
> 	AllocationTimeout time.Duration


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 18 lexically identified declarations, including `maxSendRetries` (L36), `Config` (L51), `client` (L71), `pendingRequest` (L90), `Client` (L109), `NewClient` (L118), `Run` (L138), `Allocate` (L168), `handleStream` (L221), `pullRequestHandler` (L264), `handlePullRequest` (L281), `sendBatch` (L319), `handleBatchResponse` (L367), `connectAndRun` (L483), `connect` (L513), `healthCheck` (L537), `registerClient` (L560), `drainPendingRequests` (L578). The full declaration list is recorded under `agones:pkg/gameserverallocations/processor/client.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/allocation/go`, `agones.dev/agones/pkg/util/errors`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.

7. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

8. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1a500bc872c8ab0c"></a>


### File `pkg/gameserverallocations/processor/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3841a7cc0c5979c94932bcecf80ee47968f069fe`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/processor/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4f692e66789ffb4321.** [agones/pkg/gameserverallocations/processor/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/processor/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4dcde4035723da2ada0dd43ccf6c7b2abb456dfca56514bd71b0ff8ca3de2373`.

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
> // Package processor provides client functionality for the Agones allocation processor system


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2742bbe2d534ae2d"></a>


### File `pkg/gameserverallocations/processor/grpc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ef0f52fb60b2b75d21fb154d4bf494dedf716231`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/processor/grpc.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a48d3266be7a85a5fc.** [agones/pkg/gameserverallocations/processor/grpc.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/processor/grpc.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2aaa241779a351119fc07f0c5cab308bc700fa14716ebe0b78fa594cd290d415`.

> 
> // GRPCCodeFromHTTPStatus converts an HTTP status code to the corresponding gRPC status code
> func GRPCCodeFromHTTPStatus(httpStatusCode int) codes.Code {
> 	switch httpStatusCode {
> 	case http.StatusOK:
> 		return codes.OK
> 	case 499:
> 		return codes.Canceled
> 	case http.StatusInternalServerError:
> 		return codes.Internal
> 	case http.StatusBadRequest:
> 		return codes.InvalidArgument
> 	case http.StatusGatewayTimeout:
> 		return codes.DeadlineExceeded
> 	case http.StatusNotFound:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 2 lexically identified declarations, including `grpcLogger` (L24), `GRPCCodeFromHTTPStatus` (L27). The full declaration list is recorded under `agones:pkg/gameserverallocations/processor/grpc.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5cb75532fc0672ff"></a>


### File `pkg/gameserverallocations/processor/handler.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `97f90f4328d153d6979beeb535336f9fc3957d4e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/gameserverallocations/processor/handler.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-cf70a6cd84b37cce52.** [agones/pkg/gameserverallocations/processor/handler.go, lines 44–58](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/gameserverallocations/processor/handler.go#L44-L58). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `475a85349312ed58c54a028bd1a72a6f3318ee7fe59da31539d096aaea3cbdec`.

> 
> // GameServerAllocator represents the interface to allocate game servers.
> type GameServerAllocator interface {
> 	Allocate(ctx context.Context, gsa *allocationv1.GameServerAllocation) (k8sruntime.Object, error)
> }
> 
> // allocationResult represents the result of an allocation attempt.
> type allocationResult struct {
> 	response *allocationpb.AllocationResponse
> 	error    *rpcstatus.Status
> }
> 
> // Handler is the gRPC server for processing allocation requests.
> type Handler struct {
> 	allocationpb.UnimplementedProcessorServer


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 14 lexically identified declarations, including `handlerLogger` (L43), `GameServerAllocator` (L46), `allocationResult` (L51), `Handler` (L57), `NewServiceHandler` (L68), `StreamBatches` (L80), `StartPullRequestTicker` (L156), `sendPullRequestsToClients` (L174), `processAllocationsConcurrently` (L198), `processAllocation` (L214), `submitBatch` (L263), `GetGRPCServerOptions` (L291), `addClient` (L310), `removeClient` (L318). The full declaration list is recorded under `agones:pkg/gameserverallocations/processor/handler.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/allocation/converters`, `agones.dev/agones/pkg/allocation/go`, `agones.dev/agones/pkg/apis/allocation/v1`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

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
