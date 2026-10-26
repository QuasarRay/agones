# Agones-rs and CPG handoff

Canonical ledger: `AGONES_RS_STATE.json`, revision 329. Native Goal:
`01a17928-8551-7701-8497-5333c680f283`, ACTIVE, no budget. Current work made
concrete source progress; no wait or blocking condition is live.
Authority: combined attachment/roadmap SHA
`a9b797fc624be04548c62e63a634bcc2cacab25c4d9f45af346148ba32c1baea`,
`roadmap/0000-contract.md`, `roadmap/alternation-policy.md`. Read only the active
milestone/affected files; do not reload the full roadmap. The archived standalone
first-generation terminal rule is superseded for this combined Goal.

Phase IMPLEMENTATION_ONLY; global BOTH-route gate CLOSED; verification
NOT_STARTED. ZERO Cargo check/build/test/clippy/doc, formatting/lint campaigns,
compiler/IDE/macro/API probes, fixtures/apps/examples/replay/proofs, generator
validation or CI until all required production and generated outputs in BOTH
routes are authored and connected. An already runnable emitter may emit without
assurance hooks. A genuinely required prohibited emitter build is a real blocker.
Source reading/authoring/content ownership bookkeeping/local commits are allowed.
No project executable, verification or generator ran in this batch.

M0001 MANUAL: IMPLEMENTED_UNVERIFIED, `53a958f76`, 13 entries.
M0002 CPG: IMPLEMENTED_UNVERIFIED, `95161696e`, 49 entries / 62 operation groups;
see its adoption/body-ownership receipts. M0006/M0008/M0010 full schema/tools/
domain work remains. Flattened cpg-rs/generated is not literal full hierarchy
credit. Preserve cpg-rs/src, CPGSchema and historical 695-candidate body gap.

Current cursor: M0003 / MANUAL / IMPLEMENTING / READY. No route switch until
M0003's entire authored milestone is complete. Authoritative partial record:
`verification/agones/roadmap-M0003-partial-map.json`; retained-body map:
`verification/agones/roadmap-M0003-reuse-adoption.json`. All 14 mirror entries
now contain authored code at varying depth; this is NOT all-body completion,
compilation, parity or assurance. `runtime.NewServerMux` remains UNAUTHORED.
The partial record identifies that named missing body separately from file count.

Immediate next action: implement runtime.NewServerMux with actual HTTPBody and
proto JSON use-proto-names / emit-unpopulated / discard-unknown behavior, backed
by a real codec and mux consumer, not stored flags alone. Then reconcile the
required source profiles in the same record. Keep the cursor; do not jump to M4.

Existing util package registration is `rust-support/agones/src/utility_lib.rs`.
Literal mirror paths are under agones-rs/pkg/util; Rust-only effect/wiring files
are in rust-support/agones/src. derive-deftly templates SourceErrorOrigin,
UtilityFeatureProviders, UtilityCrdProviders, UtilityRuntimeProviders and new
UtilityQueueProviders connect typed declarations to static CGP providers.
The queue template uses the pinned syntax `<$tgens> $ttype`; source API docs
were read, but no expansion/compiler probe ran. No runtime provider registry.

Current authored utility behavior:
- Prior error prefix/wrappers/type origins, log fields and complete feature
  operations reuse the core parser. M0001 feature state moves into one RwLock
  owner. SourceFormat still is NOT a Go fmt parser; type/format/caller work open.
- CRD independent deadline/immediate GET/fixed ticker, ignored parent cancel,
  Rusternetes HTTP reader and shared source logger. Native client/error profiles
  and the old controller CRD/cache barrier bypass remain open.
- Runtime JSON log state/backoff/Must/logger identities and shared weak-referenced
  task ownership. Root uses UtilityTaskScope::reporting: bounded first failure
  wakes a CGP observer immediately, retaining original errors/panic payloads for
  join. Observation does not call user Display. HTTP request-local scopes discard
  reaped failures rather than retain a growing failure history. Parent fatal/
  recovery policy and real command integration still require their source owner.
- Concrete HTTP/TLS bounded streaming transport, sensitive-header redaction,
  request/error/404 callbacks, 60-second header timeout, plaintext HTTP1 and TLS
  ALPN-selected HTTP1/HTTP2. Initial/reload hardcoded /certs paths remain distinct
  from passed fallback certificate files; failed reload retains the prior cert.
  Connection drain transfers to the parent when a listener returns, including
  permanent failure, without cancelling the caller token. A closing parent
  receives rejected live resources back for inline cancellation/join.
- Actual Linux/Android inotify fd, source batching/closed/error channel policy,
  captured-nil-error logging and fd-close completion. Other platform watchers,
  exact read/Add/close/path/errno profiles and parent adoption remain open.
- Webhook ordered current-review matching, returned-review error handling,
  default allow only for nil response, request UID/nil panic and full review
  encoding. Admission raw JSON/base64 patch and wire declarations are present.
- APIServer OpenAPI v2/v3, 406 /apis with late Content-Type, resource discovery,
  namespace routing, metadata copy and JSON/YAML/Kubernetes protobuf encoding.
  Vendored comprende/better_comprehension are reused. Full decoder/serializer/
  negotiation/HTTP behavior is still partial, not parity credit.

