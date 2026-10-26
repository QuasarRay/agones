# Agones: first complete CPG-to-Rust generation

## Objective and invocation

**Vendor the required sources, reimplement all handwritten production behavior in the Code Property Graph repository as `cpg-rs`, including its schema machinery, connect it to Genco, and generate the first complete Rust translation of the original Agones production implementation—then immediately stop.**

The deliverable is the **first complete generated translation**. Compilation, correctness, compatibility, and production readiness of generated Agones remain **unverified**. Compilation errors and behavioral flaws do not extend this goal. Missing production bodies, untranslated operations replaced by placeholders, and placeholder-only files prevent completion.

This is a standalone execution contract derived from [the existing continuation prompt](/home/astral/Projects/agones/AGONES_RS_CONTINUATION_PROMPT.md). Preserve that file, its local edits, the original reference source, and the current implementation. Work in `/home/astral/Projects/agones`, repository `https://github.com/QuasarRay/agones`, on `agones-rs-dev`; never modify `main`. Generate into a separate workspace. The broader migration's verification, production-readiness, property-extraction, and delivery gates are deferred beyond this goal. Do not enter that continuation loop after generation.

Copy this invocation into the implementation chat:

```text
/goal Execute /home/astral/Projects/agones/AGONES_CPG_FIRST_GENERATION_GOAL_PROMPT.md. Recover existing work and the established original-Agones baseline; vendor pinned Genco and Code Property Graph sources; automatically classify handwritten versus generated CPG code; reimplement all handwritten CPG production behavior, including schema machinery, as cpg-rs; generate Rust domain types from the complete schema; integrate cpg-rs and Genco through CGP; reuse compatible pinned Joern gosrc2cpg and goastgen build-time tools; and generate every in-scope Agones production body into a separate Rust workspace with source mappings and content-bound receipts. Preserve the existing prompt and implementation, CGP lifecycle ownership, Rusternetes-only integration, no plugin system, Arcane-rs reuse, and the required metaprogramming toolkit. Complete only at transactional FIRST_FULL_GENERATION_COMPLETE, then perform only native Goal completion bookkeeping and a short final response. Generated Agones compilation, correctness, compatibility, and production readiness remain unverified. Never compile, test, format, or repair the completed Agones output within this goal. Reuse an intact completion record after interruption without regenerating. Set no token budget unless I explicitly supply one.
```

**Terminology:** **CPG** means the **Code Property Graph** representation and its upstream repository. **CGP** means the **Context-Generic Programming** architecture for capabilities, providers, composition, and lifecycle. They are different systems. Genco supplies Rust emission; the integration developed here supplies the Go-to-Rust translation rules.

## Compact execution contract

1. **Recover before replacing.** Inspect applicable repository guidance, branch and dirty state, existing source pins, inventories, Genco integration, tools, jobs, caches, and receipts. Preserve user work and completed implementation. A valid completion record takes the terminal recovery path in Appendix D before substantive work.
2. **Keep both coverage obligations exhaustive.** Port every handwritten production module in the pinned CPG repository, including modules unused by Agones. Translate every in-scope original Agones production body. Generated CPG domain classes are regenerated from the complete schema; generated Agones production source is not excluded merely because it is generated. Unknown required scope cannot count as complete.
3. **Use one source-derived pipeline.** `CPG input bundle → normalized CPG → typed translation IR → Genco Rust tokens → generated workspace`. Preserve source identities through every stage. Enrich missing facts from upstream AST/type information inside the bundle; never bypass CPG with an independent direct-source emitter.
4. **Retain the architecture.** CGP owns composition and application/pipeline lifecycle. Rusternetes is the sole orchestrator integration. No plugin system, runtime provider registry, service locator, extension ABI, or alternative orchestrator adapter. Reuse Arcane-rs and the proc_macro2-oriented approach; actively use CGP, `derive-deftly`, `comprende`, and `better_comprehension` where they remove repeated work with idiomatic Rust semantics.
5. **Distinguish support from verified output.** Implement the CPG port and generation machinery concretely; wrappers around Scala, stubs, and fixtures standing in for production support do not satisfy them. Focused builds and synthetic fixtures for that machinery are permitted before the first full Agones output. Generated Agones is not subject to a compile, test, or correctness acceptance gate here.
6. **Finish support before final emission.** Establish the independent source inventory, all output obligations, complete schema, full handwritten-port accounting, frontend reconciliation, translation-rule coverage, and required support implementations before emitting the full workspace. Missing rules require generator work; they cannot become blanket `todo!`, `unimplemented!`, unconditional unsupported panics, silent no-ops, or fabricated success values.
7. **Publish once and stop.** The generation operation itself produces coverage receipts, digests, and the completion record and transactionally publishes the whole artifact. That commit is `FIRST_FULL_GENERATION_COMPLETE`. Afterward, do only native Goal completion bookkeeping and a short final response. No generated-Agones compilation, tests, formatting, model checking, property extraction, repairs, refactoring, another full generation, deployment, or PR work.
8. **Use native Goals and compact recovery.** Reuse a matching active Goal and content-bound repository checkpoints. Honor user pauses, host limits, and actual tool transition rules. Set a token budget only when explicitly supplied. No replacement Goal scheduler, persistence service, or new background automation.

