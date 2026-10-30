# CPG → Genco → Rust — combined implementation roadmap entrypoint


## Native Rust policy — AGONES_NATIVE_RUST_V1

**Direct quotation — USER_SUPPLIED_CURRENT_PROJECT_GUIDANCE; Q-9ea3891cc21daae253.** [Current AGENTS.MD, line 23](/home/astral/Projects/agones/AGENTS.MD:23). Source: uncommitted working tree on 30 October 2026; SHA-256 `d11c9403e7e0d16150c0f54f7bbdfe01bba04c9a7a1618eae4985d86981b3311`.

> 10. Existing Rust Ecosystem must be used to the fullest potential. Do NOT try to recreate Go semantics in Agones Rust reimplementation. Application behavior and semantics must only correspond to original Agones exclusively in Agones-specific semantics, all traces of go semantics must be fully removed and replaced with Rust Semantics in Rust reimplementation of Agones.

**Derived requirements.**

1. Preserve Agones-specific behavior: resource/API/protocol contracts, validation/defaults, allocation/scheduling/scaling decisions, lifecycle transitions, health, counters/lists and required operational outcomes. Rust supplies the language/runtime semantics throughout handwritten code, templates and generated output.
2. Use the Rust ecosystem to its fullest practical extent. Start with std and existing selected dependencies: Tokio/futures, serde and protocol bindings, selected native HTTP/TLS, Rusternetes, native filesystem notification and Rust logging facilities. Inspect supported APIs before adding infrastructure. Record a concrete Agones policy or integration gap for custom code; do not recreate an entire dependency because the original used a Go library.
3. **AGONES_REQUIRED** identifies domain/API obligations with source/docs evidence. **RUST_NATIVE** identifies the Rust library, type or effect providing infrastructure. **GO_ONLY_SUPERSEDED** retains the source identity and reason a Go-only mechanism is no longer required. Apply these within mixed files; never discard an Agones rule because its original implementation used a superseded mechanism.
4. Do not recreate Go fmt/strconv/reflection, typed-nil/interface method-set behavior, panic/recover/Goexit, goroutine scheduling/channel/select semantics, map hashing/range/growth, slice headers, GC, allocator or ABI behavior. These are not milestone gaps or final acceptance requirements. Existing compatibility code must be reviewed for reuse, replacement or retirement during implementation; file existence creates no obligation to finish that machinery.
5. Preserve JSON/protobuf/API names, field numbers, presence, ranges, defaults, required ordering and error responses where an Agones contract requires them. Use Rust types, Option/Result, checked domain arithmetic and serialization attributes/adapters. Wire compatibility does not authorize language/runtime emulation. Business errors remain required even when their Go implementation previously panicked; implement the intended domain outcome with Rust error policy.
6. Rust tasks, async cancellation, synchronization and RAII implement service lifetimes. CGP owns capabilities, runtime policy, resources, supervision and shutdown; derive-deftly generates repeated declarations and wiring. There is no separate Go runtime or compatibility manager behind these providers.
7. CPG retains source AST/types and graph facts as analysis/provenance inputs. The typed translation IR distinguishes Agones obligations from native implementation choices and superseded Go mechanisms. Genco emits Rust types, Rust library calls, CGP composition and derive-deftly declarations; it emits no Go interpreter or runtime compatibility engine. The complete native CPG graph/schema objective remains required.
8. Source-body coverage means every required Agones operation is mapped to a real Rust implementation or selected library facility with a production consumer. Retire Go-only obligations explicitly; do not erase source identities, substitute placeholders or weaken Agones behavior. Unknown Agones requirements remain open; unknown Go runtime internals are not blockers. Generation/coverage receipts bind this semantic-policy ID, Agones obligation IDs, native-library mappings and exact superseded Go-only identities; historical raw body counts cannot silently redefine the amended required set.
9. After the global gate, preserve independent Agones invariants and fixture identities in both reference-domain and production observations. Supersede Go-only assertions with authority/source IDs and add relevant Rust-native coverage. Do not prove Go runtime equivalence or count a superseded assertion as passed.
10. This amendment corrects active instructions and existing ledgers without restarting the Goal or changing alternation. Historical snapshots, prior measurements and forecast v1 remain historical. The current M0003 cursor remains IMPLEMENTING; no completion or verification credit follows from this policy change. The global production gate requires native adoption of required Agones operations and removal of Go-emulation machinery from their production paths; a Rust facade around that machinery is insufficient. Preserve existing code until its consumers and replacement/retirement are accounted for.

## Objective and invocation

Finish the complete required native Code Property Graph port, including all handwritten schema/build behavior and unused production modules; implement the source-derived CPG → normalized CPG → typed IR → Genco → Rust pipeline; generate complete Agones output; AND finish production Agones-rs, assurance and authorized delivery under one native Goal. Preserve existing implementation and the two source mirrors. First generation is a required UNVERIFIED artifact milestone, not the end of the objective.

