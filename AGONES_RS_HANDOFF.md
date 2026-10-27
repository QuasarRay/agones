# Agones-rs and CPG handoff

Canonical ledger: `AGONES_RS_STATE.json`, revision 336. Native Goal:
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
compilation, parity or assurance. `runtime.NewServerMux` now has an actual
authored factory/codec/mux consumer, still PARTIAL and UNVERIFIED. The named
missing-body list is empty; required behavior/platform gaps remain open.

Immediate next action: continue M0003 MANUAL without a route switch. Finish
mutable request-field/method/body and header-slice alias/non-UTF8 representations;
HTTP parser/rejection/1.0 keep-alive/upgrade/partial-I/O/flush/reset/abort/Continue
and native schedules; listener service-name/bind/error-origin/display/shutdown;
selected fsnotify v1.4.7 failure/close/race/BSD timed-read/stale-fd; and all existing
gateway/config/queue/logging/CRD/features/admission/body gaps. Keep the actual
pinned gopkg.in/fsnotify.v1 1.4.7 utility binding. HTTP/native details remain in
roadmap-M0003-http-authoring and roadmap-M0003-hyper-adoption JSON records.
Shared body discard/Close, H1 fields/framing/count/completion/reuse are authored,
uncompiled and unexecuted. No source mutation/native-profile credit is inferred.

Existing util package registration is `rust-support/agones/src/utility_lib.rs`.
Literal mirror paths are under agones-rs/pkg/util; Rust-only effect/wiring files
are in rust-support/agones/src. derive-deftly templates SourceErrorOrigin,
UtilityFeatureProviders, UtilityCrdProviders, UtilityRuntimeProviders and new
UtilityQueueProviders and UtilityGatewayProviders connect typed declarations
to static CGP providers.
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
owns restoration and joins. Windows console adapter is now authored; exact race/failure/old Tokio-handler
interaction and non-Unix/non-Windows profiles remain open. Final failed native restoration
aborts to prevent writing into a closed fd; this failure policy is explicit and
unverified. No signals were delivered or APIs executed as probes.

Platform continuation is recorded in
`verification/agones/roadmap-M0003-platform-effects.json`. Common static CGP
signal subscription/session construction now serves Unix and Windows through
one SourceSignalState/Hub. Windows SetConsoleCtrlHandler uses exclusive process
ownership, an atomic pending/waker bridge and a parent-owned Tokio reader.
Ctrl+C/Break map to SIGINT; Close/Logoff/Shutdown map to SIGTERM. Like the Go
runtime, terminal OS-created callback threads park after notification so cleanup
can proceed before process exit/OS timeout. Parked threads retain no Rust lease,
task, pointer or waker. M0027 must own actual application cleanup/process exit.
Stop removes future hooks and quiesces active notification callbacks, clears
static wakers and cancels readers. Late old readers cannot consume a replacement
session. Failed final native restoration still has an explicit abort policy.

Filesystem source-binding correction: pinned pkg/util/fswatch/fswatch.go imports
gopkg.in/fsnotify.v1 v1.4.7. Revision 332 followed the edited Go file's newer
fsnotify 1.9 import. Those policy claims are superseded, with no parity or
completion credit. The selected v1 sources and both consumer hashes are bound
in roadmap-M0003-platform-effects.json. FSNOTIFY-V1-LICENSE is retained.

BSD/macOS kqueue now has a concrete CGP-owned queue/file watcher, eager sorted
FileInfo scans, source-relative full symlink traversal and ignored broken-link
failures, seen/external/flags maps, missing-directory recheck, zero-Op delivery,
rename/remove child cleanup and overwrite/create branches. Solaris/illumos
NewWatcher returns the exact selected source unsupported error plus newline;
no modern FEN backend is delivered or credited for this utility. BSD source
100ms timed-read/close schedules and register-error stale-fd reuse profiles remain
open. Native AsyncFd readiness is owned, with no detached reader thread.

