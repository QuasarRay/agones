"""Curated instructions for the documentation renderer, not application code."""

# Each paragraph is attached to the actual pinned files assigned to this family.
A = {'a-foundation': ('Source mirror, version and shared contracts',
                  'site/content/en/docs/Overview/_index.md',
                  ['Establish the literal agones-rs source mirror without moving or deleting the original Go '
                   'reference or existing crates. Put Cargo/module registration in rust-support/agones and use '
                   'explicit path attributes for source files, including dotted filenames.',
                   'Carry pkg version defaults, feature-gate declarations, build/environment precedence and '
                   'shared API-group constants from their exact source definitions. Connect them to explicit CGP '
                   'configuration capabilities; derive-deftly owns repeated declarations and projections.',
                   'Record current crates as reuse candidates, with source-file and production-call-path '
                   'ownership. Do not count a Rust file or previous passing test as complete parity. Keep server '
                   'state and configuration owners unique.']),
 'a-utilities': ('Errors, queues, clocks, webhooks and utility effects',
                 'site/content/en/docs/Advanced/high-availability-agones.md',
                 ['Implement Agones error classes, event/log fields, readiness and CRD behavior, admission '
                  'outcomes, queue retry/backoff and shutdown. Identify domain requirements from the quoted '
                  'Agones utility files; source dependencies are references rather than additional Go porting '
                  'objectives.',
                  'Use existing Rust error/formatting/logging, HTTP/TLS, serialization, filesystem notification, '
                  'signal, timer and async queue/task facilities through CGP. Prefer selected native '
                  'implementations and thin Agones adapters; do not reconstruct Go standard-library/runtime '
                  'behavior.',
                  'Keep one lifecycle/resource owner and derive-deftly declarations for repeated wiring. Retire '
                  'Go-only formatting/reflection/typed-nil/panic/channel/map/slice/GC/allocator/ABI gaps '
                  'explicitly. Preserve unresolved Agones contracts; M0003 remains implementing until native '
                  'production consumers are complete.']),
 'a-protocol': ('SDK and allocation wire contracts',
                'site/content/en/docs/Guides/Client SDKs/_index.md',
                ['Account for every proto message, enum, service, streaming direction, field number, presence '
                 'rule, error/status mapping and compatibility version. Treat generated .pb.go and gateway files '
                 'as regeneration obligations, not disposable source.',
                 'Preserve alpha/beta/stable SDK boundaries, Health/WatchGameServer stream behavior, allocation '
                 'messages and JSON gateway mapping. Keep protocol types distinct from Rusternetes storage '
                 'objects.',
                 'Reuse existing Rust protocol generation and SDK types where their source mapping is known. '
                 'Genco/derive-deftly own repeated bindings; retain exact source basenames such as sdk.pb.rs '
                 'through explicit module registration.']),
 'a-api': ('Agones resource schemas, defaults and admission behavior',
           'site/content/en/docs/Reference/gameserver.md',
           ['Implement the exact API groups and versions for GameServer, GameServerSet, Fleet, FleetAutoscaler, '
            'GameServerAllocation and GameServerAllocationPolicy. Preserve metadata, selectors, resource '
            'versions, deprecated served fields and feature gates.',
            'Implement Agones constructors, defaults, validation, state transitions and resource/schema support '
            'using Rust types, serde/protobuf bindings and native ownership. Preserve field presence, ranges and '
            'errors only where required by an Agones API/schema contract; never reproduce Go nil/interface or '
            'copy/runtime mechanics.',
            'Assign resource lifecycle decisions to CGP providers and use derive-deftly templates for repeated '
            'resource/accessor/conversion structure. Rust public wire compatibility takes precedence over '
            'renaming fields for appearance; explain the mapping for Python readers.']),
 'a-apply': ('Declarative apply configurations',
             'site/content/en/docs/Reference/_index.md',
             ['Regenerate the source apply-configuration family with exact resource/version/package membership. '
              'Preserve fluent builder field-presence, append/replace behavior, Agones-required presence '
              'distinctions expressed through native Rust types, metadata and extraction logic.',
              'Keep each source file and dotted/generated basename in the mirror. Derive templates from canonical '
              'API declarations rather than hand-maintaining repeated field setters.',
              'Make the Python-facing reading order clear: construct configuration, select fields, then submit '
              'through a CGP effect capability. Do not introduce a runtime provider registry or a second writable '
              'representation.']),
 'a-client': ('Versioned clients and request construction',
              'site/content/en/docs/Guides/access-api.md',
              ['Port versioned clientset, scheme and typed-resource request construction while retaining the '
               'source package hierarchy. Preserve group/version, verb, resource, namespace, query, timeout, '
               'patch and watch behavior.',
               'Bind concrete transport/storage to Rusternetes only; Kubernetes-shaped public contracts are '
               'compatibility data, not permission to add another orchestrator adapter.',
               'Generate repeated forwarding with derive-deftly under CGP capabilities. Keep fake client source '
               'accounted for; any fake used solely by harnesses is deferred assurance support, never a '
               'production substitute.']),
 'a-observation': ('Informer, lister and cache observation contracts',
                   'site/content/en/docs/Advanced/high-availability-agones.md',
                   ['Implement informer factories, version/resource lookup, listers and cache-index contracts '
                    'with the original namespace/selector behavior and exact package/file names.',
                    'Preserve list/watch handoff, resync/reconnect, ordering, duplicate delivery, cache '
                    'synchronization and stop behavior. The production context must not provide stronger '
                    'consistency than the source contract.',
                    'CGP owns watch/queue scheduling, cancellation and resources; derive-deftly emits repetitive '
                    'resource wiring. Keep the read snapshot separate from the unique mutable backend owner.']),
 'a-cloud': ('Cloud hooks through the permitted Rusternetes boundary',
             'site/content/en/docs/Advanced/controlling-disruption.md',
             ['Separate generic cloud-product policy, eviction behavior and GKE-specific source assumptions by '
              'exact symbols. Retain required observable hooks, labels, annotations and shutdown behavior through '
              'the Rusternetes context.',
              'Preserve source GKE paths and account for their behavior without installing a GKE/Kubernetes '
              'integration or inventing an alternative platform. Record an explicit conflict if an indispensable '
              'source effect cannot fit the permitted adapter.',
              'Compose hook behavior through CGP and derive-deftly ownership declarations. Neither a neutral name '
              'nor an unused provider establishes portability or completion.']),
 'a-metrics': ('Telemetry and metric semantics',
               'site/content/en/docs/Guides/metrics.md',
               ['Port metric names, labels, counters/gauges/histograms, registration lifetimes, aggregation and '
                'source update sites. Preserve values for lifecycle events and errors; do not silently reduce '
                'dimensionality.',
                'Keep observation policy and collector lifetime in CGP providers. Generate repetitive '
                'descriptions and projections from derive-deftly declarations without calculating expected values '
                'from a separate implementation.',
                'Use readable named transformations for fleet/GameServer observations. Metrics should observe '
                'production state, not own a competing copy of that state.']),
 'a-ports': ('Port ownership and allocation policy',
             'site/content/en/docs/Reference/gameserver.md',
             ['Port exact port ranges, allocation/release, protocol/address distinctions, '
              'static/dynamic/passthrough rules and exhaustion behavior.',
              'Preserve collision handling, initial occupied-port reconciliation, release ordering and error '
              'conditions. Tie each allocation to one CGP resource owner and explicit cleanup path.',
              'Use derive-deftly for repeated port-record wiring; keep selection and mutation policy explicit and '
              'readable. Do not claim allocation correctness before post-gate assurance.']),
 'a-gameservers': ('GameServer construction, reconciliation and shutdown',
                   'site/content/en/docs/Reference/gameserver.md',
                   ['Implement GameServer reconciliation, pod/workload construction, sidecar settings, '
                    'health/state handling, finalizers and admission hooks from the source-derived Agones '
                    'contracts using native Rust libraries.',
                    'Retain scheduling, port, resource, service-account, feature-gate, deletion and '
                    'Agones-required failure outcomes. A source transition and its persisted mutation/event must '
                    'remain connected to the real production path.',
                    'Express controllers and services as CGP providers with derive-deftly-generated '
                    'ownership/wiring. Keep initialization, watch, queue, readiness, cancellation, drain and '
                    'cleanup policies in the same lifecycle tree.']),
 'a-gameserversets': ('GameServerSet scaling and scheduling',
                      'site/content/en/docs/Advanced/scheduling-and-autoscaling.md',
                      ['Port replica calculation, GameServer selection, create/delete decisions, allocation-aware '
                       'scaling and source status updates for GameServerSets.',
                       'Preserve Packed/Distributed policy, Ready/Allocated accounting, deletion candidates, '
                       'overflow metadata, ordering and conflict retries. Do not hide unfinished update paths '
                       'behind a state-only controller.',
                       'Implement decisions as small CGP provider functions; derive repeated state/resource '
                       'projections. Comprehensions use Rust iteration semantics and retain ordering only where '
                       'Agones requires it.']),
 'a-fleets': ('Fleet reconciliation and rollout policy',
              'site/content/en/docs/Reference/fleet.md',
              ['Port Fleet replica helpers, GameServerSet creation/selection, rolling update and recreate '
               'behavior. Preserve maxSurge/maxUnavailable rounding, percentage and absolute forms, allocation '
               'overflow and status fields.',
               'Keep generation/resource-version conflict semantics and source handling of allocated servers. '
               'Record exact source helper dependencies and arithmetic domains.',
               'CGP owns rollout/retry/lifecycle policy; derive-deftly owns repeated declarative wiring and '
               'projections. Name each transformation so a Python developer can follow desired state to decisions '
               'to mutations.']),
 'a-fleetautoscalers': ('FleetAutoscaler policies and external decisions',
                        'site/content/en/docs/Reference/fleetautoscaler.md',
                        ['Implement source buffer/counter/list/webhook/scheduled and other discovered scaler '
                         'policies, status and minimum/maximum/percentage behavior without narrowing the selected '
                         'baseline.',
                         'Preserve webhook request/response, time-zone/timer behavior, retries and failure '
                         'outcomes. Existing Wasm policy support does not authorize a plugin system; retain its '
                         'exact boundary and surface genuine contract conflicts.',
                         'Compose policy selection statically through CGP and generate repeated '
                         'policy/configuration structure with derive-deftly. No runtime service locator, '
                         'alternate orchestrator or generic extension loader.']),
 'a-allocation': ('Allocation processing and multi-cluster contracts',
                  'site/content/en/docs/Reference/gameserverallocation.md',
                  ['Implement source allocation processing, converters, local selection, required/preferred '
                   'selectors, priorities, counts/lists, patching and contention handling. Keep '
                   'processor/converter dependencies in this cohesive milestone.',
                   'Retain multi-cluster policy and remote Agones protocol behavior, batching, timeouts, retries, '
                   'response states and de-duplication. Remote allocation is not permission for another cluster '
                   'orchestrator.',
                   'Bind allocation state changes and persistence to CGP providers with one resource owner. '
                   'Derive repeatable request/projection/wiring structure; do not regenerate semantic policy from '
                   'test outcomes.']),
 'a-sdkserver': ('SDK server streaming, health and mutation behavior',
                 'site/content/en/docs/Guides/Client SDKs/_index.md',
                 ['Implement SDK server RPCs, local mode, health signals, Ready/Allocated/Shutdown/Reserve '
                  'behavior, WatchGameServer streaming, metadata updates and alpha/beta/stable '
                  'counters/lists/player contracts.',
                  'Preserve source update ordering, retries, validation, status codes, stream cancellation, '
                  'disconnect behavior and shutdown. Every stream/task must have a CGP owner and join/drain '
                  'policy.',
                  'Use derive-deftly for repeated service/message/wiring adapters. Keep the ordinary operation '
                  'path short enough to read as request, decision, mutation, notification and cleanup.']),
 'a-sdkclients': ('Source-compatible clients and existing Rust SDK reuse',
                  'site/content/en/docs/Guides/Client SDKs/rust.md',
                  ['Reuse the existing Rust SDK while accounting for first-party Go SDK behavior and public '
                   'protocol compatibility. Preserve client construction, endpoints, retries, health loops, '
                   'streams, errors and shutdown.',
                   'Keep existing language SDKs as compatibility references; this contract does not authorize '
                   'wholesale rewrites of every language client. Give each retained path an explicit inventory '
                   'disposition.',
                   'Preserve Rust SDK filenames and APIs where already compatible. Any required source-derived Go '
                   'translation retains its exact Go-relative path; do not create two independently maintained '
                   'canonical client semantics.']),
 'a-entrypoints': ('Production entrypoints and complete lifecycle composition',
                   'site/content/en/docs/Advanced/system-diagram.md',
                   ['Implement controller, allocator, extensions, ping, processor and SDK-server command '
                    'surfaces, flags/environment precedence, startup errors, readiness, logging and shutdown.',
                    'The entrypoint only constructs the declared production context and invokes the CGP lifecycle '
                    'consumer. Derive-deftly-generated wiring must include every actual child service/resource; '
                    'no detached manager bypasses it.',
                    'Preserve authentication, mTLS, request limits, remote allocation behavior, health endpoints '
                    'and cancellation/drain deadlines. Do not run any command as a smoke test before the gate.']),
 'a-examples': ('Examples and compatibility artifacts',
                'site/content/en/docs/Examples/_index.md',
                ['Port required first-party Go example behavior and retain YAML/JSON/proto examples with exact '
                 'source-relative names. Preserve example request/response, port, image/configuration and SDK '
                 'usage contracts.',
                 'Keep examples on the same production APIs and CGP context composition. Use concise '
                 'Python-familiar Rust and derive-deftly templates already established in production.',
                 'Do not run examples or provision clusters. An example copied as a comment is not a translated '
                 'production body; assets remain distinct from executable behavior.']),
 'a-tooling': ('First-party build and documentation tooling',
               'CONTRIBUTING.md',
               ['Account for each first-party Go build/report/version/schema/documentation tool. Preserve command '
                'flags, file selection, parse/format rules, failure behavior, and deterministic output names.',
                'Translate required logic into the literal Rust mirror or bind a precise dependency-backed '
                'equivalent. Preserve external-tool boundaries and immutable inputs; do not execute builds, CI, '
                'or validation campaigns.',
                'Use the shared CGP/derive-deftly architecture for effects and repeated declarations; '
                'comprehensions must preserve filename/order/duplicate handling. Generation required for '
                'implementation is subject to the strict executable-availability rule.']),
 'a-delivery-inputs': ('Build, deployment and configuration implementation',
                       'site/content/en/docs/Installation/Install Agones/helm.md',
                       ['Preserve every required deployment, packaging, build and configuration input and its '
                        'literal path. Translate behavior where the Rust toolchain requires a concrete '
                        'equivalent; keep original names as compatibility inputs.',
                        'Separate source Kubernetes deployment references from permitted Rusternetes integration. '
                        'Retain values, ports, resources, certificates, feature gates, service contracts and '
                        'user-visible configuration semantics without adding another orchestrator.',
                        'Author CI and packaging changes without executing them or pushing events that trigger '
                        'project verification. Do not weaken protections or claim skipped checks are passing.']),
 'a-macros': ('Complete the required metaprogramming authoring family',
              'CONTRIBUTING.md',
              ['Complete proc_macro_2, proc_macro_attribute_2, proc_macro_derive_2 and proc_macro_delegate_2 as '
               'one proc_macro2-oriented family with same-logical-crate definition/use and cross-crate '
               'export/import. Compiler shims and companion units stay internal.',
               'Reuse CGP, derive-deftly, agones-macros, Genco and the pinned Arcane-rs synthez/delegation/arcane '
               'facilities before inventing machinery. Record concrete reuse gaps and a short positive '
               'remaining-work argument.',
               'Keep host/target dependencies acyclic, establish companion manifests before Cargo resolution, '
               'preserve source spans/hygiene and ordinary Cargo/IDE entrypoints. Author all required '
               'capabilities now; defer compile/IDE/packaging fixtures until the global gate.']),
 'a-assurance': ('Agones harnesses and final behavioral assurance',
                 'CONTRIBUTING.md',
                 ['POST-GATE ONLY. Retain every original source::fixture::test identity. Preserve independent '
                  'Agones cases, setup/actions/observations and invariants; classify individual Go-only '
                  'assertions GO_ONLY_SUPERSEDED with source evidence and requirement-10 authority. Go '
                  'name_test.go maps to name_harness.rs.',
                  'Use typed extraction, independent Agones domains and non-vacuity witnesses. Generate Bolero '
                  'properties with the required Kani backend and add appropriate Rust-native coverage. Keep '
                  'sampling, reference/production proofs, replay and Rusternetes evidence distinct; no Go runtime '
                  'equivalence campaign.',
                  'Bind assertions to real production consumers and required Agones API/domain outcomes. Unknown '
                  'Agones obligations remain open; superseded Go mechanisms do not block acceptance. Production '
                  'defects reopen the implementation gate before further assurance.']),
 'a-delivery': ('Agones acceptance and authorized delivery',
                'CONTRIBUTING.md',
                ['POST-GATE ONLY. Finish joint architecture, Pythonic readability, Agones domain/API parity '
                 'implemented with native Rust semantics, source-path fidelity, all _2 cells and required '
                 'assurance evidence for the exact delivered source snapshot.',
                 'Prepare authorized stacked milestones against agones-rs-dev; never modify main or weaken '
                 'protected checks. Preserve source-bound history through squash/restack and distinguish local '
                 'commits, publication, merge and visible checkout.',
                 'Confirm every required scope row has current evidence and all source/external obligations are '
                 'resolved. Completing this route does not by itself complete the shared Goal.'])}