```text
/goal Finish both Agones-rs and the complete required native CPG port and CPG -> Genco -> Rust generation pipeline under /home/astral/Projects/agones/roadmap/0000-contract.md and /home/astral/Projects/agones/roadmap/alternation-policy.md. Reuse the one matching native Goal and existing implementation. Follow the numbered dependency-ready roadmap, shared accounting policy and literal Agones/Scala path mappings. Make CGP AND derive-deftly the joint architecture, programming paradigm and lifecycle owners; give Python-familiar valid Rust equal status with idiomatic Rust. Make ZERO attempts to build, test, probe or verify any project component until all required production implementation, generated outputs, integration and architecture are authored and connected. Treat FIRST_FULL_GENERATION_COMPLETE as a milestone. Continue through the shared post-gate assurance and authorized delivery requirements until both objectives are fully complete. Set no token budget unless I explicitly provide one.
```

CPG means Code Property Graph. CGP means Context-Generic Programming. Genco emits source from explicit semantic inputs. CGP AND derive-deftly jointly own the architecture and lifecycle throughout both workstreams.

## Shared roadmap authority and exact source ownership

1. This route entrypoint refines the existing prompt; it participates in the **one combined objective**, not a competing Goal. Read [/home/astral/Projects/agones/roadmap/0000-contract.md](/home/astral/Projects/agones/roadmap/0000-contract.md), the current numbered milestone, and the separately authoritative [execution/accounting policy](/home/astral/Projects/agones/roadmap/alternation-policy.md). The [combined roadmap](/home/astral/Projects/agones/roadmap.md) is the deterministic portable edition; load bounded sections during execution.
2. Finish BOTH scopes. Preserve the full CPG port, including handwritten schema/build modules unused by Agones, and the complete source-derived generated Agones artifact as well as production Agones-rs and final assurance/delivery. The policy, not a cheaper-route preference, determines execution. Compare actuals with [cost-analysis.md](/home/astral/Projects/agones/cost-analysis.md) and record real milestone observations in [cost-analysis-comparison/](/home/astral/Projects/agones/cost-analysis-comparison/README.md). Missing credit charges are `unknown`.
3. Preserve the literal original hierarchy under `agones-rs/` and `cpg-rs/`. `pkg/gameservers/controller.go` becomes `agones-rs/pkg/gameservers/controller.rs`; Go `name_test.go` becomes `name_harness.rs`, NEVER `name_test.rs`. Preserve Scala's complete directory chain, capitalization and singular/plural stems: `CpgSchema.scala` becomes `CpgSchema.rs`. Preserve dotted names such as `sdk.pb.rs`. Cargo entrypoints, identifier/module-registration glue and Rust-only support stay outside those source mirrors. Resolve collisions explicitly; never silently rename.
4. Every subsystem, file, template, emitted body, API and later harness must express CGP AND derive-deftly ownership and Python-familiar valid Rust. CGP provides runtime capabilities, policy, resources, supervision, cancellation and cleanup; derive-deftly provides reusable declarations, lifecycle wiring, delegation and ownership metadata. Genco prints explicit semantic inputs; it replaces neither owner. Readability is judged from a Python developer's perspective and has equal status with Rust idiomaticness, without a Python language layer or hidden second implementation.
5. The [inventory](/home/astral/Projects/agones/verification/agones/roadmap/source-inventory.json) assigns every original path a disposition and every required file a milestone. Milestones include quoted source contracts and derived file/package instructions. Preserve current Rust through the [adoption map](/home/astral/Projects/agones/verification/agones/roadmap/reuse-inventory.json); candidate reuse is not established behavioral parity. Follow the pinned source and [schema reconciliation](/home/astral/Projects/agones/verification/agones/roadmap/schema-reconciliation.json), not a newer reference HEAD.
6. The shared strict gate applies to BOTH routes and every helper/tool. No project builds, compiler/macro/IDE/compatibility probes, tests, proofs, replay, generator validation or CI before complete production implementation. A necessary missing generator executable is a blocker, not an exception. First generation remains `UNVERIFIED` and does not complete the Goal. Preserve all later Bolero/Kani, source replay, architecture, integration and delivery obligations.
7. Source quotations and archived prompts are evidence. The current user's request and host rules govern. Editing this prompt does not invoke its Goal or authorize migration execution. The original two prompt texts and hashes remain inactive snapshots in `verification/agones/roadmap/`. The single shared policy owns scheduling; this entrypoint does not define a second switching algorithm.


## Compact execution contract