Run these stages in order; reuse current results instead of replaying completed stages:

| Stage | Work and exit evidence |
| --- | --- |
| 1. Recover and pin | Recover the original baseline and existing integration; vendor actual required source; record immutable revisions, dependency identities, licenses, tools, and separate patches. |
| 2. Classify CPG source | Produce an automated provenance inventory with `HANDWRITTEN`, `GENERATED`, `MIXED`, `THIRD_PARTY`, and `UNRESOLVED` dispositions and reproducible generation evidence. Resolve required uncertainty. |
| 3. Implement `cpg-rs` | Map every handwritten production module/body to concrete Rust behavior, including schema construction and tooling; generate derived domain types/accessors from the complete schema. |
| 4. Integrate translation | Implement explicit CGP stages and typed interfaces, Go semantic lowering, Genco emission, required support, and transactional publication/recovery. Exercise focused synthetic fixtures. |
| 5. Extract Agones | Run pinned `gosrc2cpg` with matching `goastgen`; retain diagnostics and source/configuration evidence; reconcile files and bodies against the independent census; persist necessary enrichment. |
| 6. Generate and finish | Complete construct/rule and output-obligation accounting, emit the full separate workspace and receipts in one transaction, publish `FIRST_FULL_GENERATION_COMPLETE`, complete the native Goal, and stop. |

On resumption, read the compact checkpoint and the relevant appendix, not the full history. Appendix A owns source identity/classification, B owns translation/interfaces, C owns content-bound recovery, and D owns publication and stopping.

## Appendix A — Inputs, provenance, and source classification

### A1. Preserve and pin the actual inputs

Choose original Agones source in this order: an explicit user-selected revision; a valid established baseline with provenance; otherwise the continuation prompt's fallback, commit `6eb4a02588d8e7b581de187f3bc849733a78dcb2` of [QuasarRay/agones](https://github.com/QuasarRay/agones). Never substitute latest upstream. The existing [baseline manifest](/home/astral/Projects/agones/verification/agones/baseline.json) records that commit and tree `f50ac1ccf92be912e9475833b17deaebc5a6622f`; revalidate its source identity and inventory bindings on execution. If the selected source is unavailable, recover that exact source or record a blocker. Its test counts are not a production-body inventory.

Inspect [the current state](/home/astral/Projects/agones/AGONES_RS_STATE.json), [handoff](/home/astral/Projects/agones/AGONES_RS_HANDOFF.md), existing baseline/scope manifests, and source-generation code before introducing replacements. Reuse suitable Genco facilities, including [the existing build integration](/home/astral/Projects/agones/crates/agones-rusternetes/build.rs), and discover other current use sites. Existing Rust implementation is reusable machinery/reference material, not a replacement for the original-source census or evidence that a new body was generated through CPG.

