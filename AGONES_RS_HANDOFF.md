# Agones-rs and CPG handoff

1. Canonical ledger: `AGONES_RS_STATE.json`. Apply the current root AGENTS.MD, `roadmap/0000-contract.md`, the separate `roadmap/alternation-policy.md`, and semantic policy `AGONES_NATIVE_RUST_V1`. The immutable original Goal attachment remains a historical input where the later user-authorized amendment supersedes it.
2. Current cursor: M0003 / MANUAL / IMPLEMENTING / READY. The existing native Goal is active; implementation has resumed under the amendment. M0001 and M0002 retain their historical IMPLEMENTED_UNVERIFIED records. Neither is newly tested or recertified by this amendment.
3. Phase: IMPLEMENTATION_ONLY. Global BOTH-route gate: CLOSED. Verification: NOT_STARTED. No project build/test/probe/proof/generator/CI run is authorized until the complete required production scope and outputs are authored. A needed prohibited emitter build remains an explicit blocker.

## Immediate next action

Continue M0003 MANUAL under AGONES_NATIVE_RUST_V1. Native queue keys/storage, owned Tokio DelayQueue scheduling, Rust-duration retry policy and fallible worker supervision are authored; see roadmap-M0003-native-queue-adoption.json. Transfer remaining controller queue consumers and reconcile HTTP/gateway/crash consumers so legacy runtime resources and Go emulation can leave production. Preserve all 13 utility gaps until native consumer/body reconciliation is complete. No route switch or project executable run; global BOTH-route gate remains CLOSED.

## Current semantic obligations and records

1. Preserve Agones resource/API/protocol contracts, defaults, validation, allocation/scheduling rules, lifecycle/health/counter/list behavior and required operational outcomes. Use native Rust ownership, errors, formatting/logging, HTTP/TLS/serialization and async/effect facilities through CGP; derive-deftly supplies declarations, metadata and wiring.
2. Active M0003 gaps are in `verification/agones/roadmap-M0003-partial-map.json`. Preserve all 14 file owners, but reconcile their actual Agones obligations and native production consumers. The absence of a named-body gap is not semantic completion.
3. Go fmt/reflection/typed-nil/panic/recover/Goexit, goroutine/channel/select, map/slice layout, GC/allocator/ABI reconstruction is GO_ONLY_SUPERSEDED. Existing implementations and their historical authored-operation records remain available; they are not additional completion prerequisites.
4. Preserve unresolved Agones endpoints, error/status classes, retry/backoff, readiness, admission, watching, configuration, signal shutdown and resource/task lifecycle requirements. Retiring a Go mechanism cannot remove a domain requirement.
5. Retain the complete native CPG graph/schema/tooling and source-derived generation objectives. CPG input may retain original Go facts for provenance; generated Agones emits native Rust and real library calls, under the same policy as the manual route.
6. Preserve M0003 usage-start/progress observations and completed M0001/M0002 cost comparisons. No new implementation completion, route switch, fabricated measurement or credit charge is recorded by this amendment. Original forecast v1 is retained as a historical baseline.

## Recovery and provenance

Read `verification/agones/roadmap/native-rust-policy-amendment.md` and the current milestone/affected file entries. The amendment inputs and compressed history preserve previous state/handoff, source profiles and gap text. Historical compatibility requirements do not reactivate on resumption. Exact source paths, harness naming, CGP/derive-deftly ownership, Pythonic valid Rust and final post-gate assurance/delivery remain required.

## Native runtime adoption checkpoint

Native Rust JSON logging, structured error fields, Agones error construction and cause traversal, queue conflict/trace classification, configuration-fallback warnings and parent-owned Tokio supervision are authored in this partial batch. See `verification/agones/roadmap-M0003-native-runtime-adoption.json` for source operations, concrete consumers, exact payload identities and remaining legacy paths. The unfinished Go text-logging draft is preserved in `verification/agones/roadmap/superseded-M0003-klog-draft.json.gz` and removed from production registration. No compilation, test, runtime, proof or milestone-completion credit is claimed. The policy amendment is preserved in local commit `84f90aeb7444aeb9805bd1950449f3ddfaa3c20a`; native changes receive a separate local checkpoint. No push or CI was invoked.

## Native queue authoring checkpoint

Native Arc<str> keys, standard collections, DelayQueue scheduling, Duration-based retry/burst policy and fallible supervised worker execution are authored. The prior util::workerqueue module is preserved on disk and unregistered after source caller reconciliation; the public path now exports the canonical source-derived module. Existing Rusternetes controller queues remain unchanged and require explicit adoption. See `verification/agones/roadmap-M0003-native-queue-adoption.json`. State revision 386 remains M0003 / MANUAL / IMPLEMENTING, uncompiled and unverified, with the global gate CLOSED.