1. Recover current pins, source inventories, existing work, jobs, manifests and intact generation transactions. Preserve originals and user changes. A historical generation event supplies artifact evidence, not full combined completion.
2. Port every required handwritten CPG module/body, including schema and build machinery unused by Agones. Regenerate domain/protobuf material from complete inputs. Translate every required Agones production body without interpreter wrappers, copied handwritten output credit or semantic placeholders.
3. Retain one source-derived pipeline and versioned source/body/configuration/rule identities. Persist missing AST/type facts as bundle enrichment; never bypass CPG with an independent direct-source emitter.
4. Keep CGP AND derive-deftly central, Pythonic valid Rust coequal with Rust idioms, Rusternetes the only orchestrator and no plugin system. Preserve literal source-relative Go/Scala names; place Cargo/module support outside the mirrors.
5. Make ZERO attempts at project verification, including focused support builds and synthetic fixtures, until ALL required production implementation on both routes is authored and connected. The shared contract governs permitted emission-only operations and missing-executable blockers.
6. Implement complete schema, native port, bundle, IR, rules, runtime support, production adoption and transaction recovery. Receipt construction is intrinsic implementation bookkeeping; it grants no parsing, type, behavioral or proof credit.
7. Publish a complete generated artifact transactionally and record `FIRST_FULL_GENERATION_COMPLETE`; retain UNVERIFIED assurances and continue the shared objective under the separate policy. Do not regenerate an intact artifact merely to resume.
8. Use one matching native Goal and the existing canonical state/checkpoint links. Milestones are durable records, not invented native APIs. Read only the current milestone and relevant appendices; set no Goal token budget without explicit user instruction.

The numbered roadmap expands these retained obligations into source-derived file/package work. Appendix A owns source provenance/classification; B owns schema/translation/interfaces; C owns recovery; D owns transactional generation and combined completion. Scheduling and cost-accounting mechanics remain solely in the separate policy.

## Appendix A — Inputs, provenance, and source classification

### A1. Preserve and pin the actual inputs

Choose original Agones source in this order: an explicit user-selected revision; a valid established baseline with provenance; otherwise the continuation prompt's fallback, commit `6eb4a02588d8e7b581de187f3bc849733a78dcb2` of [QuasarRay/agones](https://github.com/QuasarRay/agones). Never substitute latest upstream. The existing [baseline manifest](/home/astral/Projects/agones/verification/agones/baseline.json) records that commit and tree `f50ac1ccf92be912e9475833b17deaebc5a6622f`; recover its source identity and inventory bindings using permitted read-only bookkeeping. If the selected source is unavailable, recover that exact source or record a blocker. Its test counts are not a production-body inventory.

Inspect [the current state](/home/astral/Projects/agones/AGONES_RS_STATE.json), [handoff](/home/astral/Projects/agones/AGONES_RS_HANDOFF.md), existing baseline/scope manifests, and source-generation code before introducing replacements. Reuse suitable Genco facilities, including [the existing build integration](/home/astral/Projects/agones/crates/agones-rusternetes/build.rs), and discover other current use sites. Existing Rust implementation is reusable machinery/reference material, not a replacement for the original-source census or evidence that a new body was generated through CPG.

Vendor real Genco and Code Property Graph source trees, including required schema/generator inputs, build metadata, and Genco's supporting macro crates. URLs, empty vendor directories, binary-only distributions, downloaded JARs, or Cargo dependency declarations alone do not meet source vendoring. Preserve upstream license/NOTICE files. Record repository URL, full commit and tree, per-file/content digests, package/version/checksum identities, submodules and LFS content where required, and the resolved dependency/tool closure. Keep a pristine upstream identity plus separately recorded patch files, order, reasons, licenses, and resulting digests; never present patched source as pristine.

Recover compatible installed/pinned tools first. Pin Joern `gosrc2cpg`, its actual CPG/schema/storage dependencies, and its matching `goastgen` dependency as one compatible tool set. Retain the required frontend source/build configuration and any export/enrichment patches, as well as tool binary checksums and source identities. Pin the Go/JDK/Scala/sbt/Rust and code-generation tools actually needed; record build flags, lockfiles, and non-secret environment affecting output. Reuse suitable Rust crates for storage, collections, protobuf/serialization, compression, and other underlying facilities with their dependency identities and licenses.

The earlier supplied CPG revision `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` and Joern revision `9f2171242e48d7fed3021087d45838a83c1e39da` are concrete source-inspection anchors, not an asserted compatible pair. Resolve and record compatibility from the selected frontend's `Versions.cpg`, resolved artifacts, schema, and graph format; choose compatible immutable revisions together and explain any departure from these anchors. The [Go frontend build configuration](https://github.com/joernio/joern/blob/9f2171242e48d7fed3021087d45838a83c1e39da/joern-cli/frontends/gosrc2cpg/build.sbt) obtains the `goastgen` version from application configuration and checks/stages matching binaries. Do not accept an arbitrary executable on `PATH` as matching evidence. Porting the Joern platform or replacing its Go frontend is outside this objective.

The retained pipeline now pins CPG `301c4010d28ecdac05bc2ed44310f56446459465` (v1.7.79). Current reference HEADs are separate. The roadmap records identical schema source blobs at the older CPGSchema anchor and this pipeline revision; native behavior reuse still requires exact body mappings. Do not upgrade pins or treat the old 394/695 map as current remaining work.

