# Agones-rs and CPG handoff

Canonical ledger: `AGONES_RS_STATE.json`, revision 327. One active native Goal:
`01a17928-8551-7701-8497-5333c680f283`. Authority is the combined attachment plus
`roadmap/0000-contract.md` and `roadmap/alternation-policy.md`. The archived
standalone first-generation terminal rule is superseded for this invocation.

Phase **IMPLEMENTATION_ONLY**. Global gate **CLOSED**. Verification NOT_STARTED.
Do not build, compile/check/test, lint/format, run IDE/compiler/macro/API probes,
execute fixtures/apps/examples/generators, extract a semantic property campaign,
prove anything or trigger CI before the complete BOTH-route implementation gate.
An already runnable emitter may emit required implementation without assurance
hooks; a required forbidden generator build is a real blocker when encountered.

M0001 MANUAL is **IMPLEMENTED_UNVERIFIED** at `53a958f76`.
M0002 CPG is **IMPLEMENTED_UNVERIFIED** at `95161696e`; its source ownership,
62 operation/closure accounting groups across 11 production files, native build
facilities, cache/codec/version/effect ownership and 49-entry adoption are in
`verification/agones/roadmap-adoption-M0002.json`. The partial predecessor is
historical. Current M0006/M0008/M0010 full schema/tool/domain output requirements
remain open; flattened current generated output is not complete literal hierarchy
credit. Preserve cpg-rs/src, CPGSchema and the historical 695-candidate body gap.

Current cursor: **M0003 / MANUAL / IMPLEMENTING / READY**. Do not switch to CPG
until the entire M0003 authored production milestone is complete. Source entry
ranges: `verification/agones/roadmap/entry-index.tsv`. Partial ownership is in
`verification/agones/roadmap-M0003-partial-map.json`; existing-body adoption is in
`verification/agones/roadmap-M0003-reuse-adoption.json`.

M0003 now has authored source in five of its fourteen entries, at varying depth:
- doc.rs and source log-field constants/augmentation;
- error origins/prefixes/wrappers and literal package/function-name handling;
- feature operations and read/write state using the existing core parser;
- the CRD wait policy and concrete Rusternetes HTTP read capability.
This is partial implementation, not compilation, source parity or completion.
No placeholder files have been credited for the remaining nine mirror entries.

The util package's Cargo library entrypoint is now external registration at
`rust-support/agones/src/utility_lib.rs`; mirror bodies retain exact source names.
`UtilityFeatureProviders`, `UtilityCrdProviders` and `SourceErrorOrigin` are real
derive-deftly/CGP compositions. `InitializedAgonesConfiguration::into_parts` and
`UtilityFeatureContext::from_initialized` move the M0001 map into one synchronized
owner without cloning it. Existing query/default/encoding bodies remain canonical
in agones-core. The prior native error file now re-exports the mirror owner, with
its legacy public methods/macros preserved. The existing utility worker queue and
Rusternetes controller_work_queue are preserved and receive no new source credit.

The CRD helper keeps immediate first GET, fixed 1-second ticks, an independent
60-second deadline and the source's ignored parent cancellation. Timer/GET futures
are owned; no detached child is created. Rusternetes is its only concrete adapter.
Source path-name rejection and typed CRD projections are authored. Shared source
logging and source client error/encoding boundary accounting are still open. The
existing controller-stack CRD/cache barrier bypass has not yet been replaced.

Next M0003 work: actual runtime logger/error/config effects; actual HTTP/TLS
servers, header redaction/404/error callbacks, cert reload and graceful shutdown;
real filesystem watcher batching and source captured-error behavior; real signal
registration/cleanup; ordered webhook/admission callbacks; exact queue policies,
limiter/key/type/error/retry/delay/shutdown/health semantics and child ownership.
Complete feature flag/Viper profiles and source type/format/caller metadata too.
SourceFormat is not a Go fmt parser. Source traceError has no Unwrap: the old native
TraceError source() behavior must not silently become the source implementation.

M0003 entry usage is 656008 host tokens; latest saved progress is 752278.
Only progress is recorded; do not create a completed M0003 cost comparison yet.
M0001/M0002 comparisons are under `cost-analysis-comparison/`. M0002's observed
work segment is 482306 (153672 -> 635978), versus original base 40000. The 20030
completion/switch segment is charged once as shared (635978 -> 656008). M0003
entry bookkeeping, including the final M0002 boundary paragraph write, is allocated
once to M0003 entry overhead. No credit charges or exact full M0002-only cost are
inferred. Preserve all original forecasts and compatible raw observations.

Work only on `agones-rs-dev`; never modify main. Local commits are allowed;
push/PR/merge remains deferred because it would trigger forbidden CI. The combined
Goal stays active until both complete scopes, required generation, post-gate
assurance and authorized delivery are truly complete. Prior state, handoff and
receipts remain recoverable; older proofs/checks remain historical and stale.