C = {'c-build': ('Pinned CPG source, build machinery and Rust support boundaries',
             ['Preserve the complete pinned CPG tree and source-relative naming. Account for Bazel helpers, sbt '
              'definitions, project helpers, launcher and protobuf/domain generation inputs; do not exclude '
              'build-only handwritten behavior.',
              'Establish Rust-only Cargo/module glue outside the mirror and reuse the existing cpg-rs '
              'Grafeo-backed graph/persistence support. Record source/configuration identities and separate '
              'pristine upstream from guidance/patch overlays.',
              'Author concrete Rust equivalents for required source build/tool behavior. Inspect available '
              'generator receipts without running executables. Missing runnable emission tools are explicit '
              'blockers under the strict build prohibition.']),
 'c-utilities': ('Native CPG utilities and effect ownership',
                 ['Implement CPG-required file/stream/encoding behavior, temporary-copy cleanup, root discovery '
                  'and graph interning through existing native Rust facilities. Preserve CPG '
                  'functional/interchange contracts without implementing a Scala/JVM or Go runtime.',
                  'Expose current/global execution-context policy and resource lifetimes through CGP. Use '
                  'derive-deftly for repeated projections and adapters; preserve error ordering and cleanup on '
                  'failure.',
                  'Reuse the Rust standard library and selected ecosystem utilities before custom code. Record '
                  'the concrete CPG contract/integration gap for any adapter; keep valid Rust concise for Python '
                  'readers.']),
 'c-schema-base': ('Schema base declarations and native identity model',
                   ['Preserve SchemaBase, Base and Operators spelling and paths. Establish typed schema '
                    'property/node/edge/constant handles, shared builder ownership, declaration order and source '
                    'equality/identity distinctions.',
                    'Reconcile reusable CPGSchema and cpg-rs schema implementations at the source-declaration '
                    'level. The two recorded CPG revisions have identical schema/src/main/scala blobs; this '
                    'removes that text-difference concern but proves no Rust behavior or builder compatibility.',
                    'Use CGP providers for schema construction, native value/graph ownership and effects; '
                    'derive-deftly generates repeated declaration projections. Retain signed widths, raw floating '
                    'bits, UTF-16 values, nulls and scalar-list/cardinality distinctions.']),
 'c-schema-identity': ('File, namespace, type and metadata schema layers',
                       ['Construct FileSystem after Base, Namespace after Base/FileSystem, and Type after '
                        'Base/FileSystem. Preserve independent Configuration and MetaData declarations and exact '
                        'type/property/default/cardinality definitions.',
                        'Retain caller-supplied builder handles and aliases; later modules may mutate earlier '
                        'definitions. Do not flatten the object model into a name-only JSON map.',
                        'Reuse existing native module implementations by exact source path and symbols, recording '
                        'unverified reuse rather than subtracting old candidate counts as if proved.']),
 'c-schema-program': ('Method, AST and call-graph schema layers',
                      ['Construct Method after type/file/base definitions, Ast after namespace/method/type/file '
                       'definitions, and CallGraph after Method/Ast.',
                       'Retain AST ordering, argument/parameter indices, method/type references, inheritance, '
                       'contained relationships and source defaults. Preserve source module and class spellings.',
                       'Express module composition through CGP and repeated declarations through derive-deftly. '
                       'Keep individual module APIs available; a complete factory alone does not replace them.']),
 'c-schema-flow': ('Control, dominance and data-flow schema layers',
                   ['Construct Cfg, Dominators and Pdg from their already implemented Method/Ast dependencies. '
                    'Preserve edge direction, endpoints, ordering, property values and layer relationships.',
                    'Do not equate a named graph edge with a completed analysis algorithm. Implement the actual '
                    'source-owned schema behavior and keep external analysis boundaries explicit.',
                    'Generate repetitive typed metadata/access paths from the canonical declarations under the '
                    'joint CGP/derive-deftly architecture.']),
 'c-schema-enrichment': ('Shortcuts, tags, annotations, findings and hidden schema',
                         ['Follow exact constructor dependencies for Shortcuts, Comment, Tags, Binding, '
                          'Annotation, Finding and Hidden. Hidden is composed after its CFG/PDG/tag/shortcut '
                          'dependencies.',
                          'Retain hidden nodes/edges/properties, duplicate declarations, inherited fields, '
                          'source-specific links and mutation order. Do not use the filtered public JSON export '
                          'as the complete internal schema.',
                          'Reuse the pinned source facts once in canonical declarations; derive API structure '
                          'without dropping modules unused by Agones.']),
 'c-schema-compose': ('Complete schema composition, IDs and protobuf definition',
                      ['Implement CpgSchema, ProtoIds, ProtoSerialize and Protogen with the original names and '
                       'declaration order. Compose every layer and preserve protobuf identifiers, defaults, '
                       'reserved fields and options.',
                       'Retain build-time factories, individual parameterized APIs and mutation/equality/identity '
                       'behavior. Separate immutable resolved export models from mutable source builder handles.',
                       'Author complete native generation inputs and provider/template wiring. Execute emission '
                       'only through already available permitted generators; record unavailable fresh builds as '
                       'blockers.']),
 'c-schema-tools': ('Schema export, protobuf tool inputs and launcher behavior',
                    ['Preserve standard Schema2Json export behavior, including hidden-element filtering, while '
                     'defining a separate complete internal schema representation for translation.',
                     'Account for schema2json.sh, proto-bindings build behavior and source resource placeholders. '
                     'Retain launch arguments, errors, output names and declared external-tool boundaries.',
                     'Use canonical typed declarations, CGP lifecycle/effect providers and derive-deftly '
                     'structure. Do not make a JVM service the permanent implementation of native schema '
                     'operations.']),
 'c-domain': ('Complete schema-derived domain hierarchy',
              ['Regenerate every source-generated node, edge, starter, traversal, property accessor and domain '
               'class from the complete schema; never hand-port the generated Scala corpus.',
               'Preserve every original generated source-relative directory and filename stem in Rust. Register '
               'case-sensitive/dotted paths explicitly in external module glue.',
               'Preserve stored/detached/reference types, inheritance, defaults, cardinalities, contained-node '
               'relationships and neighbor ordering. Reuse native Grafeo ownership and projections; derive-deftly '
               'and Genco share canonical inputs with one output owner.']),
 'c-graph': ('Native CPG facade and graph ownership',
             ['Port Cpg and package-level APIs with exact graph construction, borrowed access, mutation, '
              'context/close and persistence behavior.',
              'Keep Grafeo as the existing canonical graph/persistence owner, with disposable projections rather '
              'than a parallel writable graph. Preserve source IDs, tombstones, property values and '
              'edge/adjacency order.',
              'CGP owns graph/effect/lifecycle policy; derive-deftly generates repeated typed adapters and '
              'ownership metadata. All operations must reach real native implementation.']),
 'c-serialization': ('Serialized graph and overlay behavior',
                     ['Port SerializedCpg archive/overlay serialization, entry naming, order, counters, overwrite '
                      'behavior, mounting and close/publication semantics.',
                      'Preserve unknown protobuf fields, source scalar representations, partial failure side '
                      'effects and cleanup ownership. Do not treat a successful archive write as semantic '
                      'equivalence.',
                      'Keep serialization data transformations readable, with CGP I/O capabilities and '
                      'derive-deftly-generated record/projection structure.']),
 'c-protobuf-loading': ('Protobuf loading, archive ownership and node mapping',
                        ['Port ZipArchive, ProtoToGraphNodeMappings and ProtoCpgLoader including two-pass '
                         'loading, node/type de-duplication, aliases, overlays and import ordering.',
                         'Retain missing/invalid entries, repeated import, persistent/open/create behavior, '
                         'failure classifications and source mutations that occur before failure.',
                         'Compose loaders and resource lifetimes through CGP with derive-deftly-generated mapping '
                         'scaffolding; do not collapse reference identity into names.']),
 'c-storage-loading': ('Format detection and native storage compatibility',
                       ['Port CpgLoader dispatch and deprecated loader paths for source-supported proto, '
                        'Flatgraph and legacy OverflowDB formats, reusing existing native compatibility code.',
                        'Retain format probes, overwrite/missing-input order, destination creation, persistent '
                        'versus memory-only behavior and source failure side effects. Grafeo remains canonical '
                        'storage; format compatibility is not a second storage owner.',
                        'Keep native parsing/import transformations explicit and Python-readable; reuse '
                        'source-backed Rust dependencies without delegating core behavior to Scala.']),
 'c-passes': ('Pass, diff and execution lifecycle',
              ['Port sequential and fork/join pass behavior, accumulators, callbacks, converters, '
               'finish-on-failure/unwind, ordered diff merge and application count.',
               'Preserve task ownership, exception propagation, execution-context selection and cleanup order. Do '
               'not replace concurrent source semantics with an undocumented stronger sequential model.',
               'CGP is the runtime lifecycle owner and derive-deftly the repeatable declaration/wiring owner. '
               'Keep source policy visible rather than hiding it in opaque templates.']),
 'c-bundle': ('Pinned frontend and self-contained CPG input bundle',
              ['Use the recorded compatible Joern gosrc2cpg/goastgen/CPG/Flatgraph pins. Retain their actual '
               'source, configuration and graph format identities; a PATH binary is not compatibility evidence.',
               'Author the bundle loader and source/body/configuration census interfaces: original graph, '
               'retained source, Go AST/type enrichment, build tags/platforms/generated inputs/cgo boundaries and '
               'diagnostics.',
               'Resolve every required source file/body to graph nodes or persisted source-bound enrichment. '
               'Missing frontend executables or a required new build remain blockers; do not execute an '
               'unapproved probe.']),
 'c-normalization': ('Normalized CPG with source identity preservation',
                     ['Implement deterministic normalization of graph labels, typed properties, multiplicities, '
                      'endpoint/direction/order, source spans, configuration membership and enrichment.',
                      'Carry conflicting or unknown facts explicitly; preserve null/empty/default and numeric '
                      'precision. No emitter side channel may read Go text to fabricate an untracked body.',
                      'Use small CGP stage providers and derive-deftly record/projection templates. Normalize '
                      'representation without changing source meaning.']),
 'c-ir': ('Typed translation IR and ownership/effect interfaces',
          ['Define a native-Rust-oriented typed IR for Agones domain values, decisions, calls, effects, '
           'configuration and lifecycle operations. Bind each node to source/body/configuration and rule '
           'identities; distinguish AGONES_REQUIRED, RUST_NATIVE and GO_ONLY_SUPERSEDED obligations.',
           'Map Agones public contracts and Rusternetes capabilities to real native Rust destination interfaces '
           'and library operations. Keep original Go AST/type facts as provenance and analysis data; they do not '
           'prescribe target runtime behavior.',
           'Required Agones operations must have real implementations. Excluded Go-only mechanisms retain '
           'supersession records, not unsupported-operation blockers. Genco emits the typed native Rust plan; CGP '
           'and derive-deftly own stages and repeated structure.']),
 'c-lowering-data': ('Native Rust domain values, types and initialization',
                     ['Lower Agones resource/configuration values to Rust structs/enums, native collections, '
                      'Option/Result and existing serializers. Preserve API/protobuf field names, ranges, '
                      'presence and domain arithmetic only where required by Agones contracts.',
                      'Implement Agones startup/configuration and external-call requirements using native Rust '
                      'initialization, ownership, checked conversions and selected libraries. Do not emulate Go '
                      'integers/floating formatting, typed-nil interfaces, reflection, aliases, map/slice '
                      'headers, allocation or ABI behavior.',
                      'Maintain source-to-Agones-obligation-to-Rust-operation provenance. Generate readable '
                      'Python-familiar valid Rust and reusable derive-deftly declarations; Go-only constructs '
                      'receive explicit supersession rather than compatibility helpers.']),
 'c-lowering-control': ('Native Rust decisions, errors, concurrency and lifecycle',
                        ['Lower Agones decisions and failure outcomes into Rust control flow, Result and native '
                         'library operations. Use RAII/scoped guards for resources and explicit async shutdown '
                         'where Agones requires it.',
                         'Use existing Rust tasks, channels/streams, synchronization, clocks and cancellation '
                         'through CGP-owned services. Preserve Agones retry, readiness, notification and shutdown '
                         'contracts; never emulate goroutine scheduling, channel/select mechanics, '
                         'defer/panic/recover or Goexit.',
                         'Generate the same native lifecycle architecture used by manual Agones. Unknown Agones '
                         'requirements remain gaps; Go runtime details are GO_ONLY_SUPERSEDED. Keep every '
                         'required domain operation connected to a real provider and native production '
                         'consumer.']),
 'c-emission': ('Genco emission into the exact Rust source mirror',
                ['Implement typed-IR-to-Genco emission for every required declaration/body/resource/support '
                 'artifact. Use existing Genco facilities; it supplies tokens/imports/layout rather than Go '
                 'semantics.',
                 'Emit the exact source-relative names, Go test-to-harness mapping, Scala spelling, module '
                 'metadata and external Rust-only wiring. Never rename plural/singular stems for convention.',
                 'Emit CGP provider/context composition and derive-deftly declarations/templates as central '
                 'architecture. Stage generated output separately; no overwriting manual edits or claiming copied '
                 'handwritten bodies were translated through CPG.']),
 'c-publication': ('Full generation, coverage receipts and recoverable publication',
                   ['Implement and, only with a permitted already runnable emitter, perform the complete '
                    'generation transaction into generated/agones-first-generation. Reconcile every required '
                    'file/body/output with the independent source inventory.',
                    'Publish manifests, source-to-IR-to-output mappings, rule/schema/tool/input digests and '
                    'FIRST_FULL_GENERATION_COMPLETE atomically. Interrupted staging is not a completed artifact; '
                    'recover intact publication without duplicate work.',
                    'Complete required production adoption interfaces, ownership transfers and call paths before '
                    'the global gate, preserving the separate source-derived artifact. The artifact remains '
                    'UNVERIFIED. A missing executable/build is a blocker and prevents this milestone and the '
                    'global implementation gate from completing. First generation is a milestone of the combined '
                    'objective, not a command to stop the Goal.']),
 'c-assurance': ('CPG, schema, lowering and generation assurance',
                 ['POST-GATE ONLY. Preserve every original Scala fixture/test identity and assertion. Translate '
                  'harnesses with original Scala filename stems, including Test/Tests capitalization; the special '
                  '_harness rule is for Go _test.go files.',
                  'Establish native CPG graph/schema/persistence/loading/pass/interchange evidence. Validate '
                  'generated Agones against independent domain invariants and API observations, with Rust-native '
                  'assurance for infrastructure. Retain source test IDs but supersede Go-only assertions '
                  'explicitly.',
                  'Exercise generator determinism, actual source/body coverage, generated compilation, naming, '
                  'CGP/derive-deftly boundaries and exact tool pins. Unknown/stale/unsupported/empty selections '
                  'remain unresolved.']),
 'c-delivery': ('Generated Agones integration and shared objective closure',
                ['POST-GATE ONLY. Establish evidence for the production integration and ownership-transfer paths '
                 'authored before the gate. Preserve manual changes and provenance. Retain one canonical behavior '
                 'and a distinct complete generated artifact; copied manual code cannot acquire CPG-generation '
                 'credit.',
                 'Finish integration assurance, production-linkage evidence, compatibility and required checks on '
                 'the actual delivered revision. A newly discovered missing production path reopens the '
                 'implementation gate; do not disguise it as assurance. Preserve full CPG scope including modules '
                 'unused by Agones.',
                 'Close the shared Goal only when both routes, all architecture/readability/naming obligations, '
                 'required assurance and authorized delivery are complete. Keep actual usage and '
                 'unverified/generated/verified states distinct.'])}

