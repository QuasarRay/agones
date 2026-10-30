<a id="m0017"></a>


# M0017 — Telemetry and metric semantics


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0017-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0015](0015-fleet-reconciliation-and-rollout-policy.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d20197f20d5941a48b.** [agones/site/content/en/docs/Guides/metrics.md, lines 6–18](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/metrics.md#L6-L18). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b566dd89ea173711f01def9b3817e1134f5c9beab831dfb0aba5377b17d12ded`.

> 
> Agones controller exposes metrics via [OpenCensus](https://opencensus.io/). OpenCensus is a single distribution of libraries that collect metrics and distributed traces from your services, we only use it for metrics but it will allow us to support multiple exporters in the future.
> 
> We choose to start with [Prometheus](https://prometheus.io/) as this is the most popular with Kubernetes but it is also compatible with Cloud Monitoring.
> If you need another exporter, check the [list of supported](https://opencensus.io/exporters/supported-exporters/go/) exporters. It should be pretty straightforward to register a new one. (GitHub PRs are more than welcome.)
> 
> We plan to support multiple exporters in the future via environment variables and helm flags.
> 
> ## Backend integrations
> 
> ### Prometheus
> 
> If you are running a [Prometheus](https://prometheus.io/) instance you just need to ensure that metrics and kubernetes service discovery are enabled. (helm chart values `agones.metrics.prometheusEnabled` and `agones.metrics.prometheusServiceDiscovery`). This will automatically add annotations required by Prometheus to discover Agones metrics and start collecting them. (see [example](https://github.com/prometheus/prometheus/tree/main/documentation/examples/kubernetes-rabbitmq))


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Port metric names, labels, counters/gauges/histograms, registration lifetimes, aggregation and source update sites. Preserve values for lifecycle events and errors; do not silently reduce dimensionality.

2. Keep observation policy and collector lifetime in CGP providers. Generate repetitive descriptions and projections from derive-deftly declarations without calculating expected values from a separate implementation.

3. Use readable named transformations for fleet/GameServer observations. Metrics should observe production state, not own a competing copy of that state.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.




## Architecture, readability and authored outputs


1. Own 8 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:pkg/metrics` -> module `pkg::metrics`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-e568fc8794b952f7ae.** [agones/pkg/metrics/doc.go, lines 15–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/doc.go#L15-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c3d4d2623cc0c8ecda1165c2af0b47a82ec402bdbe258d6d95c30d21993343de`.

> // Package metrics records stats of agones controllers
> package metrics


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-523d31ee4813d411"></a>


### File `pkg/metrics/controller.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `22ae56c7119e4e46189f0b3e1fd80f66d002cb7b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/metrics/controller.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e2a90789167a796d76.** [agones/pkg/metrics/controller.go, lines 67–81](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/controller.go#L67-L81). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bcd63e28988bd7793685c446325652c7b225d44b353ad2308ad1ca8c8be280c4`.

> 
> func init() {
> 	registerViews()
> }
> 
> // Controller is a metrics controller collecting Agones state metrics
> //
> //nolint:govet // ignore fieldalignment, singleton
> type Controller struct {
> 	logger                    *logrus.Entry
> 	errs                      *errors.Errors
> 	gameServerLister          listerv1.GameServerLister
> 	nodeLister                v1.NodeLister
> 	gameServerSynced          cache.InformerSynced
> 	fleetSynced               cache.InformerSynced


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones metric identities, labels, event values, aggregation and collector lifetime. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 22 lexically identified declarations, including `init` (L68), `Controller` (L75), `NewController` (L95), `recordFleetAutoScalerChanges` (L161), `recordFleetAutoScalerDeletion` (L226), `recordFleetChanges` (L237), `recordFleetRolloutPercentage` (L262), `filterGameServerSetByActive` (L295), `recordFleetDeletion` (L310), `resyncFleets` (L325), `resyncFleetAutoScaler` (L351), `recordFleetReplicas` (L368), `recordCounters` (L385), `recordLists` (L402), `recordGameServerStatusChanges` (L426), `calcDuration` (L480), `Run` (L524), `collect` (L535), `collectGameServerCounts` (L544), `collectNodeCounts` (L558), `removeSystemNodes` (L589), `isSystemNode` (L602). The complete list remains in `agones:pkg/metrics/controller.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-f01bed7f99ecf7eb"></a>


### File `pkg/metrics/controller_metrics.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e83a9e0836dd28e9d3cf8e52107670209dc7a367`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/metrics/controller_metrics.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-43990898a4c6244303.** [agones/pkg/metrics/controller_metrics.go, lines 199–213](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/controller_metrics.go#L199-L213). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `cd0a11532c3799e56186e5595907a94ff738ec4637d54c9bfd66641d28442341`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones metric identities, labels, event values, aggregation and collector lifetime. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 3 lexically identified declarations, including `registerViews` (L201), `unRegisterViews` (L210), `resetViews` (L219). The complete list remains in `agones:pkg/metrics/controller_metrics.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-56ee327965dc1f1f"></a>


### File `pkg/metrics/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a17bd5bad38d46e92322f4dbc7d21375ba62285e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/metrics/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2f05b555f76af4874c.** [agones/pkg/metrics/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c3d4d2623cc0c8ecda1165c2af0b47a82ec402bdbe258d6d95c30d21993343de`.

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
> // Package metrics records stats of agones controllers


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones metric identities, labels, event values, aggregation and collector lifetime. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b719b7acd9addcf8"></a>


### File `pkg/metrics/exporter.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4aefe3f85e41fa9f39ba31490ef8832c47f698cd`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/metrics/exporter.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-73afc34eba321a0a58.** [agones/pkg/metrics/exporter.go, lines 33–47](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/exporter.go#L33-L47). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `794f63b9b2e56caa98eeebbb3c068d241e0ee48dc67edd262eb417e2a556dc48`.

> 
> // Config holds configuration for metrics reporting
> type Config struct {
> 	GCPProjectID      string
> 	StackdriverLabels string
> 	Stackdriver       bool
> 	PrometheusMetrics bool
> }
> 
> // RegisterPrometheusExporter register a prometheus exporter to OpenCensus with a given prometheus metric registry.
> // It will automatically add go runtime and process metrics using default prometheus collectors.
> // The function return an http.handler that you can use to expose the prometheus endpoint.
> func RegisterPrometheusExporter(registry *prom.Registry) (http.Handler, error) {
> 	pe, err := prometheus.NewExporter(prometheus.Options{
> 		Namespace: "agones",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones metric identities, labels, event values, aggregation and collector lifetime. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 6 lexically identified declarations, including `Config` (L35), `RegisterPrometheusExporter` (L45), `RegisterStackdriverExporter` (L66), `SetReportingPeriod` (L94), `getMonitoredResource` (L108), `SetupMetrics` (L134). The complete list remains in `agones:pkg/metrics/exporter.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-d877c199d24df47c"></a>


### File `pkg/metrics/gameservers_count.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9c9c677882c1bc43a6880d64a1854b75e9e966db`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/metrics/gameservers_count.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-db072f7d0367a4b869.** [agones/pkg/metrics/gameservers_count.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/gameservers_count.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c96ecd84ea4218b42aca546fca2552ac4f10b393c4de924a0cfae2b75340acfc`.

> 
> // GameServerCount  is the count of gameserver per current state and per fleet name
> type GameServerCount map[agonesv1.GameServerState]map[fleetKey]int64
> 
> type fleetKey struct {
> 	name      string
> 	namespace string
> }
> 
> // increment adds the count of gameservers for a given fleetName and state
> func (c GameServerCount) increment(key fleetKey, state agonesv1.GameServerState) {
> 	fleets, ok := c[state]
> 	if !ok {
> 		fleets = map[fleetKey]int64{}
> 		c[state] = fleets


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones metric identities, labels, event values, aggregation and collector lifetime. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 6 lexically identified declarations, including `defaultFleetTag` (L26), `GameServerCount` (L29), `fleetKey` (L31), `increment` (L37), `reset` (L47), `record` (L56). The complete list remains in `agones:pkg/metrics/gameservers_count.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-e406932d57728cd0"></a>


### File `pkg/metrics/kubernetes_client.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1ddc0fcf1ccdf80793736540d5167a4db4ddd443`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/metrics/kubernetes_client.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9ceaf0aeb45ddddcaa.** [agones/pkg/metrics/kubernetes_client.go, lines 54–68](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/kubernetes_client.go#L54-L68). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `752e189ae0e3e778a7b3cae781b56f975720049332f1b1e855cd9df8c0c7a3f6`.

> 
> func init() {
> 	distributionSeconds := []float64{0, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1, 2, 3}
> 	distributionNumbers := []float64{0, 10, 50, 100, 150, 250, 300}
> 
> 	runtime.Must(view.Register(&view.View{
> 		Name:        "k8s_client_http_request_total",
> 		Measure:     httpRequestTotalStats,
> 		Description: "The total of HTTP requests to the Kubernetes API by status code",
> 		Aggregation: view.Count(),
> 		TagKeys:     []tag.Key{keyVerb, keyStatusCode},
> 	}))
> 
> 	runtime.Must(view.Register(&view.View{
> 		Name:        "k8s_client_http_request_duration_seconds",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones metric identities, labels, event values, aggregation and collector lifetime. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 36 lexically identified declarations, including `init` (L55), `clientGoMetricAdapter` (L192), `Register` (L194), `Increment` (L202), `Observe` (L207), `ocMetric` (L214), `newOcMetric` (L219), `withTag` (L226), `Inc` (L235), `Dec` (L239), `observeFunc` (L245), `Observe` (L247), `Observe` (L251), `Set` (L255), `NewListsMetric` (L259), `NewListDurationMetric` (L263), `NewItemsInListMetric` (L267), `NewWatchesMetric` (L271), `NewShortWatchesMetric` (L275), `NewWatchDurationMetric` (L279), `NewItemsInWatchMetric` (L283), `NewLastResourceVersionMetric` (L287), `NewDepthMetric` (L291), `NewAddsMetric` (L295). The complete list remains in `agones:pkg/metrics/kubernetes_client.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


10. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-419604e174551049"></a>


### File `pkg/metrics/metrics.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `38e5b37f2dd822eeca1461ef729942a41cbf13bb`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/metrics/metrics.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-597a5c929e11b1ae33.** [agones/pkg/metrics/metrics.go, lines 18–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/metrics.go#L18-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `55aacde8982b0b064c058bcb87edd57783cad7fbe51738e02d7d4d0dc84c5e0d`.

> 
> var errs = errors.FromPackage()


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones metric identities, labels, event values, aggregation and collector lifetime. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 1 lexically identified declarations, including `errs` (L19). The complete list remains in `agones:pkg/metrics/metrics.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-68bfe538d4e9fc10"></a>


### File `pkg/metrics/util.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b3c4bb9ef982985ad84b119221188ad4d98b4abe`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/metrics/util.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-8af1895b203c40fcfe.** [agones/pkg/metrics/util.go, lines 44–58](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/metrics/util.go#L44-L58). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9597b9f7bf81b6b2226f7cce184c07e3757dd8c0222879d65165c0c11c579a66`.

> 
> // RecordWithTags records a metric value and tags
> func RecordWithTags(ctx context.Context, mutators []tag.Mutator, ms ...stats.Measurement) {
> 	if err := stats.RecordWithTags(ctx, mutators, ms...); err != nil {
> 		logger.WithError(err).Warn("error while recoding stats")
> 	}
> }
> 
> // MustTagKey creates a new `tag.Key` from a string, panic if the key is not a valid.
> func MustTagKey(key string) tag.Key {
> 	t, err := tag.NewKey(key)
> 	if err != nil {
> 		panic(err)
> 	}
> 	return t


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones metric identities, labels, event values, aggregation and collector lifetime. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 3 lexically identified declarations, including `RecordWithTags` (L46), `MustTagKey` (L53), `parseLabels` (L61). The complete list remains in `agones:pkg/metrics/util.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