### A2. Automatically distinguish handwritten and generated CPG code

Build a repeatable inventory over the pinned repository, not a hand-picked set of large Scala files. Inspect generation tasks, build configuration, managed/unmanaged source directories, generated headers, checked-in output paths, generator templates, and recorded generation provenance. Fresh regeneration comparisons are deferred until the global gate. **File size is not classification evidence.** Directory names and headers are evidence to reconcile with build/generation facts, not infallible rules.

For each file, retain its path/blob, production/test/tool role, classification, supporting evidence, generator/input/tool identities when applicable, historical regeneration comparison (or explicitly deferred fresh comparison), and destination or disposition. Split `MIXED` files by symbols/spans: port handwritten extensions and preserve generator customizations while regenerating derived sections. Label unresolved provenance explicitly and resolve it before it can remove required work. Tests, examples, and third-party dependencies remain visible with justified roles; classifying a facility as third-party does not eliminate the behavior the port must provide through a Rust dependency or implementation.

Upstream explicitly derives domain classes from `CpgSchema.instance` using its Flatgraph configuration. Use that generation relationship to identify outputs and retain handwritten schema definitions as implementation inputs. [Pinned CPG schema build configuration](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/d02d76ea5c9e3b398317d334228d7e8e78ab45aa/schema/build.sbt).

The port inventory must cover **all handwritten production behavior across the repository**, including graph creation/mutation/query-facing APIs, loading and persistence, serialization/deserialization, passes and their lifecycle, utilities, schema definitions/composition/validation, schema export, protobuf/domain-generation machinery, and handwritten parts of nominally generated modules. Discover the actual module/body set from source and build definitions; this list is not an exclusion filter. Build-only schema machinery counts. Being unused by Agones, difficult to port, or located next to generated code is not an exclusion.

Map each required handwritten source body/module to Rust symbols and its implemented behavior or dependency-backed equivalent. A dependency may supply an underlying facility; the handwritten CPG semantics around it still need a complete mapping. Platform-specific build glue may have a concrete Rust-toolchain equivalent, with its behavior accounted for. An entire module marked “delegated,” a source file copied as a comment, or a successful narrow demo cannot establish port completion.

Generate Rust domain types, graph accessors, and derived bindings from the complete schema. Do not manually translate or maintain the generated Scala domain corpus. The resulting `cpg-rs` must perform its core operations and schema machinery in Rust. Upstream Scala tools may provide build-time extraction, bootstrap observations, and comparison fixtures; permanently delegating core graph/schema behavior back to Scala, Joern subprocesses, or a JVM service fails the port contract.

### A3. Establish independent Agones coverage

Inventory the pinned original source independently of Joern's output, using the source tree plus Go package/AST/type information and declared production build configurations. Discover relevant directories rather than treating a fixed directory list as scope. Include shipped first-party Go production code in `cmd`, `pkg`, SDK servers, and any additional production packages; generated Go production code; initialization; protobuf/schema/API definitions; embedded assets; configuration/defaults; and build/generator inputs needed to reproduce the output. Record dependencies and non-Go surfaces separately. Preserve the existing Rust SDK and public client protocols; rewriting every language SDK in Rust is not implied. Tests and examples used only as tests/examples are reference inputs; production behavior located there is still in scope.

Use stable body identities binding baseline, package, path/blob, receiver or containing symbol, source span, and build-configuration identity. Enumerate functions, methods, closures/function literals, `init` bodies, and executable package/global initializers. Track types, constants, schemas, resources, and required external-call/runtime support as additional output obligations. Account for the union of production build tags, platform variants, generated inputs, cgo/FFI boundaries, and module/replacement/vendor resolution; one successful host configuration cannot silently drop the others.

Carry forward existing scope decisions and their authority. Include deprecated-but-served behavior and unresolved obligations. Do not inherit a broad `ORCHESTRATOR_SPECIFIC_REFERENCE_ONLY` label as permission to omit Kubernetes-dependent production logic: separate deployment-specific material from required behavior and translate that behavior through the permitted CGP/Rusternetes architecture. No exclusion to make the translator succeed; user-authorized exclusions need exact identities and rationale, and uncertain required classifications block completion.

Explicitly account for GameServer, GameServerSet, Fleet, FleetAutoscaler, GameServerAllocation, GameServerAllocationPolicy and multi-cluster allocation; allocator gRPC/REST, authentication and mTLS; SDK streaming/health/counters/lists; schemas, protobuf/OpenAPI, admission/defaulting/validation; metadata/selectors/resource versions/deletion; feature gates/deprecated fields; configuration, telemetry, and lifecycle. Required remote Agones allocation protocols do not authorize another cluster orchestrator.

