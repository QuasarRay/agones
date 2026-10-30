<a id="m0007"></a>


# M0007 — Apply, clients, informers and listers


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0007-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md)


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


1. Regenerate the source apply-configuration family with exact resource/version/package membership. Preserve fluent builder field-presence, append/replace behavior, Agones-required presence distinctions expressed through native Rust types, metadata and extraction logic.

2. Keep each source file and dotted/generated basename in the mirror. Derive templates from canonical API declarations rather than hand-maintaining repeated field setters.

3. Make the Python-facing reading order clear: construct configuration, select fields, then submit through a CGP effect capability. Do not introduce a runtime provider registry or a second writable representation.

4. Port versioned clientset, scheme and typed-resource request construction while retaining the source package hierarchy. Preserve group/version, verb, resource, namespace, query, timeout, patch and watch behavior.

5. Bind concrete transport/storage to Rusternetes only; Kubernetes-shaped public contracts are compatibility data, not permission to add another orchestrator adapter.

6. Generate repeated forwarding with derive-deftly under CGP capabilities. Keep fake client source accounted for; any fake used solely by harnesses is deferred assurance support, never a production substitute.

7. Implement informer factories, version/resource lookup, listers and cache-index contracts with the original namespace/selector behavior and exact package/file names.

8. Preserve list/watch handoff, resync/reconnect, ordering, duplicate delivery, cache synchronization and stop behavior. The production context must not provide stronger consistency than the source contract.

9. CGP owns watch/queue scheduling, cancellation and resources; derive-deftly emits repetitive resource wiring. Keep the read snapshot separate from the unique mutable backend owner.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.




## Architecture, readability and authored outputs