Windows retains the existing owned completion event/wait callback, now using
the selected source 4096-byte array and LAST_ACCESS/LAST_WRITE/ATTRIBUTES plus
file/dir name filters. ERROR_MORE_DATA uses sizeof([4096]byte). A zero-byte
completion sends an Op==0 event before the short-read error. Linux uses v1's
fsnotify queue overflow text and EOF/short-read continuation. These corrections
replace the earlier modern buffer/filter/slice-header/error descriptions.
Normal cancellation drains native resources before channel/completion close.
Emergency Windows Drop unregisters its waiter before synchronous I/O drain;
failed final unregister or undrainable I/O retains its explicit unverified
abort policy. Path/UTF16/native error/failure ordering/close/races remain open.
Shared Rendezvous keeps per-offer receipts and retraction, zero or source-50
buffering and closed-buffer drain. Errors and Unix Events are unbuffered.

The cfg(windows) dependency reuses existing root windows-sys 0.61.2, enabling
Foundation/Security/Storage_FileSystem/System_IO/System_Console/System_Threading.
No package version/block was added or resolved. Pinned native APIs/source hashes
and fsnotify/Go BSD license attribution are retained in the platform receipt.
No console handler, native filesystem watch, syscall probe or project was run.

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
HTTP mux/path-clean/redirect and URL/RawPath/CONNECT branches are authored but still need exact source profiles;
1xx/sniff/framing/alias and temporary-accept profiles; TLS and Go JSON/YAML/log/
string/number/type/format profiles remain in the partial record. Native errors
are retained, without asserting Go equivalence. Full feature Viper/pflag and
actual command/controller/CRD/admission parent adoption remain open. Async Drop
cannot itself join; normal cleanup is explicit. Do not claim compilation or
parity from code presence, manifest declaration, source mapping or this handoff.

SourceMux continuation: utility_http_mux.rs owns method/host precedence,
wildcards/captures/conflicts, sorted 405 Allow, clean/slash redirects and CONNECT.
Go121 is the explicit 301 compatibility profile. Go126 and Go127 both use 307
with their distinct escaped redirect construction. Actual source tool/GODEBUG
bootstrap and complete byte-path caller adoption remain required.

HTTP response continuation splits utility_http_response.rs, sniff.rs and
status.rs from the existing effect. Full 512-byte ordered sniff signatures,
source status text, H1 2048-byte/H2 4096-byte buffering, attempted int64 counts,
H1 signed versus H2 deferred unsigned-63-bit Content-Length rules, empty writes,
sticky original I/O errors, header snapshots/no-body/HEAD and Date-key suppression
are authored. UtilityRuntimeProviders now supplies static HTTP clock and default
stdio diagnostic capabilities. Static SourceHttpDelay supplies accept sleeps.
Exact framing/partial writes/flush/alias/URL/diagnostic/native profiles remain open.

Server-level OPTIONS * now bypasses mux rejection, uses zero Content-Length,
source 4KB body-drain bounds and H1 over-limit close. Unsupported H1 expectations
produce the source empty 417 with Connection close. H1 and bundled H2 Expect
parsing and Continue state retain their different source rules.

Actual 1xx output uses the exact pinned Hyper 1.12.0 source archive vendored in
rust-support/agones/vendor/hyper-1.12.0, MIT license and original hashes retained.
The opt-in agones-informational feature attaches a bounded per-request sender;
the existing H1 dispatcher/encoder owns flush acknowledgement, and the existing
H2 stream uses the already pinned h2 0.4.20 send_informational API. Dropping the
native owner drops pending acknowledgements. Header operations keep final header
state uncommitted; H1 101 remains the upgrade path. Root Cargo patch/lock source
identity bookkeeping is authored; no version was upgraded or Cargo run.

SourceHttpExecutor uses a weak per-connection UtilityTaskScope for Hyper stream/
upgrade futures. Normal and error exits cancel/join that same owned scope;
emergency Drop aborts its children. There is no extra application manager or
provider registry. Full reset/cancel/stream/late Continue/error profiles remain
unverified; all post-gate compile/runtime/proof work remains prohibited now.

