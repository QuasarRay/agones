<a id="m0005"></a>


# M0005 — Resource and wire contracts, defaults and validation


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0005-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0001](0001-source-mirror-version-and-shared-contracts.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Account for every proto message, enum, service, streaming direction, field number, presence rule, error/status mapping and compatibility version. Treat generated .pb.go and gateway files as regeneration obligations, not disposable source.

2. Preserve alpha/beta/stable SDK boundaries, Health/WatchGameServer stream behavior, allocation messages and JSON gateway mapping. Keep protocol types distinct from Rusternetes storage objects.

3. Reuse existing Rust protocol generation and SDK types where their source mapping is known. Genco/derive-deftly own repeated bindings; retain exact source basenames such as sdk.pb.rs through explicit module registration.

4. Implement the exact API groups and versions for GameServer, GameServerSet, Fleet, FleetAutoscaler, GameServerAllocation and GameServerAllocationPolicy. Preserve metadata, selectors, resource versions, deprecated served fields and feature gates.

5. Port constructors, defaulting, validation, state transitions, copy semantics and generated schema/deep-copy support. Keep nil, omitted, empty and default values distinct; retain source integer widths and error paths.

6. Assign resource lifecycle decisions to CGP providers and use derive-deftly templates for repeated resource/accessor/conversion structure. Rust public wire compatibility takes precedence over renaming fields for appearance; explain the mapping for Python readers.


## Architecture, readability and authored outputs


1. Own 57 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:pkg/allocation/go` -> module `pkg::allocation::go`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-91db28d9d0d3f3d3"></a>


### File `pkg/allocation/go/allocation.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `26a9387d4aa3e0712e054efca7d5370fbca143a8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/allocation/go/allocation.pb.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2668fcd0dc1513e3b6.** [agones/pkg/allocation/go/allocation.pb.go, lines 55–69](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/allocation/go/allocation.pb.go#L55-L69). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `362fd4901053a13adff62e26c21ac4b08803cf2bef0c8d2e11cc0d17b88b7fdd`.

> 
> type AllocationRequest_SchedulingStrategy int32
> 
> const (
> 	AllocationRequest_Packed      AllocationRequest_SchedulingStrategy = 0
> 	AllocationRequest_Distributed AllocationRequest_SchedulingStrategy = 1
> )
> 
> // Enum value maps for AllocationRequest_SchedulingStrategy.
> var (
> 	AllocationRequest_SchedulingStrategy_name = map[int32]string{
> 		0: "Packed",
> 		1: "Distributed",
> 	}
> 	AllocationRequest_SchedulingStrategy_value = map[string]int32{


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 205 lexically identified declarations, including `AllocationRequest_SchedulingStrategy` (L56), `Enum` (L75), `String` (L81), `Descriptor` (L85), `Type` (L89), `Number` (L93), `EnumDescriptor` (L98), `LabelMatchExpressions_Operator` (L104), `Enum` (L129), `String` (L135), `Descriptor` (L139), `Type` (L143), `Number` (L147), `EnumDescriptor` (L152), `GameServerSelector_GameServerState` (L156), `Enum` (L175), `String` (L181), `Descriptor` (L185), `Type` (L189), `Number` (L193), `EnumDescriptor` (L198), `Priority_Type` (L202), `Enum` (L221), `String` (L227). The full declaration list is recorded under `agones:pkg/allocation/go/allocation.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ebed78b5d12d742b"></a>


### File `pkg/allocation/go/allocation.pb.gw.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8fa6962d45b11dfb0a01b3556977d773abd35f1c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/allocation/go/allocation.pb.gw.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a7600e18962fe37981.** [agones/pkg/allocation/go/allocation.pb.gw.go, lines 52–66](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/allocation/go/allocation.pb.gw.go#L52-L66). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a852bfe56111a71ed90c60cedbafcb0c7c4b4eccde34715e84c96de2dbb68e02`.

> 
> func request_AllocationService_Allocate_0(ctx context.Context, marshaler runtime.Marshaler, client AllocationServiceClient, req *http.Request, pathParams map[string]string) (proto.Message, runtime.ServerMetadata, error) {
> 	var (
> 		protoReq AllocationRequest
> 		metadata runtime.ServerMetadata
> 	)
> 	if err := marshaler.NewDecoder(req.Body).Decode(&protoReq); err != nil && !errors.Is(err, io.EOF) {
> 		return nil, metadata, status.Errorf(codes.InvalidArgument, "%v", err)
> 	}
> 	if req.Body != nil {
> 		_, _ = io.Copy(io.Discard, req.Body)
> 	}
> 	msg, err := client.Allocate(ctx, &protoReq, grpc.Header(&metadata.HeaderMD), grpc.Trailer(&metadata.TrailerMD))
> 	return msg, metadata, err
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 6 lexically identified declarations, including `request_AllocationService_Allocate_0` (L53), `local_request_AllocationService_Allocate_0` (L68), `RegisterAllocationServiceHandlerServer` (L85), `RegisterAllocationServiceHandlerFromEndpoint` (L112), `RegisterAllocationServiceHandler` (L136), `RegisterAllocationServiceHandlerClient` (L145). The full declaration list is recorded under `agones:pkg/allocation/go/allocation.pb.gw.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

8. Preserve source floating width, narrowing, exceptional values and relevant bits.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-18e7565012a350f6"></a>


### File `pkg/allocation/go/allocation_grpc.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1fe07a0fe59cf0e646ff43c84cdd8e12871ae68d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/allocation/go/allocation_grpc.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-586bbd6b35bce104b3.** [agones/pkg/allocation/go/allocation_grpc.pb.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/allocation/go/allocation_grpc.pb.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c3081054c806cf8c9ef85f52fa1d725f12095b08fd8efa9c55cedd02ce895651`.

> 
> // AllocationServiceClient is the client API for AllocationService service.
> //
> // For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
> type AllocationServiceClient interface {
> 	Allocate(ctx context.Context, in *AllocationRequest, opts ...grpc.CallOption) (*AllocationResponse, error)
> }
> 
> type allocationServiceClient struct {
> 	cc grpc.ClientConnInterface
> }
> 
> func NewAllocationServiceClient(cc grpc.ClientConnInterface) AllocationServiceClient {
> 	return &allocationServiceClient{cc}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 12 lexically identified declarations, including `_` (L35), `AllocationServiceClient` (L40), `allocationServiceClient` (L44), `NewAllocationServiceClient` (L48), `Allocate` (L52), `AllocationServiceServer` (L64), `UnimplementedAllocationServiceServer` (L69), `Allocate` (L72), `UnsafeAllocationServiceServer` (L79), `RegisterAllocationServiceServer` (L83), `_AllocationService_Allocate_Handler` (L87), `AllocationService_ServiceDesc` (L108). The full declaration list is recorded under `agones:pkg/allocation/go/allocation_grpc.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-26f6ed182273ce81"></a>


### File `pkg/allocation/go/processor.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `80462abfe8a11bd5b83b8ad89a9c87f65bee486c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/allocation/go/processor.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a5f1fd84e589d88628.** [agones/pkg/allocation/go/processor.pb.go, lines 53–67](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/allocation/go/processor.pb.go#L53-L67). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c91c8298b8248ad8f1cd260462b1e8223e257063a0b758e79fd8afb47f017d57`.

> 
> // The ProcessorMessage is used to send and receive messages between the client and server.
> type ProcessorMessage struct {
> 	state         protoimpl.MessageState
> 	sizeCache     protoimpl.SizeCache
> 	unknownFields protoimpl.UnknownFields
> 
> 	// The client ID is used to identify the client that is sending the message.
> 	ClientId string `protobuf:"bytes,1,opt,name=client_id,json=clientId,proto3" json:"client_id,omitempty"`
> 	// The payload is a oneof field that can contain either a PullRequest, Batch, or BatchResponse.
> 	//
> 	// Types that are assignable to Payload:
> 	//
> 	//	*ProcessorMessage_Pull
> 	//	*ProcessorMessage_BatchRequest


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 72 lexically identified declarations, including `ProcessorMessage` (L55), `Reset` (L72), `String` (L81), `ProtoMessage` (L85), `ProtoReflect` (L87), `Descriptor` (L100), `GetClientId` (L104), `GetPayload` (L111), `GetPull` (L118), `GetBatchRequest` (L125), `GetBatchResponse` (L132), `isProcessorMessage_Payload` (L139), `ProcessorMessage_Pull` (L143), `ProcessorMessage_BatchRequest` (L148), `ProcessorMessage_BatchResponse` (L153), `isProcessorMessage_Payload` (L158), `isProcessorMessage_Payload` (L160), `isProcessorMessage_Payload` (L162), `PullRequest` (L165), `Reset` (L173), `String` (L182), `ProtoMessage` (L186), `ProtoReflect` (L188), `Descriptor` (L201). The full declaration list is recorded under `agones:pkg/allocation/go/processor.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-97a3656f712e6a71"></a>


### File `pkg/allocation/go/processor_grpc.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4dba53819275fa8a0220a854c4e1901e63d43dfe`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/allocation/go/processor_grpc.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-5bc5b8f4c0f12fd9b6.** [agones/pkg/allocation/go/processor_grpc.pb.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/allocation/go/processor_grpc.pb.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0a91cb64c13a5b093e38b44506fa1c9db3c07c9c33e35240c5a0e13043d08ca6`.

> 
> // ProcessorClient is the client API for Processor service.
> //
> // For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
> type ProcessorClient interface {
> 	// StreamBatches processes a stream of ProcessorMessage requests and returns a stream of ProcessorMessage responses.
> 	StreamBatches(ctx context.Context, opts ...grpc.CallOption) (Processor_StreamBatchesClient, error)
> }
> 
> type processorClient struct {
> 	cc grpc.ClientConnInterface
> }
> 
> func NewProcessorClient(cc grpc.ClientConnInterface) ProcessorClient {
> 	return &processorClient{cc}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `_` (L35), `ProcessorClient` (L40), `processorClient` (L45), `NewProcessorClient` (L49), `StreamBatches` (L53), `Processor_StreamBatchesClient` (L62), `processorStreamBatchesClient` (L68), `Send` (L72), `Recv` (L76), `ProcessorServer` (L87), `UnimplementedProcessorServer` (L93), `StreamBatches` (L96), `UnsafeProcessorServer` (L103), `RegisterProcessorServer` (L107), `_Processor_StreamBatches_Handler` (L111), `Processor_StreamBatchesServer` (L115), `processorStreamBatchesServer` (L121), `Send` (L125), `Recv` (L129), `Processor_ServiceDesc` (L140). The full declaration list is recorded under `agones:pkg/allocation/go/processor_grpc.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis` -> module `pkg::apis`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-1208d0cf83f87fa466.** [agones/pkg/apis/doc.go, lines 15–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/doc.go#L15-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7a68012b836912ebaa843d4053adcb2de5aac2a1bae5c8216dc88cfc534068a4`.

> // Package apis contains all the types for the Custom Resource Definitions
> package apis


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-8ba3cbb8472678f2"></a>


### File `pkg/apis/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `27ecc691316220a46bce31cae1655c11243d9d3f`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f9d65b47cf40b34d33.** [agones/pkg/apis/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7a68012b836912ebaa843d4053adcb2de5aac2a1bae5c8216dc88cfc534068a4`.

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
> // Package apis contains all the types for the Custom Resource Definitions


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-24cc764796a80d1e"></a>


### File `pkg/apis/scheduling.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `07595a8f976278cad0d8ea6593dd0f730f2eeb08`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/scheduling.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-2d35b5a63ed836f874.** [agones/pkg/apis/scheduling.go, lines 32–35](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/scheduling.go#L32-L35). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bc78723c6c4558f2b394ee31805a08fc9153d232a433f13bf1b372970c0f03a4`.

> 
> // SchedulingStrategy is the strategy that a Fleet & GameServers will use
> // when scheduling GameServers' Pods across a cluster.
> type SchedulingStrategy string


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `SchedulingStrategy` (L35). The full declaration list is recorded under `agones:pkg/apis/scheduling.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis/agones` -> module `pkg::apis::agones`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-5407a8156854ca32"></a>


### File `pkg/apis/agones/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b26b5864b3ef848a1f28a17f031aa6e7aea557d3`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/register.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4e391c3b61ac816996.** [agones/pkg/apis/agones/register.go, lines 17–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/register.go#L17-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1076f846227fdad5ad8d52f0b4b26da71948373487cd43e6aecb72e2da1d851d`.

> 
> const (
> 	// GroupName is the Scheme Group Name for Agones Custom Resource Definitions
> 	GroupName = "agones.dev"
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis/agones/v1` -> module `pkg::apis::agones::v1`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-f42598721a64d3ae7d.** [agones/pkg/apis/agones/v1/doc.go, lines 17–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/doc.go#L17-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `cea07da3169f9c776298009e0bd27c606bcd804f23c8f91c0d7c01de6d0ae50d`.

> // Package v1 is the v1 version of the API.
> // +groupName=agones.dev
> package v1


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-3a645ac78141181b"></a>


### File `pkg/apis/agones/v1/apihooks.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `41baa254be058de99806bfd1b0ccf0d77f8e4b80`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/apihooks.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-156cb2940b6d4919b1.** [agones/pkg/apis/agones/v1/apihooks.go, lines 22–36](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/apihooks.go#L22-L36). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3ddd2e6c35fc62c6c54ded97a97f7edc68218ed91fbfc1fd4341fc424c6a278b`.

> 
> // APIHooks is a subset of the cloudproduct.CloudProduct interface for cloud product hooks specific to this package.
> type APIHooks interface {
> 	// ValidateGameServerSpec is called by GameServer.Validate to allow for product specific validation.
> 	ValidateGameServerSpec(*GameServerSpec, *field.Path) field.ErrorList
> 
> 	// ValidateScheduling is called by Fleet and GameServerSet Validate() to allow for product specific validation of scheduling strategy.
> 	ValidateScheduling(apis.SchedulingStrategy, *field.Path) field.ErrorList
> 
> 	// MutateGameServerPod is called by createGameServerPod to allow for product specific pod mutation.
> 	MutateGameServerPod(*GameServerSpec, *corev1.Pod) error
> 
> 	// SetEviction is called by gs.Pod to enforce GameServer.Status.Eviction.
> 	SetEviction(*Eviction, *corev1.Pod) error
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `APIHooks` (L24). The full declaration list is recorded under `agones:pkg/apis/agones/v1/apihooks.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fdcbf72867c9be96"></a>


### File `pkg/apis/agones/v1/apihooksfake.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `037e9bbf726346a21ed329bbcda2b79bd4ac4d31`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/apihooksfake.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4da551d0f971f8ff66.** [agones/pkg/apis/agones/v1/apihooksfake.go, lines 22–36](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/apihooksfake.go#L22-L36). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be5b9878ef112cedbe3165ec69d87e35acaa08d5850f26d15274b11b7e2317cb`.

> 
> // fakeAPIHooks is a stubabble, fake implementation of APIHooks
> // This needs to be private, so it doesn't get picked up by the DeepCopy() generation toolkit.
> type fakeAPIHooks struct {
> 	StubValidateGameServerSpec func(*GameServerSpec, *field.Path) field.ErrorList
> 	StubValidateScheduling     func(apis.SchedulingStrategy, *field.Path) field.ErrorList
> 	StubMutateGameServerPod    func(*GameServerSpec, *corev1.Pod) error
> 	StubSetEviction            func(*Eviction, *corev1.Pod) error
> }
> 
> var _ APIHooks = fakeAPIHooks{}
> 
> // ValidateGameServerSpec is called by GameServer.Validate to allow for product specific validation.
> func (f fakeAPIHooks) ValidateGameServerSpec(gss *GameServerSpec, fldPath *field.Path) field.ErrorList {
> 	if f.StubValidateGameServerSpec != nil {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `fakeAPIHooks` (L25), `_` (L32), `ValidateGameServerSpec` (L35), `ValidateScheduling` (L43), `MutateGameServerPod` (L51), `SetEviction` (L59). The full declaration list is recorded under `agones:pkg/apis/agones/v1/apihooksfake.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-beff32b5dffeb98d"></a>


### File `pkg/apis/agones/v1/common.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4ac7664ad765dd35dc5c95e25bae30ed2a057318`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/common.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-01de7522804960815e.** [agones/pkg/apis/agones/v1/common.go, lines 51–65](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/common.go#L51-L65). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b0657e44bfcff65bceb01062a6051e1b3afad0065ea797db3b08b4b4de2fb9f2`.

> 
> // AggregatedPlayerStatus stores total player tracking values
> type AggregatedPlayerStatus struct {
> 	Count    int64 `json:"count"`
> 	Capacity int64 `json:"capacity"`
> }
> 
> // AggregatedCounterStatus stores total and allocated Counter tracking values
> type AggregatedCounterStatus struct {
> 	AllocatedCount    int64 `json:"allocatedCount"`
> 	AllocatedCapacity int64 `json:"allocatedCapacity"`
> 	Count             int64 `json:"count"`
> 	Capacity          int64 `json:"capacity"`
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 14 lexically identified declarations, including `AggregatedPlayerStatus` (L53), `AggregatedCounterStatus` (L59), `AggregatedListStatus` (L67), `crd` (L75), `validateName` (L81), `gsSpec` (L93), `validateGSSpec` (L99), `validateObjectMeta` (L108), `AllocationOverflow` (L117), `Validate` (L127), `CountMatches` (L135), `Apply` (L160), `Priority` (L177), `SafeAdd` (L189). The full declaration list is recorded under `agones:pkg/apis/agones/v1/common.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-42e0a0b755f1bfd3"></a>


### File `pkg/apis/agones/v1/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `be7e58580bb18e7784977b510036d471954be94d`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-78f90674ed26fcd780.** [agones/pkg/apis/agones/v1/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `cea07da3169f9c776298009e0bd27c606bcd804f23c8f91c0d7c01de6d0ae50d`.

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
> // +k8s:deepcopy-gen=package,register


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3a759c6bf237d760"></a>


### File `pkg/apis/agones/v1/fleet.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1963047a51f47fd1f52ec50ff82a65b8aa0257c1`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/fleet.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e74ebec9742c092474.** [agones/pkg/apis/agones/v1/fleet.go, lines 33–47](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/fleet.go#L33-L47). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `52ff0dd6d1b3f8ac7feed3ba35356c16434a8ca77f531eff3434cb3ad25f0e28`.

> 
> // +genclient
> // +genclient:method=GetScale,verb=get,subresource=scale,result=k8s.io/api/autoscaling/v1.Scale
> // +genclient:method=UpdateScale,verb=update,subresource=scale,input=k8s.io/api/autoscaling/v1.Scale,result=k8s.io/api/autoscaling/v1.Scale
> // +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
> 
> // Fleet is the data structure for a Fleet resource
> type Fleet struct {
> 	metav1.TypeMeta   `json:",inline"`
> 	metav1.ObjectMeta `json:"metadata,omitempty"`
> 
> 	Spec   FleetSpec   `json:"spec"`
> 	Status FleetStatus `json:"status"`
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 12 lexically identified declarations, including `Fleet` (L40), `FleetList` (L51), `FleetSpec` (L59), `FleetStatus` (L88), `GameServerSet` (L116), `ApplyDefaults` (L156), `GetGameServerSpec` (L187), `validateRollingUpdate` (L191), `Validate` (L207), `UpperBoundReplicas` (L232), `LowerBoundReplicas` (L241), `SumGameServerSets` (L250). The full declaration list is recorded under `agones:pkg/apis/agones/v1/fleet.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg`, `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fa66bd85da38a44d"></a>


### File `pkg/apis/agones/v1/gameserver.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `839d7591380bbc6ef7ca4148a795448b144b140a`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/gameserver.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d01c77956f2735d37d.** [agones/pkg/apis/agones/v1/gameserver.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/gameserver.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7df5b0d1da0985786664c31a0f27eb01ffcabeabdabcdf519193c5b7184fbaa0`.

> 
> // GameServerState is the state for the GameServer
> type GameServerState string
> 
> const (
> 	// GameServerStatePortAllocation is for when a dynamically allocating GameServer
> 	// is being created, an open port needs to be allocated
> 	GameServerStatePortAllocation GameServerState = "PortAllocation"
> 	// GameServerStateCreating is before the Pod for the GameServer is being created
> 	GameServerStateCreating GameServerState = "Creating"
> 	// GameServerStateStarting is for when the Pods for the GameServer are being
> 	// created but are not yet Scheduled
> 	GameServerStateStarting GameServerState = "Starting"
> 	// GameServerStateScheduled is for when we have determined that the Pod has been
> 	// scheduled in the cluster -- basically, we have a NodeName


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 61 lexically identified declarations, including `GameServerState` (L38), `PortPolicy` (L72), `EvictionSafe` (L89), `SdkServerLogLevel` (L104), `GameServer` (L192), `GameServerList` (L203), `GameServerTemplateSpec` (L211), `GameServerSpec` (L217), `PlayersSpec` (L250), `Eviction` (L255), `Health` (L264), `GameServerPort` (L277), `SdkServer` (L304), `GameServerStatus` (L314), `GameServerStatusPort` (L341), `PlayerStatus` (L347), `CounterStatus` (L354), `ListStatus` (L360), `ApplyDefaults` (L366), `ApplyDefaults` (L380), `applySdkServerDefaults` (L390), `applyContainerDefaults` (L403), `applyHealthDefaults` (L410), `applyStatusDefaults` (L425). The full declaration list is recorded under `agones:pkg/apis/agones/v1/gameserver.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg`, `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a74def882a78008e"></a>


### File `pkg/apis/agones/v1/gameserverset.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `59771a74c6853507156e52ef20b16b866e0db623`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/gameserverset.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-8cca9b3f55ede2f541.** [agones/pkg/apis/agones/v1/gameserverset.go, lines 30–46](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/gameserverset.go#L30-L46). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `aa62bbb516f092cfd52e44c2090f2031d925d2d0a19bdf4fff8ba464e2982ece`.

> 
> // +genclient
> // +genclient:method=GetScale,verb=get,subresource=scale,result=k8s.io/api/autoscaling/v1.Scale
> // +genclient:method=UpdateScale,verb=update,subresource=scale,input=k8s.io/api/autoscaling/v1.Scale,result=k8s.io/api/autoscaling/v1.Scale
> // +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
> 
> // GameServerSet is the data structure for a set of GameServers.
> // This matches philosophically with the relationship between
> // Deployments and ReplicaSets
> type GameServerSet struct {
> 	metav1.TypeMeta   `json:",inline"`
> 	metav1.ObjectMeta `json:"metadata,omitempty"`
> 
> 	Spec   GameServerSetSpec   `json:"spec"`
> 	Status GameServerSetStatus `json:"status"`
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `GameServerSet` (L39), `GameServerSetList` (L50), `GameServerSetSpec` (L58), `GameServerSetStatus` (L85), `ValidateUpdate` (L113), `Validate` (L123), `GetGameServerSpec` (L135), `GameServer` (L141). The full declaration list is recorded under `agones:pkg/apis/agones/v1/gameserverset.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-25b59f69cfdec010"></a>


### File `pkg/apis/agones/v1/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `990cc137a14456b1019d1969564d7be6eacbd17d`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/register.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d778980659d6f412e4.** [agones/pkg/apis/agones/v1/register.go, lines 31–45](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/register.go#L31-L45). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `145c109e956724a124fa92418ea99ddc43e2f03c59ab17522f0f74200645471b`.

> 
> // Kind takes an unqualified kind and returns back a Group qualified GroupKind
> func Kind(kind string) schema.GroupKind {
> 	return SchemeGroupVersion.WithKind(kind).GroupKind()
> }
> 
> // Resource takes an unqualified resource and returns a Group qualified GroupResource
> func Resource(resource string) schema.GroupResource {
> 	return SchemeGroupVersion.WithResource(resource).GroupResource()
> }
> 
> var (
> 	// SchemeBuilder registers our types
> 	SchemeBuilder = k8sruntime.NewSchemeBuilder(addKnownTypes)
> 	// AddToScheme local alias for SchemeBuilder.AddToScheme


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `VersionAnnotation` (L27), `SchemeGroupVersion` (L30), `Kind` (L33), `Resource` (L38), `init` (L49), `addKnownTypes` (L56). The full declaration list is recorded under `agones:pkg/apis/agones/v1/register.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve source panic/recover boundaries; do not use an unsupported panic as a translation placeholder.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-04f84f77d5aef059"></a>


### File `pkg/apis/agones/v1/v1.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c80d9ebf44ebabf8a2e4de6f5fdbb326fe8032bd`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/v1.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8138c54304aa1f50d9.** [agones/pkg/apis/agones/v1/v1.go, lines 18–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/v1.go#L18-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c7328ed0c771c037a14bc8472ff10bbbf22e751d638429ddb2cf69a8aa94bd0d`.

> 
> var gameserverErrors = errors.FromStruct[GameServer](nil)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `gameserverErrors` (L19). The full declaration list is recorded under `agones:pkg/apis/agones/v1/v1.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4f92f2dbef8a9792"></a>


### File `pkg/apis/agones/v1/zz_generated.deepcopy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `eb2da427889fb51c1d1e562d163b358d3201eb28`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/agones/v1/zz_generated.deepcopy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b3822758f351d85e15.** [agones/pkg/apis/agones/v1/zz_generated.deepcopy.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/zz_generated.deepcopy.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bde683256de58cd8236ef1bb7e25a435caa719500823c5ff63c0da317105afdd`.

> 
> // DeepCopyInto is an autogenerated deepcopy function, copying the receiver, writing into out. in must be non-nil.
> func (in *AggregatedCounterStatus) DeepCopyInto(out *AggregatedCounterStatus) {
> 	*out = *in
> 	return
> }
> 
> // DeepCopy is an autogenerated deepcopy function, copying the receiver, creating a new AggregatedCounterStatus.
> func (in *AggregatedCounterStatus) DeepCopy() *AggregatedCounterStatus {
> 	if in == nil {
> 		return nil
> 	}
> 	out := new(AggregatedCounterStatus)
> 	in.DeepCopyInto(out)
> 	return out


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 60 lexically identified declarations, including `DeepCopyInto` (L30), `DeepCopy` (L36), `DeepCopyInto` (L46), `DeepCopy` (L52), `DeepCopyInto` (L62), `DeepCopy` (L68), `DeepCopyInto` (L78), `DeepCopy` (L98), `DeepCopyInto` (L108), `DeepCopy` (L114), `DeepCopyInto` (L124), `DeepCopy` (L130), `DeepCopyInto` (L140), `DeepCopy` (L150), `DeepCopyObject` (L160), `DeepCopyInto` (L168), `DeepCopy` (L183), `DeepCopyObject` (L193), `DeepCopyInto` (L201), `DeepCopy` (L219), `DeepCopyInto` (L229), `DeepCopy` (L254), `DeepCopyInto` (L264), `DeepCopy` (L274). The full declaration list is recorded under `agones:pkg/apis/agones/v1/zz_generated.deepcopy.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis/allocation` -> module `pkg::apis::allocation`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-24263294af66611e"></a>


### File `pkg/apis/allocation/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3c26c331714a7a5e6266d6f45e356751859527df`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/allocation/register.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8c36f378c8f55b007d.** [agones/pkg/apis/allocation/register.go, lines 17–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/allocation/register.go#L17-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b7bd85a7e02b92ffbcbbeed3d8e843b1fd4081f1e4f9c41627d14db4bd776aaa`.

> 
> const (
> 	// GroupName is the Scheme Group Name for Agones allocation
> 	GroupName = "allocation.agones.dev"
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis/allocation/v1` -> module `pkg::apis::allocation::v1`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-8bb49f8c5387e02fbd.** [agones/pkg/apis/allocation/v1/doc.go, lines 17–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/allocation/v1/doc.go#L17-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ec03634336c94d423d361275827498e32b2da863a3ecdd1d1669976f2afd4c42`.

> // Package v1 is the v1 version of the API.
> // +groupName=allocation.agones.dev
> package v1


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-416a6f371c2b1c1d"></a>


### File `pkg/apis/allocation/v1/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `bf262ca7e0da9f195071e7b5911d6a0de21207ac`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/allocation/v1/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c1504203075a87a981.** [agones/pkg/apis/allocation/v1/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/allocation/v1/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ec03634336c94d423d361275827498e32b2da863a3ecdd1d1669976f2afd4c42`.

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
> // +k8s:deepcopy-gen=package,register


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e6858e64c7e0ac54"></a>


### File `pkg/apis/allocation/v1/gameserverallocation.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9d1c7def6b94af6a4d9c3930e2d441b5e92cac5e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/allocation/v1/gameserverallocation.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-55ca177f3dd7781311.** [agones/pkg/apis/allocation/v1/gameserverallocation.go, lines 42–56](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/allocation/v1/gameserverallocation.go#L42-L56). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e419412f3041e16c903491996fbde27c43ab8f8d690b2f0ca8ad0f347af8c07`.

> 
> // GameServerAllocationState is the Allocation state
> type GameServerAllocationState string
> 
> // +genclient
> // +genclient:onlyVerbs=create
> // +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
> 
> // GameServerAllocation is the data structure for allocating against a set of
> // GameServers, defined `selectors` selectors
> type GameServerAllocation struct {
> 	metav1.TypeMeta   `json:",inline"`
> 	metav1.ObjectMeta `json:"metadata,omitempty"`
> 	Spec              GameServerAllocationSpec   `json:"spec"`
> 	Status            GameServerAllocationStatus `json:"status,omitempty"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 30 lexically identified declarations, including `GameServerAllocationState` (L44), `GameServerAllocation` (L52), `GameServerAllocationList` (L62), `GameServerAllocationSpec` (L70), `GameServerSelector` (L129), `CounterSelector` (L153), `ListSelector` (L170), `CounterAction` (L183), `ListAction` (L196), `ApplyDefaults` (L209), `Matches` (L226), `matchCounters` (L263), `CounterActions` (L290), `ListActions` (L310), `matchLists` (L334), `Validate` (L363), `validateCounters` (L386), `validateLists` (L414), `validatePriorities` (L433), `validateCounterActions` (L452), `validateListActions` (L474), `MultiClusterSetting` (L487), `MetaPatch` (L493), `Validate` (L500). The full declaration list is recorded under `agones:pkg/apis/allocation/v1/gameserverallocation.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones/v1`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve source panic/recover boundaries; do not use an unsupported panic as a translation placeholder.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c629602a8c9c72da"></a>


### File `pkg/apis/allocation/v1/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `daa47aa8af9b68d8c7b7cf34a0d31dabc19e3d15`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/allocation/v1/register.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-3df24fec784fbc4d9a.** [agones/pkg/apis/allocation/v1/register.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/allocation/v1/register.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c08778247f2f04094579bd6d028974f9952674ad6201a335089d16fcea0315f2`.

> 
> // Kind takes an unqualified kind and returns back a Group qualified GroupKind
> func Kind(kind string) schema.GroupKind {
> 	return SchemeGroupVersion.WithKind(kind).GroupKind()
> }
> 
> // Resource takes an unqualified resource and returns a Group qualified GroupResource
> func Resource(resource string) schema.GroupResource {
> 	return SchemeGroupVersion.WithResource(resource).GroupResource()
> }
> 
> var (
> 	// SchemeBuilder registers our types
> 	SchemeBuilder = k8sruntime.NewSchemeBuilder(addKnownTypes)
> 	// AddToScheme local alias for SchemeBuilder.AddToScheme


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 5 lexically identified declarations, including `SchemeGroupVersion` (L26), `Kind` (L29), `Resource` (L34), `init` (L45), `addKnownTypes` (L52). The full declaration list is recorded under `agones:pkg/apis/allocation/v1/register.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/allocation`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve source panic/recover boundaries; do not use an unsupported panic as a translation placeholder.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-dface43d536c30e1"></a>


### File `pkg/apis/allocation/v1/zz_generated.deepcopy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `aa277c5e58b0e49e758483786c5d6ad438cdcda6`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/allocation/v1/zz_generated.deepcopy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-22f3f3a22f878f0267.** [agones/pkg/apis/allocation/v1/zz_generated.deepcopy.go, lines 29–43](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/allocation/v1/zz_generated.deepcopy.go#L29-L43). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c17631284ea7acc82d88795d77ec3c34f1606937d028f8f77629a8dbdaca352b`.

> 
> // DeepCopyInto is an autogenerated deepcopy function, copying the receiver, writing into out. in must be non-nil.
> func (in *CounterAction) DeepCopyInto(out *CounterAction) {
> 	*out = *in
> 	if in.Action != nil {
> 		in, out := &in.Action, &out.Action
> 		*out = new(string)
> 		**out = **in
> 	}
> 	if in.Amount != nil {
> 		in, out := &in.Amount, &out.Amount
> 		*out = new(int64)
> 		**out = **in
> 	}
> 	if in.Capacity != nil {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 26 lexically identified declarations, including `DeepCopyInto` (L31), `DeepCopy` (L52), `DeepCopyInto` (L62), `DeepCopy` (L68), `DeepCopyInto` (L78), `DeepCopy` (L88), `DeepCopyObject` (L98), `DeepCopyInto` (L106), `DeepCopy` (L121), `DeepCopyObject` (L131), `DeepCopyInto` (L139), `DeepCopy` (L181), `DeepCopyInto` (L191), `DeepCopy` (L226), `DeepCopyInto` (L236), `DeepCopy` (L256), `DeepCopyInto` (L266), `DeepCopy` (L292), `DeepCopyInto` (L302), `DeepCopy` (L323), `DeepCopyInto` (L333), `DeepCopy` (L339), `DeepCopyInto` (L349), `DeepCopy` (L369). The full declaration list is recorded under `agones:pkg/apis/allocation/v1/zz_generated.deepcopy.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis/autoscaling` -> module `pkg::apis::autoscaling`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-13a7692303947795"></a>


### File `pkg/apis/autoscaling/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5f40ecc42ce3c9924b990b5b86c31234f904fa11`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/autoscaling/register.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a1df2e86cca4a4bcd7.** [agones/pkg/apis/autoscaling/register.go, lines 17–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/autoscaling/register.go#L17-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `dcd249cf71f6a6ad981a3f4dd93b59c18da9846f5f6fbb51fc22466ec9c1d340`.

> 
> const (
> 	// GroupName is the Scheme Group Name for Agones Custom Resource Definitions
> 	GroupName = "autoscaling.agones.dev"
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis/autoscaling/v1` -> module `pkg::apis::autoscaling::v1`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-ae80ccea0676a922ee.** [agones/pkg/apis/autoscaling/v1/doc.go, lines 17–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/autoscaling/v1/doc.go#L17-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `894dc2fa1f087ef5b0ae5fc53b4c56b50b0bf56abb3018cdd15c39e3927b63a3`.

> // Package v1 is the v1 version of the API.
> // +groupName=autoscaling.agones.dev
> package v1


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-f64c98f704ab1fa1"></a>


### File `pkg/apis/autoscaling/v1/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e13562d4e0b4393b66c6d111cee425bb092ca3d5`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/autoscaling/v1/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-98ca8a2af76137bf09.** [agones/pkg/apis/autoscaling/v1/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/autoscaling/v1/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `894dc2fa1f087ef5b0ae5fc53b4c56b50b0bf56abb3018cdd15c39e3927b63a3`.

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
> // +k8s:deepcopy-gen=package,register


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-87d723f41ead424f"></a>


### File `pkg/apis/autoscaling/v1/fleetautoscaler.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c31c8ed3e8fe9b0e4f0bc1d6ce30526755c57628`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/autoscaling/v1/fleetautoscaler.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-058068f65745b6d485.** [agones/pkg/apis/autoscaling/v1/fleetautoscaler.go, lines 35–49](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/autoscaling/v1/fleetautoscaler.go#L35-L49). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e33b6d8b5844108e535aea20c666ef5138be8bedbc6dc68c1fa6f488a89b5d93`.

> 
> // +genclient
> // +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
> 
> // FleetAutoscaler is the data structure for a FleetAutoscaler resource
> type FleetAutoscaler struct {
> 	metav1.TypeMeta   `json:",inline"`
> 	metav1.ObjectMeta `json:"metadata,omitempty"`
> 
> 	Spec   FleetAutoscalerSpec   `json:"spec"`
> 	Status FleetAutoscalerStatus `json:"status"`
> }
> 
> // +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 34 lexically identified declarations, including `FleetAutoscaler` (L40), `FleetAutoscalerList` (L51), `FleetAutoscalerSpec` (L59), `FleetAutoscalerPolicy` (L70), `FleetAutoscalerPolicyType` (L107), `FleetAutoscalerSync` (L110), `FleetAutoscalerSyncType` (L120), `BufferPolicy` (L160), `URLConfiguration` (L185), `CounterPolicy` (L188), `ListPolicy` (L209), `Between` (L230), `ActivePeriod` (L242), `SchedulePolicy` (L261), `ChainEntry` (L273), `ChainPolicy` (L282), `WasmFrom` (L285), `WasmPolicy` (L292), `FixedIntervalSync` (L307), `FleetAutoscalerStatus` (L313), `FleetAutoscaleRequest` (L339), `FleetAutoscaleResponse` (L358), `FleetAutoscaleReview` (L370), `Validate` (L376). The full declaration list is recorded under `agones:pkg/apis/autoscaling/v1/fleetautoscaler.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-58c9bfb8b451d55c"></a>


### File `pkg/apis/autoscaling/v1/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `23fcc34a89be83af80564289f57f6caf1c360953`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/autoscaling/v1/register.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d92cd762855c2b56d9.** [agones/pkg/apis/autoscaling/v1/register.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/autoscaling/v1/register.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `eabf80c15cac1497b6d8658c55f6cc86059cbf9975e0819ec7962e17520e8317`.

> 
> // Kind takes an unqualified kind and returns back a Group qualified GroupKind
> func Kind(kind string) schema.GroupKind {
> 	return SchemeGroupVersion.WithKind(kind).GroupKind()
> }
> 
> // Resource takes an unqualified resource and returns a Group qualified GroupResource
> func Resource(resource string) schema.GroupResource {
> 	return SchemeGroupVersion.WithResource(resource).GroupResource()
> }
> 
> var (
> 	// SchemeBuilder registers our types
> 	SchemeBuilder = k8sruntime.NewSchemeBuilder(addKnownTypes)
> 	// AddToScheme local alias for SchemeBuilder.AddToScheme


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 5 lexically identified declarations, including `SchemeGroupVersion` (L26), `Kind` (L29), `Resource` (L34), `init` (L45), `addKnownTypes` (L52). The full declaration list is recorded under `agones:pkg/apis/autoscaling/v1/register.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/autoscaling`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve source panic/recover boundaries; do not use an unsupported panic as a translation placeholder.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-aebd9c97b2b94f56"></a>


### File `pkg/apis/autoscaling/v1/zz_generated.deepcopy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `08286127941ce315c377dceef973a189258673d7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/autoscaling/v1/zz_generated.deepcopy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-8d054fade5e6506f3e.** [agones/pkg/apis/autoscaling/v1/zz_generated.deepcopy.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/autoscaling/v1/zz_generated.deepcopy.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a9cef515b2b876230705b2074dd5c5a47ec0af99231c0b61f580fb531f4f87e8`.

> 
> // DeepCopyInto is an autogenerated deepcopy function, copying the receiver, writing into out. in must be non-nil.
> func (in *ActivePeriod) DeepCopyInto(out *ActivePeriod) {
> 	*out = *in
> 	return
> }
> 
> // DeepCopy is an autogenerated deepcopy function, copying the receiver, creating a new ActivePeriod.
> func (in *ActivePeriod) DeepCopy() *ActivePeriod {
> 	if in == nil {
> 		return nil
> 	}
> 	out := new(ActivePeriod)
> 	in.DeepCopyInto(out)
> 	return out


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 44 lexically identified declarations, including `DeepCopyInto` (L30), `DeepCopy` (L36), `DeepCopyInto` (L46), `DeepCopy` (L54), `DeepCopyInto` (L64), `DeepCopy` (L71), `DeepCopyInto` (L81), `DeepCopy` (L88), `DeepCopyInto` (L98), `DeepCopy` (L110), `DeepCopyInto` (L120), `DeepCopy` (L127), `DeepCopyInto` (L137), `DeepCopy` (L143), `DeepCopyInto` (L153), `DeepCopy` (L174), `DeepCopyInto` (L184), `DeepCopy` (L190), `DeepCopyInto` (L200), `DeepCopy` (L216), `DeepCopyInto` (L226), `DeepCopy` (L236), `DeepCopyObject` (L246), `DeepCopyInto` (L254). The full declaration list is recorded under `agones:pkg/apis/autoscaling/v1/zz_generated.deepcopy.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis/multicluster` -> module `pkg::apis::multicluster`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-44932597e790e394"></a>


### File `pkg/apis/multicluster/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `250a244a608a0c0fc1f9f92bfe8b89468701c14d`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/multicluster/register.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-77760c97ab75ea9b18.** [agones/pkg/apis/multicluster/register.go, lines 17–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/multicluster/register.go#L17-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8b9e9fd3cf05733bb5c73e33bae0a05030d650592d3e744ac98dd7a7a3c83571`.

> 
> const (
> 	// GroupName is the Scheme Group Name for Agones Custom Resource Definitions
> 	GroupName = "multicluster.agones.dev"
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/apis/multicluster/v1` -> module `pkg::apis::multicluster::v1`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-6d8f6b8031f860f581.** [agones/pkg/apis/multicluster/v1/doc.go, lines 17–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/multicluster/v1/doc.go#L17-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b997183746caead1411a1e7314535eefa76e37d658d58da65e8ba1f5bde00183`.

> // Package v1 is the v1 version of the API.
> // +groupName=multicluster.agones.dev
> package v1


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-195c978e8a5de191"></a>


### File `pkg/apis/multicluster/v1/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `adeec5d09cfc53acb89fe61e322e0456e9189ff0`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/multicluster/v1/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-13f584202c92d528e7.** [agones/pkg/apis/multicluster/v1/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/multicluster/v1/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b997183746caead1411a1e7314535eefa76e37d658d58da65e8ba1f5bde00183`.

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
> // +k8s:deepcopy-gen=package,register


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9f6b840d48c2dbce"></a>


### File `pkg/apis/multicluster/v1/gameserverallocationpolicy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d499097aa02e9b874d7e525c0ccca641910b5c76`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/multicluster/v1/gameserverallocationpolicy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e95d49bc86d9ca3402.** [agones/pkg/apis/multicluster/v1/gameserverallocationpolicy.go, lines 23–37](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/multicluster/v1/gameserverallocationpolicy.go#L23-L37). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2aa82ca67517ae015ebd827ea6225b0c4c8b5959bff25df6371517ed19eceeb4`.

> 
> // GameServerAllocationPolicySpec defines the desired state of GameServerAllocationPolicy
> type GameServerAllocationPolicySpec struct {
> 	// +kubebuilder:validation:Minimum=0
> 	Priority int32 `json:"priority"`
> 	// +kubebuilder:validation:Minimum=0
> 	Weight         int                   `json:"weight"`
> 	ConnectionInfo ClusterConnectionInfo `json:"connectionInfo,omitempty"`
> }
> 
> // ClusterConnectionInfo defines the connection information for a cluster
> type ClusterConnectionInfo struct {
> 	// Optional: the name of the targeted cluster
> 	ClusterName string `json:"clusterName"`
> 	// The endpoints for the allocator service in the targeted cluster.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 10 lexically identified declarations, including `GameServerAllocationPolicySpec` (L25), `ClusterConnectionInfo` (L34), `GameServerAllocationPolicy` (L54), `GameServerAllocationPolicyList` (L64), `ConnectionInfoIterator` (L71), `Next` (L83), `NewConnectionInfoIterator` (L103), `getClusterConnectionInfo` (L136), `avgWeight` (L155), `selectRandomWeighted` (L167). The full declaration list is recorded under `agones:pkg/apis/multicluster/v1/gameserverallocationpolicy.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3d9de9fe8af3ca6b"></a>


### File `pkg/apis/multicluster/v1/register.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0596a153ca8a34099c666ad51cc6a5ff362beee7`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/multicluster/v1/register.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-27419b28fd3bdba4aa.** [agones/pkg/apis/multicluster/v1/register.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/multicluster/v1/register.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bec319e8e32a096c48b150c3fa0aae5977e1c192f7d04f886cc56f92e26b3cb8`.

> 
> // Kind takes an unqualified kind and returns back a Group qualified GroupKind
> func Kind(kind string) schema.GroupKind {
> 	return SchemeGroupVersion.WithKind(kind).GroupKind()
> }
> 
> // Resource takes an unqualified resource and returns a Group qualified GroupResource
> func Resource(resource string) schema.GroupResource {
> 	return SchemeGroupVersion.WithResource(resource).GroupResource()
> }
> 
> var (
> 	// SchemeBuilder registers our types
> 	SchemeBuilder = k8sruntime.NewSchemeBuilder(addKnownTypes)
> 	// AddToScheme local alias for SchemeBuilder.AddToScheme


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 5 lexically identified declarations, including `SchemeGroupVersion` (L26), `Kind` (L29), `Resource` (L34), `init` (L45), `addKnownTypes` (L52). The full declaration list is recorded under `agones:pkg/apis/multicluster/v1/register.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/multicluster`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve source panic/recover boundaries; do not use an unsupported panic as a translation placeholder.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f6f8de60f83c15c9"></a>


### File `pkg/apis/multicluster/v1/zz_generated.deepcopy.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `da0aae864c0101c137514a69af0097598b201f46`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/apis/multicluster/v1/zz_generated.deepcopy.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-20279bad82d2720be4.** [agones/pkg/apis/multicluster/v1/zz_generated.deepcopy.go, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/multicluster/v1/zz_generated.deepcopy.go#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `24dbdc47661b06920ba518f3e596a8edce4c83f3febcde2049825599412db447`.

> 
> // DeepCopyInto is an autogenerated deepcopy function, copying the receiver, writing into out. in must be non-nil.
> func (in *ClusterConnectionInfo) DeepCopyInto(out *ClusterConnectionInfo) {
> 	*out = *in
> 	if in.AllocationEndpoints != nil {
> 		in, out := &in.AllocationEndpoints, &out.AllocationEndpoints
> 		*out = make([]string, len(*in))
> 		copy(*out, *in)
> 	}
> 	if in.ServerCA != nil {
> 		in, out := &in.ServerCA, &out.ServerCA
> 		*out = make([]byte, len(*in))
> 		copy(*out, *in)
> 	}
> 	return


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 12 lexically identified declarations, including `DeepCopyInto` (L29), `DeepCopy` (L45), `DeepCopyInto` (L55), `DeepCopy` (L104), `DeepCopyInto` (L114), `DeepCopy` (L123), `DeepCopyObject` (L133), `DeepCopyInto` (L141), `DeepCopy` (L156), `DeepCopyObject` (L166), `DeepCopyInto` (L174), `DeepCopy` (L181). The full declaration list is recorded under `agones:pkg/apis/multicluster/v1/zz_generated.deepcopy.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/sdk` -> module `pkg::sdk`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-7b7f5c5fe11fc388"></a>


### File `pkg/sdk/sdk.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8d2f22d509acdb293fece8898a24e26be5cb03ba`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/sdk.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-010b307a12b71614f6.** [agones/pkg/sdk/sdk.pb.go, lines 54–68](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/sdk.pb.go#L54-L68). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7405fe668a9cd631b585f412393bf4706b83e5210f34f1e649f8094751b1fe2d`.

> 
> // I am Empty
> type Empty struct {
> 	state         protoimpl.MessageState
> 	sizeCache     protoimpl.SizeCache
> 	unknownFields protoimpl.UnknownFields
> }
> 
> func (x *Empty) Reset() {
> 	*x = Empty{}
> 	if protoimpl.UnsafeEnabled {
> 		mi := &file_sdk_proto_msgTypes[0]
> 		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
> 		ms.StoreMessageInfo(mi)
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 124 lexically identified declarations, including `Empty` (L56), `Reset` (L62), `String` (L71), `ProtoMessage` (L75), `ProtoReflect` (L77), `Descriptor` (L90), `KeyValue` (L95), `Reset` (L104), `String` (L113), `ProtoMessage` (L117), `ProtoReflect` (L119), `Descriptor` (L132), `GetKey` (L136), `GetValue` (L143), `Duration` (L151), `Reset` (L159), `String` (L168), `ProtoMessage` (L172), `ProtoReflect` (L174), `Descriptor` (L187), `GetSeconds` (L191), `GameServer` (L201), `Reset` (L211), `String` (L220). The full declaration list is recorded under `agones:pkg/sdk/sdk.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-643382baf4d258de"></a>


### File `pkg/sdk/sdk.pb.gw.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `86718ce6e319708302d37f23b76bf225bffe75f3`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/sdk.pb.gw.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0ebedadd69bec50574.** [agones/pkg/sdk/sdk.pb.gw.go, lines 52–66](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/sdk.pb.gw.go#L52-L66). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ad34ce6da1cfd3c792b4dd780817b0abab26d0a03dc215a907e863b29e878e89`.

> 
> func request_SDK_Ready_0(ctx context.Context, marshaler runtime.Marshaler, client SDKClient, req *http.Request, pathParams map[string]string) (proto.Message, runtime.ServerMetadata, error) {
> 	var (
> 		protoReq Empty
> 		metadata runtime.ServerMetadata
> 	)
> 	if err := marshaler.NewDecoder(req.Body).Decode(&protoReq); err != nil && !errors.Is(err, io.EOF) {
> 		return nil, metadata, status.Errorf(codes.InvalidArgument, "%v", err)
> 	}
> 	if req.Body != nil {
> 		_, _ = io.Copy(io.Discard, req.Body)
> 	}
> 	msg, err := client.Ready(ctx, &protoReq, grpc.Header(&metadata.HeaderMD), grpc.Trailer(&metadata.TrailerMD))
> 	return msg, metadata, err
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `request_SDK_Ready_0` (L53), `local_request_SDK_Ready_0` (L68), `request_SDK_Allocate_0` (L80), `local_request_SDK_Allocate_0` (L95), `request_SDK_Shutdown_0` (L107), `local_request_SDK_Shutdown_0` (L122), `request_SDK_Health_0` (L134), `request_SDK_GetGameServer_0` (L175), `local_request_SDK_GetGameServer_0` (L187), `request_SDK_WatchGameServer_0` (L196), `request_SDK_SetLabel_0` (L216), `local_request_SDK_SetLabel_0` (L231), `request_SDK_SetAnnotation_0` (L243), `local_request_SDK_SetAnnotation_0` (L258), `request_SDK_Reserve_0` (L270), `local_request_SDK_Reserve_0` (L285), `RegisterSDKHandlerServer` (L302), `RegisterSDKHandlerFromEndpoint` (L463), `RegisterSDKHandler` (L487), `RegisterSDKHandlerClient` (L496). The full declaration list is recorded under `agones:pkg/sdk/sdk.pb.gw.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

8. Preserve source floating width, narrowing, exceptional values and relevant bits.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0606d5b8efdaa902"></a>


### File `pkg/sdk/sdk_grpc.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `01a864445bc0800c9503d65f1487c9e46472b11e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/sdk_grpc.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-fe8943caca9d119642.** [agones/pkg/sdk/sdk_grpc.pb.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/sdk_grpc.pb.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `91979eb0cb7ab0684dd0411be776cfa2514bb44a0a96526827b559f4890d72db`.

> 
> // SDKClient is the client API for SDK service.
> //
> // For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
> type SDKClient interface {
> 	// Call when the GameServer is ready
> 	Ready(ctx context.Context, in *Empty, opts ...grpc.CallOption) (*Empty, error)
> 	// Call to self Allocation the GameServer
> 	Allocate(ctx context.Context, in *Empty, opts ...grpc.CallOption) (*Empty, error)
> 	// Call when the GameServer is shutting down
> 	Shutdown(ctx context.Context, in *Empty, opts ...grpc.CallOption) (*Empty, error)
> 	// Send a Empty every d Duration to declare that this GameSever is healthy
> 	Health(ctx context.Context, opts ...grpc.CallOption) (SDK_HealthClient, error)
> 	// Retrieve the current GameServer data
> 	GetGameServer(ctx context.Context, in *Empty, opts ...grpc.CallOption) (*GameServer, error)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 50 lexically identified declarations, including `_` (L35), `SDKClient` (L40), `sDKClient` (L61), `NewSDKClient` (L65), `Ready` (L69), `Allocate` (L78), `Shutdown` (L87), `Health` (L96), `SDK_HealthClient` (L105), `sDKHealthClient` (L111), `Send` (L115), `CloseAndRecv` (L119), `GetGameServer` (L130), `WatchGameServer` (L139), `SDK_WatchGameServerClient` (L154), `sDKWatchGameServerClient` (L159), `Recv` (L163), `SetLabel` (L171), `SetAnnotation` (L180), `Reserve` (L189), `SDKServer` (L201), `UnimplementedSDKServer` (L223), `Ready` (L226), `Allocate` (L229). The full declaration list is recorded under `agones:pkg/sdk/sdk_grpc.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/sdk/alpha` -> module `pkg::sdk::alpha`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-fec9d4fd9a95fa01"></a>


### File `pkg/sdk/alpha/alpha.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1aa6939939987f006fbf45c0b43958002e3cb8ac`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/alpha/alpha.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4f56806e8304cf2479.** [agones/pkg/sdk/alpha/alpha.pb.go, lines 54–68](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/alpha/alpha.pb.go#L54-L68). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `39ed0b6064780e794d04c6e49a3dae18a8ac7df9224f182449c004e492fc496c`.

> 
> // I am Empty
> type Empty struct {
> 	state         protoimpl.MessageState
> 	sizeCache     protoimpl.SizeCache
> 	unknownFields protoimpl.UnknownFields
> }
> 
> func (x *Empty) Reset() {
> 	*x = Empty{}
> 	if protoimpl.UnsafeEnabled {
> 		mi := &file_alpha_proto_msgTypes[0]
> 		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
> 		ms.StoreMessageInfo(mi)
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 42 lexically identified declarations, including `Empty` (L56), `Reset` (L62), `String` (L71), `ProtoMessage` (L75), `ProtoReflect` (L77), `Descriptor` (L90), `Count` (L95), `Reset` (L103), `String` (L112), `ProtoMessage` (L116), `ProtoReflect` (L118), `Descriptor` (L131), `GetCount` (L135), `Bool` (L143), `Reset` (L151), `String` (L160), `ProtoMessage` (L164), `ProtoReflect` (L166), `Descriptor` (L179), `GetBool` (L183), `PlayerID` (L191), `Reset` (L199), `String` (L208), `ProtoMessage` (L212). The full declaration list is recorded under `agones:pkg/sdk/alpha/alpha.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f32c038d279736c1"></a>


### File `pkg/sdk/alpha/alpha.pb.gw.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `440dcd265a195f6cbe193bfa96fe817d4dfc28a7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/alpha/alpha.pb.gw.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6ae6ae9fb89ea8c334.** [agones/pkg/sdk/alpha/alpha.pb.gw.go, lines 52–66](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/alpha/alpha.pb.gw.go#L52-L66). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `317d400f2b1997ece41d7929657c1d686fd0f6f8a56c649488878861016784eb`.

> 
> func request_SDK_PlayerConnect_0(ctx context.Context, marshaler runtime.Marshaler, client SDKClient, req *http.Request, pathParams map[string]string) (proto.Message, runtime.ServerMetadata, error) {
> 	var (
> 		protoReq PlayerID
> 		metadata runtime.ServerMetadata
> 	)
> 	if err := marshaler.NewDecoder(req.Body).Decode(&protoReq); err != nil && !errors.Is(err, io.EOF) {
> 		return nil, metadata, status.Errorf(codes.InvalidArgument, "%v", err)
> 	}
> 	if req.Body != nil {
> 		_, _ = io.Copy(io.Discard, req.Body)
> 	}
> 	msg, err := client.PlayerConnect(ctx, &protoReq, grpc.Header(&metadata.HeaderMD), grpc.Trailer(&metadata.TrailerMD))
> 	return msg, metadata, err
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 18 lexically identified declarations, including `request_SDK_PlayerConnect_0` (L53), `local_request_SDK_PlayerConnect_0` (L68), `request_SDK_PlayerDisconnect_0` (L80), `local_request_SDK_PlayerDisconnect_0` (L95), `request_SDK_SetPlayerCapacity_0` (L107), `local_request_SDK_SetPlayerCapacity_0` (L122), `request_SDK_GetPlayerCapacity_0` (L134), `local_request_SDK_GetPlayerCapacity_0` (L146), `request_SDK_GetPlayerCount_0` (L155), `local_request_SDK_GetPlayerCount_0` (L167), `request_SDK_IsPlayerConnected_0` (L176), `local_request_SDK_IsPlayerConnected_0` (L197), `request_SDK_GetConnectedPlayers_0` (L215), `local_request_SDK_GetConnectedPlayers_0` (L227), `RegisterSDKHandlerServer` (L241), `RegisterSDKHandlerFromEndpoint` (L388), `RegisterSDKHandler` (L412), `RegisterSDKHandlerClient` (L421). The full declaration list is recorded under `agones:pkg/sdk/alpha/alpha.pb.gw.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

8. Preserve source floating width, narrowing, exceptional values and relevant bits.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cdd3a588f77bef40"></a>


### File `pkg/sdk/alpha/alpha_grpc.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9b6b298da630512867b564f38d2366bc99b2afa1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/alpha/alpha_grpc.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-5d4f9f4997da77ca89.** [agones/pkg/sdk/alpha/alpha_grpc.pb.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/alpha/alpha_grpc.pb.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f291315d3fcec2a0350aece6469fadefa21754edbb4a6de55434e14da0b7aa12`.

> 
> // SDKClient is the client API for SDK service.
> //
> // For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
> type SDKClient interface {
> 	// PlayerConnect increases the SDK’s stored player count by one, and appends this playerID to GameServer.Status.Players.IDs.
> 	//
> 	// GameServer.Status.Players.Count and GameServer.Status.Players.IDs are then set to update the player count and id list a second from now,
> 	// unless there is already an update pending, in which case the update joins that batch operation.
> 	//
> 	// PlayerConnect returns true and adds the playerID to the list of playerIDs if this playerID was not already in the
> 	// list of connected playerIDs.
> 	//
> 	// If the playerID exists within the list of connected playerIDs, PlayerConnect will return false, and the list of
> 	// connected playerIDs will be left unchanged.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 30 lexically identified declarations, including `_` (L35), `SDKClient` (L40), `sDKClient` (L96), `NewSDKClient` (L100), `PlayerConnect` (L104), `PlayerDisconnect` (L113), `SetPlayerCapacity` (L122), `GetPlayerCapacity` (L131), `GetPlayerCount` (L140), `IsPlayerConnected` (L149), `GetConnectedPlayers` (L158), `SDKServer` (L170), `UnimplementedSDKServer` (L227), `PlayerConnect` (L230), `PlayerDisconnect` (L233), `SetPlayerCapacity` (L236), `GetPlayerCapacity` (L239), `GetPlayerCount` (L242), `IsPlayerConnected` (L245), `GetConnectedPlayers` (L248), `UnsafeSDKServer` (L255), `RegisterSDKServer` (L259), `_SDK_PlayerConnect_Handler` (L263), `_SDK_PlayerDisconnect_Handler` (L281). The full declaration list is recorded under `agones:pkg/sdk/alpha/alpha_grpc.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/sdk/beta` -> module `pkg::sdk::beta`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-c5fa17b94b48dcc3"></a>


### File `pkg/sdk/beta/beta.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9d3b898f80dfd95b4a2b52540ed3da12e5f9f803`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/beta/beta.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-58885f7149d524fc82.** [agones/pkg/sdk/beta/beta.pb.go, lines 57–71](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/beta/beta.pb.go#L57-L71). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `eac4e3c09cfefcfeb7ea47846427e0b89d84f53e124fbbcbb1ba8c20e907887a`.

> 
> // I am Empty
> type Empty struct {
> 	state         protoimpl.MessageState
> 	sizeCache     protoimpl.SizeCache
> 	unknownFields protoimpl.UnknownFields
> }
> 
> func (x *Empty) Reset() {
> 	*x = Empty{}
> 	if protoimpl.UnsafeEnabled {
> 		mi := &file_beta_proto_msgTypes[0]
> 		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
> 		ms.StoreMessageInfo(mi)
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 87 lexically identified declarations, including `Empty` (L59), `Reset` (L65), `String` (L74), `ProtoMessage` (L78), `ProtoReflect` (L80), `Descriptor` (L93), `Counter` (L98), `Reset` (L111), `String` (L120), `ProtoMessage` (L124), `ProtoReflect` (L126), `Descriptor` (L139), `GetName` (L143), `GetCount` (L150), `GetCapacity` (L157), `CounterUpdateRequest` (L165), `Reset` (L181), `String` (L190), `ProtoMessage` (L194), `ProtoReflect` (L196), `Descriptor` (L209), `GetName` (L213), `GetCount` (L220), `GetCapacity` (L227). The full declaration list is recorded under `agones:pkg/sdk/beta/beta.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0df9b186c75c64b6"></a>


### File `pkg/sdk/beta/beta.pb.gw.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d77f66729c825f824ba8cbe39ed5099dfda83f47`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/beta/beta.pb.gw.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-021d5928e0d2b47bb8.** [agones/pkg/sdk/beta/beta.pb.gw.go, lines 52–66](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/beta/beta.pb.gw.go#L52-L66). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ec13c59020ba291745428b1a039f42b76ce1297fabf2b6477f32fdd681270a27`.

> 
> func request_SDK_GetCounter_0(ctx context.Context, marshaler runtime.Marshaler, client SDKClient, req *http.Request, pathParams map[string]string) (proto.Message, runtime.ServerMetadata, error) {
> 	var (
> 		protoReq GetCounterRequest
> 		metadata runtime.ServerMetadata
> 		err      error
> 	)
> 	val, ok := pathParams["name"]
> 	if !ok {
> 		return nil, metadata, status.Errorf(codes.InvalidArgument, "missing parameter %s", "name")
> 	}
> 	protoReq.Name, err = runtime.String(val)
> 	if err != nil {
> 		return nil, metadata, status.Errorf(codes.InvalidArgument, "type mismatch, parameter: %s, error: %v", "name", err)
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 17 lexically identified declarations, including `request_SDK_GetCounter_0` (L53), `local_request_SDK_GetCounter_0` (L74), `request_SDK_UpdateCounter_0` (L92), `local_request_SDK_UpdateCounter_0` (L116), `request_SDK_GetList_0` (L137), `local_request_SDK_GetList_0` (L158), `filter_SDK_UpdateList_0` (L176), `request_SDK_UpdateList_0` (L178), `local_request_SDK_UpdateList_0` (L219), `request_SDK_AddListValue_0` (L257), `local_request_SDK_AddListValue_0` (L281), `request_SDK_RemoveListValue_0` (L302), `local_request_SDK_RemoveListValue_0` (L326), `RegisterSDKHandlerServer` (L352), `RegisterSDKHandlerFromEndpoint` (L479), `RegisterSDKHandler` (L503), `RegisterSDKHandlerClient` (L512). The full declaration list is recorded under `agones:pkg/sdk/beta/beta.pb.gw.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

8. Preserve source floating width, narrowing, exceptional values and relevant bits.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-37f599e21885fc97"></a>


### File `pkg/sdk/beta/beta_grpc.pb.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3511931ce2382f1f553ba45e1ad7a92c03aee7c1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `agones-rs/pkg/sdk/beta/beta_grpc.pb.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-9606d779b89dd8c26e.** [agones/pkg/sdk/beta/beta_grpc.pb.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdk/beta/beta_grpc.pb.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5fafd102c68b341a676c7b293235c009fd33d0fa439fc0dcb48b865573795c6d`.

> 
> // SDKClient is the client API for SDK service.
> //
> // For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
> type SDKClient interface {
> 	// Gets a Counter. Returns NOT_FOUND if the Counter does not exist.
> 	GetCounter(ctx context.Context, in *GetCounterRequest, opts ...grpc.CallOption) (*Counter, error)
> 	// UpdateCounter returns the updated Counter. Returns NOT_FOUND if the Counter does not exist (name cannot be updated).
> 	// Returns OUT_OF_RANGE if the Count is out of range [0,Capacity].
> 	UpdateCounter(ctx context.Context, in *UpdateCounterRequest, opts ...grpc.CallOption) (*Counter, error)
> 	// Gets a List. Returns NOT_FOUND if the List does not exist.
> 	GetList(ctx context.Context, in *GetListRequest, opts ...grpc.CallOption) (*List, error)
> 	// UpdateList returns the updated List. Returns NOT_FOUND if the List does not exist (name cannot be updated).
> 	// **THIS WILL OVERWRITE ALL EXISTING LIST.VALUES WITH ANY REQUEST LIST.VALUES**
> 	// Use AddListValue() or RemoveListValue() for modifying the List.Values field.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 27 lexically identified declarations, including `_` (L35), `SDKClient` (L40), `sDKClient` (L64), `NewSDKClient` (L68), `GetCounter` (L72), `UpdateCounter` (L81), `GetList` (L90), `UpdateList` (L99), `AddListValue` (L108), `RemoveListValue` (L117), `SDKServer` (L129), `UnimplementedSDKServer` (L154), `GetCounter` (L157), `UpdateCounter` (L160), `GetList` (L163), `UpdateList` (L166), `AddListValue` (L169), `RemoveListValue` (L172), `UnsafeSDKServer` (L179), `RegisterSDKServer` (L183), `_SDK_GetCounter_Handler` (L187), `_SDK_UpdateCounter_Handler` (L205), `_SDK_GetList_Handler` (L223), `_SDK_UpdateList_Handler` (L241). The full declaration list is recorded under `agones:pkg/sdk/beta/beta_grpc.pb.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:proto/allocation` -> module `proto::allocation`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-8cb129ea411314d4"></a>


### File `proto/allocation/allocation.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e04ae88eac3d98523cd0a31c3a3f783305db9cb5`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/allocation/allocation.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-7a42c2fb4fd0977a3f.** [agones/proto/allocation/allocation.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/allocation/allocation.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `56c766f0ff04b549a1ec4e0cfa3005b0e7dc099fb252bf3c15aaf459f4db7413`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-710fe71d8e7aa03e"></a>


### File `proto/allocation/processor.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e6dd2f6badfc6d1559799e34fcad62f6b971942f`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/allocation/processor.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ec91bfa2e07478d3bb.** [agones/proto/allocation/processor.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/allocation/processor.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `457d492a1e79b22764f0ab602dd6edfebd92812333d99c57c936ba7554f6e659`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:proto/googleapis/google/api` -> module `proto::googleapis::google::api`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-4ac485a0fd4f5e80"></a>


### File `proto/googleapis/google/api/annotations.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `005561ec53ec4c741303fd43d6effe2d7bbd0382`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/googleapis/google/api/annotations.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-96d6d8fb850b723840.** [agones/proto/googleapis/google/api/annotations.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/googleapis/google/api/annotations.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `30d7139338bca730deca74a7a466321796fa21aa389212bcd64c9ee983ba5e4b`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8f8c4963ccbd860f"></a>


### File `proto/googleapis/google/api/client.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a1c8589fec52e19f3ee0f32ecbe5c62e9b8cf733`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/googleapis/google/api/client.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-7b58285cd59ddd379a.** [agones/proto/googleapis/google/api/client.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/googleapis/google/api/client.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bc5653b7d9aa77138d41a03dc3a42e7a810b722df829aad2f499406fb53f7d0b`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-de3f31ad1d47240e"></a>


### File `proto/googleapis/google/api/field_behavior.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ba3d40842cb458a4bd97139811bd5967c3e79a1d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/googleapis/google/api/field_behavior.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-95ba471a0e4a923f0c.** [agones/proto/googleapis/google/api/field_behavior.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/googleapis/google/api/field_behavior.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c6ff35bf6b8e9cabd314b1df91ff6ede5465285be7bd890df450216b31a201df`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2328278ffe8f96e1"></a>


### File `proto/googleapis/google/api/http.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fa6ea99cb2ae1bef9001434d5da0b97baf8c7152`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/googleapis/google/api/http.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-235d7725814b792377.** [agones/proto/googleapis/google/api/http.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/googleapis/google/api/http.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e10cf2b54f310df0cd7d0ed951c8cc5f8578943efb1177832fa25fc19ce912c`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-023b3b54dc71a71d"></a>


### File `proto/googleapis/google/api/launch_stage.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6c96d953607a470f3b866bd06c9a442aa444a9ba`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/googleapis/google/api/launch_stage.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-00c94aba8f17ba6b1d.** [agones/proto/googleapis/google/api/launch_stage.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/googleapis/google/api/launch_stage.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `583df1f2d101a9a3ae58c7d0f6595611e25ca94ae9d08b51481707a60368a1d7`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cd6aca458b64cb6d"></a>


### File `proto/googleapis/google/api/resource.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c30716023bc8804a68eae76e14a3bebbef4c7db6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/googleapis/google/api/resource.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4b0c42b49372245180.** [agones/proto/googleapis/google/api/resource.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/googleapis/google/api/resource.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6ff1872529183a885557e566e47f21bbc57a60e13de4bab4b06727d1007d5189`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:proto/googleapis/google/rpc` -> module `proto::googleapis::google::rpc`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-64a3fb58eb1bb8ce"></a>


### File `proto/googleapis/google/rpc/status.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4be0a0b44017ff020ea709e9c1c3ef72c6801e67`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/googleapis/google/rpc/status.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e6d1d47bd0d27cc90d.** [agones/proto/googleapis/google/rpc/status.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/googleapis/google/rpc/status.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6c557c45ae6d4a2632abbda025c43455d2da3d650d96a61a463ff2dc07b9b929`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:proto/grpc-gateway` -> module `proto::grpc-gateway`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-732affea47cab8a3"></a>


### File `proto/grpc-gateway/LICENSE.txt`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6e59e318a1f4e6d97680254b0c5faca3d82f74c7`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/grpc-gateway/LICENSE.txt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-639a4276cbe12a38b1.** [agones/proto/grpc-gateway/LICENSE.txt, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/grpc-gateway/LICENSE.txt#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b8bb2030523f09d9f08c756aa0d41856ec2e345ace892630d41f16ed09f79ad5`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:proto/grpc-gateway/protoc-gen-openapiv2/options` -> module `proto::grpc-gateway::protoc-gen-openapiv2::options`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-7a69bfbdb2b5302d"></a>


### File `proto/grpc-gateway/protoc-gen-openapiv2/options/annotations.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9d4f4c99586a318ecf50b3436f4ca5cfc28d6a54`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/grpc-gateway/protoc-gen-openapiv2/options/annotations.proto`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1bc889272473c40c2e.** [agones/proto/grpc-gateway/protoc-gen-openapiv2/options/annotations.proto, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/grpc-gateway/protoc-gen-openapiv2/options/annotations.proto#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7c2cecba02fb94926a5d2a9914fe0253f32f76c5b554508835719312e2ab7aab`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bbee657745cd249b"></a>


### File `proto/grpc-gateway/protoc-gen-openapiv2/options/openapiv2.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1f01c94a34a420e04bf0cc5507406fce59287aeb`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/grpc-gateway/protoc-gen-openapiv2/options/openapiv2.proto`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5c169025a6675e8cab.** [agones/proto/grpc-gateway/protoc-gen-openapiv2/options/openapiv2.proto, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/grpc-gateway/protoc-gen-openapiv2/options/openapiv2.proto#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `759fd88829980b5aaac025070314d43d1fa9f77f5d678a4116d96645e965e569`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:proto/sdk` -> module `proto::sdk`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-c8b0fab15b118954"></a>


### File `proto/sdk/sdk.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7b08d1163fc71e4b5a5d22d2f1ae866c80b93157`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/sdk/sdk.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-22585b3b18ee7b0404.** [agones/proto/sdk/sdk.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/sdk/sdk.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f5a4a7a8e9937ed9cf7d992141d3fb90f734ad058999a89b4a66f4ab9de096e0`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:proto/sdk/alpha` -> module `proto::sdk::alpha`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ff751329e46b5cd0"></a>


### File `proto/sdk/alpha/alpha.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fb75ed5ae01f7eddf39a7d018191d4ae0ddcf854`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/sdk/alpha/alpha.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f28c1019ea3791c0fe.** [agones/proto/sdk/alpha/alpha.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/sdk/alpha/alpha.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `aabffc0a0371f37302b55f5dc77bb1bdeec14dd16415ca2e4810a607e7c05dc5`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:proto/sdk/beta` -> module `proto::sdk::beta`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d28bd01ad9252ca70f.** [agones/site/content/en/docs/Reference/gameserver.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Reference/gameserver.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `be6f87f5420b8f95d4c21b1402aa53691b6f19e8b51e913eb5d93d6b3b627e89`.

> 
> A full GameServer specification is available below and in the {{< ghlink href="examples/gameserver.yaml" >}}example folder{{< /ghlink >}} for reference :
> 
> ```yaml
> apiVersion: "agones.dev/v1"
> kind: GameServer
> # GameServer Metadata
> # {{< k8s-api-version href="#objectmeta-v1-meta" >}}
> metadata:
>   # generateName: "gds-example" # generate a unique name, with the given prefix
>   name: "gds-example" # set a fixed name
> spec:
>   # if there is more than one container, specify which one is the game server


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-67059daa38e9b50d"></a>


### File `proto/sdk/beta/beta.proto`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `97c0b61976ee8221e891e97c1084ae65c9e96a1d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/proto/sdk/beta/beta.proto`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-45ae47870f5625fbed.** [agones/proto/sdk/beta/beta.proto, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/proto/sdk/beta/beta.proto#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ab8722d966ca45080659e935f6e42f6c5681d10917edd3399d0ca260caa114b4`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