Reconcile the independent inventory with frontend diagnostics and graph/source mappings **by identity**, including files with zero bodies. For every required file/body, record its graph nodes or persisted AST/type enrichment and eventual IR/output mapping. Detect dropped files, parse failures, unresolved package loading, unexpected exclusions, and unsupported constructs even when the frontend exits successfully. Preserve diagnostics; repair extraction/configuration or persist faithful enrichment before lowering. Unknown totals, an exit code, file counts alone, and node counts alone cannot certify coverage.

## Appendix B — Schema, translation, and implementation interfaces

### B1. Complete schema representation contract

Create a versioned, machine-readable schema representation with source provenance. Preserve node and edge definitions; abstract/base and concrete types; inheritance and contained-node relationships; property types, defaults and cardinalities; identifiers and numeric/wire encodings; edge endpoints, direction and ordering constraints; hidden elements; and all other source metadata needed for graph operations, validation, serialization, and derived APIs. Preserve distinctions such as absent/default/empty values and inherited versus declared properties. Record source/tool/dependency hashes and deterministic ordering; unresolved required metadata blocks dependent generation.

Do not assume upstream's schema JSON is complete. At the earlier supplied source anchor, `Schema2Json.scala` filters hidden schema elements, and its property output and cardinality labels do not capture all types/default values needed by this contract. Extend the export or extract the full schema model with an explicit, provenance-bound transformation before generating Rust types. Preserve the standard export behavior as part of the Rust port while providing the richer internal representation. [Pinned upstream schema exporter](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/d02d76ea5c9e3b398317d334228d7e8e78ab45aa/schema2json/src/main/scala/Schema2Json.scala).

Reconcile the complete model against schema source and generation configuration by identifiers, relationships, property types/defaults/cardinalities, and hidden-element cases. Generate Rust domain types/accessors and required serialization bindings reproducibly from this model, with schema-to-output mappings. Port handwritten schema construction and transformation behavior too; a one-time JSON dump and generated structs alone do not reimplement the schema machinery. Record schema obligations during implementation; schema fixtures and ordering/default/inheritance checks are deferred until the global gate.

### B2. CPG input bundle contract

Use a versioned bundle that `cpg-rs` can load without requiring a live Joern/Scala process. Retain the original frontend artifact and its format/version, plus any faithful export needed for the Rust loader. Do not assume graph formats from different frontend/Flatgraph versions are interchangeable. Implement structural/property preservation from source contracts; demonstrate it on fixtures only after the shared global gate.

The bundle must contain or content-addressably reference retained artifacts for:

- Graph nodes, stable IDs, labels, edges/endpoints/direction, typed property values, multiplicities, and ordered relationships, including AST child order and argument/parameter indices. Preserve absent/null/empty/default distinctions and source-relevant scalar precision/encodings.
- Original repository/revision, per-file source identities and full source text, paths/spans, package/module membership, build tags, generated-source origins, embedded resources, and the independent file/body inventory.
- Go toolchain, target/configuration matrix, module/replacement/vendor resolution, relevant build configuration, frontend/schema/dependency identities, exact extraction commands, exit status, diagnostics, and unsupported/unresolved facts.
- Persisted enrichment from the pinned frontend's AST/type information or source-bound Go package/type tooling where CPG facts are missing. Bind each enriched fact to graph/source IDs, its provider/version, and its derivation; retain conflicting facts and resolve required ambiguity.
- The reconciliation result and schema/bundle digests consumed by normalization and lowering.

The emitter consumes normalized graph/typed IR, never an untracked side channel reading Go text to manufacture bodies. Source text retained in the bundle supports diagnostics and source-bound enrichment; copying it into Rust comments, opaque strings, or an embedded Go interpreter is not translation. Extra AST/type facts must be persisted into the bundle, loaded through `cpg-rs`, and carried through the same normalized representation and IR.

### B3. Explicit translation stages and joint CGP/derive-deftly ownership

Define typed stage inputs/outputs, error/diagnostic results, source maps, versioned rule identities, and content bindings. Derive-deftly owns reusable declarations, repeated stage implementations, lifecycle wiring and ownership metadata; CGP owns capabilities, runtime policy, resources and context composition. Use explicit CGP capabilities with statically selected providers for:

| Capability | Responsibility |
| --- | --- |
| Graph loading | Load the bundle through `cpg-rs`; preserve identities, schema semantics, ordering, and diagnostics. |
| Normalization | Produce a canonical CPG with explicit source/configuration relationships; retain provenance and meaningful distinctions. |
| Semantic lowering | Recover Agones domain operations and required API constraints, map them to Rust-native IR/library operations, and retain explicit Go-only supersession provenance. |
| Rust emission | Convert IR to Genco Rust tokens, imports, items, manifests, resources, and mappings deterministically. |
| Pipeline lifecycle | Own stage ordering, resource/task ownership, cancellation, recovery, staging, coverage gates, and transactional artifact publication. |

