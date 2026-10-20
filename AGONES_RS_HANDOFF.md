# Agones-rs handoff

Generated from AGONES_RS_STATE.json revision 241.
Contract SHA-256: `45ca591cea7071878e54b035fd5eb3d076f2ef94a9245077f9a1500d04cb5bf1`.
Original reference: `6eb4a02588d8e7b581de187f3bc849733a78dcb2`, scope `agones-original-v1`.
Workspace: `agones-rs-dev` at `383afa27c3216905982033391aee9b9a71262d00`.
Native Goal last reported `active`; full migration is **INCOMPLETE**.

Active work: **GS-DEFAULTS-001**, stage `IMPLEMENTING`, disposition `READY`.
Next action: Revalidate this implementation and its source-bound proof/evidence after the CGP + derive-deftly + comprende refactor before resuming new feature development.
Implementation gate: `verification/agones/gates/0ac76d721669204c616086a87015daa6232832f7376e359e20fda8cc8a3c894c.json`.

Read the active source and affected contract appendices only. Baseline metadata is in
`verification/agones/baseline.json`, `scope-v1.json`, `baseline-tree.tsv`, and `baseline-tests.json`.
The census has 490 first-party Go top-level test-like declarations; dynamic cases,
assertions and non-Go test identities remain unresolved. This is no completeness claim.


All 8 unverified `_2` form/location cells remain required architecture work.
Last refactoring decision: AGGRESSIVE_PRE_FEATURE_REFACTOR.
Next review: Continue pre-feature refactoring across the remaining high-repetition controller/configuration boundaries; only resume feature implementation after the refactor stack is validated and merged into agones-rs-dev..
No new macros or general frameworks are prerequisites for the next ordinary feature.

Delivery: `CASE005_CI_PROOF_CAMPAIGN_PREPARED_PARENT_PROOFS_RETAINED` to `agones-rs-dev`; never modify main.
Connected GitHub app has write access; local gh CLI is logged out.
Recover live jobs from the ledger and `verification/agones/runs/*.json` by comparing
the stored process identity with /proc before waiting, attaching or stopping anything.
Initial Cargo network failures are recorded; resolution subsequently succeeded.
Use Cargo.lock, Rust 1.98.1 and locked checks. Never count missing proof tools as success.

Checks and source bindings are in immutable run receipts; orthogonal assurance
counts belong in the feature's evidence manifest. A passed implementation gate
does not mean source extraction, Bolero/Kani proof, integration or delivery passed.
