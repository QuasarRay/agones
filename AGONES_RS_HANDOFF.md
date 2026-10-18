# Agones-rs handoff

Generated from AGONES_RS_STATE.json revision 57.
Contract SHA-256: `45ca591cea7071878e54b035fd5eb3d076f2ef94a9245077f9a1500d04cb5bf1`.
Original reference: `6eb4a02588d8e7b581de187f3bc849733a78dcb2`, scope `agones-original-v1`.
Workspace: `agones-rs-dev` at `6909f07ac5cb08b1af78b251e26ac17342cef933`.
Native Goal remains active; full migration is **INCOMPLETE**.

Active work: **SDK-VERSION-001**, stage `DISCOVERED`, disposition `READY`.
Next action: Verify the renderer checkpoint on existing draft PR 92, then implement the complete source SDK version annotation policy through shared CGP configuration across GameServer, Fleet and actual Rusternetes Pod paths. Gate ordinary behavior before extracting new version properties. Retain all feature-gate formal debt and review its finite-loop diagnosis after the version gate.
Implementation gate: `None`.

Read the active source and affected contract appendices only. Baseline metadata is in
`verification/agones/baseline.json`, `scope-v1.json`, `baseline-tree.tsv`, and `baseline-tests.json`.
The census has 490 first-party Go top-level test-like declarations; dynamic cases,
assertions and non-Go test identities remain unresolved. This is no completeness claim.
Preserved input: `AGONES_RS_PROMPT_AUDIT_RESOLUTION.md` is an earlier prompt audit,
not product implementation evidence; its old revised-prompt hash is historical.

All 8 unverified `_2` form/location cells remain required architecture work.
Last refactoring decision: REUSE_EXISTING.
Next review: Next public API/resource milestone or new repeated CGP/provider/property plumbing, whichever comes first; continue Agones production features while required proof debt remains visible..
No new macros or general frameworks are prerequisites for the next ordinary feature.

Delivery: `LOCAL_ERROR_CHECKPOINT_READY_FOR_PUBLICATION` to `agones-rs-dev`; never modify main.
Connected GitHub app has write access; local gh CLI is logged out.
Recover live jobs from the ledger and `verification/agones/runs/*.json` by comparing
the stored process identity with /proc before waiting, attaching or stopping anything.
Initial Cargo network failures are recorded; resolution subsequently succeeded.
Use Cargo.lock, Rust 1.98.1 and locked checks. Never count missing proof tools as success.

Checks and source bindings are in immutable run receipts; orthogonal assurance
counts belong in the feature's evidence manifest. A passed implementation gate
does not mean source extraction, Bolero/Kani proof, integration or delivery passed.