Vendor real Genco and Code Property Graph source trees, including required schema/generator inputs, build metadata, and Genco's supporting macro crates. URLs, empty vendor directories, binary-only distributions, downloaded JARs, or Cargo dependency declarations alone do not meet source vendoring. Preserve upstream license/NOTICE files. Record repository URL, full commit and tree, per-file/content digests, package/version/checksum identities, submodules and LFS content where required, and the resolved dependency/tool closure. Keep a pristine upstream identity plus separately recorded patch files, order, reasons, licenses, and resulting digests; never present patched source as pristine.

Recover compatible installed/pinned tools first. Pin Joern `gosrc2cpg`, its actual CPG/schema/storage dependencies, and its matching `goastgen` dependency as one compatible tool set. Retain the required frontend source/build configuration and any export/enrichment patches, as well as tool binary checksums and source identities. Pin the Go/JDK/Scala/sbt/Rust and code-generation tools actually needed; record build flags, lockfiles, and non-secret environment affecting output. Reuse suitable Rust crates for storage, collections, protobuf/serialization, compression, and other underlying facilities with their dependency identities and licenses.

The supplied CPG revision `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` and Joern revision `9f2171242e48d7fed3021087d45838a83c1e39da` are concrete source-inspection anchors, not an asserted compatible pair. Resolve and record compatibility from the selected frontend's `Versions.cpg`, resolved artifacts, schema, and graph format; choose compatible immutable revisions together and explain any departure from these anchors. The [Go frontend build configuration](https://github.com/joernio/joern/blob/9f2171242e48d7fed3021087d45838a83c1e39da/joern-cli/frontends/gosrc2cpg/build.sbt) obtains the `goastgen` version from application configuration and checks/stages matching binaries. Do not accept an arbitrary executable on `PATH` as matching evidence. Porting the Joern platform or replacing its Go frontend is outside this objective.

### A2. Automatically distinguish handwritten and generated CPG code

Build a repeatable inventory over the pinned repository, not a hand-picked set of large Scala files. Inspect generation tasks, build configuration, managed/unmanaged source directories, generated headers, checked-in output paths, generator templates, and clean regeneration comparisons in an isolated directory. **File size is not classification evidence.** Directory names and headers are evidence to reconcile with build/generation facts, not infallible rules.

For each file, retain its path/blob, production/test/tool role, classification, supporting evidence, generator/input/tool identities when applicable, regeneration comparison, and destination or disposition. Split `MIXED` files by symbols/spans: port handwritten extensions and preserve generator customizations while regenerating derived sections. Label unresolved provenance explicitly and resolve it before it can remove required work. Tests, examples, and third-party dependencies remain visible with justified roles; classifying a facility as third-party does not eliminate the behavior the port must provide through a Rust dependency or implementation.

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

Do not assume upstream's schema JSON is complete. At the supplied pin, `Schema2Json.scala` filters hidden schema elements, and its property output and cardinality labels do not capture all types/default values needed by this contract. Extend the export or extract the full schema model with an explicit, provenance-bound transformation before generating Rust types. Preserve the standard export behavior as part of the Rust port while providing the richer internal representation. [Pinned upstream schema exporter](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/d02d76ea5c9e3b398317d334228d7e8e78ab45aa/schema2json/src/main/scala/Schema2Json.scala).

Reconcile the complete model against schema source and generation configuration by identifiers, relationships, property types/defaults/cardinalities, and hidden-element cases. Generate Rust domain types/accessors and required serialization bindings reproducibly from this model, with schema-to-output mappings. Port handwritten schema construction and transformation behavior too; a one-time JSON dump and generated structs alone do not reimplement the schema machinery. Focused schema fixtures should expose missing metadata and ordering/default/inheritance errors before full Agones emission.

### B2. CPG input bundle contract

Use a versioned bundle that `cpg-rs` can load without requiring a live Joern/Scala process. Retain the original frontend artifact and its format/version, plus any faithful export needed for the Rust loader. Do not assume graph formats from different frontend/Flatgraph versions are interchangeable. Demonstrate structural/property preservation on focused fixtures before accepting a format adapter.

The bundle must contain or content-addressably reference retained artifacts for:

- Graph nodes, stable IDs, labels, edges/endpoints/direction, typed property values, multiplicities, and ordered relationships, including AST child order and argument/parameter indices. Preserve absent/null/empty/default distinctions and source-relevant scalar precision/encodings.
- Original repository/revision, per-file source identities and full source text, paths/spans, package/module membership, build tags, generated-source origins, embedded resources, and the independent file/body inventory.
- Go toolchain, target/configuration matrix, module/replacement/vendor resolution, relevant build configuration, frontend/schema/dependency identities, exact extraction commands, exit status, diagnostics, and unsupported/unresolved facts.
- Persisted enrichment from the pinned frontend's AST/type information or source-bound Go package/type tooling where CPG facts are missing. Bind each enriched fact to graph/source IDs, its provider/version, and its derivation; retain conflicting facts and resolve required ambiguity.
- The reconciliation result and schema/bundle digests consumed by normalization and lowering.

The emitter consumes normalized graph/typed IR, never an untracked side channel reading Go text to manufacture bodies. Source text retained in the bundle supports diagnostics and source-bound enrichment; copying it into Rust comments, opaque strings, or an embedded Go interpreter is not translation. Extra AST/type facts must be persisted into the bundle, loaded through `cpg-rs`, and carried through the same normalized representation and IR.

### B3. Explicit translation stages and CGP ownership

Define typed stage inputs/outputs, error/diagnostic results, source maps, versioned rule identities, and content bindings. Use explicit CGP capabilities with statically selected providers for:

| Capability | Responsibility |
| --- | --- |
| Graph loading | Load the bundle through `cpg-rs`; preserve identities, schema semantics, ordering, and diagnostics. |
| Normalization | Produce a canonical CPG with explicit source/configuration relationships; retain provenance and meaningful distinctions. |
| Semantic lowering | Translate source types, declarations, control/data flow, effects, and public contracts into typed translation IR using reviewable Go-to-Rust rules. |
| Rust emission | Convert IR to Genco Rust tokens, imports, items, manifests, resources, and mappings deterministically. |
| Pipeline lifecycle | Own stage ordering, resource/task ownership, cancellation, recovery, staging, coverage gates, and terminal artifact publication. |

The top-level context invokes the lifecycle consumer; providers own initialization, readiness, retries, supervision, cancellation, drain/cleanup, and publication policy. Runtime libraries provide effects. Use ordinary Rust algorithms/values inside capability boundaries; do not force every helper into its own component or create a universal backend interface. Every spawned task has an owner and cleanup policy.

