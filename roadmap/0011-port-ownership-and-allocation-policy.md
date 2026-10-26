<a id="m0011"></a>


# M0011 — Port ownership and allocation policy


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0011-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


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


1. Port exact port ranges, allocation/release, protocol/address distinctions, static/dynamic/passthrough rules and exhaustion behavior.

2. Preserve collision handling, initial occupied-port reconciliation, release ordering and error conditions. Tie each allocation to one CGP resource owner and explicit cleanup path.

3. Use derive-deftly for repeated port-record wiring; keep selection and mutation policy explicit and readable. Do not claim allocation correctness before post-gate assurance.


## Architecture, readability and authored outputs


1. Own 2 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:pkg/portallocator` -> module `pkg::portallocator`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-73867c84eefa22952e.** [agones/pkg/portallocator/doc.go, lines 15–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/portallocator/doc.go#L15-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3d8224b216a5a27bc5cae182c6949b4a1546d89f4b645d5c27e841d3c36e2391`.

> // Package portallocator defines a generic port allocator for game servers
> package portallocator


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-25e0ccc452506393"></a>


### File `pkg/portallocator/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `90bb9520de7a5bf0267f82b8a90c24eaffcd610e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/portallocator/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f018161c08c33320a3.** [agones/pkg/portallocator/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/portallocator/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3d8224b216a5a27bc5cae182c6949b4a1546d89f4b645d5c27e841d3c36e2391`.

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
> // Package portallocator defines a generic port allocator for game servers


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6f29bbbde693cd7c"></a>


### File `pkg/portallocator/portallocator.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5ec281e14e1b20895f83a886f5415c55f8f246c3`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/portallocator/portallocator.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-91e904c9b12fe58245.** [agones/pkg/portallocator/portallocator.go, lines 35–49](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/portallocator/portallocator.go#L35-L49). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4e3b761d3569ad855e1a0555824261bfa1bb4bba39444aeb4bc97e52b079a423`.

> 
> // Interface manages the dynamic port allocation strategy.
> //
> // The portallocator does not currently support mixing static portAllocations (or any pods with defined HostPort)
> // within the dynamic port range other than the ones it coordinates.
> type Interface interface {
> 	// Run sets up the current state of port allocations and
> 	// starts tracking Pod and Node changes
> 	Run(ctx context.Context) error
> 
> 	// Allocate assigns a port to the GameServer and returns it.
> 	Allocate(gs *agonesv1.GameServer) *agonesv1.GameServer
> 
> 	// DeAllocate marks the given ports as no longer allocated
> 	DeAllocate(gs *agonesv1.GameServer)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 20 lexically identified declarations, including `Interface` (L40), `PortRange` (L53), `portAllocator` (L60), `New` (L67), `newAllocator` (L73), `Run` (L87), `Allocate` (L97), `DeAllocate` (L105), `portAllocation` (L112), `portRangeAllocator` (L115), `newRangeAllocator` (L132), `Run` (L165), `Allocate` (L181), `DeAllocate` (L281), `syncDeleteGameServer` (L313), `syncAll` (L326), `registerExistingGameServerPorts` (L364), `nodePortAllocation` (L420), `newPortAllocation` (L433), `setPortAllocation` (L443). The full declaration list is recorded under `agones:pkg/portallocator/portallocator.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