The top-level context invokes the lifecycle consumer; providers own initialization, readiness, retries, supervision, cancellation, drain/cleanup, and publication policy. Runtime libraries provide effects. Use ordinary Rust algorithms/values inside capability boundaries; do not force every helper into its own component or create a universal backend interface. Every spawned task has an owner and cleanup policy.

Genco provides token construction, imports, quoting, and layout; it does not supply Agones-domain translation into native Rust. Reuse its Rust support and existing integration, then implement the missing translation stages/rules. [Genco documentation](https://github.com/udoprog/genco).

Build a source-bound Agones obligation inventory before emission. Preserve Go AST/type facts as analysis inputs, then map domain operations and required API constraints to native Rust types, errors, control flow and selected library calls. Classify each obligation AGONES_REQUIRED, RUST_NATIVE or GO_ONLY_SUPERSEDED. Go formatting/reflection, typed-nil, panic/recover/Goexit, goroutine/channel/select, map/slice internals, GC, allocator and ABI emulation are excluded.

Every required Agones operation must have a substantive native Rust implementation or library operation with a real production consumer and source mapping. Callbacks and nested domain decisions remain accounted for. Retain source IDs for Go-only mechanisms with explicit supersession. Missing domain behavior, empty providers, absent helpers and fabricated success are incomplete; a real native library call implementing the contract is valid reuse.

Record each intentional approximation with affected source/body/operation IDs, emitted behavior, rationale, and known limitation. An approximation cannot omit an Agones domain operation. Native Rust replacements for Go-only mechanisms are required semantic changes with supersession records, not unsupported constructs or missing runtime helpers. A complete generated artifact is recorded as UNVERIFIED, not as correct Rust or completed migration. Known missing source behavior prevents implementation completion. Later discovered ownership/type/behavior defects reopen the production gate for repair; first generation does not terminate this combined Goal. Never claim an approximation is verified behavior.

### B4. Architecture and reuse constraints

Translate Kubernetes-dependent decisions, controllers, watches, retries, resource operations, and lifecycle behavior into CGP capabilities with Rusternetes as the only concrete orchestrator integration. Keep public/wire compatibility types, portable policy, and adapter types distinct. Preserve required metadata, selectors, Pod-template fields, protobuf/JSON names/presence/defaults, and remote protocols; Kubernetes-shaped data is not itself an additional orchestrator. Conversions belong at the integration boundary and must retain required data rather than replacing it with lossy neutral shapes. Do not invoke original Go services or a Kubernetes client behind a Rust facade as the translation.

CGP AND derive-deftly jointly own generated application composition and lifecycle as well as the generation architecture. Use typed declarations and reusable templates to produce the CGP provider/wiring structure, with runtime policy and resources remaining explicit in providers. Emit real production wiring for source-required startup, services/controllers, ordering, shutdown and resource ownership. Preserve existing Wasm-related obligations in the census; a dependency's type named `Plugin` is not permission to build application extensibility. If a required behavior genuinely conflicts with the no-plugin/no-other-orchestrator constraint, retain that exact blocker and pursue independent work; never silently omit the body or invent an exception.

Use the inherited toolkit actively and consistently:

| Facility | Required practical role |
| --- | --- |
| [CGP](https://github.com/contextgeneric/cgp) | Capabilities, context composition, provider wiring/delegation, and lifecycle ownership. |
| [derive-deftly](https://gitlab.torproject.org/Diziet/rust-derive-deftly) | Joint architecture owner: struct/enum-driven implementation rules, lifecycle/delegation wiring, ownership metadata, repeated domain behavior and composition. |
| [comprende](https://github.com/QuasarRay/comprende) | Concise, readable comprehension-based transformations where its evaluation/ownership semantics fit. Pin this source; do not substitute another upstream. |
| [better_comprehension](https://github.com/Natural-selection1/better-comprehension-in-rust) | Complementary collection/iterator comprehensions where explicit collection types, patterns, bindings, or laziness help. |
| Arcane-rs and proc_macro2 | Reuse relevant [synthez](https://github.com/arcane-rs/synthez) parsing/validation/token facilities, [delegation](https://github.com/arcane-rs/delegation) forwarding, and [Arcane](https://github.com/arcane-rs/arcane) expansion-core/shim/facade patterns before adding equivalent machinery. |

Retain the coherent `proc_macro_2`, `proc_macro_attribute_2`, `proc_macro_derive_2`, and `proc_macro_delegate_2` direction over a shared proc_macro2-oriented substrate, with compiler shims internal and normal Cargo ergonomics. Reuse existing capability notes and relevant source pins; inspect API/AST source compatibility without executing probes. Implement concrete needed machinery progressively and finish all four same-logical-crate/cross-crate capabilities before the shared gate. Their full acceptance matrix is required after the gate and remains part of this combined Goal.

Record meaningful reuse sites and repetition removed; unused dependencies do not count. Do not force harmful toy uses or duplicate transformations to exercise both comprehension libraries. Preserve idiomatic Rust ownership, borrowing, `Option`/`Result`, evaluation count/order, short-circuiting, allocation, collection order, and duplicate-key semantics. Keep Agones-specific contracts and native Rust semantics in reviewable rules/IR, not opaque per-feature templates. If existing Egglog facilities are useful, reuse only explicit source-bound normalization rules; do not introduce a new equality/proof campaign.

Require a brief positive completion/token-cost rationale before new general machinery: the concrete repetition or blocking need, existing reusable facility, smallest gap, and the deferred post-gate evidence obligation. Use direct clear Rust when a speculative abstraction would cost more. This gate reduces duplicated work; it never excuses missing CPG modules, Agones bodies, or translation support. Preserve existing implementation through additive tooling or minimal behavior-preserving integration; never blindly overwrite current crates with generated output. Author required adoption paths and source-to-canonical-owner transfers before the gate; preserve manual meaning and references, retain the separate generated artifact, and establish integration evidence after the gate.

### B5. Generation receipt contract

The generation artifact must include a machine-readable receipt and retained manifests with:

- Contract/objective identity; original Agones repository/commit/tree and scope/configuration/inventory digests; current tooling checkout and dirty-patch identity kept separate from the reference baseline.
- Genco, CPG, Joern, `goastgen`, Rust dependencies, tools, and patch identities/licenses; CPG provenance classification; handwritten-port mappings to implemented Rust symbols; complete schema and domain-generation identities.
- Original graph, complete input bundle/enrichment, frontend diagnostics/reconciliation, normalized CPG, typed IR, translation rules, emitter, support implementation, and configuration digests.
- The full expected file/body/resource/support sets; all emitted workspace files/manifests/resources with digests; every required body's source/graph/IR/rule IDs mapped to actual output paths, symbols and spans or byte ranges with content digests. Retain operation-level mappings where needed to show a body was not partially stubbed.
- Explicit required/emitted/missing/placeholder/unresolved coverage totals **and exact identity sets**; evidence that coverage is exhaustive, not just equal counts; known approximations and their affected identities; permitted exclusions with authority.
- Generation invocation, artifact/transaction ID and final path, receipt format version, publication protocol, and the `FIRST_FULL_GENERATION_COMPLETE` record. Set generated-Agones compilation, correctness, compatibility, and production readiness to `UNVERIFIED`.

Coverage receipts attest to complete source-derived generation. They are not formal proof, passing tests, verified compatibility, or production-readiness evidence. Preserve earlier verification receipts under their own bindings without upgrading them to cover this output.

## Appendix C — Native Goal, checkpoints and recovery

### C1. Use the actual native Goal controls

On implementation invocation, inspect the exposed controls and objective. Reuse the matching combined Goal. Create one only when the user's invocation authorizes it and no conflicting active Goal exists. `get_goal`, `create_goal` and `update_goal` are examples of exposed controls, not a milestone API. Respect live schemas, user pauses, host limits and the actual blocked-state threshold. Do not manufacture blocked turns, pause independently, set an unsolicited token budget or mark complete because a limit was reached. If controls/accounting are unavailable, report the limitation and retain ordinary authorized checkpoints; do not build a substitute scheduler.

### C2. Reuse one canonical project ledger

Reuse `AGONES_RS_STATE.json`, `AGONES_RS_HANDOFF.md` and the existing first-generation checkpoint as linked records. Add the current roadmap milestone, source bindings, owned outputs, gap/blocker, artifact path and usage-observation references to that canonical state when implementation is invoked. Do not create a parallel migration queue. Bind reusable work to the full relevant source/configuration/schema/rule/tool/dependency closure; additions, deletions and renames stale affected work. Preserve historical check receipts as historical; do not refresh them before the gate.

Checkpoint before long operations or interruption boundaries. Recover jobs by command, source binding, process/start identity or host job token and outputs; a PID alone is insufficient. Reuse caches and unchanged artifacts without rescanning everything. No duplicate jobs, deterministic retries without a changed hypothesis, destructive reset, silent stash or overwrite. No other chats, subagents, services or automations unless independently authorized.

### C3. Recover a generation milestone without declaring the Goal complete

Recover the checkpointed artifact's receipt using only necessary read-only identity/digest bookkeeping. An intact published `FIRST_FULL_GENERATION_COMPLETE` record establishes that transaction even if the external checkpoint lagged. Recover its milestone and accounting record, preserve its sealed source identities and continue the shared roadmap. It does not prove the native CPG port, generated behavior, final assurance or delivery complete. Do not regenerate or mutate an intact historical artifact merely because unrelated reference HEADs changed.

An incomplete staging directory, prepared record outside the committed artifact, missing required payload or damaged identity binding remains incomplete. Preserve recoverable content-bound work and the exact blocker. Never overwrite an unrelated artifact, infer completion from a directory name or fabricate the event. Required production repairs use new source-bound transactions with explicit ownership, and follow the global gate rather than a terminal first-generation stop.

## Appendix D — Transactional generation and full-objective completion

### D1. Implementation readiness before full emission

Finish all independent source/configuration/body inventories, CPG handwritten-port mappings, complete schema/domain data, bundle/IR/rule support, output obligations and runtime capabilities before the complete generation operation. Resolve required missing semantics through real implementation. All fresh fixtures, support compilation, API/macro probes, architecture validation and generator checks wait for the shared gate. Already runnable pinned generation tools may perform authorized source extraction/emission without check hooks; a required forbidden build is an explicit dependency blocker.

Do not disguise a full generation as a benchmark, preview, dry run or accumulated fixture shards. Intrinsic emission identity accounting and content hashes construct the artifact and its provenance; they are not a general verification campaign or proof of equivalent behavior. Finish required production adoption/call paths before the global gate, not in a supposedly post-gate integration implementation phase.

### D2. Publish a self-contained artifact transactionally

Choose and checkpoint a separate final artifact path, normally `generated/agones-first-generation/`, reusing an existing compatible location. It contains a literal `agones-rs/` source mirror, external `rust-support/agones/` Cargo/module wiring, complete resources/runtime providers and B5 evidence. Keep it outside the tooling build graph so later tooling checks do not accidentally build output before permission.

1. Freeze contract, source/configuration inventory, schema, bundle, IR, rules, tool and dependency identities. Resolve complete required file/body/resource/support sets before publication.
2. Emit into a new owned staging location. Produce actual non-placeholder bodies, exact paths and source-to-graph-to-IR-to-output mappings as part of emission. Never overwrite manual output or count copied handwritten bodies as translated.
3. Account for emitted identities, paths, ranges, resources and digests intrinsically during emission. Missing/placeholder/unresolved required identities prevent publication; equal counts alone are insufficient. No parsing, compilation, tests, formatting, semantic comparison or repair hooks attach to this operation.
4. Write B5 receipts and a non-circular payload digest manifest. The completion record binds the manifest rather than hashing itself. Include artifact identity/path, required/emitted/missing identity sets, known approximations and compilation/correctness/compatibility/production-readiness fields set to `UNVERIFIED`.
5. Flush and close files and writers, then atomically publish the complete self-contained directory on the appropriate filesystem. A flag in staging, partial copy or non-atomic multi-step move is not the completed transaction. Preserve interrupted staging with its identity and recover without overwriting unrelated work.
6. Record the atomic publication as `FIRST_FULL_GENERATION_COMPLETE`. Update the existing milestone/checkpoint/accounting references under the sole policy and continue the combined objective. A stale external checkpoint cannot invalidate an intact publication. This local artifact publication is distinct from Git push, PR, merge, release or deployment.

### D3. Continue until both full objectives are complete

First generation remains explicitly UNVERIFIED. The same Goal continues through all remaining production work and, only after the global implementation gate, source-test/property extraction, Bolero with Kani, source replay, native schema/graph/pipeline assurance, full `_2` matrix, Rusternetes integration and authorized delivery. Fix discovered production defects by closing the gate, finishing repairs, renewing the implementation record and then resuming affected assurance. Preserve generated-source provenance through repairs to rules/templates rather than untracked output patches.

Mark the matching native Goal complete only when the shared contract's full completion conditions hold for BOTH objectives, with no required unresolved work. Report host-required final usage if applicable. A pause, limit or actual external blocker yields an incomplete report with preserved paths, exact remaining obligations and the unlock condition. Neither prompt engineering, design, a partial port nor an unverified generated artifact is full completion.

### D4. Required decision scenarios

| Scenario | Required decision |
|---|---|
| All required bodies have substantive generated translations, but output has not been compiled | Record the complete generated artifact as UNVERIFIED; continue the shared objective without early checks. |
| A required source body, nested operation or runtime helper is absent/stubbed | Retain incomplete status and implement missing source-derived rules/support; never shrink expected scope or hand-patch generated output. |
| The frontend reports success but omits a required file/body | Retain unresolved coverage; repair source-bound extraction/enrichment before claiming complete generation. |
| Schema export loses hidden elements, defaults, identifiers, types or relationships | Finish the complete schema representation and dependent rules; Schema2Json's partial view is not full schema fidelity. |
| Interruption occurs after atomic publication | Recover intact artifact identity and its milestone; continue the one Goal without duplicate generation. |
| Interruption occurs before publication | Recover staging and keep milestone incomplete until the transaction is genuinely complete. |
| A needed new generator cannot run without a prohibited build | Record the exact dependency blocker; neither a renamed milestone nor output deferral opens the gate. |
| Unused handwritten CPG production behavior is missing | Keep CPG scope incomplete; unused-by-Agones is not an exclusion. |
| Full generation is complete but assurance/delivery remains | Continue required roadmap work in the same Goal. First generation is not the final completion event. |
