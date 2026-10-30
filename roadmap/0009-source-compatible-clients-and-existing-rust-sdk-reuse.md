<a id="m0009"></a>


# M0009 — Source-compatible clients and existing Rust SDK reuse


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0009-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Reuse the existing Rust SDK while accounting for first-party Go SDK behavior and public protocol compatibility. Preserve client construction, endpoints, retries, health loops, streams, errors and shutdown.

2. Keep existing language SDKs as compatibility references; this contract does not authorize wholesale rewrites of every language client. Give each retained path an explicit inventory disposition.

3. Preserve Rust SDK filenames and APIs where already compatible. Any required source-derived Go translation retains its exact Go-relative path; do not create two independently maintained canonical client semantics.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.




## Architecture, readability and authored outputs


1. Own 27 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:sdks/go` -> module `sdks::go`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-0282466776fd77d7"></a>


### File `sdks/go/alpha.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6fb2fd30ef5949365a4036c7318e1bed2f2ddbfc`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/sdks/go/alpha.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f6b39b0e1f8fb091c7.** [agones/sdks/go/alpha.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/go/alpha.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a8d30b8bc73cfd92221f725c16eaaef2822f146923c3551a8c73031d0a3d343f`.

> 
> // Alpha is the struct for Alpha SDK functionality.
> type Alpha struct {
> 	client alpha.SDKClient
> 	errs   *errors.Errors
> }
> 
> // newAlpha creates a new Alpha SDK with the passed in connection.
> func newAlpha(conn *grpc.ClientConn) *Alpha {
> 	a := &Alpha{client: alpha.NewSDKClient(conn)}
> 	a.errs = errors.FromStruct(a)
> 	return a
> }
> 
> // GetPlayerCapacity gets the last player capacity that was set through the SDK.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 9 lexically identified declarations, including `Alpha` (L27), `newAlpha` (L33), `GetPlayerCapacity` (L41), `SetPlayerCapacity` (L47), `PlayerConnect` (L55), `PlayerDisconnect` (L63), `GetPlayerCount` (L69), `IsPlayerConnected` (L76), `GetConnectedPlayers` (L83). The complete list remains in `agones:sdks/go/alpha.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-9dde2e3b221d5bc2"></a>


### File `sdks/go/beta.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `659a13a0e167624ea599bf7376263adfc2bfe423`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/sdks/go/beta.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-589f61a5ae9ed342ab.** [agones/sdks/go/beta.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/go/beta.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f4a95720fb71156d027b0f583f8eb39668d0cc39e92cb0606d710cee0f274768`.

> 
> // Beta is the struct for Beta SDK functionality.
> type Beta struct {
> 	client beta.SDKClient
> 	errs   *errors.Errors
> }
> 
> // newBeta creates a new Beta SDK with the passed in connection.
> func newBeta(conn *grpc.ClientConn) *Beta {
> 	b := &Beta{client: beta.NewSDKClient(conn)}
> 	b.errs = errors.FromStruct(b)
> 	return b
> }
> 
> // GetCounterCount returns the Count for a Counter, given the Counter's key (name).


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 15 lexically identified declarations, including `Beta` (L30), `newBeta` (L36), `GetCounterCount` (L44), `IncrementCounter` (L61), `DecrementCounter` (L79), `SetCounterCount` (L96), `GetCounterCapacity` (L110), `SetCounterCapacity` (L119), `GetListCapacity` (L133), `SetListCapacity` (L143), `ListContains` (L160), `GetListLength` (L173), `GetListValues` (L183), `AppendListValue` (L194), `DeleteListValue` (L205). The complete list remains in `agones:sdks/go/beta.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-28535ba08d4b0895"></a>


### File `sdks/go/sdk.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d96ba669c497e531a677736ce973ffaebbe37810`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/sdks/go/sdk.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-20fc98f1a30d051c60.** [agones/sdks/go/sdk.go, lines 32–46](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/go/sdk.go#L32-L46). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ecf7737ddff8f8fef42e324fc54166fd8a059edf2e37f74ec38072315abefeb0`.

