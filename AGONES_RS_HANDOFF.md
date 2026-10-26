# Agones-rs and CPG handoff

Canonical ledger: `AGONES_RS_STATE.json`, revision 328. One active native Goal:
`01a17928-8551-7701-8497-5333c680f283`. Combined attachment/roadmap SHA:
`a9b797fc624be04548c62e63a634bcc2cacab25c4d9f45af346148ba32c1baea`.
Authority is `roadmap/0000-contract.md` plus `roadmap/alternation-policy.md`.
The archived standalone first-generation stop rule is superseded for this Goal.
Do not reread the complete combined roadmap; use the current milestone/inputs.

Phase **IMPLEMENTATION_ONLY**; global gate **CLOSED**; verification NOT_STARTED.
ZERO Cargo check/build/test/clippy/doc, formatting/lint campaigns, compiler/IDE/
macro/API probes, fixtures/apps/examples/replay/proofs, generator validation or
CI before every required production body/output in BOTH routes is authored and
connected. An already runnable emitter may emit required implementation without
assurance hooks. A genuinely required forbidden emitter build is a real blocker.
Source/API reading, authoring, content/ownership bookkeeping and local commits
are allowed. No project executable or verification was run in this batch.

M0001 MANUAL: **IMPLEMENTED_UNVERIFIED**, `53a958f76`, 13 owned entries.
M0002 CPG: **IMPLEMENTED_UNVERIFIED**, `95161696e`, 49 entries (11 production +
38 retained references), 62 operation/closure groups. See roadmap-adoption-M0002
and roadmap-M0002-body-ownership receipts. M0006/M0008/M0010 schema/tool/domain
requirements remain open. Flattened cpg-rs/generated is not complete literal
hierarchy credit. Preserve cpg-rs/src, CPGSchema and the historical 695 body gap.

Current cursor: **M0003 / MANUAL / IMPLEMENTING / READY**. Do not switch to CPG
until the entire authored M0003 milestone is complete. Current partial ownership:
`verification/agones/roadmap-M0003-partial-map.json`; retained-body adoption:
`verification/agones/roadmap-M0003-reuse-adoption.json`. The map records actual
symbols, caller/resource ownership, exact remaining profiles and payload hashes.
Twelve of fourteen entries contain authored code at varying depth; this is NOT
source parity, successful compilation or milestone completion. The only absent
literal mirror entries are signals/signals.rs and workerqueue/workerqueue.rs.

The existing util workspace package registers through
`rust-support/agones/src/utility_lib.rs`. Literal bodies live under
`agones-rs/pkg/util`; Rust-only composition lives outside that mirror.
`SourceErrorOrigin`, `UtilityFeatureProviders`, `UtilityCrdProviders` and
`UtilityRuntimeProviders` connect typed declarations through derive-deftly to
actual CGP capabilities/providers. No runtime effect-provider registry was added.

Authored M0003 paths now include:
- Error origins/prefixes/wrappers, source log fields and all feature operations;
  one synchronized feature map moves from M0001, using the existing core parser.
- CRD independent 60-second wait/immediate GET/fixed 1-second ticker, ignored
  parent cancellation, Rusternetes HTTP transport and shared source logger.
- Runtime JSON logger/level/timestamp/error/backoff, Must and logger identities.
  `UtilityRuntimeOwner` owns shared logger/backoff and root `UtilityTaskScope`.
  Contexts carry Weak task-scope references to prevent child/self-owner cycles.
  Close/cancel/join, request reaping, spawn-or-return and Drop abort are authored.
- Concrete bounded streaming HTTP bodies/writer, source exact/prefix mux,
  sensitive header redaction, request/error/404 callbacks and plaintext server.
  Plaintext uses HTTP1; TLS selects HTTP1/HTTP2 from ALPN. Header timeout is 60s.
- TLS initial/reload hardcoded /certs paths, passed listener fallback paths,
  mutex-protected dynamic certificate, retained old cert on reload failure,
  Fatal/Exit branch and real Rustls listener/handshake effects.
- Connection/request cleanup transfers to the root owner when a listener
  returns, including permanent accept error; it does not cancel the caller
  token. Existing requests drain on source/owner cancellation. A closing owner
  receives rejected live resources back and cancels/joins inline.
