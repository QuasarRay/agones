# Agones-rs handoff

Generated from AGONES_RS_STATE.json revision 17.
Contract SHA-256: `45ca591cea7071878e54b035fd5eb3d076f2ef94a9245077f9a1500d04cb5bf1`.
Original reference: `6eb4a02588d8e7b581de187f3bc849733a78dcb2`, scope `agones-original-v1`.
Workspace: `agones-rs-dev` at `d88f536cbd36fb2fd9ca75ced6fe23d7f7649762`.
Native Goal remains active; full migration is **INCOMPLETE**.

Active work: **FG-CONFIG-001**, stage `VERIFYING`, disposition `READY`.
Next action: Publish the incremental CI/guidance repair to the existing PR, verify its exact head results, and continue FG-INPUT-001.
Implementation gate: `verification/agones/gates/9bc7d7e7fc872f42369903027b46116bdc3a2fed1d783f66476870a1aa451357.json`.

Read the active source and affected contract appendices only. Baseline metadata is in
`verification/agones/baseline.json`, `scope-v1.json`, `baseline-tree.tsv`, and `baseline-tests.json`.
The census has 490 first-party Go top-level test-like declarations; dynamic cases,
assertions and non-Go test identities remain unresolved. This is no completeness claim.
Preserved input: `AGONES_RS_PROMPT_AUDIT_RESOLUTION.md` is an earlier prompt audit,
not product implementation evidence; its old revised-prompt hash is historical.

All 8 unverified `_2` form/location cells remain required architecture work.
Last refactoring decision: REUSE_EXISTING.
Next review: FG-INPUT-001 implementation gate; compare a small Genco exact-case emitter with existing invariant/macro facilities, and reassess the source-aware derive path as the next narrow cross-crate adoption opportunity..
No new macros or general frameworks are prerequisites for the next ordinary feature.

Delivery: `DRAFT_PR_OPEN_CI_REPAIR` to `agones-rs-dev`; never modify main.
Connected GitHub app has write access; local gh CLI is logged out.
Recover live jobs from the ledger and `verification/agones/runs/*.json` by comparing
the stored process identity with /proc before waiting, attaching or stopping anything.
Initial Cargo network failures are recorded; resolution subsequently succeeded.
Use Cargo.lock, Rust 1.98.1 and locked checks. Never count missing proof tools as success.

Checks and source bindings are in immutable run receipts; orthogonal assurance
counts belong in the feature's evidence manifest. A passed implementation gate
does not mean source extraction, Bolero/Kani proof, integration or delivery passed.