1. Own 88 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:pkg/client/applyconfiguration` -> module `pkg::client::applyconfiguration`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-aa25adc01d7c0630"></a>


### File `pkg/client/applyconfiguration/utils.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4ebe650b624aa326e9ca68d4b1da714a18cda5d8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/utils.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c5e5c5bc8d54cebf66.** [agones/pkg/client/applyconfiguration/utils.go, lines 33–47](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/utils.go#L33-L47). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `80faa4b0351289204b48f1a25b12f53a0b9c6b021366dc943c6b7d626183f30d`.

> 
> // ForKind returns an apply configuration type for the given GroupVersionKind, or nil if no
> // apply configuration type exists for the given GroupVersionKind.
> func ForKind(kind schema.GroupVersionKind) interface{} {
> 	switch kind {
> 	// Group=agones.dev, Version=v1
> 	case v1.SchemeGroupVersion.WithKind("AggregatedCounterStatus"):
> 		return &agonesv1.AggregatedCounterStatusApplyConfiguration{}
> 	case v1.SchemeGroupVersion.WithKind("AggregatedListStatus"):
> 		return &agonesv1.AggregatedListStatusApplyConfiguration{}
> 	case v1.SchemeGroupVersion.WithKind("AggregatedPlayerStatus"):
> 		return &agonesv1.AggregatedPlayerStatusApplyConfiguration{}
> 	case v1.SchemeGroupVersion.WithKind("AllocationOverflow"):
> 		return &agonesv1.AllocationOverflowApplyConfiguration{}
> 	case v1.SchemeGroupVersion.WithKind("CounterStatus"):


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 2 lexically identified declarations, including `ForKind` (L36), `NewTypeConverter` (L134). The complete list remains in `agones:pkg/client/applyconfiguration/utils.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/applyconfiguration/agones/v1` -> module `pkg::client::applyconfiguration::agones::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-44f8f67d94baa492"></a>


### File `pkg/client/applyconfiguration/agones/v1/aggregatedcounterstatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4f605840c510a8671f05344f76a0da73d88fbad0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/aggregatedcounterstatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b67b729a1e91e0e077.** [agones/pkg/client/applyconfiguration/agones/v1/aggregatedcounterstatus.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/aggregatedcounterstatus.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4dec4a6cdefb5e4502359ac8914323ad44a06dde94c32014002c1ff7a1a935ec`.

> 
> // AggregatedCounterStatusApplyConfiguration represents a declarative configuration of the AggregatedCounterStatus type for use
> // with apply.
> //
> // AggregatedCounterStatus stores total and allocated Counter tracking values
> type AggregatedCounterStatusApplyConfiguration struct {
> 	AllocatedCount    *int64 `json:"allocatedCount,omitempty"`
> 	AllocatedCapacity *int64 `json:"allocatedCapacity,omitempty"`
> 	Count             *int64 `json:"count,omitempty"`
> 	Capacity          *int64 `json:"capacity,omitempty"`
> }
> 
> // AggregatedCounterStatusApplyConfiguration constructs a declarative configuration of the AggregatedCounterStatus type for use with
> // apply.
> func AggregatedCounterStatus() *AggregatedCounterStatusApplyConfiguration {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `AggregatedCounterStatusApplyConfiguration` (L25), `AggregatedCounterStatus` (L34), `WithAllocatedCount` (L41), `WithAllocatedCapacity` (L49), `WithCount` (L57), `WithCapacity` (L65). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/aggregatedcounterstatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-47a3abc38deb141a"></a>


### File `pkg/client/applyconfiguration/agones/v1/aggregatedliststatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c469ac1a2a8f7269df449969871b7460711c07ed`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/aggregatedliststatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-dcb2d4cf8aac0e7987.** [agones/pkg/client/applyconfiguration/agones/v1/aggregatedliststatus.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/aggregatedliststatus.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8712ea9bff6b70cfc62dbc067134c2d52c8a53775618c2c9b36caaca6b45572b`.

> 
> // AggregatedListStatusApplyConfiguration represents a declarative configuration of the AggregatedListStatus type for use
> // with apply.
> //
> // AggregatedListStatus stores total and allocated List tracking values
> type AggregatedListStatusApplyConfiguration struct {
> 	AllocatedCount    *int64 `json:"allocatedCount,omitempty"`
> 	AllocatedCapacity *int64 `json:"allocatedCapacity,omitempty"`
> 	Count             *int64 `json:"count,omitempty"`
> 	Capacity          *int64 `json:"capacity,omitempty"`
> }
> 
> // AggregatedListStatusApplyConfiguration constructs a declarative configuration of the AggregatedListStatus type for use with
> // apply.
> func AggregatedListStatus() *AggregatedListStatusApplyConfiguration {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `AggregatedListStatusApplyConfiguration` (L25), `AggregatedListStatus` (L34), `WithAllocatedCount` (L41), `WithAllocatedCapacity` (L49), `WithCount` (L57), `WithCapacity` (L65). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/aggregatedliststatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-ee98bd40016ad984"></a>


### File `pkg/client/applyconfiguration/agones/v1/aggregatedplayerstatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `48bc171a741b472e4d0a23fd61c6a42009bb18ff`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/aggregatedplayerstatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b6c3d970433a19d7f9.** [agones/pkg/client/applyconfiguration/agones/v1/aggregatedplayerstatus.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/aggregatedplayerstatus.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `da05d6c610dfce7ca61166854425999ce8d0f449500f415669d0a313e5b9ef05`.

> 
> // AggregatedPlayerStatusApplyConfiguration represents a declarative configuration of the AggregatedPlayerStatus type for use
> // with apply.
> //
> // AggregatedPlayerStatus stores total player tracking values
> type AggregatedPlayerStatusApplyConfiguration struct {
> 	Count    *int64 `json:"count,omitempty"`
> 	Capacity *int64 `json:"capacity,omitempty"`
> }
> 
> // AggregatedPlayerStatusApplyConfiguration constructs a declarative configuration of the AggregatedPlayerStatus type for use with
> // apply.
> func AggregatedPlayerStatus() *AggregatedPlayerStatusApplyConfiguration {
> 	return &AggregatedPlayerStatusApplyConfiguration{}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `AggregatedPlayerStatusApplyConfiguration` (L25), `AggregatedPlayerStatus` (L32), `WithCount` (L39), `WithCapacity` (L47). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/aggregatedplayerstatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-f1a1d614d89f76f7"></a>


### File `pkg/client/applyconfiguration/agones/v1/allocationoverflow.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3f220926eaaaf062b6c75fc0a52ce6bf4fb198d4`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/allocationoverflow.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ce859c610a7185f22a.** [agones/pkg/client/applyconfiguration/agones/v1/allocationoverflow.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/allocationoverflow.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `62d526d23eeceadb5df9f7f19e225fedf42b86ee41a55dcf28fd812f3e802346`.

> 
> // AllocationOverflowApplyConfiguration represents a declarative configuration of the AllocationOverflow type for use
> // with apply.
> //
> // AllocationOverflow specifies what labels and/or annotations to apply on Allocated GameServers
> // if the desired number of the underlying `GameServerSet` drops below the number of Allocated GameServers
> // attached to it.
> type AllocationOverflowApplyConfiguration struct {
> 	// Labels to be applied to the `GameServer`
> 	Labels map[string]string `json:"labels,omitempty"`
> 	// Annotations to be applied to the `GameServer`
> 	Annotations map[string]string `json:"annotations,omitempty"`
> }
> 
> // AllocationOverflowApplyConfiguration constructs a declarative configuration of the AllocationOverflow type for use with


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `AllocationOverflowApplyConfiguration` (L27), `AllocationOverflow` (L36), `WithLabels` (L44), `WithAnnotations` (L58). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/allocationoverflow.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-12b3b1bdc8ba3b8d"></a>


### File `pkg/client/applyconfiguration/agones/v1/counterstatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `eb620114d648ac22c2c88cea28bb25e6a87e38d0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/counterstatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0784d279d6f1fa15ff.** [agones/pkg/client/applyconfiguration/agones/v1/counterstatus.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/counterstatus.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `091359678c8d3078b3af28b0b34f134caf3431d8be2ea0f1217e0216e6056932`.

> 
> // CounterStatusApplyConfiguration represents a declarative configuration of the CounterStatus type for use
> // with apply.
> //
> // CounterStatus stores the current counter values and maximum capacity
> type CounterStatusApplyConfiguration struct {
> 	Count    *int64 `json:"count,omitempty"`
> 	Capacity *int64 `json:"capacity,omitempty"`
> }
> 
> // CounterStatusApplyConfiguration constructs a declarative configuration of the CounterStatus type for use with
> // apply.
> func CounterStatus() *CounterStatusApplyConfiguration {
> 	return &CounterStatusApplyConfiguration{}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `CounterStatusApplyConfiguration` (L25), `CounterStatus` (L32), `WithCount` (L39), `WithCapacity` (L47). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/counterstatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-e2e6ddfaf8430279"></a>


### File `pkg/client/applyconfiguration/agones/v1/eviction.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5a064fa170a37ea53bf1f6424dbcbb9dbd9a0a75`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/eviction.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-6f625b423b8ab0a5bf.** [agones/pkg/client/applyconfiguration/agones/v1/eviction.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/eviction.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2af33f911b9e53a63317227e5217389e2225531301063b30cdc0d52167d3d57b`.

> 
> // EvictionApplyConfiguration represents a declarative configuration of the Eviction type for use
> // with apply.
> //
> // Eviction specifies the eviction tolerance of the GameServer
> type EvictionApplyConfiguration struct {
> 	// Game server supports termination via SIGTERM:
> 	// - Always: Allow eviction for both Cluster Autoscaler and node drain for upgrades
> 	// - OnUpgrade: Allow eviction for upgrades alone
> 	// - Never (default): Pod should run to completion
> 	Safe *agonesv1.EvictionSafe `json:"safe,omitempty"`
> }
> 
> // EvictionApplyConfiguration constructs a declarative configuration of the Eviction type for use with
> // apply.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 3 lexically identified declarations, including `EvictionApplyConfiguration` (L29), `Eviction` (L39), `WithSafe` (L46). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/eviction.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-857c1b4ed25d338b"></a>


### File `pkg/client/applyconfiguration/agones/v1/fleet.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `281a8535b5de859ea2e3671069137617c08c31e9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/fleet.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a48b8f589f640ec19c.** [agones/pkg/client/applyconfiguration/agones/v1/fleet.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/fleet.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `19b284cde3980c654463a054887b05e69f8bc803fd2ffe7e72157d30e7f6801d`.

> 
> // FleetApplyConfiguration represents a declarative configuration of the Fleet type for use
> // with apply.
> //
> // Fleet is the data structure for a Fleet resource
> type FleetApplyConfiguration struct {
> 	metav1.TypeMetaApplyConfiguration    `json:",inline"`
> 	*metav1.ObjectMetaApplyConfiguration `json:"metadata,omitempty"`
> 	Spec                                 *FleetSpecApplyConfiguration   `json:"spec,omitempty"`
> 	Status                               *FleetStatusApplyConfiguration `json:"status,omitempty"`
> }
> 
> // Fleet constructs a declarative configuration of the Fleet type for use with
> // apply.
> func Fleet(name, namespace string) *FleetApplyConfiguration {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 25 lexically identified declarations, including `FleetApplyConfiguration` (L31), `Fleet` (L40), `IsApplyConfiguration` (L49), `WithKind` (L54), `WithAPIVersion` (L62), `WithName` (L70), `WithGenerateName` (L79), `WithNamespace` (L88), `WithUID` (L97), `WithResourceVersion` (L106), `WithGeneration` (L115), `WithCreationTimestamp` (L124), `WithDeletionTimestamp` (L133), `WithDeletionGracePeriodSeconds` (L142), `WithLabels` (L152), `WithAnnotations` (L167), `WithOwnerReferences` (L181), `WithFinalizers` (L195), `ensureObjectMetaApplyConfigurationExists` (L203), `WithSpec` (L212), `WithStatus` (L220), `GetKind` (L226), `GetAPIVersion` (L231), `GetName` (L236). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/fleet.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-00f40a23e9249a69"></a>


### File `pkg/client/applyconfiguration/agones/v1/fleetspec.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `570b31de66e0cad27522531b6c1a809f24df19d7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/fleetspec.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-72384f4304988cb687.** [agones/pkg/client/applyconfiguration/agones/v1/fleetspec.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/fleetspec.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `36ec4fd16bdae68c6424bddbcccf46e919bd099d82034ae79908117a29d8912d`.

> 
> // FleetSpecApplyConfiguration represents a declarative configuration of the FleetSpec type for use
> // with apply.
> //
> // FleetSpec is the spec for a Fleet
> type FleetSpecApplyConfiguration struct {
> 	// Replicas are the number of GameServers that should be in this set. Defaults to 0.
> 	Replicas *int32 `json:"replicas,omitempty"`
> 	// Labels and/or Annotations to apply to overflowing GameServers when the number of Allocated GameServers is more
> 	// than the desired replicas on the underlying `GameServerSet`
> 	AllocationOverflow *AllocationOverflowApplyConfiguration `json:"allocationOverflow,omitempty"`
> 	// Deployment strategy
> 	Strategy *appsv1.DeploymentStrategy `json:"strategy,omitempty"`
> 	// Scheduling strategy. Defaults to "Packed".
> 	Scheduling *apis.SchedulingStrategy `json:"scheduling,omitempty"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 8 lexically identified declarations, including `FleetSpecApplyConfiguration` (L30), `FleetSpec` (L58), `WithReplicas` (L65), `WithAllocationOverflow` (L73), `WithStrategy` (L81), `WithScheduling` (L89), `WithPriorities` (L97), `WithTemplate` (L110). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/fleetspec.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-2fb900c4e9a9f9a9"></a>


### File `pkg/client/applyconfiguration/agones/v1/fleetstatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c8559692f99e6bc981c3f9cc4715532edeb78f95`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/fleetstatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0cac1e6fa984f5ce15.** [agones/pkg/client/applyconfiguration/agones/v1/fleetstatus.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/fleetstatus.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `23adddba803015b9ac64d8ca2bb8c7b39b4efe37bde6fa175492a516be4af6a3`.

> 
> // FleetStatusApplyConfiguration represents a declarative configuration of the FleetStatus type for use
> // with apply.
> //
> // FleetStatus is the status of a Fleet
> type FleetStatusApplyConfiguration struct {
> 	// Replicas the total number of current GameServer replicas
> 	Replicas *int32 `json:"replicas,omitempty"`
> 	// ReadyReplicas are the number of Ready GameServer replicas
> 	ReadyReplicas *int32 `json:"readyReplicas,omitempty"`
> 	// ReservedReplicas are the total number of Reserved GameServer replicas in this fleet.
> 	// Reserved instances won't be deleted on scale down, but won't cause an autoscaler to scale up.
> 	ReservedReplicas *int32 `json:"reservedReplicas,omitempty"`
> 	// AllocatedReplicas are the number of Allocated GameServer replicas
> 	AllocatedReplicas *int32 `json:"allocatedReplicas,omitempty"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 10 lexically identified declarations, including `FleetStatusApplyConfiguration` (L25), `FleetStatus` (L51), `WithReplicas` (L58), `WithReadyReplicas` (L66), `WithReservedReplicas` (L74), `WithAllocatedReplicas` (L82), `WithAllocations` (L90), `WithPlayers` (L98), `WithCounters` (L107), `WithLists` (L121). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/fleetstatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-76e0a4d8af1bdc10"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameserver.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `13c769b154b74211400a0076d36c9d641b067c7f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameserver.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d91b5972e8a3e39cab.** [agones/pkg/client/applyconfiguration/agones/v1/gameserver.go, lines 26–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameserver.go#L26-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e33eee7cdbc610b051a88a3a65e10c2692122e5ed2b57070e95c5cd464c422e9`.

> 
> // GameServerApplyConfiguration represents a declarative configuration of the GameServer type for use
> // with apply.
> //
> // GameServer is the data structure for a GameServer resource.
> // It is worth noting that while there is a `GameServerStatus` Status entry for the `GameServer`, it is not
> // defined as a subresource - unlike `Fleet` and other Agones resources.
> // This is so that we can retain the ability to change multiple aspects of a `GameServer` in a single atomic operation,
> // which is particularly useful for operations such as allocation.
> type GameServerApplyConfiguration struct {
> 	metav1.TypeMetaApplyConfiguration    `json:",inline"`
> 	*metav1.ObjectMetaApplyConfiguration `json:"metadata,omitempty"`
> 	Spec                                 *GameServerSpecApplyConfiguration   `json:"spec,omitempty"`
> 	Status                               *GameServerStatusApplyConfiguration `json:"status,omitempty"`
> }
> 
> // GameServer constructs a declarative configuration of the GameServer type for use with


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 25 lexically identified declarations, including `GameServerApplyConfiguration` (L35), `GameServer` (L44), `IsApplyConfiguration` (L53), `WithKind` (L58), `WithAPIVersion` (L66), `WithName` (L74), `WithGenerateName` (L83), `WithNamespace` (L92), `WithUID` (L101), `WithResourceVersion` (L110), `WithGeneration` (L119), `WithCreationTimestamp` (L128), `WithDeletionTimestamp` (L137), `WithDeletionGracePeriodSeconds` (L146), `WithLabels` (L156), `WithAnnotations` (L171), `WithOwnerReferences` (L185), `WithFinalizers` (L199), `ensureObjectMetaApplyConfigurationExists` (L207), `WithSpec` (L216), `WithStatus` (L224), `GetKind` (L230), `GetAPIVersion` (L235), `GetName` (L240). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameserver.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-2f908a74c80d227e"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameserverport.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e1f48beef57e1419b8c070b17292b346dfeac6b8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameserverport.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-9ffc082a6571cb1918.** [agones/pkg/client/applyconfiguration/agones/v1/gameserverport.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameserverport.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b891f6079289f00e4f1187e36f527a4288cb2a7eede682216216a1b0f9e78963`.

> 
> // GameServerPortApplyConfiguration represents a declarative configuration of the GameServerPort type for use
> // with apply.
> //
> // GameServerPort defines a set of Ports that
> // are to be exposed via the GameServer
> type GameServerPortApplyConfiguration struct {
> 	// Name is the descriptive name of the port
> 	Name *string `json:"name,omitempty"`
> 	// Range is the port range name from which to select a port when using a
> 	// 'Dynamic' or 'Passthrough' port policy.
> 	Range *string `json:"range,omitempty"`
> 	// PortPolicy defines the policy for how the HostPort is populated.
> 	// Dynamic port will allocate a HostPort within the selected MIN_PORT and MAX_PORT range passed to the controller
> 	// at installation time.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 9 lexically identified declarations, including `GameServerPortApplyConfiguration` (L31), `GameServerPort` (L57), `WithName` (L64), `WithRange` (L72), `WithPortPolicy` (L80), `WithContainer` (L88), `WithContainerPort` (L96), `WithHostPort` (L104), `WithProtocol` (L112). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameserverport.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-92c6eeb404ed0a16"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameserverset.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `91f7d045a2cb3323d17c2a7b39a82f85069f9f4b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameserverset.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-14e3d2f2a8332e1823.** [agones/pkg/client/applyconfiguration/agones/v1/gameserverset.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameserverset.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5bc3354a2333630f2dac92bb260f673ed4fe33b494dcc8b2e4e385716ad511a2`.

> 
> // GameServerSetApplyConfiguration represents a declarative configuration of the GameServerSet type for use
> // with apply.
> //
> // GameServerSet is the data structure for a set of GameServers.
> // This matches philosophically with the relationship between
> // Deployments and ReplicaSets
> type GameServerSetApplyConfiguration struct {
> 	metav1.TypeMetaApplyConfiguration    `json:",inline"`
> 	*metav1.ObjectMetaApplyConfiguration `json:"metadata,omitempty"`
> 	Spec                                 *GameServerSetSpecApplyConfiguration   `json:"spec,omitempty"`
> 	Status                               *GameServerSetStatusApplyConfiguration `json:"status,omitempty"`
> }
> 
> // GameServerSet constructs a declarative configuration of the GameServerSet type for use with


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 25 lexically identified declarations, including `GameServerSetApplyConfiguration` (L33), `GameServerSet` (L42), `IsApplyConfiguration` (L51), `WithKind` (L56), `WithAPIVersion` (L64), `WithName` (L72), `WithGenerateName` (L81), `WithNamespace` (L90), `WithUID` (L99), `WithResourceVersion` (L108), `WithGeneration` (L117), `WithCreationTimestamp` (L126), `WithDeletionTimestamp` (L135), `WithDeletionGracePeriodSeconds` (L144), `WithLabels` (L154), `WithAnnotations` (L169), `WithOwnerReferences` (L183), `WithFinalizers` (L197), `ensureObjectMetaApplyConfigurationExists` (L205), `WithSpec` (L214), `WithStatus` (L222), `GetKind` (L228), `GetAPIVersion` (L233), `GetName` (L238). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameserverset.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b0335bdc882cec73"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameserversetspec.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `220536024e5b4d69e675a29f781b7488d5cf2ba0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameserversetspec.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0d24506c2165048879.** [agones/pkg/client/applyconfiguration/agones/v1/gameserversetspec.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameserversetspec.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `136c2c28ae930e70770742ead456b2b6f659c522d8240bcf9d2c5dd61b8e32fc`.

> 
> // GameServerSetSpecApplyConfiguration represents a declarative configuration of the GameServerSetSpec type for use
> // with apply.
> //
> // GameServerSetSpec the specification for GameServerSet
> type GameServerSetSpecApplyConfiguration struct {
> 	// Replicas are the number of GameServers that should be in this set
> 	Replicas *int32 `json:"replicas,omitempty"`
> 	// Labels and Annotations to apply to GameServers when the number of Allocated GameServers drops below
> 	// the desired replicas on the underlying `GameServerSet`
> 	AllocationOverflow *AllocationOverflowApplyConfiguration `json:"allocationOverflow,omitempty"`
> 	// Scheduling strategy. Defaults to "Packed".
> 	Scheduling *apis.SchedulingStrategy `json:"scheduling,omitempty"`
> 	// [Stage: Beta]
> 	// [FeatureFlag:CountsAndLists]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 7 lexically identified declarations, including `GameServerSetSpecApplyConfiguration` (L29), `GameServerSetSpec` (L55), `WithReplicas` (L62), `WithAllocationOverflow` (L70), `WithScheduling` (L78), `WithPriorities` (L86), `WithTemplate` (L99). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameserversetspec.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-44c1411e46a3d567"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameserversetstatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3750437a53e0126286e68ccc03434162846bbf9f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameserversetstatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-bae8cf55f63ceb3255.** [agones/pkg/client/applyconfiguration/agones/v1/gameserversetstatus.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameserversetstatus.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a16defcc7e6833921a06e89b8299f16fff9b9515fdd82bdb594e38a023ff6ef5`.

> 
> // GameServerSetStatusApplyConfiguration represents a declarative configuration of the GameServerSetStatus type for use
> // with apply.
> //
> // GameServerSetStatus is the status of a GameServerSet
> type GameServerSetStatusApplyConfiguration struct {
> 	// Replicas is the total number of current GameServer replicas
> 	Replicas *int32 `json:"replicas,omitempty"`
> 	// ReadyReplicas is the number of Ready GameServer replicas
> 	ReadyReplicas *int32 `json:"readyReplicas,omitempty"`
> 	// ReservedReplicas is the number of Reserved GameServer replicas
> 	ReservedReplicas *int32 `json:"reservedReplicas,omitempty"`
> 	// AllocatedReplicas is the number of Allocated GameServer replicas
> 	AllocatedReplicas *int32 `json:"allocatedReplicas,omitempty"`
> 	// ShutdownReplicas is the number of Shutdown GameServers replicas


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 10 lexically identified declarations, including `GameServerSetStatusApplyConfiguration` (L25), `GameServerSetStatus` (L50), `WithReplicas` (L57), `WithReadyReplicas` (L65), `WithReservedReplicas` (L73), `WithAllocatedReplicas` (L81), `WithShutdownReplicas` (L89), `WithPlayers` (L97), `WithCounters` (L106), `WithLists` (L120). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameserversetstatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-8751110d1a79925f"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameserverspec.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ae884cf060fb9352a98369d441bb03628165fa42`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameserverspec.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f88128a14ae2c4c687.** [agones/pkg/client/applyconfiguration/agones/v1/gameserverspec.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameserverspec.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d75d081fa25a90c266ca75d397593d839e16fc375a534e79de33a80435b000c3`.

> 
> // GameServerSpecApplyConfiguration represents a declarative configuration of the GameServerSpec type for use
> // with apply.
> //
> // GameServerSpec is the spec for a GameServer resource
> type GameServerSpecApplyConfiguration struct {
> 	// Container specifies which Pod container is the game server. Only required if there is more than one
> 	// container defined
> 	Container *string `json:"container,omitempty"`
> 	// Ports are the array of ports that can be exposed via the game server
> 	Ports []GameServerPortApplyConfiguration `json:"ports,omitempty"`
> 	// Health configures health checking
> 	Health *HealthApplyConfiguration `json:"health,omitempty"`
> 	// Scheduling strategy. Defaults to "Packed"
> 	Scheduling *apis.SchedulingStrategy `json:"scheduling,omitempty"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 12 lexically identified declarations, including `GameServerSpecApplyConfiguration` (L30), `GameServerSpec` (L59), `WithContainer` (L66), `WithPorts` (L74), `WithHealth` (L87), `WithScheduling` (L95), `WithSdkServer` (L103), `WithTemplate` (L111), `WithPlayers` (L119), `WithCounters` (L128), `WithLists` (L142), `WithEviction` (L155). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameserverspec.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-ab8e96c53ca875be"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameserverstatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `bd80b8f2cd9432627abe8d2a40c9adcdccd6ba30`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameserverstatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-6cd320a5828392f45b.** [agones/pkg/client/applyconfiguration/agones/v1/gameserverstatus.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameserverstatus.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0c5b4e10d9316b008190ebe60943593d2f6dba6fee30882bd331757351b7239c`.

> 
> // GameServerStatusApplyConfiguration represents a declarative configuration of the GameServerStatus type for use
> // with apply.
> //
> // GameServerStatus is the status for a GameServer resource
> type GameServerStatusApplyConfiguration struct {
> 	// GameServerState is the current state of a GameServer, e.g. Creating, Starting, Ready, etc
> 	State   *agonesv1.GameServerState                `json:"state,omitempty"`
> 	Ports   []GameServerStatusPortApplyConfiguration `json:"ports,omitempty"`
> 	Address *string                                  `json:"address,omitempty"`
> 	// Addresses is the array of addresses at which the GameServer can be reached; copy of Node.Status.addresses.
> 	Addresses     []corev1.NodeAddress `json:"addresses,omitempty"`
> 	NodeName      *string              `json:"nodeName,omitempty"`
> 	ReservedUntil *metav1.Time         `json:"reservedUntil,omitempty"`
> 	// [Stage:Alpha]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 12 lexically identified declarations, including `GameServerStatusApplyConfiguration` (L31), `GameServerStatus` (L52), `WithState` (L59), `WithPorts` (L67), `WithAddress` (L80), `WithAddresses` (L88), `WithNodeName` (L98), `WithReservedUntil` (L106), `WithPlayers` (L114), `WithCounters` (L123), `WithLists` (L137), `WithEviction` (L150). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameserverstatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-402cdbd3b42530f0"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameserverstatusport.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `08779532f9f688aa074c976cbf091cdae3f6f971`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameserverstatusport.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a76cb74b4247f9b710.** [agones/pkg/client/applyconfiguration/agones/v1/gameserverstatusport.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameserverstatusport.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `dceac423f0b89c3c71dc2e114abbca8c8d0aef5a6192f77bac999f5a643440c8`.

> 
> // GameServerStatusPortApplyConfiguration represents a declarative configuration of the GameServerStatusPort type for use
> // with apply.
> //
> // GameServerStatusPort shows the port that was allocated to a
> // GameServer.
> type GameServerStatusPortApplyConfiguration struct {
> 	Name *string `json:"name,omitempty"`
> 	Port *int32  `json:"port,omitempty"`
> }
> 
> // GameServerStatusPortApplyConfiguration constructs a declarative configuration of the GameServerStatusPort type for use with
> // apply.
> func GameServerStatusPort() *GameServerStatusPortApplyConfiguration {
> 	return &GameServerStatusPortApplyConfiguration{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `GameServerStatusPortApplyConfiguration` (L26), `GameServerStatusPort` (L33), `WithName` (L40), `WithPort` (L48). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameserverstatusport.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-5618e55c1c8a41fa"></a>


### File `pkg/client/applyconfiguration/agones/v1/gameservertemplatespec.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a574b2ea3af7fe65d60705cd4a82cced4431442b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/gameservertemplatespec.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-95831a0bc26cde8937.** [agones/pkg/client/applyconfiguration/agones/v1/gameservertemplatespec.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/gameservertemplatespec.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `60af2dbd236509ab58fddfc8597f008d8333f01c2bf97248dc7d5beae999ae7a`.

> 
> // GameServerTemplateSpecApplyConfiguration represents a declarative configuration of the GameServerTemplateSpec type for use
> // with apply.
> //
> // GameServerTemplateSpec is a template for GameServers
> type GameServerTemplateSpecApplyConfiguration struct {
> 	*metav1.ObjectMetaApplyConfiguration `json:"metadata,omitempty"`
> 	Spec                                 *GameServerSpecApplyConfiguration `json:"spec,omitempty"`
> }
> 
> // GameServerTemplateSpecApplyConfiguration constructs a declarative configuration of the GameServerTemplateSpec type for use with
> // apply.
> func GameServerTemplateSpec() *GameServerTemplateSpecApplyConfiguration {
> 	return &GameServerTemplateSpecApplyConfiguration{}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 19 lexically identified declarations, including `GameServerTemplateSpecApplyConfiguration` (L31), `GameServerTemplateSpec` (L38), `WithName` (L45), `WithGenerateName` (L54), `WithNamespace` (L63), `WithUID` (L72), `WithResourceVersion` (L81), `WithGeneration` (L90), `WithCreationTimestamp` (L99), `WithDeletionTimestamp` (L108), `WithDeletionGracePeriodSeconds` (L117), `WithLabels` (L127), `WithAnnotations` (L142), `WithOwnerReferences` (L156), `WithFinalizers` (L170), `ensureObjectMetaApplyConfigurationExists` (L178), `WithSpec` (L187), `GetName` (L193), `GetNamespace` (L199). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/gameservertemplatespec.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-d6a0b18d46b7c256"></a>


### File `pkg/client/applyconfiguration/agones/v1/health.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e8738b18fbaefbf09ebcfdb48017c44d440e1f84`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/health.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-708019558bed116fe4.** [agones/pkg/client/applyconfiguration/agones/v1/health.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/health.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `96fefe6a0e759fa08be5824dcc7e4003f9c44db0c494e488c08929a057425b4f`.

> 
> // HealthApplyConfiguration represents a declarative configuration of the Health type for use
> // with apply.
> //
> // Health configures health checking on the GameServer
> type HealthApplyConfiguration struct {
> 	// Disabled is whether health checking is disabled or not
> 	Disabled *bool `json:"disabled,omitempty"`
> 	// PeriodSeconds is the number of seconds each health ping has to occur in
> 	PeriodSeconds *int32 `json:"periodSeconds,omitempty"`
> 	// FailureThreshold how many failures in a row constitutes unhealthy
> 	FailureThreshold *int32 `json:"failureThreshold,omitempty"`
> 	// InitialDelaySeconds initial delay before checking health
> 	InitialDelaySeconds *int32 `json:"initialDelaySeconds,omitempty"`
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `HealthApplyConfiguration` (L25), `Health` (L38), `WithDisabled` (L45), `WithPeriodSeconds` (L53), `WithFailureThreshold` (L61), `WithInitialDelaySeconds` (L69). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/health.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-c5aa13ff4c9424c5"></a>


### File `pkg/client/applyconfiguration/agones/v1/liststatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `444f48a7cf809cdd0b52e3f752fb705dc2007863`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/liststatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-33cdfa857006942707.** [agones/pkg/client/applyconfiguration/agones/v1/liststatus.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/liststatus.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `774f6680736e97560fac639a2ff40619da155b02914775f2022af8d09e3828e9`.

> 
> // ListStatusApplyConfiguration represents a declarative configuration of the ListStatus type for use
> // with apply.
> //
> // ListStatus stores the current list values and maximum capacity
> type ListStatusApplyConfiguration struct {
> 	Capacity *int64   `json:"capacity,omitempty"`
> 	Values   []string `json:"values,omitempty"`
> }
> 
> // ListStatusApplyConfiguration constructs a declarative configuration of the ListStatus type for use with
> // apply.
> func ListStatus() *ListStatusApplyConfiguration {
> 	return &ListStatusApplyConfiguration{}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `ListStatusApplyConfiguration` (L25), `ListStatus` (L32), `WithCapacity` (L39), `WithValues` (L47). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/liststatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-ae1c46684f25b4ad"></a>


### File `pkg/client/applyconfiguration/agones/v1/playersspec.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `53f1e627e42796adb0ebaa59210f2ae16cd5ca91`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/playersspec.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-168d00f8f60fa38338.** [agones/pkg/client/applyconfiguration/agones/v1/playersspec.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/playersspec.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `cfbeae0fa775bb8808c1468cf20223fe08725dd24ca6336ecd10efae30a71b7a`.

> 
> // PlayersSpecApplyConfiguration represents a declarative configuration of the PlayersSpec type for use
> // with apply.
> //
> // PlayersSpec tracks the initial player capacity
> type PlayersSpecApplyConfiguration struct {
> 	InitialCapacity *int64 `json:"initialCapacity,omitempty"`
> }
> 
> // PlayersSpecApplyConfiguration constructs a declarative configuration of the PlayersSpec type for use with
> // apply.
> func PlayersSpec() *PlayersSpecApplyConfiguration {
> 	return &PlayersSpecApplyConfiguration{}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 3 lexically identified declarations, including `PlayersSpecApplyConfiguration` (L25), `PlayersSpec` (L31), `WithInitialCapacity` (L38). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/playersspec.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-f294feecd4be5621"></a>


### File `pkg/client/applyconfiguration/agones/v1/playerstatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3594174a87883287c92081d2af6e58b86c6daff0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/playerstatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b9cc11bce2981be042.** [agones/pkg/client/applyconfiguration/agones/v1/playerstatus.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/playerstatus.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `715b7801862a87ee174774d4c8d57b85a8a0c518e422110ad32cf181198a5961`.

> 
> // PlayerStatusApplyConfiguration represents a declarative configuration of the PlayerStatus type for use
> // with apply.
> //
> // PlayerStatus stores the current player capacity values
> type PlayerStatusApplyConfiguration struct {
> 	Count    *int64   `json:"count,omitempty"`
> 	Capacity *int64   `json:"capacity,omitempty"`
> 	IDs      []string `json:"ids,omitempty"`
> }
> 
> // PlayerStatusApplyConfiguration constructs a declarative configuration of the PlayerStatus type for use with
> // apply.
> func PlayerStatus() *PlayerStatusApplyConfiguration {
> 	return &PlayerStatusApplyConfiguration{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `PlayerStatusApplyConfiguration` (L25), `PlayerStatus` (L33), `WithCount` (L40), `WithCapacity` (L48), `WithIDs` (L56). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/playerstatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-59dfdf4d2f3bcc6c"></a>


### File `pkg/client/applyconfiguration/agones/v1/priority.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `57a4c5e210a9eaaf7e42030b0427282ca92e6e87`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/priority.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d277b4d544a74ed5d1.** [agones/pkg/client/applyconfiguration/agones/v1/priority.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/priority.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `89160eb5e4603a2c103ed707fdf68d9e5d0ed5f47b7b3d4c691557ec8b2d1769`.

> 
> // PriorityApplyConfiguration represents a declarative configuration of the Priority type for use
> // with apply.
> //
> // Priority is a sorting option for GameServers with Counters or Lists based on the available capacity,
> // i.e. the current Capacity value, minus either the Count value or List length.
> type PriorityApplyConfiguration struct {
> 	// Type: Sort by a "Counter" or a "List".
> 	Type *string `json:"type,omitempty"`
> 	// Key: The name of the Counter or List. If not found on the GameServer, has no impact.
> 	Key *string `json:"key,omitempty"`
> 	// Order: Sort by "Ascending" or "Descending". "Descending" a bigger available capacity is preferred.
> 	// "Ascending" would be smaller available capacity is preferred.
> 	// The default sort order is "Ascending"
> 	Order *string `json:"order,omitempty"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `PriorityApplyConfiguration` (L26), `Priority` (L39), `WithType` (L46), `WithKey` (L54), `WithOrder` (L62). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/priority.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-50d20c6361e33116"></a>


### File `pkg/client/applyconfiguration/agones/v1/sdkserver.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `baea34e6d6af77c3877bc570f396209e26e04d15`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/agones/v1/sdkserver.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0993c463e38be6d802.** [agones/pkg/client/applyconfiguration/agones/v1/sdkserver.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/agones/v1/sdkserver.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `cb5fcfe45dffda34399a641abefb14dc8936b849cabf5bfa6632d1f169e2fa20`.

> 
> // SdkServerApplyConfiguration represents a declarative configuration of the SdkServer type for use
> // with apply.
> //
> // SdkServer specifies parameters for the Agones SDK Server sidecar container
> type SdkServerApplyConfiguration struct {
> 	// LogLevel for SDK server (sidecar) logs. Defaults to "Info"
> 	LogLevel *agonesv1.SdkServerLogLevel `json:"logLevel,omitempty"`
> 	// GRPCPort is the port on which the SDK Server binds the gRPC server to accept incoming connections
> 	GRPCPort *int32 `json:"grpcPort,omitempty"`
> 	// HTTPPort is the port on which the SDK Server binds the HTTP gRPC gateway server to accept incoming connections
> 	HTTPPort *int32 `json:"httpPort,omitempty"`
> }
> 
> // SdkServerApplyConfiguration constructs a declarative configuration of the SdkServer type for use with


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `SdkServerApplyConfiguration` (L29), `SdkServer` (L40), `WithLogLevel` (L47), `WithGRPCPort` (L55), `WithHTTPPort` (L63). The complete list remains in `agones:pkg/client/applyconfiguration/agones/v1/sdkserver.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/applyconfiguration/autoscaling/v1` -> module `pkg::client::applyconfiguration::autoscaling::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-0efdf7f8c66f6d4f"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/activeperiod.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a8bf0d0fc3b3093924cc670b20718ffa175ff2bd`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/activeperiod.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a75cfedc075a6fdaeb.** [agones/pkg/client/applyconfiguration/autoscaling/v1/activeperiod.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/activeperiod.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2b9360e0fbf3c3e33eadaf2cf0c67bc35c7544fba889b4defe4ecbd34e461f03`.

> 
> // ActivePeriodApplyConfiguration represents a declarative configuration of the ActivePeriod type for use
> // with apply.
> //
> // ActivePeriod defines the time period that the policy is applied.
> type ActivePeriodApplyConfiguration struct {
> 	// Timezone to be used for the startCron field. If not set, startCron is defaulted to the UTC timezone.
> 	Timezone *string `json:"timezone,omitempty"`
> 	// StartCron defines when the policy should be applied.
> 	// If not set, the policy is always to be applied within the start and end time.
> 	// This field must conform to UNIX cron syntax.
> 	StartCron *string `json:"startCron,omitempty"`
> 	// Duration is the length of time that the policy is applied.
> 	// If not set, the duration is indefinite.
> 	// A duration string is a possibly signed sequence of decimal numbers,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `ActivePeriodApplyConfiguration` (L25), `ActivePeriod` (L43), `WithTimezone` (L50), `WithStartCron` (L58), `WithDuration` (L66). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/activeperiod.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-ed65e1f5e1536303"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/between.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `22455215226beba263caebc0469799f107db58fb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/between.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-8ace659fd39de575d9.** [agones/pkg/client/applyconfiguration/autoscaling/v1/between.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/between.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5b3937fc72c882d3fe3552c5d872ad211218ce7347823559b2cc9be5eb679dd2`.

> 
> // BetweenApplyConfiguration represents a declarative configuration of the Between type for use
> // with apply.
> //
> // Between defines the time period that the policy is eligible to be applied.
> type BetweenApplyConfiguration struct {
> 	// Start is the datetime that the policy is eligible to be applied.
> 	// This field must conform to RFC3339 format. If this field not set or is in the past, the policy is eligible to be applied
> 	// as soon as the fleet autoscaler is running.
> 	Start *metav1.Time `json:"start,omitempty"`
> 	// End is the datetime that the policy is no longer eligible to be applied.
> 	// This field must conform to RFC3339 format. If not set, the policy is always eligible to be applied, after the start time above.
> 	End *metav1.Time `json:"end,omitempty"`
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `BetweenApplyConfiguration` (L29), `Between` (L41), `WithStart` (L48), `WithEnd` (L56). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/between.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-395098c6c778f6cf"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/bufferpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5b92a56ded41b885cc19fad4f6dedea248bb07bb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/bufferpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-1ff3226ba43d3596e2.** [agones/pkg/client/applyconfiguration/autoscaling/v1/bufferpolicy.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/bufferpolicy.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `dec7e17d99aae74b939dd0677f71d4ee61039822845fbf6105b490650a64dd8d`.

> 
> // BufferPolicyApplyConfiguration represents a declarative configuration of the BufferPolicy type for use
> // with apply.
> //
> // BufferPolicy controls the desired behavior of the buffer policy.
> type BufferPolicyApplyConfiguration struct {
> 	// MaxReplicas is the maximum amount of replicas that the fleet may have.
> 	// It must be bigger than both MinReplicas and BufferSize
> 	MaxReplicas *int32 `json:"maxReplicas,omitempty"`
> 	// MinReplicas is the minimum amount of replicas that the fleet must have
> 	// If zero, it is ignored.
> 	// If non zero, it must be smaller than MaxReplicas and bigger than BufferSize
> 	MinReplicas *int32 `json:"minReplicas,omitempty"`
> 	// BufferSize defines how many replicas the autoscaler tries to have ready all the time
> 	// Value can be an absolute number (ex: 5) or a percentage of desired gs instances (ex: 15%)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `BufferPolicyApplyConfiguration` (L29), `BufferPolicy` (L52), `WithMaxReplicas` (L59), `WithMinReplicas` (L67), `WithBufferSize` (L75). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/bufferpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-9ecd7fc682a8cadb"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/chainentry.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a643a17ae3a043002ebd7806426182ca42dd511f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/chainentry.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-66ba224455c637e934.** [agones/pkg/client/applyconfiguration/autoscaling/v1/chainentry.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/chainentry.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2ed3f90e548347800cead4eebf551e7769e0f26e725d58ec091dd1dd0fdf3275`.

> 
> // ChainEntryApplyConfiguration represents a declarative configuration of the ChainEntry type for use
> // with apply.
> //
> // ChainEntry defines a single entry in the ChainPolicy.
> type ChainEntryApplyConfiguration struct {
> 	// ID is the unique identifier for a ChainEntry. If not set the identifier will be set to the index of chain entry.
> 	ID *string `json:"id,omitempty"`
> 	// Policy is the name of the policy to be applied. Required field.
> 	FleetAutoscalerPolicyApplyConfiguration `json:",inline"`
> }
> 
> // ChainEntryApplyConfiguration constructs a declarative configuration of the ChainEntry type for use with
> // apply.
> func ChainEntry() *ChainEntryApplyConfiguration {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 11 lexically identified declarations, including `ChainEntryApplyConfiguration` (L29), `ChainEntry` (L38), `WithID` (L45), `WithType` (L53), `WithBuffer` (L61), `WithWebhook` (L69), `WithCounter` (L77), `WithList` (L85), `WithSchedule` (L93), `WithChain` (L101), `WithWasm` (L109). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/chainentry.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-52c65aaa92e42eb0"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/counterpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e444397832a7deaeb9e4378a5d56334f0b4bad81`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/counterpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-bd588d9395a3a08b50.** [agones/pkg/client/applyconfiguration/autoscaling/v1/counterpolicy.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/counterpolicy.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `59f7a3598b411ef849b9c20186ca015fa025538f3cf2ee21cc619a7f6a147660`.

> 
> // CounterPolicyApplyConfiguration represents a declarative configuration of the CounterPolicy type for use
> // with apply.
> //
> // CounterPolicy controls the desired behavior of the Counter autoscaler policy.
> type CounterPolicyApplyConfiguration struct {
> 	// Key is the name of the Counter. Required field.
> 	Key *string `json:"key,omitempty"`
> 	// MaxCapacity is the maximum aggregate Counter total capacity across the fleet.
> 	// MaxCapacity must be bigger than both MinCapacity and BufferSize. Required field.
> 	MaxCapacity *int64 `json:"maxCapacity,omitempty"`
> 	// MinCapacity is the minimum aggregate Counter total capacity across the fleet.
> 	// If zero, MinCapacity is ignored.
> 	// If non zero, MinCapacity must be smaller than MaxCapacity and bigger than BufferSize.
> 	MinCapacity *int64 `json:"minCapacity,omitempty"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `CounterPolicyApplyConfiguration` (L29), `CounterPolicy` (L48), `WithKey` (L55), `WithMaxCapacity` (L63), `WithMinCapacity` (L71), `WithBufferSize` (L79). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/counterpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-7bdebd024fc6139b"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/fixedintervalsync.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `17e8f4f3e9edc7e78ebbf4e1af51899fadd3a246`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/fixedintervalsync.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-42fd0e2372e0994903.** [agones/pkg/client/applyconfiguration/autoscaling/v1/fixedintervalsync.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/fixedintervalsync.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a9c3faf2998ece9d2481035b1166e269f8b31724d095894e46b96a2332cd5417`.

> 
> // FixedIntervalSyncApplyConfiguration represents a declarative configuration of the FixedIntervalSync type for use
> // with apply.
> //
> // FixedIntervalSync controls the desired behavior of the fixed interval based sync.
> type FixedIntervalSyncApplyConfiguration struct {
> 	// Seconds defines how often we run fleet autoscaling in seconds
> 	Seconds *int32 `json:"seconds,omitempty"`
> }
> 
> // FixedIntervalSyncApplyConfiguration constructs a declarative configuration of the FixedIntervalSync type for use with
> // apply.
> func FixedIntervalSync() *FixedIntervalSyncApplyConfiguration {
> 	return &FixedIntervalSyncApplyConfiguration{}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 3 lexically identified declarations, including `FixedIntervalSyncApplyConfiguration` (L25), `FixedIntervalSync` (L32), `WithSeconds` (L39). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/fixedintervalsync.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-d6e517b823f6b451"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/fleetautoscaler.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0604ff6807d43a89bee2a5946b6563311800f51a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscaler.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e33634c5731b0f5dda.** [agones/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscaler.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscaler.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e2b9c7fa8c7c7c4671793126c200b3fa876b145e5732169433f81154f1c1d282`.

> 
> // FleetAutoscalerApplyConfiguration represents a declarative configuration of the FleetAutoscaler type for use
> // with apply.
> //
> // FleetAutoscaler is the data structure for a FleetAutoscaler resource
> type FleetAutoscalerApplyConfiguration struct {
> 	metav1.TypeMetaApplyConfiguration    `json:",inline"`
> 	*metav1.ObjectMetaApplyConfiguration `json:"metadata,omitempty"`
> 	Spec                                 *FleetAutoscalerSpecApplyConfiguration   `json:"spec,omitempty"`
> 	Status                               *FleetAutoscalerStatusApplyConfiguration `json:"status,omitempty"`
> }
> 
> // FleetAutoscaler constructs a declarative configuration of the FleetAutoscaler type for use with
> // apply.
> func FleetAutoscaler(name, namespace string) *FleetAutoscalerApplyConfiguration {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 25 lexically identified declarations, including `FleetAutoscalerApplyConfiguration` (L31), `FleetAutoscaler` (L40), `IsApplyConfiguration` (L49), `WithKind` (L54), `WithAPIVersion` (L62), `WithName` (L70), `WithGenerateName` (L79), `WithNamespace` (L88), `WithUID` (L97), `WithResourceVersion` (L106), `WithGeneration` (L115), `WithCreationTimestamp` (L124), `WithDeletionTimestamp` (L133), `WithDeletionGracePeriodSeconds` (L142), `WithLabels` (L152), `WithAnnotations` (L167), `WithOwnerReferences` (L181), `WithFinalizers` (L195), `ensureObjectMetaApplyConfigurationExists` (L203), `WithSpec` (L212), `WithStatus` (L220), `GetKind` (L226), `GetAPIVersion` (L231), `GetName` (L236). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/fleetautoscaler.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b29579ea09639d81"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `aa3ad4e2d0160854ef94534fbd564fd2fbd7a491`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-9a50c8fe644ee61989.** [agones/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerpolicy.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerpolicy.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `aa93d24a3d76df7923f6226d6bd24bbb10ff1718432389afb94b53371d3759fa`.

> 
> // FleetAutoscalerPolicyApplyConfiguration represents a declarative configuration of the FleetAutoscalerPolicy type for use
> // with apply.
> //
> // FleetAutoscalerPolicy describes how to scale a fleet
> type FleetAutoscalerPolicyApplyConfiguration struct {
> 	// Type of autoscaling policy.
> 	Type *autoscalingv1.FleetAutoscalerPolicyType `json:"type,omitempty"`
> 	// Buffer policy config params. Present only if FleetAutoscalerPolicyType = Buffer.
> 	Buffer *BufferPolicyApplyConfiguration `json:"buffer,omitempty"`
> 	// Webhook policy config params. Present only if FleetAutoscalerPolicyType = Webhook.
> 	Webhook *URLConfigurationApplyConfiguration `json:"webhook,omitempty"`
> 	// [Stage:Beta]
> 	// [FeatureFlag:CountsAndLists]
> 	// Counter policy config params. Present only if FleetAutoscalerPolicyType = Counter.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 10 lexically identified declarations, including `FleetAutoscalerPolicyApplyConfiguration` (L29), `FleetAutoscalerPolicy` (L58), `WithType` (L65), `WithBuffer` (L73), `WithWebhook` (L81), `WithCounter` (L89), `WithList` (L97), `WithSchedule` (L105), `WithChain` (L113), `WithWasm` (L121). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b4b8cfc8cdc967e3"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerspec.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e396aa76979c35ee654637e9d78293cf9cd20d0c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerspec.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c3b277963a7353d8c1.** [agones/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerspec.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerspec.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `232eb53963a169fa2a3d946e32cda745a7589b9f6cfc4d046e44a62cbb90b8e8`.

> 
> // FleetAutoscalerSpecApplyConfiguration represents a declarative configuration of the FleetAutoscalerSpec type for use
> // with apply.
> //
> // FleetAutoscalerSpec is the spec for a Fleet Scaler
> type FleetAutoscalerSpecApplyConfiguration struct {
> 	FleetName *string `json:"fleetName,omitempty"`
> 	// Autoscaling policy
> 	Policy *FleetAutoscalerPolicyApplyConfiguration `json:"policy,omitempty"`
> 	// Sync defines when FleetAutoscalers runs autoscaling
> 	Sync *FleetAutoscalerSyncApplyConfiguration `json:"sync,omitempty"`
> }
> 
> // FleetAutoscalerSpecApplyConfiguration constructs a declarative configuration of the FleetAutoscalerSpec type for use with
> // apply.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `FleetAutoscalerSpecApplyConfiguration` (L25), `FleetAutoscalerSpec` (L35), `WithFleetName` (L42), `WithPolicy` (L50), `WithSync` (L58). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerspec.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-8781c9643b22d8c7"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerstatus.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6dca4c0c3514e4b7f49b78d20e7485adf1b6bc1d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerstatus.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-2c9e123e0451523780.** [agones/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerstatus.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerstatus.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0df456e39fd9c62c866b20d73a0a9345aef138cb130f5dbcff1165113c893e5e`.

> 
> // FleetAutoscalerStatusApplyConfiguration represents a declarative configuration of the FleetAutoscalerStatus type for use
> // with apply.
> //
> // FleetAutoscalerStatus defines the current status of a FleetAutoscaler
> type FleetAutoscalerStatusApplyConfiguration struct {
> 	// CurrentReplicas is the current number of gameserver replicas
> 	// of the fleet managed by this autoscaler, as last seen by the autoscaler
> 	CurrentReplicas *int32 `json:"currentReplicas,omitempty"`
> 	// DesiredReplicas is the desired number of gameserver replicas
> 	// of the fleet managed by this autoscaler, as last calculated by the autoscaler
> 	DesiredReplicas *int32 `json:"desiredReplicas,omitempty"`
> 	// lastScaleTime is the last time the FleetAutoscaler scaled the attached fleet,
> 	LastScaleTime *metav1.Time `json:"lastScaleTime,omitempty"`
> 	// AbleToScale indicates that we can access the target fleet


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 8 lexically identified declarations, including `FleetAutoscalerStatusApplyConfiguration` (L30), `FleetAutoscalerStatus` (L51), `WithCurrentReplicas` (L58), `WithDesiredReplicas` (L66), `WithLastScaleTime` (L74), `WithAbleToScale` (L82), `WithScalingLimited` (L90), `WithLastAppliedPolicy` (L98). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalerstatus.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-d50ef802fbbcddac"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalersync.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8388d588f5fc1c38ee1693cbc8cdeffb9a480968`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalersync.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-54be2c17341cd2de27.** [agones/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalersync.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalersync.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d3eea2d206a0130cda1293d16b4256781245c3b29fd58405acf539e4b23b6f5b`.

> 
> // FleetAutoscalerSyncApplyConfiguration represents a declarative configuration of the FleetAutoscalerSync type for use
> // with apply.
> //
> // FleetAutoscalerSync describes when to sync a fleet
> type FleetAutoscalerSyncApplyConfiguration struct {
> 	// Type of autoscaling sync.
> 	Type *autoscalingv1.FleetAutoscalerSyncType `json:"type,omitempty"`
> 	// FixedInterval config params. Present only if FleetAutoscalerSyncType = FixedInterval.
> 	FixedInterval *FixedIntervalSyncApplyConfiguration `json:"fixedInterval,omitempty"`
> }
> 
> // FleetAutoscalerSyncApplyConfiguration constructs a declarative configuration of the FleetAutoscalerSync type for use with
> // apply.
> func FleetAutoscalerSync() *FleetAutoscalerSyncApplyConfiguration {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `FleetAutoscalerSyncApplyConfiguration` (L29), `FleetAutoscalerSync` (L38), `WithType` (L45), `WithFixedInterval` (L53). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/fleetautoscalersync.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-8cbb6c703968ce2f"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/listpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `04af8ce548db3699e1c00e9236336cddb1e1ee0d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/listpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-cb8d25b1427a1c70a3.** [agones/pkg/client/applyconfiguration/autoscaling/v1/listpolicy.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/listpolicy.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3f7ec395c92ffe90614d59c1192367c12452a8375f70fbb77a4182279021a9a4`.

> 
> // ListPolicyApplyConfiguration represents a declarative configuration of the ListPolicy type for use
> // with apply.
> //
> // ListPolicy controls the desired behavior of the List autoscaler policy.
> type ListPolicyApplyConfiguration struct {
> 	// Key is the name of the List. Required field.
> 	Key *string `json:"key,omitempty"`
> 	// MaxCapacity is the maximum aggregate List total capacity across the fleet.
> 	// MaxCapacity must be bigger than both MinCapacity and BufferSize. Required field.
> 	MaxCapacity *int64 `json:"maxCapacity,omitempty"`
> 	// MinCapacity is the minimum aggregate List total capacity across the fleet.
> 	// If zero, it is ignored.
> 	// If non zero, it must be smaller than MaxCapacity and bigger than BufferSize.
> 	MinCapacity *int64 `json:"minCapacity,omitempty"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `ListPolicyApplyConfiguration` (L29), `ListPolicy` (L48), `WithKey` (L55), `WithMaxCapacity` (L63), `WithMinCapacity` (L71), `WithBufferSize` (L79). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/listpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-62165c89a8f23aaf"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/schedulepolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `75a90e2cd1824f6c55601f4427d4c2a29f2b615e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/schedulepolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e720f7a696802fa243.** [agones/pkg/client/applyconfiguration/autoscaling/v1/schedulepolicy.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/schedulepolicy.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `69fed71e8b1d6721988821c4f52c0b89a8ac4e7e2a6c9ba5ece02f48a6049654`.

> 
> // SchedulePolicyApplyConfiguration represents a declarative configuration of the SchedulePolicy type for use
> // with apply.
> //
> // SchedulePolicy controls the desired behavior of the Schedule autoscaler policy.
> type SchedulePolicyApplyConfiguration struct {
> 	// Between defines the time period that the policy is eligible to be applied.
> 	Between *BetweenApplyConfiguration `json:"between,omitempty"`
> 	// ActivePeriod defines the time period that the policy is applied.
> 	ActivePeriod *ActivePeriodApplyConfiguration `json:"activePeriod,omitempty"`
> 	// Policy is the name of the policy to be applied. Required field.
> 	Policy *FleetAutoscalerPolicyApplyConfiguration `json:"policy,omitempty"`
> }
> 
> // SchedulePolicyApplyConfiguration constructs a declarative configuration of the SchedulePolicy type for use with


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `SchedulePolicyApplyConfiguration` (L25), `SchedulePolicy` (L36), `WithBetween` (L43), `WithActivePeriod` (L51), `WithPolicy` (L59). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/schedulepolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-6c9a9d4a2f973575"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/urlconfiguration.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8f9979e2e1faef36a0702d6901b855eb96b48b50`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/urlconfiguration.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-2ec23497645153f0be.** [agones/pkg/client/applyconfiguration/autoscaling/v1/urlconfiguration.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/urlconfiguration.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `093e6a4607cf2a3187f136f61ec6a24774b90941d212f226c202c6c491ca0b3c`.

> 
> // URLConfigurationApplyConfiguration represents a declarative configuration of the URLConfiguration type for use
> // with apply.
> //
> // URLConfiguration is a generic configuration for any integration with an external URL or
> // cluster provided service. For example, it can be used to configure a webhook or Wasm module
> type URLConfigurationApplyConfiguration struct {
> 	// `url` gives the location of the webhook, in standard URL form
> 	// (`scheme://host:port/path`). Exactly one of `url` or `service`
> 	// must be specified.
> 	//
> 	// The `host` should not refer to a service running in the cluster; use
> 	// the `service` field instead. The host might be resolved via external
> 	// DNS in some apiservers (e.g., `kube-apiserver` cannot resolve
> 	// in-cluster DNS as that would be a layering violation). `host` may


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `URLConfigurationApplyConfiguration` (L30), `URLConfiguration` (L69), `WithURL` (L76), `WithService` (L84), `WithCABundle` (L92). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/urlconfiguration.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-be2129c578724a38"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/wasmfrom.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `28af6aa909acad5f429548a4003e64acac3c3e39`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/wasmfrom.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-72271908932d709cbe.** [agones/pkg/client/applyconfiguration/autoscaling/v1/wasmfrom.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/wasmfrom.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fc0889df7135f1d1ca98bd2b1b05c72ebe21ca6d254771d69397bfebbf150948`.

> 
> // WasmFromApplyConfiguration represents a declarative configuration of the WasmFrom type for use
> // with apply.
> //
> // WasmFrom defines the source of the Wasm module
> type WasmFromApplyConfiguration struct {
> 	// URL is the URL of the Wasm module to use for autoscaling.
> 	URL *URLConfigurationApplyConfiguration `json:"url,omitempty"`
> }
> 
> // WasmFromApplyConfiguration constructs a declarative configuration of the WasmFrom type for use with
> // apply.
> func WasmFrom() *WasmFromApplyConfiguration {
> 	return &WasmFromApplyConfiguration{}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 3 lexically identified declarations, including `WasmFromApplyConfiguration` (L25), `WasmFrom` (L32), `WithURL` (L39). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/wasmfrom.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-701113e5db40489e"></a>


### File `pkg/client/applyconfiguration/autoscaling/v1/wasmpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2e3be7760bbc9483c00d40a36f6f212d658ae490`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/autoscaling/v1/wasmpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-70877e3112d196db32.** [agones/pkg/client/applyconfiguration/autoscaling/v1/wasmpolicy.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/autoscaling/v1/wasmpolicy.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be44d0f3715971be30269af987d3b7c3a37149fb30976686404738dd82367481`.

> 
> // WasmPolicyApplyConfiguration represents a declarative configuration of the WasmPolicy type for use
> // with apply.
> //
> // WasmPolicy controls the desired behavior of the Wasm policy.
> // It contains the configuration for the WebAssembly module used for autoscaling.
> type WasmPolicyApplyConfiguration struct {
> 	// Function is the exported function to call in the wasm module, defaults to 'scale'
> 	Function *string `json:"function,omitempty"`
> 	// Config values to pass to the wasm program on startup
> 	Config map[string]string `json:"config,omitempty"`
> 	// WasmFrom defines the source of the Wasm module
> 	From *WasmFromApplyConfiguration `json:"from,omitempty"`
> 	// Hash of the Wasm module, used to verify the integrity of the module
> 	Hash *string `json:"hash,omitempty"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `WasmPolicyApplyConfiguration` (L26), `WasmPolicy` (L39), `WithFunction` (L46), `WithConfig` (L55), `WithFrom` (L68), `WithHash` (L76). The complete list remains in `agones:pkg/client/applyconfiguration/autoscaling/v1/wasmpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/applyconfiguration/internal` -> module `pkg::client::applyconfiguration::internal`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-d3d6511ebe2c9bff"></a>


### File `pkg/client/applyconfiguration/internal/internal.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `979fc0cc6d31680a6729c8fe71c94edb7bac5677`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/internal/internal.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8c7214aa830751f155.** [agones/pkg/client/applyconfiguration/internal/internal.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/internal/internal.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `81dd6284a45e3cb20b67799e2722222f39c12056e1fcb4606dd1247245e01eb4`.

> 
> func Parser() *typed.Parser {
> 	parserOnce.Do(func() {
> 		var err error
> 		parser, err = typed.NewParser(schemaYAML)
> 		if err != nil {
> 			panic(fmt.Sprintf("Failed to parse schema: %v", err))
> 		}
> 	})
> 	return parser
> }
> 
> var parserOnce sync.Once
> var parser *typed.Parser
> var schemaYAML = typed.YAMLObject(`types:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `Parser` (L28), `parserOnce` (L39), `parser` (L40), `schemaYAML` (L41). The complete list remains in `agones:pkg/client/applyconfiguration/internal/internal.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/applyconfiguration/multicluster/v1` -> module `pkg::client::applyconfiguration::multicluster::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-258f249070034196"></a>


### File `pkg/client/applyconfiguration/multicluster/v1/clusterconnectioninfo.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0a003ef560ff4499b776e9083342e1a57eeed850`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/multicluster/v1/clusterconnectioninfo.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-5c6126202629f52a99.** [agones/pkg/client/applyconfiguration/multicluster/v1/clusterconnectioninfo.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/multicluster/v1/clusterconnectioninfo.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d8edac66e6bfcc030503cc3dda305eb48ffa871989f7d3faf560a8b08d5d1021`.

> 
> // ClusterConnectionInfoApplyConfiguration represents a declarative configuration of the ClusterConnectionInfo type for use
> // with apply.
> //
> // ClusterConnectionInfo defines the connection information for a cluster
> type ClusterConnectionInfoApplyConfiguration struct {
> 	// Optional: the name of the targeted cluster
> 	ClusterName *string `json:"clusterName,omitempty"`
> 	// The endpoints for the allocator service in the targeted cluster.
> 	// If the AllocationEndpoints is not set, the allocation happens on local cluster.
> 	// If there are multiple endpoints any of the endpoints that can handle allocation request should suffice
> 	AllocationEndpoints []string `json:"allocationEndpoints,omitempty"`
> 	// The name of the secret that contains TLS client certificates to connect the allocator server in the targeted cluster
> 	SecretName *string `json:"secretName,omitempty"`
> 	// The cluster namespace from which to allocate gameservers


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 7 lexically identified declarations, including `ClusterConnectionInfoApplyConfiguration` (L25), `ClusterConnectionInfo` (L42), `WithClusterName` (L49), `WithAllocationEndpoints` (L57), `WithSecretName` (L67), `WithNamespace` (L75), `WithServerCA` (L83). The complete list remains in `agones:pkg/client/applyconfiguration/multicluster/v1/clusterconnectioninfo.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-7ff822c0582762e2"></a>


### File `pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `caa8bb57f79dc5fef7e2bb6c3146867ef330ca7e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4dcc4887ba107e1050.** [agones/pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicy.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicy.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b4081de4045a8f88b0c2d22935f64007d37a2deb5949bfd427d71eff6ca7b1a6`.

> 
> // GameServerAllocationPolicyApplyConfiguration represents a declarative configuration of the GameServerAllocationPolicy type for use
> // with apply.
> //
> // GameServerAllocationPolicy is the Schema for the gameserverallocationpolicies API
> type GameServerAllocationPolicyApplyConfiguration struct {
> 	metav1.TypeMetaApplyConfiguration    `json:",inline"`
> 	*metav1.ObjectMetaApplyConfiguration `json:"metadata,omitempty"`
> 	Spec                                 *GameServerAllocationPolicySpecApplyConfiguration `json:"spec,omitempty"`
> }
> 
> // GameServerAllocationPolicy constructs a declarative configuration of the GameServerAllocationPolicy type for use with
> // apply.
> func GameServerAllocationPolicy(name, namespace string) *GameServerAllocationPolicyApplyConfiguration {
> 	b := &GameServerAllocationPolicyApplyConfiguration{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 24 lexically identified declarations, including `GameServerAllocationPolicyApplyConfiguration` (L31), `GameServerAllocationPolicy` (L39), `IsApplyConfiguration` (L48), `WithKind` (L53), `WithAPIVersion` (L61), `WithName` (L69), `WithGenerateName` (L78), `WithNamespace` (L87), `WithUID` (L96), `WithResourceVersion` (L105), `WithGeneration` (L114), `WithCreationTimestamp` (L123), `WithDeletionTimestamp` (L132), `WithDeletionGracePeriodSeconds` (L141), `WithLabels` (L151), `WithAnnotations` (L166), `WithOwnerReferences` (L180), `WithFinalizers` (L194), `ensureObjectMetaApplyConfigurationExists` (L202), `WithSpec` (L211), `GetKind` (L217), `GetAPIVersion` (L222), `GetName` (L227), `GetNamespace` (L233). The complete list remains in `agones:pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-576687833679d0fd"></a>


### File `pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicyspec.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `41f8e94a9b678e8e0a7a3e98beb4a4fde4c37642`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicyspec.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ac4f842087888c7078.** [agones/pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicyspec.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicyspec.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2152e3fd6190a5eab066cf09965c9bb2d09de3a1cd76ad19da10a7ea17783a69`.

> 
> // GameServerAllocationPolicySpecApplyConfiguration represents a declarative configuration of the GameServerAllocationPolicySpec type for use
> // with apply.
> //
> // GameServerAllocationPolicySpec defines the desired state of GameServerAllocationPolicy
> type GameServerAllocationPolicySpecApplyConfiguration struct {
> 	Priority       *int32                                   `json:"priority,omitempty"`
> 	Weight         *int                                     `json:"weight,omitempty"`
> 	ConnectionInfo *ClusterConnectionInfoApplyConfiguration `json:"connectionInfo,omitempty"`
> }
> 
> // GameServerAllocationPolicySpecApplyConfiguration constructs a declarative configuration of the GameServerAllocationPolicySpec type for use with
> // apply.
> func GameServerAllocationPolicySpec() *GameServerAllocationPolicySpecApplyConfiguration {
> 	return &GameServerAllocationPolicySpecApplyConfiguration{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones declarative resource field presence, append/replace decisions and submission inputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `GameServerAllocationPolicySpecApplyConfiguration` (L25), `GameServerAllocationPolicySpec` (L33), `WithPriority` (L40), `WithWeight` (L48), `WithConnectionInfo` (L56). The complete list remains in `agones:pkg/client/applyconfiguration/multicluster/v1/gameserverallocationpolicyspec.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/clientset/versioned` -> module `pkg::client::clientset::versioned`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-aecd9c327ff317e6"></a>


### File `pkg/client/clientset/versioned/clientset.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `47670c6ce32aeaa184d1e29651172fffcdd93f01`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/clientset.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-44fa300c4286cca92e.** [agones/pkg/client/clientset/versioned/clientset.go, lines 33–47](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/clientset.go#L33-L47). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ffca3b32c6782e4b4d819cf966fb9168db86be88db5d064a1ff5392eab252f66`.

> 
> type Interface interface {
> 	Discovery() discovery.DiscoveryInterface
> 	AgonesV1() agonesv1.AgonesV1Interface
> 	AllocationV1() allocationv1.AllocationV1Interface
> 	AutoscalingV1() autoscalingv1.AutoscalingV1Interface
> 	MulticlusterV1() multiclusterv1.MulticlusterV1Interface
> }
> 
> // Clientset contains the clients for groups.
> type Clientset struct {
> 	*discovery.DiscoveryClient
> 	agonesV1       *agonesv1.AgonesV1Client
> 	allocationV1   *allocationv1.AllocationV1Client
> 	autoscalingV1  *autoscalingv1.AutoscalingV1Client


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 11 lexically identified declarations, including `Interface` (L34), `Clientset` (L43), `AgonesV1` (L52), `AllocationV1` (L57), `AutoscalingV1` (L62), `MulticlusterV1` (L67), `Discovery` (L72), `NewForConfig` (L84), `NewForConfigAndClient` (L104), `NewForConfigOrDie` (L141), `New` (L150). The complete list remains in `agones:pkg/client/clientset/versioned/clientset.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/clientset/versioned/scheme` -> module `pkg::client::clientset::versioned::scheme`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-b529e6d6dfd15bc5b6.** [agones/pkg/client/clientset/versioned/scheme/doc.go, lines 19–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/scheme/doc.go#L19-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f86049c435c8cf89027e6504807c520bab7799cbcc396fd5a3fe9f6346439a15`.

> // This package contains the scheme of the automatically generated clientset.
> package scheme


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-20158cbbab9faa6f"></a>


### File `pkg/client/clientset/versioned/scheme/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7535b9b329912d5a02cea300e02148b66d749f17`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/scheme/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fc474180b8d84c4a38.** [agones/pkg/client/clientset/versioned/scheme/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/scheme/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f86049c435c8cf89027e6504807c520bab7799cbcc396fd5a3fe9f6346439a15`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-33ad1147df14bfbe"></a>


### File `pkg/client/clientset/versioned/scheme/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9bca2e3bdcd58efa9d0816a6fbd097b5a4004da8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/scheme/register.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2e7a0eefbde28746eb.** [agones/pkg/client/clientset/versioned/scheme/register.go, lines 58–62](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/scheme/register.go#L58-L62). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `38b45009a5f5479c613b94cbba24ced253651c4cc933ecbf345377d1852962e6`.

> 
> func init() {
> 	v1.AddToGroupVersion(Scheme, schema.GroupVersion{Version: "v1"})
> 	utilruntime.Must(AddToScheme(Scheme))
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `Scheme` (L33), `Codecs` (L34), `ParameterCodec` (L35), `localSchemeBuilder` (L36), `AddToScheme` (L57), `init` (L59). The complete list remains in `agones:pkg/client/clientset/versioned/scheme/register.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/clientset/versioned/typed/agones/v1` -> module `pkg::client::clientset::versioned::typed::agones::v1`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-5212bed515ffd893a5.** [agones/pkg/client/clientset/versioned/typed/agones/v1/doc.go, lines 19–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/agones/v1/doc.go#L19-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d9c6cadac67028810c1609351c89fb4e1dd6b4904537009e894baec94d2334aa`.

> // This package has the automatically generated typed clients.
> package v1


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-488f53ebc24eb022"></a>


### File `pkg/client/clientset/versioned/typed/agones/v1/agones_client.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `189f30ebcfb40e87b376b1b65ce74b0a14fae6c5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/agones/v1/agones_client.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4a7eb347b1827957de.** [agones/pkg/client/clientset/versioned/typed/agones/v1/agones_client.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/agones/v1/agones_client.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bd2cd157f261a818f2726bcc28dd5a5baba41f21d7af6f94c180f729d0ee5a80`.

> 
> type AgonesV1Interface interface {
> 	RESTClient() rest.Interface
> 	FleetsGetter
> 	GameServersGetter
> 	GameServerSetsGetter
> }
> 
> // AgonesV1Client is used to interact with features provided by the agones.dev group.
> type AgonesV1Client struct {
> 	restClient rest.Interface
> }
> 
> func (c *AgonesV1Client) Fleets(namespace string) FleetInterface {
> 	return newFleets(c, namespace)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 11 lexically identified declarations, including `AgonesV1Interface` (L29), `AgonesV1Client` (L37), `Fleets` (L41), `GameServers` (L45), `GameServerSets` (L49), `NewForConfig` (L56), `NewForConfigAndClient` (L68), `NewForConfigOrDie` (L80), `New` (L89), `setConfigDefaults` (L93), `RESTClient` (L106). The complete list remains in `agones:pkg/client/clientset/versioned/typed/agones/v1/agones_client.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-71a440ff4b05519f"></a>


### File `pkg/client/clientset/versioned/typed/agones/v1/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1fb1c9db9b91f284fd3c66f28a09ab0b26b10112`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/agones/v1/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-23d9af173b67522d6f.** [agones/pkg/client/clientset/versioned/typed/agones/v1/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/agones/v1/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d9c6cadac67028810c1609351c89fb4e1dd6b4904537009e894baec94d2334aa`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-c216b930be85ad2b"></a>


### File `pkg/client/clientset/versioned/typed/agones/v1/fleet.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2c610d5f393b89e10b1085004eb091fb61a8a385`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/agones/v1/fleet.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-5854a35e6e5cc0e3e7.** [agones/pkg/client/clientset/versioned/typed/agones/v1/fleet.go, lines 33–47](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/agones/v1/fleet.go#L33-L47). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `935eb14a809f5684b4a76ead33721f0ec3468f0260cd62018a5b251315ae9ee5`.

> 
> // FleetsGetter has a method to return a FleetInterface.
> // A group's client should implement this interface.
> type FleetsGetter interface {
> 	Fleets(namespace string) FleetInterface
> }
> 
> // FleetInterface has methods to work with Fleet resources.
> type FleetInterface interface {
> 	Create(ctx context.Context, fleet *agonesv1.Fleet, opts metav1.CreateOptions) (*agonesv1.Fleet, error)
> 	Update(ctx context.Context, fleet *agonesv1.Fleet, opts metav1.UpdateOptions) (*agonesv1.Fleet, error)
> 	// Add a +genclient:noStatus comment above the type to avoid generating UpdateStatus().
> 	UpdateStatus(ctx context.Context, fleet *agonesv1.Fleet, opts metav1.UpdateOptions) (*agonesv1.Fleet, error)
> 	Delete(ctx context.Context, name string, opts metav1.DeleteOptions) error
> 	DeleteCollection(ctx context.Context, opts metav1.DeleteOptions, listOpts metav1.ListOptions) error


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `FleetsGetter` (L36), `FleetInterface` (L41), `fleets` (L62), `newFleets` (L67), `GetScale` (L81), `UpdateScale` (L95). The complete list remains in `agones:pkg/client/clientset/versioned/typed/agones/v1/fleet.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-9f28cebbb25a71ef"></a>


### File `pkg/client/clientset/versioned/typed/agones/v1/gameserver.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b9b167cc5ab79ae94b690965dfdf7b80761c8276`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/agones/v1/gameserver.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-7bcbe57171b7f40857.** [agones/pkg/client/clientset/versioned/typed/agones/v1/gameserver.go, lines 32–46](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/agones/v1/gameserver.go#L32-L46). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `30c1b7ee96e69ead7a9e49c1c1acadbd248978518a87e6f008ca50733068830b`.

> 
> // GameServersGetter has a method to return a GameServerInterface.
> // A group's client should implement this interface.
> type GameServersGetter interface {
> 	GameServers(namespace string) GameServerInterface
> }
> 
> // GameServerInterface has methods to work with GameServer resources.
> type GameServerInterface interface {
> 	Create(ctx context.Context, gameServer *agonesv1.GameServer, opts metav1.CreateOptions) (*agonesv1.GameServer, error)
> 	Update(ctx context.Context, gameServer *agonesv1.GameServer, opts metav1.UpdateOptions) (*agonesv1.GameServer, error)
> 	Delete(ctx context.Context, name string, opts metav1.DeleteOptions) error
> 	DeleteCollection(ctx context.Context, opts metav1.DeleteOptions, listOpts metav1.ListOptions) error
> 	Get(ctx context.Context, name string, opts metav1.GetOptions) (*agonesv1.GameServer, error)
> 	List(ctx context.Context, opts metav1.ListOptions) (*agonesv1.GameServerList, error)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `GameServersGetter` (L35), `GameServerInterface` (L40), `gameServers` (L54), `newGameServers` (L59). The complete list remains in `agones:pkg/client/clientset/versioned/typed/agones/v1/gameserver.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-47e71fada94e48e8"></a>


### File `pkg/client/clientset/versioned/typed/agones/v1/gameserverset.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2c7920ef4f2f24bb0e87acd73f952c73dc8a1280`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/agones/v1/gameserverset.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-5c57b7b9555ec37da7.** [agones/pkg/client/clientset/versioned/typed/agones/v1/gameserverset.go, lines 33–47](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/agones/v1/gameserverset.go#L33-L47). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `27e09c065bd2c7f10cd3e3c905ad68d665014ec4f6936d5ef75afde184b0ff90`.

> 
> // GameServerSetsGetter has a method to return a GameServerSetInterface.
> // A group's client should implement this interface.
> type GameServerSetsGetter interface {
> 	GameServerSets(namespace string) GameServerSetInterface
> }
> 
> // GameServerSetInterface has methods to work with GameServerSet resources.
> type GameServerSetInterface interface {
> 	Create(ctx context.Context, gameServerSet *agonesv1.GameServerSet, opts metav1.CreateOptions) (*agonesv1.GameServerSet, error)
> 	Update(ctx context.Context, gameServerSet *agonesv1.GameServerSet, opts metav1.UpdateOptions) (*agonesv1.GameServerSet, error)
> 	// Add a +genclient:noStatus comment above the type to avoid generating UpdateStatus().
> 	UpdateStatus(ctx context.Context, gameServerSet *agonesv1.GameServerSet, opts metav1.UpdateOptions) (*agonesv1.GameServerSet, error)
> 	Delete(ctx context.Context, name string, opts metav1.DeleteOptions) error
> 	DeleteCollection(ctx context.Context, opts metav1.DeleteOptions, listOpts metav1.ListOptions) error


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `GameServerSetsGetter` (L36), `GameServerSetInterface` (L41), `gameServerSets` (L62), `newGameServerSets` (L67), `GetScale` (L81), `UpdateScale` (L95). The complete list remains in `agones:pkg/client/clientset/versioned/typed/agones/v1/gameserverset.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-3e64a3277fcec2cf"></a>


### File `pkg/client/clientset/versioned/typed/agones/v1/generated_expansion.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `87b65938171844e47d8c57339e7c02f28f8fbce1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/agones/v1/generated_expansion.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6f1fe932424517cb0b.** [agones/pkg/client/clientset/versioned/typed/agones/v1/generated_expansion.go, lines 20–25](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/agones/v1/generated_expansion.go#L20-L25). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a020631ae58b00558b2c2e2a80dc6c2ce4a43d0a14e4c80b9a3b4b7ad968a362`.

> 
> type FleetExpansion interface{}
> 
> type GameServerExpansion interface{}
> 
> type GameServerSetExpansion interface{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 3 lexically identified declarations, including `FleetExpansion` (L21), `GameServerExpansion` (L23), `GameServerSetExpansion` (L25). The complete list remains in `agones:pkg/client/clientset/versioned/typed/agones/v1/generated_expansion.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/clientset/versioned/typed/allocation/v1` -> module `pkg::client::clientset::versioned::typed::allocation::v1`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-71b1db06e00528d90f.** [agones/pkg/client/clientset/versioned/typed/allocation/v1/doc.go, lines 19–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/allocation/v1/doc.go#L19-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d9c6cadac67028810c1609351c89fb4e1dd6b4904537009e894baec94d2334aa`.

> // This package has the automatically generated typed clients.
> package v1


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-f9f05821d8431ca5"></a>


### File `pkg/client/clientset/versioned/typed/allocation/v1/allocation_client.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1b3aeb866c6787e2ae0fb84c0a04012871ae746a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/allocation/v1/allocation_client.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-67ffa5bcb8af38c516.** [agones/pkg/client/clientset/versioned/typed/allocation/v1/allocation_client.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/allocation/v1/allocation_client.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `921725197574a8abc0173ac9243c1619269d9bd6612559da530b0bc346a2c009`.

> 
> type AllocationV1Interface interface {
> 	RESTClient() rest.Interface
> 	GameServerAllocationsGetter
> }
> 
> // AllocationV1Client is used to interact with features provided by the allocation.agones.dev group.
> type AllocationV1Client struct {
> 	restClient rest.Interface
> }
> 
> func (c *AllocationV1Client) GameServerAllocations(namespace string) GameServerAllocationInterface {
> 	return newGameServerAllocations(c, namespace)
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 9 lexically identified declarations, including `AllocationV1Interface` (L29), `AllocationV1Client` (L35), `GameServerAllocations` (L39), `NewForConfig` (L46), `NewForConfigAndClient` (L58), `NewForConfigOrDie` (L70), `New` (L79), `setConfigDefaults` (L83), `RESTClient` (L96). The complete list remains in `agones:pkg/client/clientset/versioned/typed/allocation/v1/allocation_client.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-2b8fb8f42c473812"></a>


### File `pkg/client/clientset/versioned/typed/allocation/v1/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1fb1c9db9b91f284fd3c66f28a09ab0b26b10112`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/allocation/v1/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ca7cb2ddac858c91d5.** [agones/pkg/client/clientset/versioned/typed/allocation/v1/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/allocation/v1/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d9c6cadac67028810c1609351c89fb4e1dd6b4904537009e894baec94d2334aa`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-088cf4250fe3d6d1"></a>


### File `pkg/client/clientset/versioned/typed/allocation/v1/gameserverallocation.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8736e8da1a63fca4842b43021faa12bb97b79e5b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/allocation/v1/gameserverallocation.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-aeace99637748781c7.** [agones/pkg/client/clientset/versioned/typed/allocation/v1/gameserverallocation.go, lines 29–43](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/allocation/v1/gameserverallocation.go#L29-L43). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9863b2026e3eb873ee9a54537931bb6f36e67216a0da8c0427c01c6078172452`.

> 
> // GameServerAllocationsGetter has a method to return a GameServerAllocationInterface.
> // A group's client should implement this interface.
> type GameServerAllocationsGetter interface {
> 	GameServerAllocations(namespace string) GameServerAllocationInterface
> }
> 
> // GameServerAllocationInterface has methods to work with GameServerAllocation resources.
> type GameServerAllocationInterface interface {
> 	Create(ctx context.Context, gameServerAllocation *allocationv1.GameServerAllocation, opts metav1.CreateOptions) (*allocationv1.GameServerAllocation, error)
> 	GameServerAllocationExpansion
> }
> 
> // gameServerAllocations implements GameServerAllocationInterface
> type gameServerAllocations struct {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `GameServerAllocationsGetter` (L32), `GameServerAllocationInterface` (L37), `gameServerAllocations` (L43), `newGameServerAllocations` (L48). The complete list remains in `agones:pkg/client/clientset/versioned/typed/allocation/v1/gameserverallocation.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b33754ad126bc923"></a>


### File `pkg/client/clientset/versioned/typed/allocation/v1/generated_expansion.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `88b64fa75d23645f2692e87062323fe381981992`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/allocation/v1/generated_expansion.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-94e2de3217340512d1.** [agones/pkg/client/clientset/versioned/typed/allocation/v1/generated_expansion.go, lines 20–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/allocation/v1/generated_expansion.go#L20-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `766aab251cc0e8686ba572a3e55a04ec6d9a0fc08cd756c30ffd894a85434b32`.

> 
> type GameServerAllocationExpansion interface{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 1 lexically identified declarations, including `GameServerAllocationExpansion` (L21). The complete list remains in `agones:pkg/client/clientset/versioned/typed/allocation/v1/generated_expansion.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/clientset/versioned/typed/autoscaling/v1` -> module `pkg::client::clientset::versioned::typed::autoscaling::v1`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-9962680b15dc2f85da.** [agones/pkg/client/clientset/versioned/typed/autoscaling/v1/doc.go, lines 19–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/autoscaling/v1/doc.go#L19-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d9c6cadac67028810c1609351c89fb4e1dd6b4904537009e894baec94d2334aa`.

> // This package has the automatically generated typed clients.
> package v1


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-0912505785b257f1"></a>


### File `pkg/client/clientset/versioned/typed/autoscaling/v1/autoscaling_client.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4c2fba9c12e2584159665700652d47a32b0581d6`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/autoscaling/v1/autoscaling_client.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-db40e36ae8b1494842.** [agones/pkg/client/clientset/versioned/typed/autoscaling/v1/autoscaling_client.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/autoscaling/v1/autoscaling_client.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ad2e460f774ad378d47ac77ba76ae023d1a45a724567755c35d8762419162780`.

> 
> type AutoscalingV1Interface interface {
> 	RESTClient() rest.Interface
> 	FleetAutoscalersGetter
> }
> 
> // AutoscalingV1Client is used to interact with features provided by the autoscaling.agones.dev group.
> type AutoscalingV1Client struct {
> 	restClient rest.Interface
> }
> 
> func (c *AutoscalingV1Client) FleetAutoscalers(namespace string) FleetAutoscalerInterface {
> 	return newFleetAutoscalers(c, namespace)
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 9 lexically identified declarations, including `AutoscalingV1Interface` (L29), `AutoscalingV1Client` (L35), `FleetAutoscalers` (L39), `NewForConfig` (L46), `NewForConfigAndClient` (L58), `NewForConfigOrDie` (L70), `New` (L79), `setConfigDefaults` (L83), `RESTClient` (L96). The complete list remains in `agones:pkg/client/clientset/versioned/typed/autoscaling/v1/autoscaling_client.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-f833aee1c4fcca6d"></a>


### File `pkg/client/clientset/versioned/typed/autoscaling/v1/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1fb1c9db9b91f284fd3c66f28a09ab0b26b10112`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/autoscaling/v1/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2febadf47989eae331.** [agones/pkg/client/clientset/versioned/typed/autoscaling/v1/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/autoscaling/v1/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d9c6cadac67028810c1609351c89fb4e1dd6b4904537009e894baec94d2334aa`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-960087e2764f684b"></a>


### File `pkg/client/clientset/versioned/typed/autoscaling/v1/fleetautoscaler.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `862629fdc762a7d6dd16a6045bda006066538969`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/autoscaling/v1/fleetautoscaler.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-cf14aef6b47e25adce.** [agones/pkg/client/clientset/versioned/typed/autoscaling/v1/fleetautoscaler.go, lines 32–46](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/autoscaling/v1/fleetautoscaler.go#L32-L46). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `47e7dec673366034e2fe659b4f09e7d2872197c4e0c0a1937d51e0f419deffc7`.

> 
> // FleetAutoscalersGetter has a method to return a FleetAutoscalerInterface.
> // A group's client should implement this interface.
> type FleetAutoscalersGetter interface {
> 	FleetAutoscalers(namespace string) FleetAutoscalerInterface
> }
> 
> // FleetAutoscalerInterface has methods to work with FleetAutoscaler resources.
> type FleetAutoscalerInterface interface {
> 	Create(ctx context.Context, fleetAutoscaler *autoscalingv1.FleetAutoscaler, opts metav1.CreateOptions) (*autoscalingv1.FleetAutoscaler, error)
> 	Update(ctx context.Context, fleetAutoscaler *autoscalingv1.FleetAutoscaler, opts metav1.UpdateOptions) (*autoscalingv1.FleetAutoscaler, error)
> 	// Add a +genclient:noStatus comment above the type to avoid generating UpdateStatus().
> 	UpdateStatus(ctx context.Context, fleetAutoscaler *autoscalingv1.FleetAutoscaler, opts metav1.UpdateOptions) (*autoscalingv1.FleetAutoscaler, error)
> 	Delete(ctx context.Context, name string, opts metav1.DeleteOptions) error
> 	DeleteCollection(ctx context.Context, opts metav1.DeleteOptions, listOpts metav1.ListOptions) error


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `FleetAutoscalersGetter` (L35), `FleetAutoscalerInterface` (L40), `fleetAutoscalers` (L58), `newFleetAutoscalers` (L63). The complete list remains in `agones:pkg/client/clientset/versioned/typed/autoscaling/v1/fleetautoscaler.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-77f18c9e0d1d457d"></a>


### File `pkg/client/clientset/versioned/typed/autoscaling/v1/generated_expansion.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c4620956a4f287db51be537346724a96ee729b17`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/autoscaling/v1/generated_expansion.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ea815fecfb2cbeac14.** [agones/pkg/client/clientset/versioned/typed/autoscaling/v1/generated_expansion.go, lines 20–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/autoscaling/v1/generated_expansion.go#L20-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `38f6d8d2a04a80d889419b660549f5b0a9783cb1328b1094baacbae731bed087`.

> 
> type FleetAutoscalerExpansion interface{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 1 lexically identified declarations, including `FleetAutoscalerExpansion` (L21). The complete list remains in `agones:pkg/client/clientset/versioned/typed/autoscaling/v1/generated_expansion.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/clientset/versioned/typed/multicluster/v1` -> module `pkg::client::clientset::versioned::typed::multicluster::v1`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-eff235de58d059df04.** [agones/pkg/client/clientset/versioned/typed/multicluster/v1/doc.go, lines 19–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/multicluster/v1/doc.go#L19-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d9c6cadac67028810c1609351c89fb4e1dd6b4904537009e894baec94d2334aa`.

> // This package has the automatically generated typed clients.
> package v1


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-4d1c422030bc92e6"></a>


### File `pkg/client/clientset/versioned/typed/multicluster/v1/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1fb1c9db9b91f284fd3c66f28a09ab0b26b10112`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/multicluster/v1/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-af54ce751c58005bbe.** [agones/pkg/client/clientset/versioned/typed/multicluster/v1/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/multicluster/v1/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d9c6cadac67028810c1609351c89fb4e1dd6b4904537009e894baec94d2334aa`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-6d37f0208b0a865d"></a>


### File `pkg/client/clientset/versioned/typed/multicluster/v1/gameserverallocationpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `340cb8b4115ee1494c06f70e4a83d816a0c967d5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/multicluster/v1/gameserverallocationpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-08856e7b8965382e01.** [agones/pkg/client/clientset/versioned/typed/multicluster/v1/gameserverallocationpolicy.go, lines 32–46](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/multicluster/v1/gameserverallocationpolicy.go#L32-L46). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fa4db0671181d8bd9e0e8285ee9a7c84e4aa11e9da84d9217d3c8b09fb07f75a`.

> 
> // GameServerAllocationPoliciesGetter has a method to return a GameServerAllocationPolicyInterface.
> // A group's client should implement this interface.
> type GameServerAllocationPoliciesGetter interface {
> 	GameServerAllocationPolicies(namespace string) GameServerAllocationPolicyInterface
> }
> 
> // GameServerAllocationPolicyInterface has methods to work with GameServerAllocationPolicy resources.
> type GameServerAllocationPolicyInterface interface {
> 	Create(ctx context.Context, gameServerAllocationPolicy *multiclusterv1.GameServerAllocationPolicy, opts metav1.CreateOptions) (*multiclusterv1.GameServerAllocationPolicy, error)
> 	Update(ctx context.Context, gameServerAllocationPolicy *multiclusterv1.GameServerAllocationPolicy, opts metav1.UpdateOptions) (*multiclusterv1.GameServerAllocationPolicy, error)
> 	Delete(ctx context.Context, name string, opts metav1.DeleteOptions) error
> 	DeleteCollection(ctx context.Context, opts metav1.DeleteOptions, listOpts metav1.ListOptions) error
> 	Get(ctx context.Context, name string, opts metav1.GetOptions) (*multiclusterv1.GameServerAllocationPolicy, error)
> 	List(ctx context.Context, opts metav1.ListOptions) (*multiclusterv1.GameServerAllocationPolicyList, error)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `GameServerAllocationPoliciesGetter` (L35), `GameServerAllocationPolicyInterface` (L40), `gameServerAllocationPolicies` (L54), `newGameServerAllocationPolicies` (L59). The complete list remains in `agones:pkg/client/clientset/versioned/typed/multicluster/v1/gameserverallocationpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-882259d3888371b7"></a>


### File `pkg/client/clientset/versioned/typed/multicluster/v1/generated_expansion.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8b59a457ed8fba226905127890da9b1e04d55f83`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/multicluster/v1/generated_expansion.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a89e1419613a33afbc.** [agones/pkg/client/clientset/versioned/typed/multicluster/v1/generated_expansion.go, lines 20–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/multicluster/v1/generated_expansion.go#L20-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4c17da1dbf2ba734b594d917dbbdaf7622db042c1ac24ff431aa310ad20aa872`.

> 
> type GameServerAllocationPolicyExpansion interface{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 1 lexically identified declarations, including `GameServerAllocationPolicyExpansion` (L21). The complete list remains in `agones:pkg/client/clientset/versioned/typed/multicluster/v1/generated_expansion.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-0b46ef7b9b6d6b4a"></a>


### File `pkg/client/clientset/versioned/typed/multicluster/v1/multicluster_client.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `142d5c7b761c79f38148407afe67f418397603d1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/clientset/versioned/typed/multicluster/v1/multicluster_client.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0c83be5266d53a92dc.** [agones/pkg/client/clientset/versioned/typed/multicluster/v1/multicluster_client.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/clientset/versioned/typed/multicluster/v1/multicluster_client.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1e91ffc36880c068ba3d999174e24cfe30a8996e63781969aff3cf157c6cef7f`.

> 
> type MulticlusterV1Interface interface {
> 	RESTClient() rest.Interface
> 	GameServerAllocationPoliciesGetter
> }
> 
> // MulticlusterV1Client is used to interact with features provided by the multicluster.agones.dev group.
> type MulticlusterV1Client struct {
> 	restClient rest.Interface
> }
> 
> func (c *MulticlusterV1Client) GameServerAllocationPolicies(namespace string) GameServerAllocationPolicyInterface {
> 	return newGameServerAllocationPolicies(c, namespace)
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 9 lexically identified declarations, including `MulticlusterV1Interface` (L29), `MulticlusterV1Client` (L35), `GameServerAllocationPolicies` (L39), `NewForConfig` (L46), `NewForConfigAndClient` (L58), `NewForConfigOrDie` (L70), `New` (L79), `setConfigDefaults` (L83), `RESTClient` (L96). The complete list remains in `agones:pkg/client/clientset/versioned/typed/multicluster/v1/multicluster_client.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/informers/externalversions` -> module `pkg::client::informers::externalversions`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-2374437f15ce95d1"></a>


### File `pkg/client/informers/externalversions/factory.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e3b46bf65de59f62ee2696bb168b9770c327841f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/factory.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ade601cd3db311304c.** [agones/pkg/client/informers/externalversions/factory.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/factory.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bafa2d9ce7c3f2686dcbd175b67b21fad6b31e87d91b6dbc7b3bf50df199ca55`.

> 
> // SharedInformerOption defines the functional option type for SharedInformerFactory.
> type SharedInformerOption func(*sharedInformerFactory) *sharedInformerFactory
> 
> type sharedInformerFactory struct {
> 	client           versioned.Interface
> 	namespace        string
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> 	lock             sync.Mutex
> 	defaultResync    time.Duration
> 	customResync     map[reflect.Type]time.Duration
> 	transform        cache.TransformFunc
> 
> 	informers map[reflect.Type]cache.SharedIndexInformer
> 	// startedInformers is used for tracking which informers have been started.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 17 lexically identified declarations, including `SharedInformerOption` (L38), `sharedInformerFactory` (L40), `WithCustomResyncConfig` (L61), `WithTweakListOptions` (L71), `WithNamespace` (L79), `WithTransform` (L87), `NewSharedInformerFactory` (L95), `NewFilteredSharedInformerFactory` (L104), `NewSharedInformerFactoryWithOptions` (L109), `Start` (L127), `Shutdown` (L151), `WaitForCacheSync` (L160), `InformerFor` (L183), `SharedInformerFactory` (L229), `Agones` (L265), `Autoscaling` (L269), `Multicluster` (L273). The complete list remains in `agones:pkg/client/informers/externalversions/factory.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-6432a7e84edf905d"></a>


### File `pkg/client/informers/externalversions/generic.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d403174da2a02780d44dd272dd866ae4fe3b1f44`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/generic.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e8031112497cc93434.** [agones/pkg/client/informers/externalversions/generic.go, lines 30–44](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/generic.go#L30-L44). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f5165f6501849435d40ed6adce5db00c7899ad2e18b26c811063746347a495b`.

> 
> // GenericInformer is type of SharedIndexInformer which will locate and delegate to other
> // sharedInformers based on type
> type GenericInformer interface {
> 	Informer() cache.SharedIndexInformer
> 	Lister() cache.GenericLister
> }
> 
> type genericInformer struct {
> 	informer cache.SharedIndexInformer
> 	resource schema.GroupResource
> }
> 
> // Informer returns the SharedIndexInformer.
> func (f *genericInformer) Informer() cache.SharedIndexInformer {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 5 lexically identified declarations, including `GenericInformer` (L33), `genericInformer` (L38), `Informer` (L44), `Lister` (L49), `ForResource` (L55). The complete list remains in `agones:pkg/client/informers/externalversions/generic.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/informers/externalversions/agones` -> module `pkg::client::informers::externalversions::agones`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-1b1373d1049b9b0a"></a>


### File `pkg/client/informers/externalversions/agones/interface.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7dac49c137406c5f714add2c45df5cf43a6da9e8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/agones/interface.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-cf73a7ea4681c54bee.** [agones/pkg/client/informers/externalversions/agones/interface.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/agones/interface.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `33e070202f8a8fba017f1dd8f01104ed4b3680e945d8dcfca12ab0070c24d435`.

> 
> // Interface provides access to each of this group's versions.
> type Interface interface {
> 	// V1 provides access to shared informers for resources in V1.
> 	V1() v1.Interface
> }
> 
> type group struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	namespace        string
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> }
> 
> // New returns a new Interface.
> func New(f internalinterfaces.SharedInformerFactory, namespace string, tweakListOptions internalinterfaces.TweakListOptionsFunc) Interface {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `Interface` (L27), `group` (L32), `New` (L39), `V1` (L44). The complete list remains in `agones:pkg/client/informers/externalversions/agones/interface.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/informers/externalversions/agones/v1` -> module `pkg::client::informers::externalversions::agones::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-f0105fbdb713f09e"></a>


### File `pkg/client/informers/externalversions/agones/v1/fleet.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ef7b92ccb59cfa4a746b42bcedc324fc18355875`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/agones/v1/fleet.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b83e7dc6882ba70fdd.** [agones/pkg/client/informers/externalversions/agones/v1/fleet.go, lines 34–48](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/agones/v1/fleet.go#L34-L48). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `972b2a7eb41ad5e3abd0288d68d405306a396677b7d1834549f17b4625d657b2`.

> 
> // FleetInformer provides access to a shared informer and lister for
> // Fleets.
> type FleetInformer interface {
> 	Informer() cache.SharedIndexInformer
> 	Lister() agonesv1.FleetLister
> }
> 
> type fleetInformer struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> 	namespace        string
> }
> 
> // NewFleetInformer constructs a new informer for Fleet type.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 7 lexically identified declarations, including `FleetInformer` (L37), `fleetInformer` (L42), `NewFleetInformer` (L51), `NewFilteredFleetInformer` (L58), `defaultInformer` (L92), `Informer` (L96), `Lister` (L100). The complete list remains in `agones:pkg/client/informers/externalversions/agones/v1/fleet.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-57d15f62021b8a26"></a>


### File `pkg/client/informers/externalversions/agones/v1/gameserver.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2db93d80fc27952a6db8848718e878e519745d16`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/agones/v1/gameserver.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a4ed113f90da9481a3.** [agones/pkg/client/informers/externalversions/agones/v1/gameserver.go, lines 34–48](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/agones/v1/gameserver.go#L34-L48). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2f2c08928fad4eac8b87e294b74fb870057ca62e2eed09e3fc8f2b614b8343d0`.

> 
> // GameServerInformer provides access to a shared informer and lister for
> // GameServers.
> type GameServerInformer interface {
> 	Informer() cache.SharedIndexInformer
> 	Lister() agonesv1.GameServerLister
> }
> 
> type gameServerInformer struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> 	namespace        string
> }
> 
> // NewGameServerInformer constructs a new informer for GameServer type.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 7 lexically identified declarations, including `GameServerInformer` (L37), `gameServerInformer` (L42), `NewGameServerInformer` (L51), `NewFilteredGameServerInformer` (L58), `defaultInformer` (L92), `Informer` (L96), `Lister` (L100). The complete list remains in `agones:pkg/client/informers/externalversions/agones/v1/gameserver.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-9b93d66979e88eec"></a>


### File `pkg/client/informers/externalversions/agones/v1/gameserverset.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5cab107596c6b544fbf1e546f786fa9bf6deccd5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/agones/v1/gameserverset.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0ab479b9e0403c1f37.** [agones/pkg/client/informers/externalversions/agones/v1/gameserverset.go, lines 34–48](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/agones/v1/gameserverset.go#L34-L48). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9d28ca9a8a88949c7a53d5335470d23d534e42483269772115b92956c3ce5bd4`.

> 
> // GameServerSetInformer provides access to a shared informer and lister for
> // GameServerSets.
> type GameServerSetInformer interface {
> 	Informer() cache.SharedIndexInformer
> 	Lister() agonesv1.GameServerSetLister
> }
> 
> type gameServerSetInformer struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> 	namespace        string
> }
> 
> // NewGameServerSetInformer constructs a new informer for GameServerSet type.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 7 lexically identified declarations, including `GameServerSetInformer` (L37), `gameServerSetInformer` (L42), `NewGameServerSetInformer` (L51), `NewFilteredGameServerSetInformer` (L58), `defaultInformer` (L92), `Informer` (L96), `Lister` (L100). The complete list remains in `agones:pkg/client/informers/externalversions/agones/v1/gameserverset.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-f6a45ab505cbaf57"></a>


### File `pkg/client/informers/externalversions/agones/v1/interface.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `146bcf3b5512aecbc0f75bfdb366b1cb8ba4ae15`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/agones/v1/interface.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-dff9a1339e77a332e4.** [agones/pkg/client/informers/externalversions/agones/v1/interface.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/agones/v1/interface.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `39f86cb8e71e371844a5160ac3a602da1966232bfc0075340086902a1d573ff2`.

> 
> // Interface provides access to all the informers in this group version.
> type Interface interface {
> 	// Fleets returns a FleetInformer.
> 	Fleets() FleetInformer
> 	// GameServers returns a GameServerInformer.
> 	GameServers() GameServerInformer
> 	// GameServerSets returns a GameServerSetInformer.
> 	GameServerSets() GameServerSetInformer
> }
> 
> type version struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	namespace        string
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `Interface` (L26), `version` (L35), `New` (L42), `Fleets` (L47), `GameServers` (L52), `GameServerSets` (L57). The complete list remains in `agones:pkg/client/informers/externalversions/agones/v1/interface.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/informers/externalversions/autoscaling` -> module `pkg::client::informers::externalversions::autoscaling`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-d0177e2815ada647"></a>


### File `pkg/client/informers/externalversions/autoscaling/interface.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e5607c737018b34574643672f940023e1741ed27`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/autoscaling/interface.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-15236c0059484dd5b6.** [agones/pkg/client/informers/externalversions/autoscaling/interface.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/autoscaling/interface.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `eefd4e9d009299f6bbdd55172dbe104aa8c61ebaf119a41c03d95f006ea787df`.

> 
> // Interface provides access to each of this group's versions.
> type Interface interface {
> 	// V1 provides access to shared informers for resources in V1.
> 	V1() v1.Interface
> }
> 
> type group struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	namespace        string
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> }
> 
> // New returns a new Interface.
> func New(f internalinterfaces.SharedInformerFactory, namespace string, tweakListOptions internalinterfaces.TweakListOptionsFunc) Interface {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `Interface` (L27), `group` (L32), `New` (L39), `V1` (L44). The complete list remains in `agones:pkg/client/informers/externalversions/autoscaling/interface.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/informers/externalversions/autoscaling/v1` -> module `pkg::client::informers::externalversions::autoscaling::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-e9ed4c7bc8cefca4"></a>


### File `pkg/client/informers/externalversions/autoscaling/v1/fleetautoscaler.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d8f41f199f2429ef122c8b14d2509b89a13f5713`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/autoscaling/v1/fleetautoscaler.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-9aec815ace3050ea03.** [agones/pkg/client/informers/externalversions/autoscaling/v1/fleetautoscaler.go, lines 34–48](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/autoscaling/v1/fleetautoscaler.go#L34-L48). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `94cb44d37f392194bb5ae19b0be17d28ba4c563b735a75f398a933c3ea643687`.

> 
> // FleetAutoscalerInformer provides access to a shared informer and lister for
> // FleetAutoscalers.
> type FleetAutoscalerInformer interface {
> 	Informer() cache.SharedIndexInformer
> 	Lister() autoscalingv1.FleetAutoscalerLister
> }
> 
> type fleetAutoscalerInformer struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> 	namespace        string
> }
> 
> // NewFleetAutoscalerInformer constructs a new informer for FleetAutoscaler type.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 7 lexically identified declarations, including `FleetAutoscalerInformer` (L37), `fleetAutoscalerInformer` (L42), `NewFleetAutoscalerInformer` (L51), `NewFilteredFleetAutoscalerInformer` (L58), `defaultInformer` (L92), `Informer` (L96), `Lister` (L100). The complete list remains in `agones:pkg/client/informers/externalversions/autoscaling/v1/fleetautoscaler.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-e287c424fe9b2406"></a>


### File `pkg/client/informers/externalversions/autoscaling/v1/interface.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `eabda0bf05531274af5446aa2423588a85dd9ae2`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/autoscaling/v1/interface.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-500f4d6a3c4e2562cf.** [agones/pkg/client/informers/externalversions/autoscaling/v1/interface.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/autoscaling/v1/interface.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `756a13cb19f7f52e8edafd143a67a487b8633b0cc9ca233f76937f7272d353f9`.

> 
> // Interface provides access to all the informers in this group version.
> type Interface interface {
> 	// FleetAutoscalers returns a FleetAutoscalerInformer.
> 	FleetAutoscalers() FleetAutoscalerInformer
> }
> 
> type version struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	namespace        string
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> }
> 
> // New returns a new Interface.
> func New(f internalinterfaces.SharedInformerFactory, namespace string, tweakListOptions internalinterfaces.TweakListOptionsFunc) Interface {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `Interface` (L26), `version` (L31), `New` (L38), `FleetAutoscalers` (L43). The complete list remains in `agones:pkg/client/informers/externalversions/autoscaling/v1/interface.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/informers/externalversions/internalinterfaces` -> module `pkg::client::informers::externalversions::internalinterfaces`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-712d1d2c1a230bdb"></a>


### File `pkg/client/informers/externalversions/internalinterfaces/factory_interfaces.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f3b8aaa0cdeba44dad46526a3bd1781edec9f33f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/internalinterfaces/factory_interfaces.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-84a01f61cd3c846b5a.** [agones/pkg/client/informers/externalversions/internalinterfaces/factory_interfaces.go, lines 29–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/internalinterfaces/factory_interfaces.go#L29-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3a1eef2394fd3a976292d03b0f3766cef396bdb575acac3ea2dedaaf4e84ca54`.

> 
> // NewInformerFunc takes versioned.Interface and time.Duration to return a SharedIndexInformer.
> type NewInformerFunc func(versioned.Interface, time.Duration) cache.SharedIndexInformer
> 
> // SharedInformerFactory a small interface to allow for adding an informer without an import cycle
> type SharedInformerFactory interface {
> 	Start(stopCh <-chan struct{})
> 	InformerFor(obj runtime.Object, newFunc NewInformerFunc) cache.SharedIndexInformer
> }
> 
> // TweakListOptionsFunc is a function that transforms a v1.ListOptions.
> type TweakListOptionsFunc func(*v1.ListOptions)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 3 lexically identified declarations, including `NewInformerFunc` (L31), `SharedInformerFactory` (L34), `TweakListOptionsFunc` (L40). The complete list remains in `agones:pkg/client/informers/externalversions/internalinterfaces/factory_interfaces.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/informers/externalversions/multicluster` -> module `pkg::client::informers::externalversions::multicluster`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-6ebe7c0b29e8bdb4"></a>


### File `pkg/client/informers/externalversions/multicluster/interface.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f5ec5f2309571cde0e9fcd4ab930d6848d2c29b4`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/multicluster/interface.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-37827d071bc691c23e.** [agones/pkg/client/informers/externalversions/multicluster/interface.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/multicluster/interface.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a2a73cf83fb8e4b9c722dec9811d766d00ad9db2161bc7a8a497b43cdd9bcea7`.

> 
> // Interface provides access to each of this group's versions.
> type Interface interface {
> 	// V1 provides access to shared informers for resources in V1.
> 	V1() v1.Interface
> }
> 
> type group struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	namespace        string
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> }
> 
> // New returns a new Interface.
> func New(f internalinterfaces.SharedInformerFactory, namespace string, tweakListOptions internalinterfaces.TweakListOptionsFunc) Interface {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `Interface` (L27), `group` (L32), `New` (L39), `V1` (L44). The complete list remains in `agones:pkg/client/informers/externalversions/multicluster/interface.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/informers/externalversions/multicluster/v1` -> module `pkg::client::informers::externalversions::multicluster::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-84102e38c25bd6a8"></a>


### File `pkg/client/informers/externalversions/multicluster/v1/gameserverallocationpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e9a913c7039535949200b91f84517f66e2f31df9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/multicluster/v1/gameserverallocationpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-87fa0d927c1a550056.** [agones/pkg/client/informers/externalversions/multicluster/v1/gameserverallocationpolicy.go, lines 34–48](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/multicluster/v1/gameserverallocationpolicy.go#L34-L48). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `06feb620153e24db3595a807d89b6d101d983ce920342e931be715edb7351ac6`.

> 
> // GameServerAllocationPolicyInformer provides access to a shared informer and lister for
> // GameServerAllocationPolicies.
> type GameServerAllocationPolicyInformer interface {
> 	Informer() cache.SharedIndexInformer
> 	Lister() multiclusterv1.GameServerAllocationPolicyLister
> }
> 
> type gameServerAllocationPolicyInformer struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> 	namespace        string
> }
> 
> // NewGameServerAllocationPolicyInformer constructs a new informer for GameServerAllocationPolicy type.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 7 lexically identified declarations, including `GameServerAllocationPolicyInformer` (L37), `gameServerAllocationPolicyInformer` (L42), `NewGameServerAllocationPolicyInformer` (L51), `NewFilteredGameServerAllocationPolicyInformer` (L58), `defaultInformer` (L92), `Informer` (L96), `Lister` (L100). The complete list remains in `agones:pkg/client/informers/externalversions/multicluster/v1/gameserverallocationpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-cb00887aa41e2a64"></a>


### File `pkg/client/informers/externalversions/multicluster/v1/interface.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b63c3ae6bde46be0ff1600a9199f5565a7300b57`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/informers/externalversions/multicluster/v1/interface.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-770d6025277af358a0.** [agones/pkg/client/informers/externalversions/multicluster/v1/interface.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/informers/externalversions/multicluster/v1/interface.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b5197488123cebdec05efe7df3785c52bdda5a507e1c884e8a762ec939e0631a`.

> 
> // Interface provides access to all the informers in this group version.
> type Interface interface {
> 	// GameServerAllocationPolicies returns a GameServerAllocationPolicyInformer.
> 	GameServerAllocationPolicies() GameServerAllocationPolicyInformer
> }
> 
> type version struct {
> 	factory          internalinterfaces.SharedInformerFactory
> 	namespace        string
> 	tweakListOptions internalinterfaces.TweakListOptionsFunc
> }
> 
> // New returns a new Interface.
> func New(f internalinterfaces.SharedInformerFactory, namespace string, tweakListOptions internalinterfaces.TweakListOptionsFunc) Interface {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 4 lexically identified declarations, including `Interface` (L26), `version` (L31), `New` (L38), `GameServerAllocationPolicies` (L43). The complete list remains in `agones:pkg/client/informers/externalversions/multicluster/v1/interface.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/listers/agones/v1` -> module `pkg::client::listers::agones::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-715015e87adc6157"></a>


### File `pkg/client/listers/agones/v1/expansion_generated.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `239a9222d89eb9b4133ffbd4acdd484197a6b9e5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/listers/agones/v1/expansion_generated.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e0e3deca524d868d6f.** [agones/pkg/client/listers/agones/v1/expansion_generated.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/listers/agones/v1/expansion_generated.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6bd01e8e495ae43a67a3cff08c54266df1ef362cb9b1deefd3a72eb84a105d44`.

> 
> // FleetListerExpansion allows custom methods to be added to
> // FleetLister.
> type FleetListerExpansion interface{}
> 
> // FleetNamespaceListerExpansion allows custom methods to be added to
> // FleetNamespaceLister.
> type FleetNamespaceListerExpansion interface{}
> 
> // GameServerListerExpansion allows custom methods to be added to
> // GameServerLister.
> type GameServerListerExpansion interface{}
> 
> // GameServerNamespaceListerExpansion allows custom methods to be added to
> // GameServerNamespaceLister.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `FleetListerExpansion` (L23), `FleetNamespaceListerExpansion` (L27), `GameServerListerExpansion` (L31), `GameServerNamespaceListerExpansion` (L35), `GameServerSetListerExpansion` (L39), `GameServerSetNamespaceListerExpansion` (L43). The complete list remains in `agones:pkg/client/listers/agones/v1/expansion_generated.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-62fa6556f445bd32"></a>


### File `pkg/client/listers/agones/v1/fleet.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `bf2fff4ea0d4ce7001feec429efcf1c3f26d9fd5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/listers/agones/v1/fleet.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ed04718f7320708aef.** [agones/pkg/client/listers/agones/v1/fleet.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/listers/agones/v1/fleet.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `28b5822bf61090de792e2542f5394e17c55e258bedb7ae7769157c69e82c1162`.

> 
> // FleetLister helps list Fleets.
> // All objects returned here must be treated as read-only.
> type FleetLister interface {
> 	// List lists all Fleets in the indexer.
> 	// Objects returned here must be treated as read-only.
> 	List(selector labels.Selector) (ret []*agonesv1.Fleet, err error)
> 	// Fleets returns an object that can list and get Fleets.
> 	Fleets(namespace string) FleetNamespaceLister
> 	FleetListerExpansion
> }
> 
> // fleetLister implements the FleetLister interface.
> type fleetLister struct {
> 	listers.ResourceIndexer[*agonesv1.Fleet]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `FleetLister` (L30), `fleetLister` (L40), `NewFleetLister` (L45), `Fleets` (L50), `FleetNamespaceLister` (L56), `fleetNamespaceLister` (L68). The complete list remains in `agones:pkg/client/listers/agones/v1/fleet.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-5ea36afd75c75f40"></a>


### File `pkg/client/listers/agones/v1/gameserver.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `35e4c224a09f43e88d117758d5992dd2809a9c6a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/listers/agones/v1/gameserver.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a736141d9756be56dc.** [agones/pkg/client/listers/agones/v1/gameserver.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/listers/agones/v1/gameserver.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `97a41ebfed646e0e4451ce4028851990bb05bb9183ec1126cb3ba808d9234e59`.

> 
> // GameServerLister helps list GameServers.
> // All objects returned here must be treated as read-only.
> type GameServerLister interface {
> 	// List lists all GameServers in the indexer.
> 	// Objects returned here must be treated as read-only.
> 	List(selector labels.Selector) (ret []*agonesv1.GameServer, err error)
> 	// GameServers returns an object that can list and get GameServers.
> 	GameServers(namespace string) GameServerNamespaceLister
> 	GameServerListerExpansion
> }
> 
> // gameServerLister implements the GameServerLister interface.
> type gameServerLister struct {
> 	listers.ResourceIndexer[*agonesv1.GameServer]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `GameServerLister` (L30), `gameServerLister` (L40), `NewGameServerLister` (L45), `GameServers` (L50), `GameServerNamespaceLister` (L56), `gameServerNamespaceLister` (L68). The complete list remains in `agones:pkg/client/listers/agones/v1/gameserver.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-d0de03770065a830"></a>


### File `pkg/client/listers/agones/v1/gameserverset.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `45a25ef435b30511583d59fd02c136d4b1d92809`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/listers/agones/v1/gameserverset.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-3c6303ad7eda692ee6.** [agones/pkg/client/listers/agones/v1/gameserverset.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/listers/agones/v1/gameserverset.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b276d456a6ac33a34097f85262a8b469e0394583282805d28fd56d14d444f80f`.

> 
> // GameServerSetLister helps list GameServerSets.
> // All objects returned here must be treated as read-only.
> type GameServerSetLister interface {
> 	// List lists all GameServerSets in the indexer.
> 	// Objects returned here must be treated as read-only.
> 	List(selector labels.Selector) (ret []*agonesv1.GameServerSet, err error)
> 	// GameServerSets returns an object that can list and get GameServerSets.
> 	GameServerSets(namespace string) GameServerSetNamespaceLister
> 	GameServerSetListerExpansion
> }
> 
> // gameServerSetLister implements the GameServerSetLister interface.
> type gameServerSetLister struct {
> 	listers.ResourceIndexer[*agonesv1.GameServerSet]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `GameServerSetLister` (L30), `gameServerSetLister` (L40), `NewGameServerSetLister` (L45), `GameServerSets` (L50), `GameServerSetNamespaceLister` (L56), `gameServerSetNamespaceLister` (L68). The complete list remains in `agones:pkg/client/listers/agones/v1/gameserverset.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/listers/autoscaling/v1` -> module `pkg::client::listers::autoscaling::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-b5335b48279ec3e0"></a>


### File `pkg/client/listers/autoscaling/v1/expansion_generated.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5a99cca8fd670deec475868289d1fe186a8a7e9a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/listers/autoscaling/v1/expansion_generated.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b05eeef3c2962f6367.** [agones/pkg/client/listers/autoscaling/v1/expansion_generated.go, lines 20–27](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/listers/autoscaling/v1/expansion_generated.go#L20-L27). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6c821c3eb8509b3714419f8f72614a853b11d184e633131d5821d77985e4608e`.

> 
> // FleetAutoscalerListerExpansion allows custom methods to be added to
> // FleetAutoscalerLister.
> type FleetAutoscalerListerExpansion interface{}
> 
> // FleetAutoscalerNamespaceListerExpansion allows custom methods to be added to
> // FleetAutoscalerNamespaceLister.
> type FleetAutoscalerNamespaceListerExpansion interface{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 2 lexically identified declarations, including `FleetAutoscalerListerExpansion` (L23), `FleetAutoscalerNamespaceListerExpansion` (L27). The complete list remains in `agones:pkg/client/listers/autoscaling/v1/expansion_generated.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b93ca497dc6d9eb4"></a>


### File `pkg/client/listers/autoscaling/v1/fleetautoscaler.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `21c039afc952ad9b193296b5873bdb62a6ea92b2`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/listers/autoscaling/v1/fleetautoscaler.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a99f7693f6cb520f43.** [agones/pkg/client/listers/autoscaling/v1/fleetautoscaler.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/listers/autoscaling/v1/fleetautoscaler.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7bec2801d27475ae422217d6030e99f6368fb62c3ef4413ef3cd507906941c9c`.

> 
> // FleetAutoscalerLister helps list FleetAutoscalers.
> // All objects returned here must be treated as read-only.
> type FleetAutoscalerLister interface {
> 	// List lists all FleetAutoscalers in the indexer.
> 	// Objects returned here must be treated as read-only.
> 	List(selector labels.Selector) (ret []*autoscalingv1.FleetAutoscaler, err error)
> 	// FleetAutoscalers returns an object that can list and get FleetAutoscalers.
> 	FleetAutoscalers(namespace string) FleetAutoscalerNamespaceLister
> 	FleetAutoscalerListerExpansion
> }
> 
> // fleetAutoscalerLister implements the FleetAutoscalerLister interface.
> type fleetAutoscalerLister struct {
> 	listers.ResourceIndexer[*autoscalingv1.FleetAutoscaler]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `FleetAutoscalerLister` (L30), `fleetAutoscalerLister` (L40), `NewFleetAutoscalerLister` (L45), `FleetAutoscalers` (L50), `FleetAutoscalerNamespaceLister` (L56), `fleetAutoscalerNamespaceLister` (L68). The complete list remains in `agones:pkg/client/listers/autoscaling/v1/fleetautoscaler.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:pkg/client/listers/multicluster/v1` -> module `pkg::client::listers::multicluster::v1`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-81ec6dc939d3b374"></a>


### File `pkg/client/listers/multicluster/v1/expansion_generated.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1f8ccd857e44ac10813b6631d39552f2eabe0e84`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/listers/multicluster/v1/expansion_generated.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-416eea9884e3c540dc.** [agones/pkg/client/listers/multicluster/v1/expansion_generated.go, lines 20–27](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/listers/multicluster/v1/expansion_generated.go#L20-L27). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3652a02c445d28fd87f6f75097572ea848f0b7831b3e856fd4379a69b226561e`.

> 
> // GameServerAllocationPolicyListerExpansion allows custom methods to be added to
> // GameServerAllocationPolicyLister.
> type GameServerAllocationPolicyListerExpansion interface{}
> 
> // GameServerAllocationPolicyNamespaceListerExpansion allows custom methods to be added to
> // GameServerAllocationPolicyNamespaceLister.
> type GameServerAllocationPolicyNamespaceListerExpansion interface{}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 2 lexically identified declarations, including `GameServerAllocationPolicyListerExpansion` (L23), `GameServerAllocationPolicyNamespaceListerExpansion` (L27). The complete list remains in `agones:pkg/client/listers/multicluster/v1/expansion_generated.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-0f9c1a47cb44dfd0"></a>


### File `pkg/client/listers/multicluster/v1/gameserverallocationpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3e109e9900e4a4980c11ba166993319f4dc03c50`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/client/listers/multicluster/v1/gameserverallocationpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d6c4efac8ec5f3ce09.** [agones/pkg/client/listers/multicluster/v1/gameserverallocationpolicy.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/client/listers/multicluster/v1/gameserverallocationpolicy.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `da3767c2de94aacd9616f078d8acc358f6884d2af5c30ceb11085033cc67d991`.

> 
> // GameServerAllocationPolicyLister helps list GameServerAllocationPolicies.
> // All objects returned here must be treated as read-only.
> type GameServerAllocationPolicyLister interface {
> 	// List lists all GameServerAllocationPolicies in the indexer.
> 	// Objects returned here must be treated as read-only.
> 	List(selector labels.Selector) (ret []*multiclusterv1.GameServerAllocationPolicy, err error)
> 	// GameServerAllocationPolicies returns an object that can list and get GameServerAllocationPolicies.
> 	GameServerAllocationPolicies(namespace string) GameServerAllocationPolicyNamespaceLister
> 	GameServerAllocationPolicyListerExpansion
> }
> 
> // gameServerAllocationPolicyLister implements the GameServerAllocationPolicyLister interface.
> type gameServerAllocationPolicyLister struct {
> 	listers.ResourceIndexer[*multiclusterv1.GameServerAllocationPolicy]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.


4. Account for 6 lexically identified declarations, including `GameServerAllocationPolicyLister` (L30), `gameServerAllocationPolicyLister` (L40), `NewGameServerAllocationPolicyLister` (L45), `GameServerAllocationPolicies` (L50), `GameServerAllocationPolicyNamespaceLister` (L56), `gameServerAllocationPolicyNamespaceLister` (L68). The complete list remains in `agones:pkg/client/listers/multicluster/v1/gameserverallocationpolicy.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

## Upstream platform contract reference

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-85fef07f5bca7ff6a3.** [kubernetes-website/content/en/docs/reference/using-api/api-concepts.md, lines 23–36](https://github.com/kubernetes/website/blob/77904cb3ec33988ae7416fbf4b5f35baba407dc9/content/en/docs/reference/using-api/api-concepts.md#L23-L36). Revision `77904cb3ec33988ae7416fbf4b5f35baba407dc9`; source SHA-256 `5bafa029a965f06eabee03ccc1e89fae9d527c6f258823e77aaa45e00d99b08a`.

> _watches_:
> {{< glossary_definition prepend="in the Kubernetes API, watch is" term_id="watch" length="short" >}}
> Kubernetes also provides consistent list operations so that API clients can
> effectively cache, track, and synchronize the state of resources.
> 
> You can view the [API reference](/docs/reference/kubernetes-api/) online,
> or read on to learn about the API in general.
> 
> <!-- body -->
> ## Kubernetes API terminology {#standard-api-terminology}
> 
> Kubernetes generally leverages common RESTful terminology to describe the
> API concepts:

**Derived requirement.** Preserve resource/watch/reconciliation observations at the Rusternetes capability boundary. This source describes Kubernetes contracts; it does not authorize a second orchestrator integration or a pre-gate probe.
