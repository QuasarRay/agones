# Agones-rs and CPG handoff

Canonical ledger: `AGONES_RS_STATE.json`, revision 322. One native Goal:
`01a17928-8551-7701-8497-5333c680f283`. The current authority is the combined
attachment recorded in `delivery.roadmap_execution`, `roadmap/0000-contract.md`
and `roadmap/alternation-policy.md`. The older standalone generation stop rule
is superseded for this invocation.

Project phase: **IMPLEMENTATION_ONLY**. Global gate: **CLOSED**. Production
implementation and generated output remain incomplete. Verification has not
started under this contract. No builds, tests, probes, proofs or CI are allowed.

M0001 (MANUAL) is **IMPLEMENTED_UNVERIFIED**. Its 13 source-owned rows, four
shared GroupName declarations, Cargo registration, raw build-version transfer,
derive-deftly/CGP configuration wiring and ownership map are in
`verification/agones/roadmap-adoption-M0001.json`. Existing version callers
re-export the one source-mirror implementation. Initialization uses the existing
feature parser and environment effect. Controller/server task lifecycle remains
required in M0027; build/tool consumer behavior remains required in M0031.
No compilation, correctness, compatibility or readiness claim is made.

Next cursor: **M0002 / CPG / READY**. Read its overview and affected sections via
`verification/agones/roadmap/entry-index.tsv`. Complete its pinned CPG tree,
Bazel/sbt/project helpers, source build/tool equivalents and support registration
before returning to MANUAL M0003. Inspect runnable-generator receipts by reading
only; do not build a missing generator or run a compatibility probe.

M0001 cost observation and forecast comparison:
`cost-analysis-comparison/0001-M0001.md`. Raw counters are preserved; credits and
unexposed model settings remain unknown. Shared setup is charged once separately.

Keep working on `agones-rs-dev`; never modify main. Local implementation
checkpoints are allowed. Remote publication/PR/merge remains deferred because it
would trigger forbidden checks. Full acceptance and authorized delivery remain
open. The previous ledger/handoff are retained by digest under
`verification/agones/states/`; previous proof/check records are historical.