- Actual Linux/Android inotify fd reader, rendezvous event/error channels,
  first-event batching timer and source captured-nil-error callback. Explicit
  close completion follows fd close. Normal parent cleanup awaits owned tasks.
- WebHook registration/ordered review callbacks, UID/default response, nil
  request panic, current-review matching, error-status updates and continued
  handlers. Admission wire includes raw extensions and base64 patch bytes.
- APIServer OpenAPI v2/v3 and 406 /apis routes, resource discovery/delegation,
  source namespace parsing, metadata-copy/target-kind serialization, JSON/YAML
  and Kubernetes runtime.Unknown protobuf encoders. Source late Content-Type
  on the 406 branch is retained. Comprehensions reuse the existing vendored
  comprende and better_comprehension, without temporary header-map allocation.

Next work remains inside M0003. Author real signals with the actual SIGINT +
SIGTERM set and one-shot handler. Stop must restore source default/previous
behavior: dropping Tokio signal streams or signal-hook actions alone does not
provide this. Source NewSigTermHandler retains its registration after the first
callback; parent lifecycle must still own final cleanup. signal-hook-registry
was read only, not adopted. No signal/provider placeholder was credited.

Author exact source workerqueue behavior and transfer existing bodies/callers
explicitly. Preserve both crates/util/src/workerqueue.rs and Rusternetes
controller_work_queue until adoption; they receive no new source credit.
Source traceError has NO Unwrap; the old Rust TraceError.source() must not become
source semantics. Preserve typed-any bad-item behavior, source key extraction,
limiter/dedup/delay/Done/Forget/retry/conflict/trace logging, shutdown, worker
count/health and parent-owned worker/timer cleanup.

Other material remaining M0003 requirements:
- Runtime NewServerMux grpc-gateway HTTPBody/proto-name/default/unknown-field
  behavior; actual InClusterBuildConfig fallback through Rusternetes only.
- Non-Linux/Android watcher backends and exact source Add/read/close/path/errno
  profiles. notify 8.2 was downloaded/read in .git but rejected (no exposed join).
- Full HTTP ServeMux host/method/wildcard/path-clean/redirect behavior, 1xx,
  Go URL/RawPath/CONNECT/escaping, content sniffing/framing/default/error/flush
  behavior, mutable header aliases and listener temporary-error backoff.
- Go logging/type/interface/byte-string/number/format/caller profiles; complete
  serde versus Go JSON and go-yaml versus native YAML profiles; Go unstable
  negotiation-sort/ParseFloat boundary. Native dependency errors are retained,
  without claiming source equivalence. SourceFormat is not a Go fmt parser.
- TLS PEM/key/crypto/default/error/shutdown profiles; admission raw/null/case/
  duplicate-key/error/mutation profiles; source feature Viper/pflag consumers.
- Immediate root failure policy/notification for child failures/panics; actual
  parent command/controller/admission/CRD integration. The old controller-stack
  CRD/cache barrier bypass remains untouched. Async Drop cannot itself join.
- Whole-body/initializer/closure/import/output reconciliation of all 14 entries.

All new root dependency bookkeeping is authored without Cargo resolution. Reuse
existing HTTP/TLS/Tokio/libc/JSON/YAML/base64 lock identities; five comprehension
package blocks were copied from the already resolved isolated CPG lock. This is
source bookkeeping, not evidence that manifests/macros/APIs compile.

M0003 usage entry 656008; progress raw observations 752278, 781155, 905410,
1069792 are retained as progress-usage-1 through -4. No completed M0003 cost
comparison or route switch. Reading/context recovery/accounting is charged once
within the active work segment; no credit charges are inferred. Preserve M0001/
M0002 original forecasts/comparisons and explicit shared completion/switch
segments. M0002 observed work was 482306 tokens; shared completion/switch was
20030. Entry bookkeeping note remains in the map, without double charging.

Work only on `agones-rs-dev`, never main. Local commits allowed; push/PR/merge
is deferred because it would trigger forbidden CI. Goal stays active until BOTH
scopes, required generation, post-gate assurance and authorized delivery are
complete. Predecessor ledger/handoff and receipts remain recoverable by hash.
Older checks/proofs remain historical and stale; do not run old checkpoint.py,
which targets the superseded contract. Resume from this cursor and concrete
remaining bodies, without rebuilding a second queue or acceptance system.