Trailer continuation: utility_http_trailers.rs owns the source forbidden-name
rule, ASCII-space/comma declarations and H1 Header.Write field encoding. The
request writer freezes declarations, selects final handler values and publishes
one SourceTrailers block before yielding a bounded trailers frame. Existing
Hyper H1 consumes it only for a chunked response, without client TE/declaration
filtering. H2 uses real trailers frames, empty-value key presence and unordered
prefixed-key promotion/collision choices. Normal H1 response headers and alias,
all selected profile and native frame/error behavior remain required work.

Bounded ResponseFrame carries H2 end-of-stream decisions for final HEADERS,
DATA and trailers. Source Connection exactly close signals the connection-owned
graceful token; the existing driver owns GOAWAY/shutdown and stream drain.
Exact native HPACK/GOAWAY/timer/reset/error schedules remain unresolved.

utility_http_listener.rs shares AcceptBackoff across plain and TLS providers:
5ms, doubling to 1s, reset on successful accept. CGP SourceHttpDelay owns the
sleep effect. The syscall-wrapped Go Errno rule retains Windows' synthetic
Errno distinction; source direct-Errno/poll/init origin and native error display,
listener-close/shutdown timing and named-service resolution remain open.

Shared-body continuation: utility_http_body.rs owns Incoming, bounded pending
frame data, remaining bytes, H1 underlying/Continue-reader EOF and closed state,
early Close and trailers. H1 header preparation uses the 256KB discard bound and
limit+1 rule; explicit Close has its own strict-greater bound and early-close
state. H2 Close wakes a blocked reader before taking its mutex. Existing
FourZeroFour awaits this owner on its successful-read/response path. OPTIONS
also limits each source read to its remaining 4KB allowance plus one byte.

utility_http_framing.rs lives inside the existing response owner. It authors
H1 inferred lengths, TE/CL/identity/chunking, no-body/HEAD, 1.0 keep-alive/close,
Continue, full-duplex opt-in, Date/sniff/presence and protocol-switch Connection
decisions. Native SourceResponse carries immutable fields/framing; handler-map
fields and extraHeader fields retain separate source order. Source chunks use
lowercase hex. Native source length mode does not truncate/count again or mark
the handler complete just because its declared length was reached.

SourceRequestPhase publishes SourceResponseEnd only after the handler and body
cleanup. Existing H1 driver/reuse state waits, including no-body replies, then
uses the writer's attempted-count/close/error and body's early-close result.
Emergency phase Drop publishes close. Physical connection disposal cancels its
request children; ordinary H1 no-body disposal does not prematurely cancel an
active handler. H2 keeps independent stream disposal. No new task manager.
Source heads bypass the separate native metadata keep-alive heuristic while
preserving explicit shutdown; close-delimited EOF also waits for the source
handler/body-cleanup phase.
These are source paths only: mutable request/header aliases, parser, 1.0 native
keep-alive, upgrade, exact close/abort/Continue/flush/error and all native
schedules remain open in the authoritative HTTP record.

Gateway authoring adds `utility_gateway.rs`, `utility_gateway_json.rs`,
`utility_gateway_wire.rs` and `utility_gateway_wkt.rs`. Actual path:
NewServerMux -> CGP factory -> GatewayContext/GatewayState -> compiled generator
pattern -> CGP dispatcher/decoder/callback/response -> existing SourceMux and
concrete HTTP/TLS writer. Compiled patterns validate stack/pool/deep wildcard,
vars and custom verbs; newest registration wins. Default routing error/status,
POST form fallback/method override and raw/unary callbacks are authored.
No route operation spawns its own task or introduces an effect registry.