> 
> // GameServerCallback is a function definition to be called
> // when a GameServer CRD has been changed.
> type GameServerCallback func(gs *sdk.GameServer)
> 
> // SDK is an instance of the Agones SDK.
> type SDK struct {
> 	client sdk.SDKClient
> 	ctx    context.Context
> 	health sdk.SDK_HealthClient
> 	alpha  *Alpha
> 	beta   *Beta
> 	errs   *errors.Errors
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 16 lexically identified declarations, including `GameServerCallback` (L35), `SDK` (L38), `ErrorLog` (L48), `Logger` (L51), `NewSDK` (L59), `Alpha` (L90), `Beta` (L95), `Ready` (L100), `Allocate` (L106), `Shutdown` (L112), `Reserve` (L120), `Health` (L126), `SetLabel` (L131), `SetAnnotation` (L138), `GameServer` (L145), `WatchGameServer` (L153). The complete list remains in `agones:sdks/go/sdk.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust` -> module `sdks::rust`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-8b874e01e27c108a"></a>


### File `sdks/rust/.gitignore`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `438618e59458cdbdb9a30e338b95c81423236cfa`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/.gitignore`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-736f5ac2a06f1ea207.** [agones/sdks/rust/.gitignore, lines 7–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/.gitignore#L7-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8161bbd07d6b1d9fb6223733fe674d7e5040beb0344c045173682e48d89c3ce9`.

> /target/
> 
> # Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
> # More information here http://doc.crates.io/guide.html#cargotoml-vs-cargolock
> Cargo.lock
> 
> # These are backup files generated by rustfmt
> **/*.rs.bk
> 
> # End of https://www.gitignore.io/api/rust


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-d8b4905ec038926d"></a>


### File `sdks/rust/Cargo.toml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `08f11f0034ea45eeb7a8cd42d46b02cce86aebf7`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/Cargo.toml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c9433926438362e1c7.** [agones/sdks/rust/Cargo.toml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/Cargo.toml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6868f8140e2269fcb790d71a14ca9874be68a88e94c61c44e09c2061eaa15b19`.

> 
> [package]
> name = "agones"
> version = "1.62.0-dev"
> edition = "2021"
> description = "The Rust SDK for Agones."
> license = "Apache-2.0"
> repository = "https://github.com/agones-dev/agones"
> documentation = "https://docs.rs/agones"
> homepage = "https://agones.dev/site/"
> 
> [dependencies]
> async-stream = "0.3.6"
> http = "1.1.0"
> prost = "0.13.3"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-dfc2ef2de7f7c5e3"></a>


### File `sdks/rust/README.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `04deafae29d6248eb4188695cc2c67a182b7fba4`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71ca5b96aa4b754d66.** [agones/sdks/rust/README.md, lines 2–7](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/README.md#L2-L7). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `537dd9f727ffea4bd46411b548ed67b6df81e7afa3d70a8917667c6cfe438edb`.

> 
> This is the Rust version of the Agones Game Server Client SDK. 
> 
> See the [Rust SDK Documentation](https://agones.dev/site/docs/guides/client-sdks/rust/) for details
> 
> The [Client SDK Documentation](https://agones.dev/site/docs/guides/client-sdks/) has details on each of the SDK functions and how to run the SDK locally.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-f8a3552d90738cf5"></a>


### File `sdks/rust/build.rs`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a78640da70304d22cc42240b9cbcf38775605c85`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/build.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d14967a8473ae4dcf4.** [agones/sdks/rust/build.rs, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/build.rs#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `eff40e73bbabdaa6e6fec232606c809cfc9b5a98e4ca3c951155309a526d7724`.

> fn main() {
>     tonic_build::configure()
>         // The SDK is just a client, no need to build the server types
>         .build_server(false)
>         .compile_protos(
>             &[
>                 "proto/sdk/alpha/alpha.proto",
>                 "proto/sdk/beta/beta.proto",
>                 "proto/sdk/sdk.proto"],
>             &[
>                 "proto/googleapis",
>                 "proto/grpc-gateway",
>                 "proto/sdk/alpha",
>                 "proto/sdk/beta",
>                 "proto/sdk",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/proto/allocation` -> module `sdks::rust::proto::allocation`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-1086005f5736b8c8"></a>


### File `sdks/rust/proto/allocation/allocation.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e04ae88eac3d98523cd0a31c3a3f783305db9cb5`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/allocation/allocation.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-01371eac4cf4a0a746.** [agones/sdks/rust/proto/allocation/allocation.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/allocation/allocation.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `56c766f0ff04b549a1ec4e0cfa3005b0e7dc099fb252bf3c15aaf459f4db7413`.

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
> syntax = "proto3";
> 
> package allocation;
> option go_package = "./allocation";
> 
> import "google/api/annotations.proto";
> import "google/protobuf/wrappers.proto";
> import "protoc-gen-openapiv2/options/annotations.proto";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-cffb23bb991f0201"></a>


### File `sdks/rust/proto/allocation/processor.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e6dd2f6badfc6d1559799e34fcad62f6b971942f`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/allocation/processor.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a4aada7c86d16ba558.** [agones/sdks/rust/proto/allocation/processor.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/allocation/processor.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `457d492a1e79b22764f0ab602dd6edfebd92812333d99c57c936ba7554f6e659`.

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
> syntax = "proto3";
> 
> package allocation;
> option go_package = "./allocation";
> 
> import "google/rpc/status.proto";
> import "proto/allocation/allocation.proto";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/proto/googleapis/google/api` -> module `sdks::rust::proto::googleapis::google::api`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-0b48546594f71b8b"></a>


### File `sdks/rust/proto/googleapis/google/api/annotations.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `005561ec53ec4c741303fd43d6effe2d7bbd0382`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/googleapis/google/api/annotations.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-3f18aebcde73047c32.** [agones/sdks/rust/proto/googleapis/google/api/annotations.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/googleapis/google/api/annotations.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `30d7139338bca730deca74a7a466321796fa21aa389212bcd64c9ee983ba5e4b`.

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
> syntax = "proto3";
> 
> package google.api;
> 
> import "google/api/http.proto";
> import "google/protobuf/descriptor.proto";
> 
> option go_package = "google.golang.org/genproto/googleapis/api/annotations;annotations";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-94ccca451d9c7fe3"></a>


### File `sdks/rust/proto/googleapis/google/api/client.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a1c8589fec52e19f3ee0f32ecbe5c62e9b8cf733`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/googleapis/google/api/client.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-84a728ae0425943e43.** [agones/sdks/rust/proto/googleapis/google/api/client.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/googleapis/google/api/client.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bc5653b7d9aa77138d41a03dc3a42e7a810b722df829aad2f499406fb53f7d0b`.

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
> syntax = "proto3";
> 
> package google.api;
> 
> import "google/api/launch_stage.proto";
> import "google/protobuf/descriptor.proto";
> import "google/protobuf/duration.proto";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-529b78e43bd57fc7"></a>


### File `sdks/rust/proto/googleapis/google/api/field_behavior.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ba3d40842cb458a4bd97139811bd5967c3e79a1d`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/googleapis/google/api/field_behavior.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-218a16d071844bb08c.** [agones/sdks/rust/proto/googleapis/google/api/field_behavior.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/googleapis/google/api/field_behavior.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c6ff35bf6b8e9cabd314b1df91ff6ede5465285be7bd890df450216b31a201df`.

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
> syntax = "proto3";
> 
> package google.api;
> 
> import "google/protobuf/descriptor.proto";
> 
> option go_package = "google.golang.org/genproto/googleapis/api/annotations;annotations";
> option java_multiple_files = true;


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-12970267172b68fb"></a>


### File `sdks/rust/proto/googleapis/google/api/http.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fa6ea99cb2ae1bef9001434d5da0b97baf8c7152`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/googleapis/google/api/http.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-5f87a5e98344180ae2.** [agones/sdks/rust/proto/googleapis/google/api/http.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/googleapis/google/api/http.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e10cf2b54f310df0cd7d0ed951c8cc5f8578943efb1177832fa25fc19ce912c`.

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
> syntax = "proto3";
> 
> package google.api;
> 
> option cc_enable_arenas = true;
> option go_package = "google.golang.org/genproto/googleapis/api/annotations;annotations";
> option java_multiple_files = true;
> option java_outer_classname = "HttpProto";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-347bcd5bcaf3c886"></a>


### File `sdks/rust/proto/googleapis/google/api/launch_stage.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6c96d953607a470f3b866bd06c9a442aa444a9ba`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/googleapis/google/api/launch_stage.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a1aebef5662179354d.** [agones/sdks/rust/proto/googleapis/google/api/launch_stage.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/googleapis/google/api/launch_stage.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `583df1f2d101a9a3ae58c7d0f6595611e25ca94ae9d08b51481707a60368a1d7`.

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
> syntax = "proto3";
> 
> package google.api;
> 
> option go_package = "google.golang.org/genproto/googleapis/api;api";
> option java_multiple_files = true;
> option java_outer_classname = "LaunchStageProto";
> option java_package = "com.google.api";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-a51c8700e84f6ab0"></a>


### File `sdks/rust/proto/googleapis/google/api/resource.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c30716023bc8804a68eae76e14a3bebbef4c7db6`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/googleapis/google/api/resource.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d6bcda36735a27523b.** [agones/sdks/rust/proto/googleapis/google/api/resource.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/googleapis/google/api/resource.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6ff1872529183a885557e566e47f21bbc57a60e13de4bab4b06727d1007d5189`.

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
> syntax = "proto3";
> 
> package google.api;
> 
> import "google/protobuf/descriptor.proto";
> 
> option cc_enable_arenas = true;
> option go_package = "google.golang.org/genproto/googleapis/api/annotations;annotations";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/proto/googleapis/google/rpc` -> module `sdks::rust::proto::googleapis::google::rpc`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-bbab7e8181ed7d80"></a>


### File `sdks/rust/proto/googleapis/google/rpc/status.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4be0a0b44017ff020ea709e9c1c3ef72c6801e67`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/googleapis/google/rpc/status.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-8703caa842fb139ff4.** [agones/sdks/rust/proto/googleapis/google/rpc/status.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/googleapis/google/rpc/status.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6c557c45ae6d4a2632abbda025c43455d2da3d650d96a61a463ff2dc07b9b929`.

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
> syntax = "proto3";
> 
> package google.rpc;
> 
> import "google/protobuf/any.proto";
> 
> option cc_enable_arenas = true;
> option go_package = "google.golang.org/genproto/googleapis/rpc/status;status";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/proto/grpc-gateway` -> module `sdks::rust::proto::grpc-gateway`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-06529915b3c0efca"></a>


### File `sdks/rust/proto/grpc-gateway/LICENSE.txt`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6e59e318a1f4e6d97680254b0c5faca3d82f74c7`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/grpc-gateway/LICENSE.txt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-77e3edec1c4552854a.** [agones/sdks/rust/proto/grpc-gateway/LICENSE.txt, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/grpc-gateway/LICENSE.txt#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b8bb2030523f09d9f08c756aa0d41856ec2e345ace892630d41f16ed09f79ad5`.

> All rights reserved.
> 
> Redistribution and use in source and binary forms, with or without modification,
> are permitted provided that the following conditions are met:
> 
>     * Redistributions of source code must retain the above copyright notice,
>       this list of conditions and the following disclaimer.
> 
>     * Redistributions in binary form must reproduce the above copyright notice,
>       this list of conditions and the following disclaimer in the documentation
>       and/or other materials provided with the distribution.
> 
>     * Neither the name of Gengo, Inc. nor the names of its
>       contributors may be used to endorse or promote products derived from this
>       software without specific prior written permission.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options` -> module `sdks::rust::proto::grpc-gateway::protoc-gen-openapiv2::options`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-59f4768c588abb29"></a>


### File `sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options/annotations.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9d4f4c99586a318ecf50b3436f4ca5cfc28d6a54`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options/annotations.proto`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e52597e2c9703bde16.** [agones/sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options/annotations.proto, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options/annotations.proto#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7c2cecba02fb94926a5d2a9914fe0253f32f76c5b554508835719312e2ab7aab`.

> syntax = "proto3";
> 
> package grpc.gateway.protoc_gen_openapiv2.options;
> 
> import "google/protobuf/descriptor.proto";
> import "protoc-gen-openapiv2/options/openapiv2.proto";
> 
> option go_package = "github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2/options";
> 
> extend google.protobuf.FileOptions {
>   // ID assigned by protobuf-global-extension-registry@google.com for gRPC-Gateway project.
>   //
>   // All IDs are the same, as assigned. It is okay that they are the same, as they extend
>   // different descriptor messages.
>   Swagger openapiv2_swagger = 1042;


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-2d1c4d55945c1b62"></a>


### File `sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options/openapiv2.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1f01c94a34a420e04bf0cc5507406fce59287aeb`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options/openapiv2.proto`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1bbd209b62078cc0e7.** [agones/sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options/openapiv2.proto, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/grpc-gateway/protoc-gen-openapiv2/options/openapiv2.proto#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `759fd88829980b5aaac025070314d43d1fa9f77f5d678a4116d96645e965e569`.

> syntax = "proto3";
> 
> package grpc.gateway.protoc_gen_openapiv2.options;
> 
> import "google/protobuf/struct.proto";
> 
> option go_package = "github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2/options";
> 
> // Scheme describes the schemes supported by the OpenAPI Swagger
> // and Operation objects.
> enum Scheme {
>   UNKNOWN = 0;
>   HTTP = 1;
>   HTTPS = 2;
>   WS = 3;


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/proto/sdk` -> module `sdks::rust::proto::sdk`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-be6d1a01fa1e5f36"></a>


### File `sdks/rust/proto/sdk/sdk.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7b08d1163fc71e4b5a5d22d2f1ae866c80b93157`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/sdk/sdk.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-5ae0cb7b52965e8322.** [agones/sdks/rust/proto/sdk/sdk.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/sdk/sdk.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f5a4a7a8e9937ed9cf7d992141d3fb90f734ad058999a89b4a66f4ab9de096e0`.

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
> syntax = "proto3";
> 
> package agones.dev.sdk;
> option go_package = "./sdk";
> 
> import "google/api/annotations.proto";
> import "protoc-gen-openapiv2/options/annotations.proto";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/proto/sdk/alpha` -> module `sdks::rust::proto::sdk::alpha`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-41f91a4f54cbba59"></a>


### File `sdks/rust/proto/sdk/alpha/alpha.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fb75ed5ae01f7eddf39a7d018191d4ae0ddcf854`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/sdk/alpha/alpha.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-94b544a1bce9edfb4f.** [agones/sdks/rust/proto/sdk/alpha/alpha.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/sdk/alpha/alpha.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `aabffc0a0371f37302b55f5dc77bb1bdeec14dd16415ca2e4810a607e7c05dc5`.

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
> syntax = "proto3";
> 
> package agones.dev.sdk.alpha;
> option go_package = "./alpha";
> 
> import "google/api/annotations.proto";
> import "protoc-gen-openapiv2/options/annotations.proto";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/proto/sdk/beta` -> module `sdks::rust::proto::sdk::beta`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-623b5400a4fe5a1d"></a>


### File `sdks/rust/proto/sdk/beta/beta.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `97c0b61976ee8221e891e97c1084ae65c9e96a1d`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/proto/sdk/beta/beta.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-91daed2433551e6fe3.** [agones/sdks/rust/proto/sdk/beta/beta.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/proto/sdk/beta/beta.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ab8722d966ca45080659e935f6e42f6c5681d10917edd3399d0ca260caa114b4`.

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
> syntax = "proto3";
> 
> package agones.dev.sdk.beta;
> option go_package = "./beta";
> 
> import "google/api/annotations.proto";
> import "google/api/client.proto";
> import "google/api/field_behavior.proto";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:sdks/rust/src` -> module `sdks::rust::src`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-ba7686e27b32cfa9ec.** [agones/site/content/en/docs/Guides/Client SDKs/rust.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/rust.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8f3f6f1a10f2e12db257b57434c47768f878626058a38af7a72477bf62d76f7f`.

> 
> Check the [Client SDK Documentation]({{< relref "_index.md" >}}) for more details on each of the SDK functions and how to run the SDK locally.
> 
> ## SDK Functionality
> 
> {{% feature publishVersion="1.52.0" %}}
> | Area            | Action              | Implemented |
> |-----------------|---------------------|-------------|
> | Lifecycle       | Ready               | ✔️          |
> | Lifecycle       | Health              | ✔️          |
> | Lifecycle       | Reserve             | ✔️          |
> | Lifecycle       | Allocate            | ✔️          |
> | Lifecycle       | Shutdown            | ✔️          |


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-3c6bb47d6a505747"></a>


### File `sdks/rust/src/alpha.rs`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3003429f769cb76848cabbdb4651031b0a8eb01b`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/src/alpha.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-33ab486466e2986df9.** [agones/sdks/rust/src/alpha.rs, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/src/alpha.rs#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `df8aeeb867a51d8a876eeac78834dd350ef74ad173d9a4ad6decfc8b08959f1a`.

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
> use tonic::transport::Channel;
> 
> mod api {
>     tonic::include_proto!("agones.dev.sdk.alpha");
> }
> 
> use api::sdk_client::SdkClient;


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-3e8a35ff3d4f9060"></a>


### File `sdks/rust/src/beta.rs`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `16ed9f9eed004edb044185bb6cc8fc934fda04f2`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/src/beta.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-facfac2343c4cd9cfa.** [agones/sdks/rust/src/beta.rs, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/src/beta.rs#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4b2ffcc0b2fb2bd7250cb05cddd368056c72fe1c8284fe4a657b74a935fb24fb`.

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
> use crate::errors::Result;
> use tonic::transport::Channel;
> 
> mod api {
>     tonic::include_proto!("agones.dev.sdk.beta");
> }
> 
> use api::sdk_client::SdkClient;


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


5. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-0e443321873b551e"></a>


### File `sdks/rust/src/errors.rs`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3184cfbfdd890fedc2ba47471176dde52e588276`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/src/errors.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-65451806a2297ad520.** [agones/sdks/rust/src/errors.rs, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/src/errors.rs#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `422f156dd93d0ee8ed9d37f806f38c34d7ff2d68f6fb7f0b7d12cd13e66e7c59`.

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
> pub type Result<T> = std::result::Result<T, Error>;
> 
> #[derive(thiserror::Error, Debug)]
> pub enum Error {
>     #[error("health ping connection failure: `{0}`")]
>     HealthPingConnectionFailure(String),
>     #[error(transparent)]
>     TimedOut(#[from] tokio::time::error::Elapsed),


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-ad2356f926b98a47"></a>


### File `sdks/rust/src/lib.rs`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1e556858326f569d453fe536820d812cf095742b`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/src/lib.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f1602d1fb6eeaf371d.** [agones/sdks/rust/src/lib.rs, lines 5–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/src/lib.rs#L5-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `740d986c703c3bef88cd44be3f6439324fa23ec0706b9f8edd52cfe7bd12b2c8`.

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
> //! the Rust game server SDK
> 
> pub mod alpha;
> pub mod beta;
> pub mod errors;
> mod sdk;
> 
> pub use sdk::{GameServer, ObjectMeta, PlayerStatus, Port, Sdk, Spec, Status};


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-0d4cb0144baa246f"></a>


### File `sdks/rust/src/sdk.rs`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d045291ac58e2d8579b10d8565f94019c9cfe0f1`. **Disposition:** `REUSE_RUST_SDK`. **Destination:** `agones-rs/sdks/rust/src/sdk.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-52e8af8539b33db088.** [agones/sdks/rust/src/sdk.rs, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/sdks/rust/src/sdk.rs#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7fc7abe02b39022dc1d84321577a6834365ca42f81c24cdc67789074937e74ec`.

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
> use std::{env, time::Duration};
> use tonic::transport::Channel;
> 
> mod api {
>     tonic::include_proto!("agones.dev.sdk");
> }
> 
> use api::sdk_client::SdkClient;


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones SDK requests, endpoints, streaming, health and shutdown contracts. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

