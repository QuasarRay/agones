# Agones-rs handoff

Generated from AGONES_RS_STATE.json revision 281.
Contract SHA-256: `45ca591cea7071878e54b035fd5eb3d076f2ef94a9245077f9a1500d04cb5bf1`.
Original reference: `6eb4a02588d8e7b581de187f3bc849733a78dcb2`, scope `agones-original-v1`.
Workspace: `agones-rs-dev` at `4523bffd1fc3ae01ecba4918b343cafaa688c2f7`.
Native Goal last reported `active`; full migration is **INCOMPLETE**.

Active work: **GS-VALIDATE-FEATURES-001**, stage `VERIFYING`, disposition `READY`.
Next action: Review a semantics-preserving expected-driven multiset comparison in the existing Genco emitter. Equal lengths plus equal multiplicities for each expected equivalence class preserves ElementsMatch for finite error lists while bounding outer iteration by the literal expected fixture. Preserve full record Eq, duplicates, all original fixtures/actions/assertions/covers/domains/unwind128. Validate equivalence and mutation controls before one distinct attempt; no identical retry.
Implementation gate: `verification/agones/gates/344f995f9464279cf16b6442afe3630de20fa067daa80a65268de552c017f166.json`.

Read the active source and affected contract appendices only. Baseline metadata is in
`verification/agones/baseline.json`, `scope-v1.json`, `baseline-tree.tsv`, and `baseline-tests.json`.
The census has 490 first-party Go top-level test-like declarations; dynamic cases,
assertions and non-Go test identities remain unresolved. This is no completeness claim.


All 8 unverified `_2` form/location cells remain required architecture work.
Last refactoring decision: REUSE_EXISTING.
Next review: After equivalence/mutation controls and one distinct protected source-case result, reassess the remaining actual comparison/production bottleneck. All eight _2cells and full source/architecture/lifecycle/conformance requirements stay open..
No new macros or general frameworks are prerequisites for the next ordinary feature.

Delivery: `PR96_CONCAT4523_PUBLISHED_CI_TERMINAL_ALL12_LOCAL_ARTIFACTS_RETAINED_FIVE_TIMEOUTS` to `agones-rs-dev`; never modify main.
Connected GitHub app has write access; local gh CLI is logged out.
Recover live jobs from the ledger and `verification/agones/runs/*.json` by comparing
the stored process identity with /proc before waiting, attaching or stopping anything.
Initial Cargo network failures are recorded; resolution subsequently succeeded.
Use Cargo.lock, Rust 1.98.1 and locked checks. Never count missing proof tools as success.

Checks and source bindings are in immutable run receipts; orthogonal assurance
counts belong in the feature's evidence manifest. A passed implementation gate
does not mean source extraction, Bolero/Kani proof, integration or delivery passed.