Fixed ProtoJSON consumes source names, implicit defaults, presence nulls,
omitted oneofs, enum names/unknown numbers, NullValue, quoted 64-bit integers,
exact decimal/exponent parsing, bytes, unknown-field/enum-name discard, aliases
and oneof duplicate checks. It retains previously decoded state when later
fields fail. Any resolves through the message's linked schema pool and emits
map keys deterministically in its wire payload. Duration and FieldMask use
source-derived rules; Timestamp now has source-derived Go fallback grammar; chrono supplies Gregorian
date arithmetic and canonical UTC formatting.
Struct/ListValue/Value/wrappers and required-field checks are authored.
Concrete HttpBody alone has the outbound raw override; inherited NewEncoder
still writes JSON plus newline. Decoder retains read-ahead between values;
unary forwarding has no delimiter, streams use result/error plus newline.
Request cancellation belongs to the existing HTTP request/body/pending-head
owners and cancels on handler return/client disposal, independently of graceful
server Shutdown. Required lexical, timestamp fallback, base64, diagnostic,
proto2/Any recursion/order, custom model/diagnostic and wire-rendering profiles
remain incomplete; the typed non-proto decoder categories now have authored code.

Typed input continuation adds utility_gateway_targets.rs, scalar.rs and
framing.rs. GatewayJsonPb::unmarshal_into and GatewayDecoder::decode consume
GatewayDecodeTarget for messages, primitives, floats, interface JSON, pointers,
slices, bytes, enums and maps. Element factories are immutable schema/zero-value
construction, not runtime effect-provider selection. Pointer/map preparation
precedes reads. Source null-slice retention, empty replacement, map merge,
last-duplicate raw map name, unspecified map traversal, non-reflexive float keys,
byte temporary assignment and partial container progress are authored.
Non-proto symbolic enums are rejected. EnumTargetFactory requires an explicit
EnumCastProfile, including legacy CVTT, Go 1.27 AMD64 ConvertHash saturation and
ARM64 FCVTZS; never infer the Go numeric profile from Rust's host architecture.
Actual source tool/target/flag selection remains a handler-owner obligation.

Timestamp parser now handles source one-digit hours, comma fractions and the
24-hour/60-minute offset boundary. Base64 uses source quantum logic and original
CR/LF/error-byte positions, selecting raw/padded before ignoring newlines.
Proto integer normalization has the source zero shortcut and int32 exponent
bound. Numeric key parsers include Go base prefixes/underscores and hex-float
round-to-even; NaN payload bits are explicit. Shared framing retains scalar
lookahead and following values; read errors are sticky, semantic decode errors
allow later values, and the source JSON maximum depth is explicit.
Go 1.26.5 minimum and Go 1.27.1 historical-tool source files were read by pinned
tag; archive/URL/digest records retain both. No tool/compiler was installed or
executed. Source identity/algorithm reading is not differential/runtime proof.
Derived Go code retains BSD-3-Clause attribution in rust-support/agones/GO-LICENSE.
RawObject still materializes an object before applying its protobuf fields;
full incremental malformed-key/error mutation and diagnostics remain open.

Full generated source descriptors/SDK/allocator path/query/body/RPC handlers,
header/deadline/metadata/trailer/response_body integration and actual command
adoption remain explicit M0025/M0027 work. This factory is a callable library
path, not credit for those preserved callers. See the gateway receipt.

Dependency declarations were authored without Cargo resolution: one exact
prost-reflect 0.14.7 package reuses existing once_cell/prost 0.13.5/prost-types.
Its optional serde layer and serde-value/ordered-float candidates are NOT
adopted. Preserve existing errno/regex/native and earlier CPG comprehension
blocks. protobuf-json-mapping 3.7.2, notify 8.2 and signal-hook-registry were
read as candidates only. Archive/index/API input hashes are in the receipt.
One read command attempted `go env GOROOT` for a source path; stdout was empty
and its dependent file read failed. No project/compile/fixture/semantic assurance
ran. Do not reinterpret source reading or identity bookkeeping as verification.

M0003 entry usage 656008. Raw progress observations 1..20 retain 752278, 781155,
905410, 1069792, 1095960, 1289081, 1320264, 1568425, 1577646, 1691561, 1702165,
1783942, 1794616, 2082051, 2115290, 2286210, 2350385, 2469680, 2482023, 2579579. Still NO completed M0003 cost comparison or
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
