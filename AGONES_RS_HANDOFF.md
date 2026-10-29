# Agones-rs and CPG handoff

Canonical ledger: `AGONES_RS_STATE.json`, revision 381. Native Goal:
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

Immediate next action: Continue M0003 MANUAL without route switch. Author source klog create/openOrCreate/logName/logDirs/shortHostname and platform username initialization, connect concrete default filesystem/clock/binary bindings, complete the print/println/printf/Fatal/Exit wrappers, source flag/vmodule/slog conversions and remaining native buffer/daemon/exit/time/error profiles. Retain every other M0003 source/platform/configuration/lifecycle and full BOTH-route requirement. Complete remaining source raw Go format/value behavior and production bindings. Canonical single/multi error wrappers now own retained interfaces and mutable shared Unwrap slice backing, with nil/len/cap/windows, element replacement and source-profile growth. Retained headers survive parent destruction and later cell replacement; the native type-match helper returns an ownership-anchored Deref match. Existing formatting and runtime/panic/marker consumers resolve the original payload and live elements. All code is uncompiled/unverified. Source Is/As/AsType/Unwrap control bodies are now authored with declared per-error methods, supplied typed equality/assignability, exact validation branches, retained targets and lazy AsType storage. Owned source queue classification now retains its root and uses static shared targets; typed pointer and APIStatus targets can be retained/re-entered, non-error APIStatus receiver data is supported, and each reason/code Status expression reloads the target variable. Concrete typed nil *StatusError and *traceError bindings now author non-nil error interfaces, true/nil pointer assignment, same/different declared nil-pointer type equality, real Error/Status nil receiver panic and fmt nil-pointer recovery. Non-nil traceError has a raw Error method and private-field/pointer base formatting. Source worker Trace(err) uses a CGP level-before-format provider and byte-preserving JSON message data, and Wrap/Wrapf use the same source operand formatter rather than calling Error directly. Complete every other required typed-nil, arbitrary/runtime/canonical/custom/value/inherited/interface declaration and all concrete source method sets, comparability/equality/assignability/type constraints, full APIStatus receiver/value/string/status/detail/list/reference-copy identities, external borrowed/opaque/native adoption and all source allocation/GC/drop/ABI/unwind/synchronization/race/sanitizer/configuration profiles. Native TypeId here identifies explicit declared pointer targets only; it is not full Go type/header/reflect/heap evidence. Retained WithError fields now defer Error until enabled JSONFormatter conversion, before reserved field clashes/encoding and under the existing output/formatter lock. Source message Sprint precedes that lock and field Error conversion; direct field Error panics are not caught by fmt. M0003 concrete callers retain their original error Boxes, and configuration/default-runtime error message consumers use lazy source operand formatting. Selected ErrorHandlers mutable header/live slots, retained callback reports, nil callback panic, Agones default Logrus replacement and existing backoff owner are now authored and connected through CGP. The ordinary wrapper filters nil before inherited dispatch; the contextual entrypoint dispatches nil reports. Source klog WithValues, format-before-deduplication interval control, output compaction and raw multiline string formatting are now authored and consumed by the native logPanic string-field path. Concrete logr Logger control/optional sink methods, the klogger sink, contextual enable/background/backing option routing, selected-writer/filter control and source original klog error handler are now authored. Agones initialization first constructs the inherited handler slice and then replaces slot zero with Logrus. Crash handling now stores the depth-four contextual logger before callbacks and panic reporting logs through it; the original error handler applies depth three and the gated UnhandledError name. The actual logr key lookup and nil/missing/type branches are authored over retained SourceErrorContext data; complete generic Go context Value/Deadline/Err/custom/nil/cancellation/lifecycle and native representations remain required. Complete native slog Handler/record/from/to/context conversions (the current slog context value requires an actual per-logger compiled conversion binding), full native typed-nil/function/sink/type/header/word/alias/heap/GC/drop/race/initializer profiles, source vmodule parsing/PC-cache/caller lookup and all header/flag/severity/writer/pool/flush/fatal/backtrace/native output/failure paths. The CGP emitter now invokes the authored header/output state after complete KV body formatting. Strict source header preparation implements caller then timeNow, source file/add_dir_header selection, buffer.Time override, fixed header and LF placement. Non-fatal output policy implements threshold/legacy/also-to-stderr logic, configured one/single/severity outputs, source high-to-low fallback, trace matching, ignored writer results, stats and create-error diagnostic/flush/sync/exit control. Native source_header_bound=false remains the default body-only projection until actual caller/clock bindings are supplied; it is not skip_headers or a fabricated runtime.Caller failure. Source syncBuffer Write/rotation/createFiles and the selected native real-clock flush daemon are now authored and connected. SourceKlogWrite retains partial counts and original errors; the 256-KiB buffered writer preserves direct large writes, retained error state, ignored Flush errors, short-write retention/compaction and exact buffered counts. Rotation triggers before writing on wrapping nbytes+len >= maxbytes, flushes/closes the previous file, replaces the current file even on create failure, initializes startup size before buffer replacement, creates a fresh buffer and writes the source file header directly with byte accounting. Error callbacks run after releasing the native buffer guard while the caller keeps logging.mu, allowing exit/flush to re-enter that same buffer. Write still returns its count and retained error after an exit hook returns. CalculateMaxSize preserves unlimited zero-MB single-file mode and wrapping uint64 multiplication. createFiles starts the same daemon before obtaining one clock value, creates severities downward and stops at the first existing file, publishing each successful buffer only after rotation. A concrete NativeKlogFile supports append/read-write startup versus truncate creation, partial writes, metadata size, sync and close; native errors remain actual std::io::Error data without claiming source *PathError/errno identities. Specific klog header Time.Format layout uses the existing calendar and one zone lookup; general Time.Format remains required. The source daemon run/isRunning/stop/start control is statically selected by UtilityRuntimeProviders -> SourceKlogFlushDaemonController -> ControlNativeSourceKlogFlushDaemon; its periodic work uses the existing UtilityTaskScope/JoinSet and parent panic policy. Stop holds the daemon mutex, sends a buffered request, waits for a final Flush and then clears channel state; completion is retained in a native Drop guard for panic unwinding. Existing parent shutdown stops before joining and native Drop only requests stop. Real-clock deadlines retain one ready tick and coalesce missed ticks, but do not establish source ticker/channel/select/native scheduler equivalence. Fatal output now authors the persistent atomic fatalNoStacks flag, status-one no-stack path, conditional current-stack stderr output, all-stack file writes from FATAL to INFO, source no-op logExitFunc replacement, unlock-before-timeoutFlush and status 255 without statistics. Actual native current/all-goroutine stack bindings are still absent. Source FlushAndExit and timed flush control are authored through a capacity-one completion channel and the existing CGP-owned blocking task scope; timeout does not cancel live flushing. Ordinary Flush releases logging.mu before file Sync; file-error exit retains it through Flush/Sync and returns normally if its process-exit effect returns. Stderr has its own stream owner outside logging.mu so a timeout diagnostic can run while the flush worker holds that mutex. The source timeout duration diagnostic uses the common Duration.String bytes. Concrete file-name/directory/username selection, actual native file/clock/source-frame/error/buffer/daemon bindings and full filesystem/configuration/platform behavior, source print/println/printf and all Fatal/Exit wrapper bodies, pool/failure/GC/ABI/race and complete native stack/timeout/timer/select/OsExit/configuration profiles remain required, with zero acceptance credit. The returning OsExit(1) source continuation performs unlocked accesses and a later second Unlock; this native branch explicitly reports that binding gap rather than inventing a relock or successful completion. Native recv_timeout does not establish source time.After/select scheduling, especially simultaneous completion/expiry and nonpositive durations. Missing/closed task owners or unavailable native runtime handles remain explicit binding failures; a lost completion sender is never successful flushing. Tokio blocking work which has started cannot be aborted; parent shutdown must join it and a permanently blocked flush can keep that join pending. Complete source/native shutdown, runtime ownership, goroutine panic/Goexit, timer resource/lifetime and process-exit-hook profiles remain required. Non-slog klog textWriter/Stringer/string/error/one-shot Marshaler/borrowed []byte dispatch, fmt %s panic recovery, AnyToString hook and JSON fallback control are now authored through retained typed argument views. Source scalar JSON escaping/integer/float/nonfinite formatting is authored; custom JSON Encoder.Encode callbacks retain returned source errors, roll back only source encoder errors, and require the success final LF before removal. generateJSON separately recurses through Marshaler and invokes Error directly. Typed slog scalar/string/duration/group/AnyValue, Kind, Attr/slice, accessors, Group argument conversion, exact 100-call nonrecursive-group Resolve, source error results, equality control with a required declared interface comparator, private-field/base projections and concrete default JSON encoders are now authored. Klog now handles concrete slog.Value before Stringer and raw LogValuer after Marshaler, with the distinct wrapper-group object and peeled []Attr encoding paths. Source Time wall/ext/location and monotonic packing/removal/equality/comparison, wrapping Unix normalization/conversions, Add/Sub saturation/overflow, UTC/fixed-location identity/cache, String/GoString/source-calendar/strict-RFC3339 JSON and retained time-specific json.MarshalerError Error/Unwrap are now authored. Slog TimeValue zero/compact/full representations, monotonic removal, Time/Any/String/Equal and the distinct method-less slog.timeTime private representation are authored and connected. Complete dynamic timezone transition/TZ extension/local initialization/loading/caching/global Local reassignment (including fixed-location identities), native wall/monotonic clock/bubble/scheduler/resource/configuration and every remaining required general Time Format/Parse/Date/text/binary/Gob API and production binding. The current time-specific json.MarshalerError retains its original error and declared methods, but complete mutable exported Type/Err/sourceFunc cells/receiver semantics, native reflect.Type/rtype construction/base fields, arbitrary JSON error families and typed-nil interfaces remain required; missing type data is explicitly unbound and never represented as source nil. Source calendar and field copies here do not establish Go32 int/year/offset/native width, pointer/layout/type/GC/allocation/retain/drop/alias/race/ABI identities. Complete source LogValuer wrong-kind type-assertion errors, every typed/named/interface equality/comparability declaration and all native receiver/type/header/pointer/word/layout/allocation/copy/heap/GC/retain/drop/race profiles. Complete actual runtime.Callers/CallersFrames capture at the CGP recovery call site, correct runtime/compiler/inlining/symbol/native stack profiles and panic(nil)/GODEBUG/Goexit/fatal behavior. Resolve stack formatting is authored, including returned PC count versus logical frames and zero Frame, but its default native capture binding is absent and returns an explicit binding gap. Complete all arbitrary/named/interface/pointer/map/slice/struct/JSON Marshaler/TextMarshaler/Number/cycle/error bindings and production callers, full source buffers/retained aliases/scratch/growth/overflow/stack/GC/failure profiles, all callback/argument/type/receiver/copy/native initializer/ABI/unwind/race/scheduler/timer/configuration and whole caller adoption. The native crash reporter now uses the actual contextual SourceLogr.Error path; the default klogger reaches the CGP SourceKlogRecordEmitter and common SourceKlogFormatter. The obsolete format_panic_strings driver was removed and panic_fields now supplies the source variadic fields without an extra pre-merge; additional typed method dispatch and slog bodies have no executed evidence and do not establish complete logging parity. Concrete borrowed []byte views cover the selected canonical declaration only; every named/mutable/retained byte-slice/backing/type binding remains required. Nonfinite scalar JSON uses the known UnsupportedValueError formatting branch; full source error allocation/receiver identities and native callback/encoder completion profiles remain required. Callback native data is retained without an interpreter, and unbound source formatting/type metadata stays explicit. Complete Logrus arbitrary field types, raw field keys, error-valued field range order/alias/concurrent mutation, function rejection/logrus_error, hooks/caller/configured formatters/pool/concurrent-level/fatal/panic/output/diagnostic/native profiles and remaining logging/klog bodies. Native BTreeMap conversion order and retained-descriptor PartialEq are not Go map range or interface equality evidence; arbitrary borrowed native WithError retains its explicitly incomplete compatibility fallback. Native Arc/RwLock cells and safe cross-thread data ownership do not establish source heap, race, synchronization or scheduling equivalence. Opaque native fallbacks can bypass unmapped custom methods and do not establish source semantics; strict query APIs report an unbound method set. All code is uncompiled/unverified. Complete arbitrary/custom Unwrap declarations, complete source array/cell/header/allocator/native layout and pointer identities, GC/drop/finalizer/retain/escape/native synchronization/race/sanitizer/experiment profiles, every source target/configuration and whole production binding. Native RwLock/Arc hosting and unsafe ownership-anchored native projections provide no executed source/native evidence. Complete arbitrary runtime string/other error-family declarations and actual source/native receiver, allocation, byte, integer-word, target/configuration, pointer, interface and production bindings; formatting these three native projections does not establish full runtime-error or ABI acceptance. Complete all opaque/undeclared error bindings, value-receiver/inherited/nil/mixed interface method sets and APIStatus combinations, complete concrete source type comparability/equality/custom Is/As bindings and mutable/retained Unwrap slice profiles, non-Copy closures/functions, full mutable reference storage and all source/native identity/lifetime/heap/allocation/GC/ABI/configuration profiles. A native Arc/interface carrier is not source GC/header/heap evidence. Full source map/hash/growth/random range/concurrent-fatal and tied-NaN provenance remain required; native map adapters are partial. Complete source/target configuration inventory and complete production context bindings, native compiler/codegen/heap/allocation/stack/pool/failure/ABI/GC profiles and post-gate edge/rounding/refinement evidence. Complete source runtime heterogeneous values, complete source map-storage/copy/reference-alias/hash/range/concurrent-fatal and NaN-tie provenance, interface type-token and pointer/word/ABI bindings, inherited/nil method declarations and copied/retained error/interface ownership. Static aggregate/native base views remain authored; source mutable storage/field addressability/copy/lifetime profiles and unknown/noncanonical reflection still require complete paths. Complete retained State/pool aliases, panic(nil)/GODEBUG/Goexit/cross-goroutine/hook/unwind/fatal/trace/exit behavior and every byte consumer. Native Display defaults/opaque fallbacks and indexed/scanned external-rendering compatibility inputs still require complete source bindings. Native fixed arrays/Vec/Cow/Box/reference arithmetic do not establish source scratch/overflow/GC/pressure/failure/sanitizer/experiment/elision/native ABI profiles. Complete mutable/aliased Unwrap() []error native profiles, copied/retained interface/error identity/lifetime/native allocation/GC semantics and all concrete custom Is/As declarations. Preserve full klog/logr header/multiline/field/context/sink/flags/caller/depth, retained/cross-goroutine panic values, Goexit/panic chains, source/native callback/timer/cancellation and trace/stack/freeze/secure/crash/core/printing/exit/atexit/native-hook profiles. Preserve remaining parent/direct HTTP/protocol/connection/JoinSet boundaries, H1 abortPendingRead/hijack/multipart/cleanup-panic/native close and H2 committed-head/no-body END_STREAM/reset/disposal requirements. All prior quote/builder/cache/control/guard-future/context/format/Arc/Bytes/RawValue/serde/native allocation, source GC/heap/elision/sanitizer/experiment/pressure/failure, Go map hash/group/growth/range/key/pointer/alias/concurrency, ABI/unwind/address/layout/lifetime, fmt pool/tiny/extreme Go32, GRPC_GO_LOG_*/custom logger/source SDK/command requirements remain. Every prior Unicode, MIME/multipart/MaxBytesReader/Values.Clone/query/escape/GODEBUG, request/header/native HTTP/listener/fsnotify v1.4.7/platform/config/queue/logging/CRD/features/admission/body requirement remains. No executed evidence, full parity or M0003 completion is claimed. Complete source create/openOrCreate/logName/createLogDirs/shortHostname and once-only username initialization for Unix/cgo/pure-Go/Windows domains and fallback/sanitization, symlink/remove ordering and ignored errors, exact raw filepath/platform/permission/umask/errno/*PathError/Stat/Close/descriptor/retained receiver identities and concrete default file/clock/binary context initialization. Actual file creation is still a required compiled binding passed to SourceKlogFileBindings.create; the native open adapter alone is not complete source filename/directory selection. The synthetic SourceKlogOutputWriter entry without its logging owner is explicitly unbound; canonical source syncBuffer.Write maps to source_write_buffer with real owner, flags and exit policy. Complete all remaining source bufio/file declared type/short-write sentinel/partial-error/invalid-count/mutable alias/return/allocator/GC/race/native profiles. Complete configurable/fake source clock effects and native NewTicker/stopDone/ticker.Stop/select/Goexit/unwind/panic/invalid-interval post-panic channel-state/Instant-domain/shutdown/resource profiles; current invalid-interval panic does not retain the source preallocated channels. Complete source global MaxSize/flag unsynchronized mutation and source word/copy/layout/GC/ABI profiles; native mutex/Arc/Instant/channel ownership provides no such evidence. All new code is uncompiled/unverified, and all earlier scope remains.

This batch authors concrete klog sync-buffer/required buffered-writer operations, partial count/error retention, source rotation/startup/header/accounting, max-size/createFiles control and native file operations. Normal and fatal output now invoke the actual declared sync-buffer path with logging ownership; callbacks release the native buffer guard before exit re-enters Flush. The flush daemon has source run/stop/restart state and a native real-clock worker owned by the existing CGP/derive-deftly task scope; parent shutdown stops it before joining. File-name/directory/username selection and complete native filesystem/error/clock/defer/configuration/default bindings remain required. All code is uncompiled/unverified; no execution or assurance credit.

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
  owner. SourceFormat has partial raw scanning/rendering bodies; complete concrete
  source type/format/caller/method binding work remains open and unverified.
- CRD independent deadline/immediate GET/fixed ticker, ignored parent cancel,
  Rusternetes HTTP reader and shared source logger. Native client/error profiles
  and the old controller CRD/cache barrier bypass remain open.
- Runtime JSON log state/backoff/Must/logger identities and shared weak-referenced
  task ownership. Returned failures retain bounded observation and original
  joined values without observer Display calls. CGP utility spawns now wrap poll
  panics in the selected parent report/exit policy; rejected drain futures retain
  the same guard and live resources. Direct native/HTTP/protocol scopes, worker
  HandleCrash/callback/ReallyCrash, full fatal runtime/trace/exit profiles and real
  command integration remain required. HTTP request recovery stays distinct.
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

HTTP continuation details live in roadmap-M0003-http-authoring.json and
roadmap-M0003-hyper-adoption.json. The existing effect registers response,
framing, body, headers/capacity, trailers, sniff and status helpers outside the
mirror. Source 2048/4096-byte buffering, attempted int64 counts, H1 signed/H2
unsigned-63-bit length parsing, empty/HEAD/no-body rules, full sniff/status tables,
Date suppression, OPTIONS 4KB and expectation/Continue distinctions are authored.

SourceRequestBody owns Incoming, pending data, remaining length, H1 underlying/
Continue EOF/closed flags, bounded 256KB discard/early Close, initial declared
trailer maps, actual trailer merging and H2 close/read wakeup. FourZeroFour awaits
its successful-read cleanup. Shared request method/content-length/body/URL
replacement and alias projections still need completion.

Headers/HeaderValues now own byte-key maps and shared backing slice views,
including nil/empty, offset/length/capacity and indexed mutation. Clone is map
assignment; shallow_copy allocates a map sharing values; H1 deep_clone uses one
combined clipped backing with source-int accumulation; H2 h2_clone makes each
slice separately, converting nil values to non-nil empty. Native H1 wire fields
sort raw keys and normalize bytes. Header capacity uses explicit Go1.26 word32/
word64/heap profiles and source size classes; constructor Go126/64/48 is explicit,
not proof of actual M0027 source ABI/tool binding. Range additions/nested prefix
promotion, pointer identity, allocator/panic/native profiles remain open.

LogRequest retains the redacted map as static SourceLogValue::HttpHeaders until
CGP SourceLogEmitter/SourceLogger serialization. Sensitive values are independent
REDACTED slices; non-sensitive arrays remain shared. Header JSON keeps nil/empty,
raw-byte key order, replacement-induced duplicate keys and exact invalid-byte
UFFFD versus valid-UFFFD escaping through already-enabled serde_json RawValue.
headers_json remains only a compatibility Value projection and cannot retain
duplicate object keys. This leaf data model is not a general Go object layer.

Exact Hyper 1.12.0 source remains vendored, with original MIT license/archive/file
identities. Bounded informational commands use existing H1 encode/flush ack and
pinned h2 0.4.20 send_informational. H1 SourceResponse carries immutable fields/
framing; source lowercase chunks and attempted-count ownership bypass native
recount/truncation. SourceTrailers publishes once before its frame. Header and
extraHeader order stay separate; metadata keep-alive heuristics do not override
the source decision or re-enable explicit shutdown.

SourceRequestPhase publishes SourceResponseEnd after handler/body cleanup.
Existing H1 driver waits before reuse or physical close, including no-body and
close-delimited responses. Connection disposal cancels its request children;
normal H1 no-body disposal does not cancel an active handler. H2 retains stream
cancellation. SourceHttpExecutor/UtilityTaskScope own protocol children through
normal cancel/join and emergency abort. The same CGP HTTP/TLS providers share
AcceptBackoff (5ms doubling to 1s), with static delay/date/diagnostic effects.
All parser/version/upgrade/partial-I/O/flush/reset/abort/Continue/error, native
schedules and production parent/source-profile adoption remain unverified work.
No new runtime registry or competing lifecycle manager was added.

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

M0003 entry usage 656008. Raw progress observations 1..110 retain 752278, 781155, 905410, 1069792, 1095960, 1289081, 1320264, 1568425, 1577646, 1691561, 1702165, 1783942, 1794616, 2082051, 2115290, 2286210, 2350385, 2469680, 2482023, 2579579, 2609990, 2707816, 2741726, 2928302, 2936025, 3233954, 3245592, 3556307, 3566347, 3630874, 3637953, 3675741, 3684341, 3800243, 3812429, 3874147, 3879089, 3947602, 3956894, 4026389, 4034406, 4102317, 4110264, 4177671, 4187033, 4284942, 4294159, 4335337, 4343582, 4385962, 4394343, 4440714, 4453626, 4518208, 4524847, 4563009, 4570717, 4623725, 4635475, 4677213, 4684244, 4717769, 4897014, 4947029, 4957725, 5016398, 5023771, 5067663, 5075095, 5319230, 5325607, 5375225, 5439488, 5503909, 5509504, 5536175, 5544752, 5605898, 5607953, 5667951, 5677473, 5733338, 5741478, 5819158, 5825143, 5937632, 5944277, 5997603, 6014391, 6075671, 6089898, 6130399, 6144002, 6196656, 6206386, 6249147, 6258582, 6356850, 6366690, 6440633, 6450488, 6530939, 6537779, 6619567, 6627225, 6686791, 6693028, 6801532, 6808129, 6865579. Still NO completed M0003 cost comparison or route switch. Source/context/read/accounting is charged once; no credit inference.
Preserve M0001/M0002 original forecasts/comparisons and shared overhead buckets.
M0002 observed work was 482306; shared completion/switch was 20030. The explicit
M0003 entry bookkeeping allocation remains unchanged.

Work only on agones-rs-dev, never main. Local commits allowed; no push/PR/merge
while they would trigger prohibited CI. Goal remains active until BOTH complete
scopes, generation, post-gate assurance and authorized delivery are achieved.
Predecessor state/handoff are retained by hash. Older checks/proofs remain stale;
do NOT run old checkpoint.py, which targets the superseded contract. No second
Goal, ledger, acceptance queue, project thread or automation is needed.