C_DEPS = {
'c-build': [], 'c-utilities':['c-build'], 'c-schema-base':['c-build'],
'c-schema-identity':['c-schema-base'], 'c-schema-program':['c-schema-identity'],
'c-schema-flow':['c-schema-program'], 'c-schema-enrichment':['c-schema-flow'],
'c-schema-compose':['c-schema-enrichment'], 'c-schema-tools':['c-schema-compose'],
'c-domain':['c-schema-compose','c-utilities'], 'c-graph':['c-domain'],
'c-serialization':['c-graph'], 'c-protobuf-loading':['c-serialization','c-schema-tools'],
'c-storage-loading':['c-protobuf-loading'], 'c-passes':['c-graph','c-utilities'],
'c-bundle':['c-storage-loading','c-passes'], 'c-normalization':['c-bundle'],
'c-ir':['c-normalization','a-api','a-protocol'], 'c-lowering-data':['c-ir'],
'c-lowering-control':['c-lowering-data','a-utilities'], 'c-emission':['c-lowering-control'],
'c-publication':['c-emission'], 'c-assurance':['c-publication'],
'c-delivery':['c-assurance','a-assurance']}

# These are source-reading priorities among dependency-ready groups, not route-switch rules.
A_PRIORITY = ['a-foundation','a-utilities','a-protocol','a-api','a-apply','a-client','a-observation','a-sdkclients','a-ports','a-cloud','a-fleets','a-metrics','a-gameservers','a-gameserversets','a-fleetautoscalers','a-allocation','a-sdkserver','a-entrypoints','a-examples','a-tooling','a-delivery-inputs','a-macros','a-assurance','a-delivery']
C_PRIORITY = list(C)