New signals source is `agones-rs/pkg/util/signals/signals.rs`, backed by
`utility_signals.rs`. Actual set is SIGINT + SIGTERM despite the source comment.
Unix sigaction hooks write through a nonblocking pipe using signal-safe atomics;
errno is preserved. A process claim prevents two utility owners. Explicit Stop
restores saved actions, waits for in-flight handlers before fd reuse and cancels
the reader when no subscriptions remain. Stale readers cannot notify a new
session. NotifyContext cancellation alone does not unregister; the one-shot
handler retains registration after its callback, matching source. Root cleanup
owns restoration and joins. Windows/non-Unix adapter and exact race/failure/old
Tokio-handler interaction profiles remain open. Final failed native restoration
aborts to prevent writing into a closed fd; this failure policy is explicit and
unverified. No signals were delivered or APIs executed as probes.

New queue source is `agones-rs/pkg/util/workerqueue/workerqueue.rs`, reached at
`util::pkg::util::workerqueue` / `util::source::workerqueue`. It includes all 22
source declarations/method groups and the actual backend, not a marker. Source
traceError has NO Unwrap/source. SourceQueueContext<Limiter> owns shared ready
state and limiter; its derive-deftly/CGP clock/key/queue/delay/handler/classifier
providers are concrete. Keys retain bytes. QueueItem represents typed comparable,
unhashable, nil and float values; ItemMap explicitly supports non-reflexive
comparison without claiming Rust Eq. FIFO dirty/processing state, separate Done/
Forget, post-shutdown backlog/dirty drain, 1000-slot delayed-add backpressure,
earliest-deadline coalescing, source heap tie operations and ten-second ticker
are authored. Signed duration/counter semantics and first-five-200ms fast/slow,
constant, default 5ms exponential + 10qps/burst100 max-of limiting are present.
Worker startup/shutdown/health uses one locked count pair and parent-owned tasks.
SourceStatus data interfaces are bridged without a registry; classification calls
Status separately for reason and code. errors.first_error_view/find_error retain
ordered multiple-cause traversal and the original bridged error's type view.

Preserve `crates/util/src/workerqueue.rs` at the old util::workerqueue API and
Rusternetes controller_work_queue and callers. They are not transferred and get
NO new source completion credit. The reuse record binds their current hashes
and differences: old TraceError unwraps, old timers/tasks detach, old counters
saturate, and controller retry/bucket behavior is approximate. Actual consumer
transfers must be explicit under the relevant controller/lifecycle source owner.
Queue value/type/format/JSON, target numeric, custom As, named metrics/updater
callbacks (M0017 ownership) and parent failure-policy profiles remain open.

InClusterBuildConfig now has a real CGP configuration reader backed by
`utility_client_config.rs`. Important pinned-source discovery: the helper creates
LoadingRules{ExplicitPath: path} only. Precedence AND DefaultClientConfig are nil.
Thus it does NOT search KUBECONFIG/home files, nor replace a valid explicit config
by in-cluster defaults. Empty config alone can cause a SECOND in-cluster attempt
if token stat/env Possible succeeds. Native code preserves these decisions and
the first warning's Sprint concatenation, token/CA reads and nonfatal CA failure,
file selection, typed named config/auth/exec/default validation, relative paths,
TLS-only credential/server projection and inert auth/exec/persistence records.
Secret Debug output is redacted. Actual credential execution/plugin architecture
is not implemented by this loader; retain data and record any later real conflict
with the no-plugin constraint instead of silently deleting source behavior.

Configuration parser/default/duplicate/extension/base64/unselected-path/Unicode/
URL/proxy/error/diagnostic/certificate profiles remain unverified/incomplete.
HTTP still needs required mux/path-clean/redirect, URL/RawPath/CONNECT/escape,
1xx/sniff/framing/alias and temporary-accept profiles; TLS and Go JSON/YAML/log/
string/number/type/format profiles remain in the partial record. Native errors
are retained, without asserting Go equivalence. Full feature Viper/pflag and
actual command/controller/CRD/admission parent adoption remain open. Async Drop
cannot itself join; normal cleanup is explicit. Do not claim compilation or
parity from code presence, manifest declaration, source mapping or this handoff.

All dependency declarations were authored without Cargo resolution. This batch
adds direct existing-root errno 0.3.14 and regex 1.13.1 identities only. Preserve
all pinned packages and the earlier comprehension blocks adopted from the CPG
lock. notify 8.2 and signal-hook-registry were read as candidates, not adopted.

M0003 entry usage 656008. Raw progress observations 1..6 retain 752278, 781155,
905410, 1069792, 1095960, 1289081. Still NO completed M0003 cost comparison or
route switch. Context/read/accounting is charged once, no credit inference.
Preserve M0001/M0002 original forecasts/comparisons and shared overhead buckets.
M0002 observed work was 482306; shared completion/switch was 20030. The explicit
M0003 entry bookkeeping allocation remains unchanged.

Work only on agones-rs-dev, never main. Local commits allowed; no push/PR/merge
while they would trigger prohibited CI. Goal remains active until BOTH complete
scopes, generation, post-gate assurance and authorized delivery are achieved.
Predecessor state/handoff are retained by hash. Older checks/proofs remain stale;
do NOT run old checkpoint.py, which targets the superseded contract. No second
Goal, ledger, acceptance queue, project thread or automation is needed.