Genco provides token construction, imports, quoting, and layout; it does not supply Go semantic translation. Reuse its Rust support and existing integration, then implement the missing translation stages/rules. [Genco documentation](https://github.com/udoprog/genco).

Build a construct/support inventory from **all** required source bodies before final emission. Cover the source's actual types, expressions/statements, evaluation order, mutation/aliasing, integer/float behavior, nil/zero/empty values, interfaces/type assertions, generics, multiple returns/errors, initialization, defer/panic/recover, goroutines/channels/select, context/cancellation, clocks, reflection/serialization, and external calls where present. Represent operations and source type information explicitly in the IR, with graph/source spans and lowering-rule IDs. Do not confuse this source analysis with deferred extraction of source-test properties.

Every required body must have a substantive source-derived Rust implementation. Preserve its operations and control-flow obligations through explicit lowering, including callbacks and nested bodies; required runtime helpers and capability providers must also contain actual implementations. Shared generated implementations are acceptable when mappings show how each original body and its operations are represented. A symbol declaration, trait signature, forwarding call to an absent helper, generic success return, unsupported-operation stub, or blanket placeholder does not discharge a body. Source-authored panic/error/no-op behavior is allowed when specifically traced to the original semantics; a text search for `panic!` is not a sufficient placeholder audit.

Record each intentional approximation with affected source/body/operation IDs, emitted behavior, rationale, and known limitation. An approximation must still implement the source operation concretely. An unimplemented construct is a coverage gap, not an approximation. Imperfect emitted ownership/types, Rust compilation defects, or behavioral mistakes in an implemented rule are acceptable at this milestone; do not turn their repair into a prerequisite once complete generation is ready. Never claim an approximation is verified behavior.

### B4. Architecture and reuse constraints

Translate Kubernetes-dependent decisions, controllers, watches, retries, resource operations, and lifecycle behavior into CGP capabilities with Rusternetes as the only concrete orchestrator integration. Keep public/wire compatibility types, portable policy, and adapter types distinct. Preserve required metadata, selectors, Pod-template fields, protobuf/JSON names/presence/defaults, and remote protocols; Kubernetes-shaped data is not itself an additional orchestrator. Conversions belong at the integration boundary and must retain required data rather than replacing it with lossy neutral shapes. Do not invoke original Go services or a Kubernetes client behind a Rust facade as the translation.

CGP owns generated application composition and lifecycle as well as generation lifecycle. Emit real production wiring for source-required startup, services/controllers, ordering, shutdown and resource ownership. Preserve existing Wasm-related obligations in the census; a dependency's type named `Plugin` is not permission to build application extensibility. If a required behavior genuinely conflicts with the no-plugin/no-other-orchestrator constraint, retain that exact blocker and pursue independent work; never silently omit the body or invent an exception.

Use the inherited toolkit actively and consistently:

| Facility | Required practical role |
| --- | --- |
| [CGP](https://github.com/contextgeneric/cgp) | Capabilities, context composition, provider wiring/delegation, and lifecycle ownership. |
| [derive-deftly](https://gitlab.torproject.org/Diziet/rust-derive-deftly) | Reusable struct/enum-driven templates for repeated domain metadata, conversions, and implementation scaffolding. |
| [comprende](https://github.com/QuasarRay/comprende) | Concise, readable comprehension-based transformations where its evaluation/ownership semantics fit. Pin this source; do not substitute another upstream. |
| [better_comprehension](https://github.com/Natural-selection1/better-comprehension-in-rust) | Complementary collection/iterator comprehensions where explicit collection types, patterns, bindings, or laziness help. |
| Arcane-rs and proc_macro2 | Reuse relevant [synthez](https://github.com/arcane-rs/synthez) parsing/validation/token facilities, [delegation](https://github.com/arcane-rs/delegation) forwarding, and [Arcane](https://github.com/arcane-rs/arcane) expansion-core/shim/facade patterns before adding equivalent machinery. |

Retain the coherent `proc_macro_2`, `proc_macro_attribute_2`, `proc_macro_derive_2`, and `proc_macro_delegate_2` direction over a shared proc_macro2-oriented substrate, with compiler shims internal and normal Cargo ergonomics. Reuse existing capability notes and relevant source pins; check actual crate API/AST-version compatibility. Implement only machinery needed for this CPG port and first-generation pipeline. The broader same-logical-crate/cross-crate acceptance matrix remains a later migration requirement and cannot extend this goal after the terminal event.

Record meaningful reuse sites and repetition removed; unused dependencies do not count. Do not force harmful toy uses or duplicate transformations to exercise both comprehension libraries. Preserve idiomatic Rust ownership, borrowing, `Option`/`Result`, evaluation count/order, short-circuiting, allocation, collection order, and duplicate-key semantics. Keep source semantics in reviewable rules/IR, not opaque per-feature templates. If existing Egglog facilities are useful, reuse only explicit source-bound normalization rules; do not introduce a new equality/proof campaign.

Require a brief positive completion/token-cost rationale before new general machinery: the concrete repetition or blocking need, existing reusable facility, smallest gap, and focused check. Use direct clear Rust when a speculative abstraction would cost more. This gate reduces duplicated work; it never excuses missing CPG modules, Agones bodies, or translation support. Preserve existing implementation through additive tooling or minimal behavior-preserving integration; never replace current crates with the newly generated workspace.

### B5. Generation receipt contract

The terminal artifact must include a machine-readable receipt and retained manifests with:

- Contract/objective identity; original Agones repository/commit/tree and scope/configuration/inventory digests; current tooling checkout and dirty-patch identity kept separate from the reference baseline.
- Genco, CPG, Joern, `goastgen`, Rust dependencies, tools, and patch identities/licenses; CPG provenance classification; handwritten-port mappings to implemented Rust symbols; complete schema and domain-generation identities.
- Original graph, complete input bundle/enrichment, frontend diagnostics/reconciliation, normalized CPG, typed IR, translation rules, emitter, support implementation, and configuration digests.
- The full expected file/body/resource/support sets; all emitted workspace files/manifests/resources with digests; every required body's source/graph/IR/rule IDs mapped to actual output paths, symbols and spans or byte ranges with content digests. Retain operation-level mappings where needed to show a body was not partially stubbed.
- Explicit required/emitted/missing/placeholder/unresolved coverage totals **and exact identity sets**; evidence that coverage is exhaustive, not just equal counts; known approximations and their affected identities; permitted exclusions with authority.
- Generation invocation, artifact/transaction ID and final path, receipt format version, publication protocol, and the `FIRST_FULL_GENERATION_COMPLETE` record. Set generated-Agones compilation, correctness, compatibility, and production readiness to `UNVERIFIED`.

Coverage receipts attest to complete source-derived generation. They are not formal proof, passing tests, verified compatibility, or production-readiness evidence. Preserve earlier verification receipts under their own bindings without upgrading them to cover this output.

## Appendix C — Native Goals, checkpoints, and recovery

### C1. Use the host's actual Goal controls

On execution, inspect the exposed Goal controls and current objective. Reuse a matching active Goal; create one only when explicitly authorized by the user's invocation and the host permits it. When available, use `get_goal`, `create_goal`, and `update_goal` according to their live schemas. Do not overwrite an unrelated active Goal or invent an objective-edit/resume API. `/goal`, `/goal pause`, `/goal resume`, and `/goal clear` are user-facing controls, not shell commands. Native Goals provide persistent objectives with completion grounded in concrete artifacts. [Official Codex Goals documentation](https://developers.openai.com/cookbook/examples/codex/using_goals_in_codex).

User pauses and host limits govern permission to continue. Set a numeric token budget only when the user explicitly supplies one; report usage with the host's units, or `unknown`. Do not mark complete because a budget is exhausted. Use `blocked` only under the host's actual transition preconditions, including any required consecutive blocked turns; do not manufacture turns or pause a Goal on your own. Preserve partial work and the precise blocker/unlock condition while continuing independent authorized work. If native Goals are unavailable, report that limitation and retain ordinary authorized progress/checkpoints without claiming automatic persistence or building a substitute.

### C2. Keep one compact repository checkpoint

Reuse existing state/manifest facilities where they can represent this narrower objective. If needed, add one small checkpoint, such as `verification/agones/first-generation-checkpoint.json`, linked from the existing state; do not build a parallel migration queue, database, or Goal framework. The checkpoint indexes artifacts rather than replacing them. Keep only contract/baseline/scope bindings; current stage; inventory/schema/bundle/rule/support digests; completed port/rule mappings; owned paths and dirty-work identity; active job identity/logs; cache references; precise blockers; next action; and the predetermined final artifact path. Save decisions and evidence, not reasoning transcripts.

Bind reusable work to its actual dependency closure: source tree/file inventory, configurations, schema, generator/rules, dependencies/tools, and patches. Detect added/deleted/renamed inputs. Reuse unchanged source analysis, vendor trees, schema/domain generation, completed handwritten-port work, extraction, IR, and focused checks; refresh only affected work when bindings change. Missing or stale evidence receives no credit. Do not repeatedly rescan the entire repository, reread every appendix, or rebuild unchanged tools. No identical deterministic retry without a changed hypothesis; record bounded retries for transient failures.

Checkpoint before long jobs and interruption boundaries. Recover a running job only after matching its command, source binding, process/start identity or host job token, and output paths; PID alone is insufficient. Do not launch duplicate extraction/generation jobs. Protect user edits and overlapping outputs; no destructive reset, silent stash, or overwrite. No other chats, sub-agents, background services, or automations unless independently authorized.

### C3. Recover completion before doing more work

At entry/resumption, first look for the final artifact's completion record at the checkpointed path. Perform only the minimal read-only identity/integrity checks needed to establish that the committed receipt belongs to this objective and its sealed files match their recorded digests. This is recovery of existing completion evidence, not a new generation or verification campaign.

An intact published `FIRST_FULL_GENERATION_COMPLETE` record is authoritative even if the checkpoint still says emission was pending or the host Goal was never marked complete. Finish native Goal bookkeeping and give the short terminal response; do not regenerate or amend the artifact. Compare its sealed inputs to its recorded contract/baseline, not to an unrelated newer dependency or checkout. A new source revision or repair campaign requires a new user instruction, not automatic invalidation of an already completed first-generation event.

An incomplete staging directory, a prepared record outside the committed artifact, missing required files, or invalid digests do not establish completion. Preserve and identify the incomplete/damaged transaction; recover intact content-bound partial work without crediting it as complete. Report an irrecoverable external blocker truthfully. Never overwrite an unrelated artifact, infer success from a directory name, or fabricate the terminal event.

## Appendix D — Final emission and immediate stopping

### D1. Establish readiness before producing the full output

Before final emission, all required CPG handwritten modules have concrete Rust mappings; the complete schema and derived types are available; the independent Agones inventory and output obligations are fixed; frontend file/body reconciliation is complete; every required construct has a lowering/emission rule; and all required pipeline/runtime support is implemented. Required unknown classifications, missing source facts needed for translation, unsupported operations, and placeholders must be resolved. Record intentional semantic approximations separately without claiming correctness.

Permit focused builds, checks, and synthetic fixtures for `cpg-rs`, schema generation, loading, lowering, Genco emission, CGP wiring, and publication/recovery **before** this operation. Exercise incomplete coverage rejection and interrupted-publication recovery using synthetic data. Do not require broad formal verification or the migration's full architecture matrix. Do not compile, test, or format the full generated Agones workspace as a hidden readiness check.

Whole-source analysis, graph extraction, and IR coverage analysis may precede emission. A full Agones Rust generation may not be disguised as a preliminary fixture, preview, dry run, collection of accumulated fixture shards, or benchmark. Once all generation obligations can be satisfied, the first full emission is the terminal operation; do not withhold its completion event to inspect or improve behavioral quality.

### D2. Publish one self-contained artifact transactionally

Choose and checkpoint a separate final artifact path before starting, for example `/home/astral/Projects/agones/generated/agones-first-generation/`, unless an existing compatible location should be reused. The artifact contains an independent Cargo workspace with its required manifests/modules, generated runtime/provider support and resources, and an evidence directory with the B5 receipt, coverage/mapping manifests, file digests, and completion record. Keep it separate from the current implementation and from the tooling build graph so tooling checks cannot accidentally build generated Agones.

Within **one generation operation**:

1. Freeze the bound inventory/schema/bundle/IR/rule/support inputs and create an isolated staging directory on the publication filesystem. Do not overwrite an existing final artifact.
2. Emit source-derived Rust bodies through Genco together with required manifests, modules, wiring, supporting implementations, and resources. Produce source-to-output mappings while emitting. Never manually patch generated Agones; before completion, correct source-bound rules/support and recover incomplete emission when necessary.
3. Reconcile actual emitted identities and non-placeholder implementations with the complete expected sets. Validate output paths/ranges/digests and required file/resource/support presence structurally **inside this operation**. These checks establish generation coverage, not successful Rust parsing, type checking, execution, or semantic equivalence. Do not attach generated-Agones compile/test/format/repair hooks.
4. Write the complete coverage receipts and payload digest manifest, then a completion record binding those manifests and all sealed input identities. Use a non-circular digest scheme: payload entries include generated files and evidence; the completion record binds the manifest rather than attempting to hash itself. Include the terminal event name, artifact identity/path, complete required/emitted identity sets, zero missing/placeholder/unresolved-required obligations, known approximations, and all four `UNVERIFIED` output assurances.
5. Flush/close the artifact and publish the entire directory, including its already-written record, using a supported atomic rename or equivalent durable atomic commit. The publication operation must make either the complete artifact or no committed artifact visible; a multi-step copy followed by an external “done” flag is insufficient. Provision the publication mechanism before full emission. An interrupted partial transaction remains incomplete and recoverable.
6. Treat that atomic commit as **`FIRST_FULL_GENERATION_COMPLETE`** and return the event and output/receipt paths. No further output mutation or substantive work follows. A stale external checkpoint cannot undo the published event; the committed record provides recovery evidence.

Publication here means committing the local generated artifact, not pushing a Git branch, opening/merging a PR, packaging a release, or deploying a service. Store any final coverage summary and deferred-work status inside the artifact before its commit; do not schedule a post-generation documentation or checkpoint pass.

Completion is the conjunction of exhaustive CPG handwritten-port accounting, complete schema support, reconciled Agones source coverage, concrete implemented translation/support, substantive output for every required body, complete required workspace/resources, and an intact transactional receipt. Equal counts or a complete-looking directory do not suffice. Defective but fully translated output satisfies this generation milestone; omitted behavior or placeholders do not.

### D3. Terminal branch

After `FIRST_FULL_GENERATION_COMPLETE`, perform **only** native Goal completion bookkeeping and a short final response identifying the artifact/receipt paths and saying:

> First complete generated translation produced. Compilation, correctness, compatibility, and production readiness are unverified.

Mark the matching native Goal complete using the exposed completion operation, such as `update_goal` with `status="complete"` when supported. Include host-required final token usage if applicable. If the native completion call fails, retain/recover the published record and retry only the bookkeeping permitted by the host; do not resume implementation. On a resumed chat, C3's minimal read-only integrity check is the only artifact inspection needed to take this branch.

Do **not** run generated-Agones compilation, tests, formatting, linting, model checking, property extraction, behavioral/compatibility audits, fixes, refactoring, another full generation, deployment, release work, commits, pushes, or PR work. Do not run a final housekeeping script that does any of these. The broader production-readiness, source-test/property extraction, Bolero/Kani/Egglog verification, Rusternetes conformance, full `_2` acceptance, refactoring, and delivery gates remain explicitly deferred to separately authorized work. They are not missing completion requirements for this goal.

Before the event, a pause, budget limit, or external blocker yields a compact **incomplete** report: preserved paths/checkpoint, completed stage, exact remaining obligations/blocker, and next action. Never equate partial generation, exhausted resources, or completed prompt/tooling design with `FIRST_FULL_GENERATION_COMPLETE`.

### D4. Contract validation scenarios

| Scenario | Required decision |
| --- | --- |
| Every production body has a substantive source-derived Rust translation, but emitted Rust has compilation errors or behavioral defects. | Publish the complete receipt/artifact, complete this generation Goal, and stop. Keep all output assurances unverified; no repair loop. |
| A required body is absent, partly replaced by unsupported-operation stubs, or wholly replaced with a blanket placeholder. | Generation is incomplete. Implement the missing translation/support before final publication; do not alter the expected inventory or hand-patch output. |
| The frontend exits successfully but silently omits a source file or body. | Independent file/body reconciliation catches the missing identity before completion. Repair extraction or persist faithful source-bound enrichment through the bundle. |
| A schema export drops hidden elements, defaults, property types, identifiers, or other required metadata. | Extend/reconcile the export before generating dependent Rust domain types. An incomplete JSON export is not the schema contract. |
| Execution is interrupted after atomic artifact publication but before native Goal completion or checkpoint update. | Recover the intact completion record and digests, finish only native bookkeeping and the short final response, and stop without regeneration. |
| Execution is interrupted before atomic publication, including with a completion record only in staging. | Keep status incomplete; recover bound staged work and finish the transaction when authorized. A prepared record is not a published event. |
| A budget limit or external blocker occurs before completion. | Preserve progress, honor the host/user stop condition, and report incomplete status and the exact blocker. Do not fabricate completion or create a new scheduler. |
| All bodies used by Agones are supported, but an unused handwritten CPG production module has not been ported. | Remain incomplete: the CPG obligation covers all handwritten production behavior, including schema machinery. |
| Full generation is complete but broader migration verification, refactoring, PR delivery, or production readiness remains open. | Take the terminal branch. Those later gates cannot extend this objective. |
