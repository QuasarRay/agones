<a id="m0022"></a>


# M0022 — Pinned frontend and self-contained CPG input bundle


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0022-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0020](0020-pass-diff-and-execution-lifecycle.md), [M0018](0018-format-detection-and-native-storage-compatibility.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71f6b7430be0dfa8be.** [genco/README.md, lines 10–22](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L22). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].
> 
> * **String Quoting** — genco knows how to [quote strings]. And can even
>   [interpolate] values *into* the quoted string if it's supported by the
>   language.


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Use the recorded compatible Joern gosrc2cpg/goastgen/CPG/Flatgraph pins. Retain their actual source, configuration and graph format identities; a PATH binary is not compatibility evidence.

2. Author the bundle loader and source/body/configuration census interfaces: original graph, retained source, Go AST/type enrichment, build tags/platforms/generated inputs/cgo boundaries and diagnostics.

3. Resolve every required source file/body to graph nodes or persisted source-bound enrichment. Missing frontend executables or a required new build remain blockers; do not execute an unapproved probe.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.

The complete native CPG graph/schema/build objective remains required. Preserve its functional graph, schema and interchange contracts through Rust. Go source facts may be retained for analysis/provenance; generated Agones must use the same native Rust boundary as handwritten Agones.




## Architecture, readability and authored outputs


1. Own 0 inventoried file entries in this milestone plus the concrete new translation/support interfaces described above.

2. Record typed declarations -> derive-deftly templates -> CGP provider implementations/delegation -> production context -> lifecycle consumer. Identify each mutable state/resource owner and effect boundary.

3. Apply Pythonic readability and idiomatic Rust as equal design criteria. Comprehensions simplify mapping/filtering; explicit small functions clarify effects. Do not add a Python interpreter, dynamic object layer, reflection registry or competing manager.

4. Preserve source-relative directories, exact case/stems and singular/plural forms. Map Go `_test.go` only to `_harness.rs`; keep Scala Test/Tests stems unchanged. Register unconventional names outside the mirror.


**Python-familiar valid Rust example (schematic; not executed):**


```rust
use comprende::c;
use better_comprehension::vector;

let ready_names = c![server.name().to_owned()
                    for server in &servers if server.is_ready()];
let selected_ids = vector![node.id() for node in nodes.iter()
                          if node.has_required_label()];
```


These examples explain reading order and borrowing. Actual APIs must come from the selected pinned dependencies. The owning CGP provider supplies the collections; derive-deftly supplies repeated declarations/projections. Preserve source ordering and avoid redundant clones or a comprehension around complicated effects.


## Package/module and file instructions


This is a required cross-cutting implementation milestone. Its concrete outputs are the stage interfaces/rules/transaction or delivery artifacts specified above. Place new Rust-only machinery under `rust-support/agones` or `rust-support/cpg`, keeping source mirrors unchanged. Bind output ownership in the existing ledger; a design document or empty provider does not complete it.


## Explicit new support/output ownership

These are planned Rust-only support or evidence files, not original-source mirror renames. Reuse an existing equivalent through a recorded path mapping; do not create a competing implementation merely to match this suggested support filename.

### New support/output `rust-support/cpg/src/bundle.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71f6b7430be0dfa8be.** [genco/README.md, lines 10–22](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L22). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].
> 
> * **String Quoting** — genco knows how to [quote strings]. And can even
>   [interpolate] values *into* the quoted string if it's supported by the
>   language.

**Derived file obligation.** Versioned complete frontend bundle reader and source/configuration identities. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/frontend.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71f6b7430be0dfa8be.** [genco/README.md, lines 10–22](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L22). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].
> 
> * **String Quoting** — genco knows how to [quote strings]. And can even
>   [interpolate] values *into* the quoted string if it's supported by the
>   language.

**Derived file obligation.** Pinned gosrc2cpg/goastgen invocation and export/enrichment contracts. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.



## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

## Specific source obligations for this new stage

**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-87cc5205a23d07c9e8.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala, lines 71–89](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala#L71-L89). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dd3fe0ad05f427e94fedcaff66234a64014f868e1fd203fafd9519fe4886fb93`.

>       .addNodeBaseType(
>         name = "AST_NODE",
>         comment = """This is the base type for all nodes of the abstract syntax tree (AST). An AST
>                     |node has a `CODE` and an `ORDER` field. The `CODE` field contains the
>                     |code (verbatim) represented by the AST node. The `ORDER` field contains the
>                     |nodes position among its siblings, encoded as an integer where the left most
>                     |sibling has the position `0`.
>                     |
>                     |AST nodes contain optional `LINE_NUMBER` and `COLUMN_NUMBER` fields. For
>                     |source-based frontends, these fields contain the start line number and
>                     |start column number of the code represented by the node.
>                     |For machine-code-based and bytecode-based frontends, `LINE_NUMBER` contains
>                     |the address at which the code starts while `COLUMN_NUMBER` is undefined.
>                     |
>                     |The optional `OFFSET` and `OFFSET_END` specify the start
>                     |and exclusive end position of the code belonging to a node within the corresponding
>                     |`FILE` nodes `CONTENT` property.
>                     |""".stripMargin
>       )

**Derived stage requirement.** Retain verbatim code, source location and sibling order as identified facts in the source-bound bundle. Preserve additional frontend/type facts explicitly rather than assuming AST code text alone is enough. No dedicated upstream Go-to-Rust translator specification is claimed; this stage is engineered from the quoted source obligations and the user-approved shared contract.
