<a id="m0029"></a>


# M0029 — Examples, source tooling and compatibility inputs


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0029-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0009](0009-source-compatible-clients-and-existing-rust-sdk-reuse.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Port required first-party Go example behavior and retain YAML/JSON/proto examples with exact source-relative names. Preserve example request/response, port, image/configuration and SDK usage contracts.

2. Keep examples on the same production APIs and CGP context composition. Use concise Python-familiar Rust and derive-deftly templates already established in production.

3. Do not run examples or provision clusters. An example copied as a comment is not a translated production body; assets remain distinct from executable behavior.

4. Account for each first-party Go build/report/version/schema/documentation tool. Preserve command flags, file selection, parse/format rules, failure behavior, and deterministic output names.

5. Translate required logic into the literal Rust mirror or bind a precise dependency-backed equivalent. Preserve external-tool boundaries and immutable inputs; do not execute builds, CI, or validation campaigns.

6. Use the shared CGP/derive-deftly architecture for effects and repeated declarations; comprehensions must preserve filename/order/duplicate handling. Generation required for implementation is subject to the strict executable-availability rule.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.




## Architecture, readability and authored outputs


1. Own 89 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


These examples explain reading order and borrowing. Actual APIs must come from the selected pinned dependencies. The owning CGP provider supplies the collections; derive-deftly supplies repeated declarations/projections. Preserve Agones-required ordering and avoid redundant clones or a comprehension around complicated effects.


## Package/module and file instructions


## Package `agones:build/agones-bot` -> module `build::agones-bot`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-63d5e7914c69aa82"></a>


### File `build/agones-bot/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `083f7ab41d1e5f37d6c63c0a4ad9041307a3ae8e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/agones-bot/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3450ea224813d6367f.** [agones/build/agones-bot/main.go, lines 43–57](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/agones-bot/main.go#L43-L57). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2420293593a7ac65210eaf686d6e8f41c9a6ee80f187e43eca3f075cb004f101`.

> 
> func main() {
> 	if err := notifiers.Main(new(githubNotifier)); err != nil {
> 		log.Fatalf("fatal error: %v", err)
> 	}
> }
> 
> type githubNotifier struct {
> 	client          *github.Client
> 	successTemplate *template.Template
> 	failureTemplate *template.Template
> }
> 
> func (g *githubNotifier) SetUp(ctx context.Context, config *notifiers.Config, _ string, sg notifiers.SecretGetter, resolver notifiers.BindingResolver) error {
> 	tokenResource, err := notifiers.FindSecretResourceName(config.Spec.Secrets, tokenSecretName)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 6 lexically identified declarations, including `main` (L44), `githubNotifier` (L50), `SetUp` (L56), `SendNotification` (L85), `successTemplate` (L151), `failureTemplate` (L176). The complete list remains in `agones:build/agones-bot/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/report` -> module `build::report`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-f937aefc873ccfa5"></a>


### File `build/report/report.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ef0a12ad939d9f8ee4d6a42704b7e27845d322d8`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/report/report.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8f6d4a3c1f113d168f.** [agones/build/report/report.go, lines 89–103](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/report/report.go#L89-L103). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4674ce64430f6a19d79fc268ad54abaea4e738aa4ccb508156f4a2c7d2fc9c98`.

> 
> type report struct {
> 	WindowStart string
> 	WindowEnd   string
> 	Flakes      []flake
> 	FlakeCount  int
> 	BuildCount  int
> 	FlakeRatio  float32
> }
> 
> type flake struct {
> 	ID         string
> 	CreateTime string
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 6 lexically identified declarations, including `report` (L90), `flake` (L99), `redirect` (L104), `main` (L108), `newReportTemplate` (L218), `newRedirectTemplate` (L222). The complete list remains in `agones:build/report/report.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/bump-image` -> module `build::scripts::bump-image`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-e2c3ff65084810e8"></a>


### File `build/scripts/bump-image/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `aeb88f8d7d8c14af36ea22174c5c7bf6729632a1`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/bump-image/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-967823cd74f6ef19dc.** [agones/build/scripts/bump-image/main.go, lines 42–56](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/bump-image/main.go#L42-L56). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `616de92d4040cd5e2a80958f3849bc3a405c562b6445b7a5d61db62b1fe8f6b2`.

> 
> func init() {
> 	flag.StringVar(&imageName, "imageName", "", "Image name to update")
> 	flag.StringVar(&version, "version", "", "Version to update")
> 	versionPatternInMakefile = regexp.MustCompile(`version\s*:=\s*\d+\.\d+`)
> }
> 
> func imageNamePrefix(imageName string) string {
> 	// Exceptions list
> 	exceptions := map[string]bool{
> 		"simple-genai-game-server": true,
> 		"simple-game-server":       true,
> 	}
> 
> 	// Use the first two words for exceptions, otherwise use the first word


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 7 lexically identified declarations, including `init` (L43), `imageNamePrefix` (L49), `firstTwoWords` (L66), `main` (L74), `incrementVersion` (L113), `updateFileVersion` (L128), `updateMakefileVersion` (L139). The complete list remains in `agones:build/scripts/bump-image/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/example-version-checker` -> module `build::scripts::example-version-checker`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-654128602154dd7b"></a>


### File `build/scripts/example-version-checker/git.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4fae6ebe3a31e297b0a3219f0c5debe43ce6d2ea`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/example-version-checker/git.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-82fc017f8afca4bd74.** [agones/build/scripts/example-version-checker/git.go, lines 18–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/example-version-checker/git.go#L18-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1be8a79d87a374b41ae901235da82e5ea632a16fdc2ce5b65ae1d5db71b48331`.

> 
> func getFailedFiles() []string {
> 	local, remote := getCommits()
> 	changed := getChangedFilenames(local, remote)
> 	sameVersionNames := getSameVersionExampleNames(local, remote)
> 
> 	return filterFailedFiles(changed, sameVersionNames)
> }
> 
> func filterFailedFiles(filePaths []string, sameVersionNames []string) []string {
> 	failed := make([]string, 0)
> 
> 	for _, filePath := range filePaths {
> 		if filenameInExamples(filePath, sameVersionNames) {
> 			failed = append(failed, filePath)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 18 lexically identified declarations, including `cloneDir` (L14), `rootDir` (L15), `gitUrl` (L16), `targetBranch` (L17), `getFailedFiles` (L19), `filterFailedFiles` (L27), `getChangedFilenames` (L39), `getLocalRepo` (L54), `getHeadCommit` (L63), `getCommits` (L77), `getChanges` (L87), `getCommitTree` (L99), `exampleVersionChanged` (L108), `getVersionFromCommit` (L126), `getFileContents` (L137), `getFileContentsFromTree` (L143), `cloneRemoteRepo` (L163), `getSameVersionExampleNames` (L176). The complete list remains in `agones:build/scripts/example-version-checker/git.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-376d6fe0151d8df2"></a>


### File `build/scripts/example-version-checker/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4a4e87156029099da347ae7e22152180eb190dc6`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/example-version-checker/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b30ef033f0ad1b449b.** [agones/build/scripts/example-version-checker/main.go, lines 6–14](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/example-version-checker/main.go#L6-L14). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7682632cfde1abcfd9f109470e480e58956c9bdfbaf2ef36a4a0e7aaffbf943a`.

> 
> func main() {
> 	log.Println("Checking that modified examples have new versions.")
> 	log.Println()
> 
> 	failed := getFailedFiles()
> 
> 	logReport(failed)
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 1 lexically identified declarations, including `main` (L7). The complete list remains in `agones:build/scripts/example-version-checker/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-a8a278ba007e6230"></a>


### File `build/scripts/example-version-checker/report.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `044b4d253e747a14232b8a72b51e143161de0528`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/example-version-checker/report.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-98bd6c3f56967365db.** [agones/build/scripts/example-version-checker/report.go, lines 8–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/example-version-checker/report.go#L8-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fb57985facbe6fabe45c9814a9c1867319a57483bef16dea76e1a1ddd528235c`.

> 
> func logReport(failedFileNames []string) {
> 	if len(failedFileNames) != 0 {
> 		grouped := groupByExampleName(failedFileNames)
> 		reportFailedFiles(grouped)
> 	}
> 
> 	log.Println()
> 	log.Println("The check for version changes succeeded.")
> }
> 
> func reportFailedFiles(groupedPaths map[string][]string) {
> 	log.Println()
> 	log.Println("The below examples were modified, without increasing the version in its Makefile.")
> 	log.Println()


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 5 lexically identified declarations, including `logReport` (L9), `reportFailedFiles` (L19), `groupByExampleName` (L36), `splitPath` (L46), `appendToGroup` (L56). The complete list remains in `agones:build/scripts/example-version-checker/report.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-31f527fa3d624ed4"></a>


### File `build/scripts/example-version-checker/version.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8847ecada8f941eefd5f3ff6b2e49159224a31e0`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/example-version-checker/version.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-31fb446877c70b5a9f.** [agones/build/scripts/example-version-checker/version.go, lines 18–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/example-version-checker/version.go#L18-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ad5b5631e76822eb081287f8bc9e2626c8c25daf9768247f5f51848ee9869e16`.

> 
> func dirIsExample(dirName string) bool {
> 	makefileName := fmt.Sprintf("%s/Makefile", dirName)
> 	if _, err := os.Stat(makefileName); err == nil {
> 		return true
> 	} else {
> 		return false
> 	}
> }
> 
> func getAllExampleNames() []string {
> 	dirNames := make([]string, 0)
> 
> 	baseDir := fmt.Sprintf("%s%s", rootDir, examplesDir)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 10 lexically identified declarations, including `examplesDir` (L14), `excludedPatterns` (L16), `excludedExamples` (L17), `dirIsExample` (L19), `getAllExampleNames` (L28), `getVersionFromMakefile` (L51), `lineContainsVersion` (L67), `getVersionFromLine` (L71), `filenameIsIrrelevant` (L83), `filenameInExamples` (L103). The complete list remains in `agones:build/scripts/example-version-checker/version.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/feature-shortcode-update` -> module `build::scripts::feature-shortcode-update`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-20d615b109d39903"></a>


### File `build/scripts/feature-shortcode-update/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `833babfdd2046d1beb816f853ca1c32e9a859106`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/feature-shortcode-update/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2dc23821c7152ab71f.** [agones/build/scripts/feature-shortcode-update/main.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/feature-shortcode-update/main.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1210892dad5d9c558e52ec74d67cb282a762677d325553c947e3324f9c1e0a81`.

> 
> func main() {
> 	dirPath := "site/content/en/docs"
> 
> 	version := flag.String("version", "", "Expiry version to remove")
> 	flag.Parse()
> 
> 	// Check if the version is provided
> 	if *version == "" {
> 		log.Fatal("Version not specified. Please provide a value for the -version flag in CLI.")
> 	}
> 
> 	modifiedFiles := 0
> 
> 	err := filepath.WalkDir(dirPath, func(filePath string, d os.DirEntry, err error) error {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 2 lexically identified declarations, including `main` (L27), `removeBlocks` (L101). The complete list remains in `agones:build/scripts/feature-shortcode-update/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/k8s-export-openapi` -> module `build::scripts::k8s-export-openapi`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-1fc380ee744e1d06"></a>


### File `build/scripts/k8s-export-openapi/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2192d7649e4145504ce1059b1c45a331aa20911a`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/k8s-export-openapi/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5064c1c5d24a9225a6.** [agones/build/scripts/k8s-export-openapi/main.go, lines 33–47](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/k8s-export-openapi/main.go#L33-L47). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ebc5187e7c82910d5d12cb8ae094d22aec76c1db530419aed29cfaa77459f092`.

> 
> func main() {
> 	tmpDir := "../../tmp"
> 
> 	if _, err := os.Stat(tmpDir); err == nil {
> 		err = os.RemoveAll(tmpDir)
> 		if err != nil {
> 			log.Fatal(err)
> 		}
> 	}
> 
> 	err := os.MkdirAll(tmpDir, os.ModePerm)
> 	if err != nil {
> 		log.Fatal(err)
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 14 lexically identified declarations, including `main` (L34), `removeFields` (L208), `removeKeysRecursively` (L244), `doExpand` (L260), `searchForRefs` (L307), `makeCreationTimestampNullable` (L333), `modifyIntOrString` (L359), `escapeDoubleBackslashes` (L382), `patchKeys` (L396), `wrapPatchKeys` (L404), `isPatchKeyLine` (L432), `leadingSpaces` (L442), `jsonToHelmYaml` (L446), `modifyJSONFiles` (L481). The complete list remains in `agones:build/scripts/k8s-export-openapi/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/previousversion` -> module `build::scripts::previousversion`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-90415d84a48067fc"></a>


### File `build/scripts/previousversion/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f576a61ec69bd494802a902f4a5b0b048079b4fc`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/previousversion/main.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f1c5e4b0845f499cdd.** [agones/build/scripts/previousversion/main.go, lines 30–44](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/previousversion/main.go#L30-L44). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6000270267d00039105bc0c346aa28b742294688a1601856e098014d9053872b`.

> 
> // gcsLister defines an interface for listing GCS objects, allowing for mocking in tests.
> type gcsLister interface {
> 	List(prefix string) (string, error)
> }
> 
> // gsutilLister is the real implementation of gcsLister that calls the gsutil command.
> type gsutilLister struct{}
> 
> func (g gsutilLister) List(prefix string) (string, error) {
> 	cmd := exec.Command("gsutil", "ls", prefix+"*.tgz")
> 	var out bytes.Buffer
> 	var stderr bytes.Buffer
> 	cmd.Stdout = &out
> 	cmd.Stderr = &stderr


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 6 lexically identified declarations, including `gcsLister` (L32), `gsutilLister` (L37), `List` (L39), `main` (L52), `getPreviousVersion` (L76), `getLatestVersionFromGsutil` (L105). The complete list remains in `agones:build/scripts/previousversion/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/remove-data-proofer-ignore` -> module `build::scripts::remove-data-proofer-ignore`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-6518ecdc17f96104"></a>


### File `build/scripts/remove-data-proofer-ignore/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0246a716af95bf5ae40df69ca97fccb555cb6ebf`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/remove-data-proofer-ignore/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f8ab325c76c6081898.** [agones/build/scripts/remove-data-proofer-ignore/main.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/remove-data-proofer-ignore/main.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `86759d09284fc4d8acc3a1fb47a6231fdecc900b404890553952d88125d07e00`.

> 
> func main() {
> 	// Define a flag to accept the file name
> 	fileName := flag.String("file", "", "Path to the file")
> 	flag.Parse()
> 
> 	if *fileName == "" {
> 		log.Println("Please provide the file name using the -file flag")
> 	}
> 
> 	filePath := "site/content/en/blog/releases/" + *fileName
> 
> 	file, err := os.OpenFile(filePath, os.O_RDWR, 0o644)
> 	if err != nil {
> 		log.Println(err)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 1 lexically identified declarations, including `main` (L25). The complete list remains in `agones:build/scripts/remove-data-proofer-ignore/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/sdk-update-version` -> module `build::scripts::sdk-update-version`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-4932469f61a261a1"></a>


### File `build/scripts/sdk-update-version/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `943c3bd3403ff15ea972da1f7ae8bb0ea4ace9d4`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/sdk-update-version/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bbef59376a976ed0fa.** [agones/build/scripts/sdk-update-version/main.go, lines 32–46](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/sdk-update-version/main.go#L32-L46). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `56b4a93283ff97dff6aaf94f16c5e48ffb785d0ee2b08a9126448ffa41c00718`.

> 
> func init() {
> 	flag.StringVar(&releaseStage, "release-stage", "", "Specify the release stage ('before', 'after', or 'patch')")
> 	flag.StringVar(&version, "version", "", "Specify the initial version")
> }
> 
> func main() {
> 	flag.Parse()
> 
> 	if releaseStage == "" || version == "" {
> 		log.Fatalf("Please provide the release stage ('before', 'after', or 'patch') and the version as command-line arguments")
> 	}
> 
> 	log.Printf("Release Stage: %s", releaseStage)
> 	log.Printf("Version: %s", version)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 8 lexically identified declarations, including `releaseStage` (L28), `version` (L29), `jsonExt` (L31), `init` (L33), `main` (L38), `updateFile` (L75), `incrementVersionAfterRelease` (L129), `incrementPatchVersion` (L143). The complete list remains in `agones:build/scripts/sdk-update-version/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/site-config-update-version` -> module `build::scripts::site-config-update-version`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-3657fd686889120c"></a>


### File `build/scripts/site-config-update-version/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e2048b161312be624828f7300396fff012f2ded5`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/site-config-update-version/main.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-75fbc852b6505c8af1.** [agones/build/scripts/site-config-update-version/main.go, lines 29–43](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/site-config-update-version/main.go#L29-L43). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5a4618980ecccea723aeb99db9d199ff2181272e44ca0a5741bd1071c3a5fbe4`.

> 
> // The Config struct holds two nested structs, DevConfig and ProdConfig, which represent the current and previous version of Kubernetes.
> type Config struct {
> 	DevConfig struct {
> 		DevGKEExampleClusterVersion      string   `toml:"dev_gke_example_cluster_version"`
> 		DevAKSExampleClusterVersion      string   `toml:"dev_aks_example_cluster_version"`
> 		DevEKSExampleClusterVersion      string   `toml:"dev_eks_example_cluster_version"`
> 		DevMinikubeExampleClusterVersion string   `toml:"dev_minikube_example_cluster_version"`
> 		DevK8sAPIVersion                 string   `toml:"dev_k8s_api_version"`
> 		DevSupportedK8s                  []string `toml:"dev_supported_k8s"`
> 	} `toml:"params"`
> 
> 	ProdConfig struct {
> 		K8sAPIVersion                 string   `toml:"k8s_api_version"`
> 		GKEExampleClusterVersion      string   `toml:"gke_example_cluster_version"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 8 lexically identified declarations, including `Config` (L31), `main` (L51), `updateSiteConfig` (L63), `tomlArray` (L116), `updateReleaseValues` (L131), `incrementMinorVersion` (L154), `incrementPatchVersion` (L175), `writeLinesToFile` (L195). The complete list remains in `agones:build/scripts/site-config-update-version/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:build/scripts/update-navbar-version` -> module `build::scripts::update-navbar-version`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-45f40ff89caafcae"></a>


### File `build/scripts/update-navbar-version/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3ce05492035e1980c649894e8be11d7afae1f24b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/build/scripts/update-navbar-version/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3a2f12424ab1fedfd8.** [agones/build/scripts/update-navbar-version/main.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/update-navbar-version/main.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `172c40a96f43441211353fc92fb8548bb66b0a5390c662aa66a7c56e1d428a1b`.

> 
> func removeLastDropdownItem(input string) string {
> 	re := regexp.MustCompile(`(?s)(<div class="dropdown-menu" aria-labelledby="navbarDropdownMenuLink">)(.*?)(</div>)`)
> 	matches := re.FindStringSubmatch(input)
> 
> 	// Determine the indentation before the <div>
> 	divStartPos := strings.Index(input, matches[1])
> 	divIndentation := ""
> 	for i := divStartPos - 1; i >= 0 && (input[i] == ' ' || input[i] == '\t'); i-- {
> 		divIndentation = string(input[i]) + divIndentation
> 	}
> 
> 	// Split links inside the dropdown
> 	links := strings.Split(matches[2], "</a>")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 2 lexically identified declarations, including `removeLastDropdownItem` (L26), `main` (L45). The complete list remains in `agones:build/scripts/update-navbar-version/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples` -> module `examples`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-f5965e3e96edc204"></a>


### File `examples/chainfleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3453381b0e6540aea31ea78fb4cb5f9880991964`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/chainfleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3bb6d457bdff4158c8.** [agones/examples/chainfleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/chainfleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a3724f495edc0b6cfd68e7b5db6df4ade55969e6fc3410795a109b9bb912e02e`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-241a5b4cdeaca3a8"></a>


### File `examples/counterfleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0b904ee4047ebfab7a1f1cfc9bfccd5a5ec6bb0b`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/counterfleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-04b6b1f25e44c62a6f.** [agones/examples/counterfleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/counterfleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d7bced023eb10ffeec24be687219effa3d7f1439c20a8151ad66437c5bf9eb00`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-3ac523af7af9bbaa"></a>


### File `examples/fleet.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a1b3aa4e53acde087818fbc2aa219c53677e56b6`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/fleet.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4e8d3777d11e6770f7.** [agones/examples/fleet.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/fleet.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `37df1d06980afe0dc876c24a42cf9bc8d7ae7b3ca4b1f13d6e424f5b1a324208`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-dd26a5c958161fff"></a>


### File `examples/fleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `342fcb2f1a2121152c85077e8984a24ddbb62980`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/fleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8f16dae35117be1a1b.** [agones/examples/fleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/fleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `dd34ffca1328b9a919df093f3a8196b8663bccff1178bfc5f2ca35ff9e478503`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-0bba574ff7ffd9df"></a>


### File `examples/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d27f2423830da977e7f6948bb3ff747311920020`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-04d27a84e3c42f9515.** [agones/examples/gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `20a9fb1d374c4705af3d819e901a93f12f765040a7e290599164850ebec8d44a`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-a3dad251f43a4435"></a>


### File `examples/gameserverallocation-deprecated.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3444645bd2647b89aa972c977bbcb4e7aa711e7f`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/gameserverallocation-deprecated.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-178a29bca85e11127a.** [agones/examples/gameserverallocation-deprecated.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/gameserverallocation-deprecated.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `95396a05f276279ad5543282b1799a4417fb40d6c13bc1dd020de98ff8f161a3`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-506295dc4536b0e7"></a>


### File `examples/gameserverallocation.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6f6bdc3649c31f912dc65613b85b09462315b26d`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/gameserverallocation.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c26eedfc478c737736.** [agones/examples/gameserverallocation.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/gameserverallocation.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e4115988b564c003c01f7544e5e74414495eda67be533f0e8fe828f709c65d75`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-674551d4e22650c6"></a>


### File `examples/listfleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fa0f82e22141e1eb4b2028402e234e00838022f2`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/listfleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9391e1647f030f695c.** [agones/examples/listfleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/listfleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d813620c2c921af98d7bfc4ff77d36cf774b31fac7bb2747eb9738ff41ddba43`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-d19502e584d39103"></a>


### File `examples/schedulefleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `038a4bbd1373334bf4521658ec0f1102678e6b58`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/schedulefleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2179b9021858c405ae.** [agones/examples/schedulefleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/schedulefleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a5feae2ef8a49ac0089861aca764efc880f02d7cd6df7434cf0a94a8a593e252`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-ae3ba19d9b5c186d"></a>


### File `examples/wasmfleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2ddc5ecf7147bab761c329e1be7b34eebbe0561e`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/wasmfleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-cf04ea2097006b3382.** [agones/examples/wasmfleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/wasmfleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `deeb23a6d1ed58730e7d44a02146d5372139b18d147218b6a5ef3802384a5fef`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b48c24ba30a29433"></a>


### File `examples/webhookfleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `68307f2ad8f5d63cb0d7c28d28e841b4067c812f`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/webhookfleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c6c283cc6c90822991.** [agones/examples/webhookfleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/webhookfleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `44664a001719efc99dce2d0e210716f25020df9f37f7124e102a39465fa6e7a2`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-07d143833f8856db"></a>


### File `examples/webhookfleetautoscalertls.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8050d4677b6ec56f40180be29256fcb72bb68e46`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/webhookfleetautoscalertls.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1dc664e3ae6ce6fe84.** [agones/examples/webhookfleetautoscalertls.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/webhookfleetautoscalertls.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9b81f5dc92e0c3281265104a31d6724577e983c3df252a035ca57a82ce018092`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/allocation-endpoint` -> module `examples::allocation-endpoint`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-ba1834922d7008e9"></a>


### File `examples/allocation-endpoint/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f161d9ef1ba6c8f2a77ab7831d14b3905dcd5e4f`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/allocation-endpoint/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9ac910cf8f7a0e097d.** [agones/examples/allocation-endpoint/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/allocation-endpoint/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c3e6ffb9bdc72a11cb665027a504ec74ee158e65fcb4ef58a6e13601570db4e5`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-4fe1bfcb7b751168"></a>


### File `examples/allocation-endpoint/patch-agones-allocator.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `197c22998edfe1e37b2a3c0130b83a9b6ae7d5d5`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/allocation-endpoint/patch-agones-allocator.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-344b5136a772b42c47.** [agones/examples/allocation-endpoint/patch-agones-allocator.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/allocation-endpoint/patch-agones-allocator.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `59a8764271412993aa4505260b2452311457897f75aa83f3b6398c975d9d8e6f`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> spec:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/allocation-endpoint/client` -> module `examples::allocation-endpoint::client`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-dee534d26d3e2529"></a>


### File `examples/allocation-endpoint/client/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `33b38d901e8ec812d9c253415db68de78dbe9077`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/allocation-endpoint/client/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e8578cbd4595b48007.** [agones/examples/allocation-endpoint/client/main.go, lines 52–66](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/allocation-endpoint/client/main.go#L52-L66). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `cb43b2b895962d0534bdd143ab97d3bec2d6ae928b4f51c10d091993fae22e30`.

> 
> func main() {
> 	flag.Parse()
> 
> 	logger.Printf("Running %d times averaged across %d allocation calls per %d clients\n", *runs, *perClientCalls, *clientCount)
> 
> 	var errorCalls, successCalls int64
> 	var successLatency float64
> 	errorGroup := make(map[string]int64)
> 	startRun := time.Now()
> 	waitBetweenRunsInSec := time.Second * 3
> 
> 	audience := fmt.Sprintf("https://%s", *serverAddr)
> 	ts, err := idtoken.NewTokenSource(context.Background(), audience, idtoken.WithCredentialsJSON([]byte(saKey)))
> 	if err != nil {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 5 lexically identified declarations, including `port` (L38), `saKey` (L51), `main` (L53), `report` (L126), `allocate` (L146). The complete list remains in `agones:examples/allocation-endpoint/client/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


10. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-230944c90a49ce14"></a>


### File `examples/allocation-endpoint/client/sa_key.json`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5f93025e365652eeb885be833b421e7a5f57c750`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/allocation-endpoint/client/sa_key.json`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7df4e7698f7b78ad29.** [agones/examples/allocation-endpoint/client/sa_key.json, lines 1–1](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/allocation-endpoint/client/sa_key.json#L1-L1). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6c4bc0dab8a4066d24c57a50547cffe17b536527de96c3fc6dcba8fce974a380`.

> // Put a service account key for authorized_members here


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/allocation-endpoint/server` -> module `examples::allocation-endpoint::server`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-66e76feafec9ed51"></a>


### File `examples/allocation-endpoint/server/clusterselector.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `368e0d5d6c8d4849c6c66cefc9d37a1c1847e0f0`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/allocation-endpoint/server/clusterselector.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-7ffc8fe3cc21cb1560.** [agones/examples/allocation-endpoint/server/clusterselector.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/allocation-endpoint/server/clusterselector.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `78ef43729fa890d6244ade404f1b79160a59e52433ae9fd1a54dcb22e75e0fb3`.

> 
> // A ClusterInfo contains the game server cluster info.
> type ClusterInfo struct {
> 	Name             string `json:"name"`
> 	Endpoint         string `json:"endpoint"`
> 	Namespace        string `json:"namespace"`
> 	AllocationWeight int    `json:"allocation_weight"`
> 	AllocationRate   float64
> }
> 
> // ClusterConns is a connection pool of a cluster.
> type ClusterConns struct {
> 	conns      []*grpc.ClientConn
> 	createdTSs []time.Time
> 	m          sync.RWMutex


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 6 lexically identified declarations, including `ClusterInfo` (L28), `ClusterConns` (L37), `NewClusterConns` (L44), `Get` (L53), `Set` (L63), `selectCluster` (L75). The complete list remains in `agones:examples/allocation-endpoint/server/clusterselector.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b01e9ca49ce59aa3"></a>


### File `examples/allocation-endpoint/server/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `51e8e0692317bad6482b97f7b28eab10a282aa36`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/allocation-endpoint/server/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a5d826f9797811cf95.** [agones/examples/allocation-endpoint/server/main.go, lines 66–80](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/allocation-endpoint/server/main.go#L66-L80). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f8ae07a30918d0b877033f8d8b13537482c343ef1dcf385a68f81af105d08942`.

> 
> type allocationEndpointService struct {
> 	pb.AllocationServiceServer
> }
> 
> type failedConnectCluster struct {
> 	failedTime time.Time
> 	count      int
> }
> 
> func main() {
> 	log.Info("Starting allocation proxy server...")
> 
> 	readEnvVariables()


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 11 lexically identified declarations, including `allocationEndpointService` (L67), `failedConnectCluster` (L71), `main` (L76), `readEnvVariables` (L105), `extractClusterInfo` (L144), `allocationRate` (L155), `getConnection` (L191), `Allocate` (L219), `updateFailed` (L259), `connectToAgonesCluster` (L272), `allocateFromAgones` (L281). The complete list remains in `agones:examples/allocation-endpoint/server/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


10. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/allocator-client` -> module `examples::allocator-client`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-80888c03135387a8"></a>


### File `examples/allocator-client/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5721372e06c76dd3ec21714af537ed923feb3402`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/allocator-client/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0792e30eb4f8673725.** [agones/examples/allocator-client/main.go, lines 32–46](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/allocator-client/main.go#L32-L46). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9f01417d6b4203a5aa2298cc5ba28038141684df67b69690dac0f549c763eefa`.

> 
> func main() {
> 	keyFile := flag.String("key", "missing key", "the private key file for the client certificate in PEM format")
> 	certFile := flag.String("cert", "missing cert", "the public key file for the client certificate in PEM format")
> 	cacertFile := flag.String("cacert", "missing cacert", "the CA cert file for server signing certificate in PEM format")
> 	externalIP := flag.String("ip", "missing external IP", "the external IP for allocator server")
> 	port := flag.String("port", "443", "the port for allocator server")
> 	namespace := flag.String("namespace", "default", "the game server kubernetes namespace")
> 	multicluster := flag.Bool("multicluster", false, "set to true to enable the multi-cluster allocation")
> 
> 	flag.Parse()
> 
> 	endpoint := *externalIP + ":" + *port
> 	cert, err := os.ReadFile(*certFile)
> 	if err != nil {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 2 lexically identified declarations, including `main` (L33), `createRemoteClusterDialOption` (L84). The complete list remains in `agones:examples/allocator-client/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go panic/recover/Goexit values, stack rendering and runtime failure behavior. RUST_NATIVE: Rust Result/error policy and owned-task failure handling without a Go runtime. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/autoscaler-wasm` -> module `examples::autoscaler-wasm`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-4bb94d9327aeca48"></a>


### File `examples/autoscaler-wasm/autoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `477fe8b0a1d351a3f0a908e2c69900d5aeb485e2`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/autoscaler-wasm/autoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6922db3107c3f77b5b.** [agones/examples/autoscaler-wasm/autoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/autoscaler-wasm/autoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `023857dd64cf7a2949a6fca17ef7d557d38c122fec598342cd70d936894dcf5e`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-38e4ed454c65c44c"></a>


### File `examples/autoscaler-wasm/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `335977e830af5b053bda3a2bf40aa518abc7f977`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/autoscaler-wasm/main.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-1a04467fa7c16c452a.** [agones/examples/autoscaler-wasm/main.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/autoscaler-wasm/main.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `62b0af7017583e469160e188e04a6cd7be7b558c93a57d17f6ab01838808d7f7`.

> 
> // Scale makes sure there is a buffer of replicas available for the fleet.
> //
> //go:wasmexport scale
> func Scale() int32 {
> 	var review FleetAutoscaleReview
> 	err := pdk.InputJSON(&review)
> 	if err != nil {
> 		pdk.SetErrorString(fmt.Sprintf("Failed to decode FleetAutoscaleReview: %v", err))
> 		return 1
> 	}
> 
> 	if review.Request == nil {
> 		pdk.SetErrorString("FleetAutoscaleReview Request is nil")
> 		return 1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 5 lexically identified declarations, including `DEFAULT_BUFFER_SIZE` (L24), `Scale` (L29), `ScaleNone` (L82), `ScaleEmpty` (L114), `main` (L119). The complete list remains in `agones:examples/autoscaler-wasm/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-9adffae593d9d525"></a>


### File `examples/autoscaler-wasm/model.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6ab5eeb76aa58cf504aa47bac98ccbb6a1171271`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/autoscaler-wasm/model.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c148575e5f65e7d58e.** [agones/examples/autoscaler-wasm/model.go, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/autoscaler-wasm/model.go#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d4c38cdfc6992600a813733166f5d98795541d3310533119498232f3a8988dc2`.

> */
> 
> // AggregatedCounterStatus stores total and allocated Counter tracking values
> type AggregatedCounterStatus struct {
> 	AllocatedCount    int64 `json:"allocatedCount"`
> 	AllocatedCapacity int64 `json:"allocatedCapacity"`
> 	Count             int64 `json:"count"`
> 	Capacity          int64 `json:"capacity"`
> }
> 
> // AggregatedListStatus stores total and allocated List tracking values
> type AggregatedListStatus struct {
> 	AllocatedCount    int64 `json:"allocatedCount"`
> 	AllocatedCapacity int64 `json:"allocatedCapacity"`
> 	Count             int64 `json:"count"`


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 7 lexically identified declarations, including `AggregatedCounterStatus` (L23), `AggregatedListStatus` (L31), `FleetStatus` (L39), `FleetAutoscalerPolicyType` (L61), `FleetAutoscaleRequest` (L64), `FleetAutoscaleResponse` (L83), `FleetAutoscaleReview` (L95). The complete list remains in `agones:examples/autoscaler-wasm/model.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/autoscaler-webhook` -> module `examples::autoscaler-webhook`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-5dd1d5ea631f86e9"></a>


### File `examples/autoscaler-webhook/autoscaler-service-tls.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `475a1fbe413f5a7e826f81d478e29e86784c0b10`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/autoscaler-webhook/autoscaler-service-tls.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-21c14aea81964b8d73.** [agones/examples/autoscaler-webhook/autoscaler-service-tls.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/autoscaler-webhook/autoscaler-service-tls.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `76273851526664904aaea42a7c6ea593abc7131a165211dc556aaef216d7ff8f`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-3d4e49a3f86deaf7"></a>


### File `examples/autoscaler-webhook/autoscaler-service.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `006fe0128101f5ee90fd6f8b150a646f4346819c`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/autoscaler-webhook/autoscaler-service.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-88dfe2d3f2b4073c7c.** [agones/examples/autoscaler-webhook/autoscaler-service.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/autoscaler-webhook/autoscaler-service.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `df4b57693c648cb3c90bcebaaeec62eb235b6fd0281ee4002e1b23697bc402ea`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-9124683bb624933f"></a>


### File `examples/autoscaler-webhook/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0cc2ec72505a63de83f60d9e47c007258a8a4c88`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/autoscaler-webhook/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ccb56d61a6e7ccac0c.** [agones/examples/autoscaler-webhook/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/autoscaler-webhook/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8d298a4e38b44f0a757536e56bd186aae9e63464760d8961de7c321f59ccd629`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-48923ab5e7b8ae29"></a>


### File `examples/autoscaler-webhook/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a3363c15562819d016383a9fb7086c1e4856c1ea`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/autoscaler-webhook/main.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0da5027580192826cd.** [agones/examples/autoscaler-webhook/main.go, lines 44–58](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/autoscaler-webhook/main.go#L44-L58). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8b42bbec157bba4310918add51d44c28b4480082a09c8edc8ae9ca14777d9055`.

> 
> // Get all parameters from ENV variables
> // Extra check is performed not to fall into the infinite loop:
> // replicaDownTrigger < replicaUpperThreshold/scaleFactor
> func getEnvVariables() {
> 	if ep := os.Getenv("SCALE_FACTOR"); ep != "" {
> 		factor, err := strconv.ParseFloat(ep, 64)
> 		if err != nil {
> 			logger.WithError(err).Fatal("Could not parse environment SCALE_FACTOR variable")
> 		} else if factor > 1 {
> 			scaleFactor = factor
> 		}
> 	}
> 
> 	if ep := os.Getenv("REPLICA_UPSCALE_TRIGGER"); ep != "" {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 4 lexically identified declarations, including `getEnvVariables` (L48), `main` (L104), `handleHealth` (L133), `handleAutoscale` (L144). The complete list remains in `agones:examples/autoscaler-webhook/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/cpp-simple` -> module `examples::cpp-simple`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-4a44888f7b2b3d2d"></a>


### File `examples/cpp-simple/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6feb17e012153f6a0f5dcf8388d40247c104cd47`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/cpp-simple/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1999f0069df8062689.** [agones/examples/cpp-simple/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/cpp-simple/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6639564f2475a17bd9cc2f785c3d6898f4a7cef77f31764aff9da08d0e1dff12`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-757c01412bc49971"></a>


### File `examples/cpp-simple/fleet.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c36d80a5aec76c0132e1eee2a97bdc7edb289817`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/cpp-simple/fleet.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7dcaa2508a1a8f34e3.** [agones/examples/cpp-simple/fleet.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/cpp-simple/fleet.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `95b7d084dc28770d73c61019d1a9c313229c2abbc8a59bb3ded57f7d247e104b`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b10b68bf040d6ae8"></a>


### File `examples/cpp-simple/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6ff93123e42488c256b6f5cb58c398ab19e12027`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/cpp-simple/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f6ad075ec8736be6e4.** [agones/examples/cpp-simple/gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/cpp-simple/gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `49e1b43ee682051749bcf0ff6098ebf63f6d4cb66ec39a8f73c461a9f9d62ece`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/crd-client` -> module `examples::crd-client`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-53c93bccca3621bb"></a>


### File `examples/crd-client/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `10dc2cd8e79e2b12ab0943a5159f544dda2a4ae3`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/crd-client/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c1b17bfa2ba4705d17.** [agones/examples/crd-client/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/crd-client/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0c080c1b49af2c9927a9c67809507167e35005e7b1f6e6f74db83ca26733087e`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-244891315463f52f"></a>


### File `examples/crd-client/create-gs.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e52bb50723b723eaf65610f1f371bdde239b53c2`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/crd-client/create-gs.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a981b4e33811ca6798.** [agones/examples/crd-client/create-gs.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/crd-client/create-gs.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `04d13729f7fba6950ca1af7cfddc4ae708146a3935103f4cd964811ce9aad046`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: batch/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-8c23747fb80f1d3e"></a>


### File `examples/crd-client/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b2bef841e6b963465497130eda72ac2db556bbf4`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/crd-client/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-db38dac51da3e615d2.** [agones/examples/crd-client/main.go, lines 42–56](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/crd-client/main.go#L42-L56). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6fc2ddba780783e24cdcc21efa6e441e99842e7e77d2ce4479740d7d162bac70`.

> 
> func main() {
> 	viper.AllowEmptyEnv(true)
> 	viper.SetEnvKeyReplacer(strings.NewReplacer("-", "_"))
> 
> 	pflag.String(gameServerImage, "", "The Address to bind the server grpcPort to. Defaults to 'localhost'")
> 	viper.SetDefault(gameServerImage, "")
> 	runtime.Must(viper.BindEnv(gameServerImage))
> 
> 	pflag.Bool(isHelmTest, false,
> 		"Is Helm test - defines whether GameServer should be shut down at the end of the test or not. Defaults to false")
> 	viper.SetDefault(isHelmTest, false)
> 	runtime.Must(viper.BindEnv(isHelmTest))
> 
> 	pflag.String(gameserversNamespace, defaultNs, "Namespace where GameServers are created. Defaults to default")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 1 lexically identified declarations, including `main` (L43). The complete list remains in `agones:examples/crd-client/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/custom-controller` -> module `examples::custom-controller`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-f13e123819d08578"></a>


### File `examples/custom-controller/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `10dc2cd8e79e2b12ab0943a5159f544dda2a4ae3`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/custom-controller/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2cb80881ec8d06c53e.** [agones/examples/custom-controller/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/custom-controller/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0c080c1b49af2c9927a9c67809507167e35005e7b1f6e6f74db83ca26733087e`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-4633b3d13fb33e16"></a>


### File `examples/custom-controller/deployment.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `054f1449d69f421b1f794a53f35add25ca56563d`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/custom-controller/deployment.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7aa520c79f504ef01c.** [agones/examples/custom-controller/deployment.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/custom-controller/deployment.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5d085ea1b43399f194ae917459d18400c49c224bacb02e37c6cc556f7366bb58`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-081e60ddb7861aef"></a>


### File `examples/custom-controller/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b8261f68097f445c55e3b5beed5fe575e2b67b26`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/custom-controller/main.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f93615c7c653a2f5ad.** [agones/examples/custom-controller/main.go, lines 31–45](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/custom-controller/main.go#L31-L45). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fe4894dc3ed5d2f7c30b8fde2d7424538a6f041b19677ba26c55b5e5c94adf05`.

> 
> // GameServerReconciler reconciles a GameServer object
> type GameServerReconciler struct {
> 	client.Client
> 	Log logr.Logger
> }
> 
> // Reconcile is part of the main kubernetes reconciliation loop which aims to
> // move the current state of the cluster closer to the desired state.
> func (r *GameServerReconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
> 	r.Log.Info("Entering Reconcile method.")
> 	gameServer := &agonesv1.GameServer{}
> 	err := r.Get(ctx, req.NamespacedName, gameServer)
> 	if err != nil {
> 		if errors.IsNotFound(err) {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 3 lexically identified declarations, including `GameServerReconciler` (L33), `Reconcile` (L40), `main` (L77). The complete list remains in `agones:examples/custom-controller/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/nodejs-simple` -> module `examples::nodejs-simple`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-885e4e864d253944"></a>


### File `examples/nodejs-simple/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1939470c12e4627c4a6b8070f40ade7fed2d46a1`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/nodejs-simple/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7c273ed56add3d69cb.** [agones/examples/nodejs-simple/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/nodejs-simple/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0ba31af2e30b6dc56d83846134e9d04df30d1fe97621dc8c6f9f9ff68f65ab2a`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-f4a80386cbf45988"></a>


### File `examples/nodejs-simple/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `faba66e948b1ef25b60b7e4918baec260efc2870`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/nodejs-simple/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7ae02f813a845ad2d3.** [agones/examples/nodejs-simple/gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/nodejs-simple/gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e94e82b1ae23079cd83add0b258f2e0a32244978fe937a0316301a96f5588225`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-860193ad292682f7"></a>


### File `examples/nodejs-simple/package-lock.json`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a15c9374ad75b965b07aa1cdb5ffa1a5de215883`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/nodejs-simple/package-lock.json`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-633253df262202d477.** [agones/examples/nodejs-simple/package-lock.json, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/nodejs-simple/package-lock.json#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `136c4d18e6a3cac9ae6f83e0c6d41460feaabfc0d249148263c6ae7f3d300af3`.

> {
>   "name": "nodejs-simple",
>   "lockfileVersion": 2,
>   "requires": true,
>   "packages": {
>     "": {
>       "dependencies": {
>         "@google-cloud/agones-sdk": "file:../../sdks/nodejs"
>       }
>     },
>     "../../sdks/nodejs": {
>       "name": "@google-cloud/agones-sdk",
>       "version": "1.41.0-dev",
>       "license": "Apache-2.0",
>       "dependencies": {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-75a0f1d101518dc4"></a>


### File `examples/nodejs-simple/package.json`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `158cac36046955a7d9ec77939f591535ce660933`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/nodejs-simple/package.json`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d4e5f8ebea5d86c5ed.** [agones/examples/nodejs-simple/package.json, lines 1–9](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/nodejs-simple/package.json#L1-L9). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `48d2142334a23d47a5d96d242ae6a7bb7f7011c38f0e5e28c5eed875759ead6b`.

> {
>   "dependencies": {
>     "@google-cloud/agones-sdk": "file:../../sdks/nodejs"
>   },
>   "description": "replace @google-cloud/agones-sdk dependency with the latest version once published",
>   "scripts": {
>     "start": "node src/index.js"
>   }
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/rust-simple` -> module `examples::rust-simple`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-b0535d6f8375ed66"></a>


### File `examples/rust-simple/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5a8c82321f650caed5b4bca88c04fd1f4b4afb97`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/rust-simple/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-50ffeabb222322d4e5.** [agones/examples/rust-simple/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/rust-simple/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ee42791a1374170a88ea4902121b0b8bec9e7225172452df53cb619d87c13f7a`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-8df29fbcc9729add"></a>


### File `examples/rust-simple/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `23890550354302f221a5fdc91a877ce47cdf4931`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/rust-simple/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5ef08ffee7990e25b8.** [agones/examples/rust-simple/gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/rust-simple/gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c8598e984bda2ec67ea010b6b882a38248bce379869a0ac7b12f6883d5b3fa44`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/simple-game-server` -> module `examples::simple-game-server`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-c360316696885f04"></a>


### File `examples/simple-game-server/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3952acbe324feab8640bd7c5024b8b9a9e856b73`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9c25f35dd3c5db6a4e.** [agones/examples/simple-game-server/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `042c27b73d2afa61c316ef5f3fec0af2198a69c6415a0df076bbb66a7d151c21`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-7030e03b5984ed1c"></a>


### File `examples/simple-game-server/dev-gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `401cea9b4ff55439f6e6a1db59e438412280671c`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/dev-gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-773e7e733bff222ddd.** [agones/examples/simple-game-server/dev-gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/dev-gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4bebc44ef72f3aa106fcd2404f6a6781bfbe296ec5b6cfea1195d5af38852cf1`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-6991aa026ba61096"></a>


### File `examples/simple-game-server/fleet-distributed.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `64215e5cd59b99f797e8878a4275b5444480fe60`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/fleet-distributed.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fa5655a021b7c2a9ee.** [agones/examples/simple-game-server/fleet-distributed.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/fleet-distributed.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3ad38c0fa372fc6c0c901f6d374ee5c64e3ff19ffcbe8613b202e45951cca2d4`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-1c8b086bbb4ef00a"></a>


### File `examples/simple-game-server/fleet-tcp.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ee0695a8528813d1271496c3a03efff67c7ca0c0`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/fleet-tcp.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a0a349b3afe468f03f.** [agones/examples/simple-game-server/fleet-tcp.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/fleet-tcp.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d396f266e13aa5ca99c95a5550a2b91c676d1805a2f94df83d99fb6e930e12ab`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-4e248b30650b9d76"></a>


### File `examples/simple-game-server/fleet.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2f1148b19b6ce4d0957b63ab6213b113f76420a6`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/fleet.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-cb42e0a6167e8ed719.** [agones/examples/simple-game-server/fleet.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/fleet.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bf8e04c4a9298b1deb6d68b9e30090af1104f79b8b8e552090285794bfabbddf`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-e3690d814deace3c"></a>


### File `examples/simple-game-server/fleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f066f17a3dc6a473a8acdd3df93f6452ddec2d4f`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/fleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-52099c33f0082b4341.** [agones/examples/simple-game-server/fleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/fleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7f49e3d58852917516b2ad6a8a19ade29081ba357acc12d63f39cd9950f16e24`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-b8dbb2efe69624f1"></a>


### File `examples/simple-game-server/gameserver-none.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c3d94b004f0781a07c827596192f7c61685ea513`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/gameserver-none.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5cf45846ff3cb2350d.** [agones/examples/simple-game-server/gameserver-none.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/gameserver-none.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `da7336ce19bc5a19a1870476b04aaebe2b3d63ee7ba53fe17de4cfa0c932f61a`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-269fc58a72c6c0a2"></a>


### File `examples/simple-game-server/gameserver-passthrough.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `51e05f1c01e68185726fe927aa92fb65fec1978d`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/gameserver-passthrough.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-afdae14b251ea45ac2.** [agones/examples/simple-game-server/gameserver-passthrough.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/gameserver-passthrough.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c84c4d6f53ae5fbc3102a02066fe838e32a39444a8f4ad54aa3b63cae04da58c`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-7040569b15913c3a"></a>


### File `examples/simple-game-server/gameserver-windows.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4a3026a7364569f42bb2a9c463ad900a92d511fa`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/gameserver-windows.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a8d71ffed73475df08.** [agones/examples/simple-game-server/gameserver-windows.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/gameserver-windows.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `85a5ccffe11bf2e1c3d35b5f1cd77f6bdc04da5c458ee2bef89a1d29eb4c5078`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-ec7d35b4c6ef2037"></a>


### File `examples/simple-game-server/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8c93f62931a172b63768366ccd3177ac9052efb5`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-53aa2a2b880b0607ce.** [agones/examples/simple-game-server/gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `af0635b4ef5cbf94c9b28bca661f3a2c32fb13830bdbbfe03d8473ff10094f3f`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-eefd8ab1e09a1f3c"></a>


### File `examples/simple-game-server/gameserverallocation.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0a9240a2c2725f44fbe19794a0696e3388bc721a`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-game-server/gameserverallocation.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6a63dcbe2d8ae95819.** [agones/examples/simple-game-server/gameserverallocation.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/gameserverallocation.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a6f287e5646332767d1a4c69a24e1c18b0508152f64b4ae8778f8942baca7e05`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-8ee7b5b3c7abdf87"></a>


### File `examples/simple-game-server/handlers.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ad431d8d84be5a743ee4ca7e62396685451d4855`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/simple-game-server/handlers.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7091e57253aa750ca0.** [agones/examples/simple-game-server/handlers.go, lines 30–44](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/handlers.go#L30-L44). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5ee871b7af9886b183db3a72dd61666830981aa16b2b3df0d0bddb3e6c30d580`.

> 
> type responseHandler func(s *sdk.SDK, parts []string, cancel ...context.CancelFunc) (response string, addACK bool, responseError error)
> 
> var responseMap = map[string]responseHandler{
> 	"EXIT":                 handleExit,
> 	"UNHEALTHY":            handleUnhealthy,
> 	"GAMESERVER":           handleGameServer,
> 	"READY":                handleReady,
> 	"ALLOCATE":             handleAllocate,
> 	"RESERVE":              handleReserve,
> 	"WATCH":                handleWatch,
> 	"LABEL":                handleLabel,
> 	"CRASH":                handleCrash,
> 	"ANNOTATION":           handleAnnotation,
> 	"GET_COUNTER_COUNT":    handleGetCounterCount,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 30 lexically identified declarations, including `responseHandler` (L31), `responseMap` (L33), `handleResponse` (L59), `handleExit` (L76), `handleUnhealthy` (L82), `handleGameServer` (L91), `handleReady` (L108), `handleAllocate` (L118), `handleReserve` (L128), `handleWatch` (L152), `handleLabel` (L167), `handleCrash` (L182), `handleAnnotation` (L194), `handleGetCounterCount` (L210), `handleIncrementCounter` (L229), `handleDecrementCounter` (L252), `handleSetCounterCount` (L275), `handleGetCounterCapacity` (L298), `handleSetCounterCapacity` (L316), `handleGetListCapacity` (L339), `handleSetListCapacity` (L357), `handleListContains` (L380), `handleGetListLength` (L398), `handleGetListValues` (L416). The complete list remains in `agones:examples/simple-game-server/handlers.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-87b16f1ba2e4980c"></a>


### File `examples/simple-game-server/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6b263f4ba5a335b52161610bee268a83fca97068`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/simple-game-server/main.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e3b68b9aee1bef19aa.** [agones/examples/simple-game-server/main.go, lines 34–48](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-game-server/main.go#L34-L48). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `55c5b40bb4d56b0e04ebbdbd0283d6a7ce51d31544174e0bc33514299f81c4c4`.

> 
> // main starts a UDP or TCP server
> func main() {
> 	sigCtx, _ := signals.NewSigKillContext()
> 
> 	port := flag.String("port", "7654", "The port to listen to traffic on")
> 	passthrough := flag.Bool("passthrough", false, "Get listening port from the SDK, rather than use the 'port' value")
> 	readyOnStart := flag.Bool("ready", true, "Mark this GameServer as Ready on startup")
> 	shutdownDelayMin := flag.Int("automaticShutdownDelayMin", 0, "[Deprecated] If greater than zero, automatically shut down the server this many minutes after the server becomes allocated (please use automaticShutdownDelaySec instead)")
> 	shutdownDelaySec := flag.Int("automaticShutdownDelaySec", 0, "If greater than zero, automatically shut down the server this many seconds after the server becomes allocated (cannot be used if automaticShutdownDelayMin is set)")
> 	readyDelaySec := flag.Int("readyDelaySec", 0, "If greater than zero, wait this many seconds each time before marking the game server as ready")
> 	readyIterations := flag.Int("readyIterations", 0, "If greater than zero, return to a ready state this number of times before shutting down")
> 	gracefulTerminationDelaySec := flag.Int("gracefulTerminationDelaySec", 0, "Delay after we've been asked to terminate (by SIGKILL or automaticShutdownDelaySec)")
> 	udp := flag.Bool("udp", true, "Server will listen on UDP")
> 	tcp := flag.Bool("tcp", false, "Server will listen on TCP")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 11 lexically identified declarations, including `main` (L36), `shutdownAfterNAllocations` (L148), `udpListener` (L196), `udpReadWriteLoop` (L207), `udpRespond` (L230), `tcpListener` (L236), `tcpHandleConnection` (L255), `tcpHandleCommand` (L264), `tcpRespond` (L282), `readPacket` (L290), `doHealth` (L301). The complete list remains in `agones:examples/simple-game-server/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/simple-genai-server` -> module `examples::simple-genai-server`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-f107e8f7bd1e82d8"></a>


### File `examples/simple-genai-server/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `081a809c81ff1a5b4d12c20f793a17a824b69b6f`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-genai-server/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0aa4503a30f9ddbd8e.** [agones/examples/simple-genai-server/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-genai-server/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5e00a6ffba13c00561409b084ff42d2bcfa310d850d1eff4fe541e271889af55`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-77dbc8f362cd7b5d"></a>


### File `examples/simple-genai-server/gameserver_autochat.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `cd481ecd128144cea51d2919dc66fddf79e4f080`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-genai-server/gameserver_autochat.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e46bb14f51e17f747b.** [agones/examples/simple-genai-server/gameserver_autochat.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-genai-server/gameserver_autochat.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `acecdf34ca06a8c0bf5e107c2ff073c7a53be9089953b7eccb4ca135dd917277`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-8ce004ff8ab631c7"></a>


### File `examples/simple-genai-server/gameserver_manualchat.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `da9adb288c9deb6c3cf7652b079d89cc8dcbb9e8`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-genai-server/gameserver_manualchat.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e23e1e85887bcdb454.** [agones/examples/simple-genai-server/gameserver_manualchat.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-genai-server/gameserver_manualchat.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3c13e79b39b14cb1b534d1f5f4ab4d3d84ff24df66988160e7abfe6c2d1c2c0d`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-2726705482b79292"></a>


### File `examples/simple-genai-server/gameserver_npcchat.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0f310e28eba4df244a1b3991e88c2bba0987b7d7`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/simple-genai-server/gameserver_npcchat.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a33c765f501e8cc069.** [agones/examples/simple-genai-server/gameserver_npcchat.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-genai-server/gameserver_npcchat.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `08d29f2adc4d20919e637cddd264505976d3b72500cb4308e081bf32bac9bd55`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-21e1b4b241e83333"></a>


### File `examples/simple-genai-server/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `99b224f04039d840a73d90fb11f0b50ff581d493`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/simple-genai-server/main.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4c993af0eee1b334b5.** [agones/examples/simple-genai-server/main.go, lines 41–55](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/simple-genai-server/main.go#L41-L55). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `182957a3f26530cdd7bcc8459a0de4883c72f892718d508eec1897aed236e635`.

> 
> // Main starts a server that serves as an example of how to integrate GenAI endpoints into your dedicated game server.
> func main() {
> 	sigCtx, _ := signals.NewSigKillContext()
> 
> 	port := flag.String("port", "7654", "The port to listen to traffic on")
> 	genAiEndpoint := flag.String("GenAiEndpoint", "", "The full base URL to send API requests to simulate computer (NPC) responses to user input")
> 	genAiContext := flag.String("GenAiContext", "", "Context for the GenAI endpoint")
> 	prompt := flag.String("Prompt", "", "The first prompt for the GenAI endpoint")
> 	simEndpoint := flag.String("SimEndpoint", "", "The full base URL to send API requests to simulate user input")
> 	simContext := flag.String("SimContext", "", "Context for the Sim endpoint")
> 	stopPhrase := flag.String("StopPhrase", "Bye!", "In autonomous chat, if either side sends this, stop after the next turn.")
> 	numChats := flag.Int("NumChats", 1, "Number of back and forth chats between the sim and genAI")
> 	genAiNpc := flag.Bool("GenAiNpc", false, "Set to true if the GenAIEndpoint is the npc-chat-api endpoint")
> 	simNpc := flag.Bool("SimNpc", false, "Set to true if the SimEndpoint is the npc-chat-api endpoint")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 12 lexically identified declarations, including `main` (L43), `initClient` (L183), `connection` (L189), `GenAIRequest` (L201), `NPCRequest` (L208), `NPCResponse` (L215), `Message` (L221), `handleGenAIRequest` (L226), `autonomousChat` (L295), `tcpListener` (L336), `tcpHandleConnection` (L355), `doHealth` (L378). The complete list remains in `agones:examples/simple-genai-server/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


10. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/supertuxkart` -> module `examples::supertuxkart`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-15c3a57b98c35567"></a>


### File `examples/supertuxkart/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2bf25036d5ac4c2de6a12000a9a427fca91874b0`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/supertuxkart/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0431415040999f7be3.** [agones/examples/supertuxkart/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/supertuxkart/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e81cdcbe6ba2632c0f84d780e1492ae203edb55f7d754d6ccd17f073a00a9dcd`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-3b48713d6fd6cd6c"></a>


### File `examples/supertuxkart/fleet.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `390284a27dd4455fa86a1a5427a46625d9f14951`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/supertuxkart/fleet.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a2a30fb6401751aeaf.** [agones/examples/supertuxkart/fleet.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/supertuxkart/fleet.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `27e65e46bfba309360a3054a85f49419fa1271a86a293b3981395940cb6a9731`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-2d5622728e79a628"></a>


### File `examples/supertuxkart/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `61028bed93087449b487bd0880a0c153e7f8cc48`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/supertuxkart/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f96a33e20428c2fbad.** [agones/examples/supertuxkart/gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/supertuxkart/gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b599f6c5f90d339a18d0e35db3a7d3579c3a069d0da3e116447de04bd4390fdf`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-0926bdbeebca7394"></a>


### File `examples/supertuxkart/gameserverallocation.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `bdbe393d6191da5f1668465a4c2bdd23319c65b8`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/supertuxkart/gameserverallocation.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ae0a05ffdb8ac93f82.** [agones/examples/supertuxkart/gameserverallocation.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/supertuxkart/gameserverallocation.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a09c75a995a7864bff66ba612ca5b5f1996b6a6002ed1350ab1b7ff48e4671d9`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: allocation.agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-072ebeefd08b6d26"></a>


### File `examples/supertuxkart/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ed157745c8feab4fbd71fbcbd9b11b323b641479`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/supertuxkart/main.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4c99aa0c966bdf7701.** [agones/examples/supertuxkart/main.go, lines 34–48](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/supertuxkart/main.go#L34-L48). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b5140ed4508c4ba4c1a30b044cd4d658fc85aeb0f3129240b36b94970aa0ad7f`.

> 
> // main intercepts the log file of the SuperTuxKart gameserver and uses it
> // to determine if the game server is ready or not.
> func main() {
> 	log.SetPrefix("[wrapper] ")
> 	input := flag.String("i", "", "the command and arguments to execute the server binary")
> 	flag.Parse()
> 
> 	log.Println("Connecting to Agones with the SDK")
> 	s, err := sdk.NewSDK()
> 	if err != nil {
> 		log.Fatalf("could not connect to SDK: %v", err)
> 	}
> 
> 	log.Println("Starting health checking")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 4 lexically identified declarations, including `logLocation` (L33), `main` (L37), `doHealth` (L120), `handleLogLine` (L131). The complete list remains in `agones:examples/supertuxkart/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go defer execution, panic/unwind ordering and runtime cleanup mechanics. RUST_NATIVE: Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/unity-simple` -> module `examples::unity-simple`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-7017b4237fb43f69"></a>


### File `examples/unity-simple/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3383328773f131d3d8edb2f4ce7c6582cec39d7e`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/unity-simple/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8ba15ce11ea99114e9.** [agones/examples/unity-simple/gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/unity-simple/gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `78e846297bf97e55e959c459ad0e61d36d53770ead6c289cc13af893fe369b83`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> # All Rights Reserved.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/unity-simple/Packages` -> module `examples::unity-simple::Packages`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-2435b14e34fcb8a6"></a>


### File `examples/unity-simple/Packages/manifest.json`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `215430d3e6b1337fee9c4029533a958be347c554`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/unity-simple/Packages/manifest.json`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e8bfbbc83687da9f81.** [agones/examples/unity-simple/Packages/manifest.json, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/unity-simple/Packages/manifest.json#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6d4794ec1254a9ddcfe6063501bac6ed88ef865ac79dbadd508b3574112379b3`.

> {
>   "dependencies": {
>     "com.unity.ads": "2.0.8",
>     "com.unity.analytics": "3.2.2",
>     "com.unity.collab-proxy": "1.2.15",
>     "com.unity.package-manager-ui": "2.0.8",
>     "com.unity.purchasing": "2.0.3",
>     "com.unity.textmeshpro": "1.4.1",
>     "com.unity.modules.ai": "1.0.0",
>     "com.unity.modules.animation": "1.0.0",
>     "com.unity.modules.assetbundle": "1.0.0",
>     "com.unity.modules.audio": "1.0.0",
>     "com.unity.modules.cloth": "1.0.0",
>     "com.unity.modules.director": "1.0.0",
>     "com.unity.modules.imageconversion": "1.0.0",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:examples/xonotic` -> module `examples::xonotic`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-1539ff062e173f1a"></a>


### File `examples/xonotic/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `09cc79e7a3a42b30b45f4741c60df6e8f21b67d9`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/xonotic/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-73a2f5270dbdbe7432.** [agones/examples/xonotic/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/xonotic/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9713615c0ff93aae15572e9f10ee1ad8b418f533a2d095fc967bfb4bdc5bec5f`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-20849e22584ded98"></a>


### File `examples/xonotic/fleet.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6db84ed2dd07365d499833c13f68a3651125c801`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/xonotic/fleet.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fa0aeb8c8e6e1829ef.** [agones/examples/xonotic/fleet.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/xonotic/fleet.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a54369c5a8544f9cb4452c155b7f293fd1ab128dab2cd3cbad173601e5af09d5`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-e945a8285cb20a9a"></a>


### File `examples/xonotic/fleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1ac524a40761b36fa6fbb5a7f2c50d431c4df187`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/xonotic/fleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0bd5eb5dcbcefd0352.** [agones/examples/xonotic/fleetautoscaler.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/xonotic/fleetautoscaler.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `46797d13d44300fbbdaa1e7aa14c890369494001c1b1b1c0bb117e54c5400333`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-1294e34b7d4f5d27"></a>


### File `examples/xonotic/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `783e47f11eb9748d780f361f4527f991a4917558`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/xonotic/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f2ee1b726b6cf7fb98.** [agones/examples/xonotic/gameserver.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/xonotic/gameserver.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e0d84fb54d1fe756e1bfc83fe655daf646f0658da576babcaf85fa43d0820ce1`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-16661cb286ee42af"></a>


### File `examples/xonotic/gameserverallocation.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b5d3280a8e9ee529ef2f9402f4c9f709adf36846`. **Disposition:** `REQUIRED_COMPATIBILITY_EXAMPLE`. **Destination:** `agones-rs/examples/xonotic/gameserverallocation.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-88aa17fd8468c1a200.** [agones/examples/xonotic/gameserverallocation.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/xonotic/gameserverallocation.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1856fe7db8a3c3a706d3e06ebc59839b8a0de6b6e76dd8697e2b2703c64df27a`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> apiVersion: allocation.agones.dev/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-854265586484ca55"></a>


### File `examples/xonotic/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2f2e3a17ae98567e23c84767563b4694db5b09de`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/examples/xonotic/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ffa3db850a0c4d3d6a.** [agones/examples/xonotic/main.go, lines 29–43](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/examples/xonotic/main.go#L29-L43). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8ba284b7d7d924dc0d49052ae2b099c4c8b8528e1f565207e7e4e9dbfd2732ad`.

> 
> type interceptor struct {
> 	forward   io.Writer
> 	intercept func(p []byte)
> }
> 
> // Write will intercept the incoming stream, and forward
> // the contents to its `forward` Writer.
> func (i *interceptor) Write(p []byte) (n int, err error) {
> 	if i.intercept != nil {
> 		i.intercept(p)
> 	}
> 
> 	return i.forward.Write(p)
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 4 lexically identified declarations, including `interceptor` (L30), `Write` (L37), `main` (L47), `doHealth` (L105). The complete list remains in `agones:examples/xonotic/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


9. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


## Package `agones:site` -> module `site`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.


<a id="file-479c46a50e70f146"></a>


### File `site/handler.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c954a87947a5bb07a1e73752aacfff5bfeab50bc`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/site/handler.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b2e07026fcd82087d6.** [agones/site/handler.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/handler.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fb61176391cb51d10841c270e32490a53b9a8ad6c225071fe29f2c91804633a1`.

> 
> type handler struct {
> 	host         string
> 	cacheControl string
> 	paths        pathConfigSet
> }
> 
> type pathConfig struct {
> 	path    string
> 	repo    string
> 	display string
> 	vcs     string
> }
> 
> func newHandler(config []byte) (*handler, error) {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 13 lexically identified declarations, including `handler` (L29), `pathConfig` (L35), `newHandler` (L42), `ServeHTTP` (L96), `serveIndex` (L131), `serveChart` (L138), `Host` (L143), `vanityTmpl` (L151), `pathConfigSet` (L164), `Len` (L166), `Less` (L170), `Swap` (L174), `find` (L178). The complete list remains in `agones:site/handler.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts. RUST_NATIVE: Rust collections, borrowing and explicit domain ordering or duplicate policy. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


7. GO_ONLY_SUPERSEDED: Go machine-word, conversion, overflow, float-printing and ABI behavior. RUST_NATIVE: Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


8. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-5698f1e3a3b093a5"></a>


### File `site/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6e245f3a89abe34b68fe30115c76cd1b97f02727`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/site/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f23a483d5b48ae1f01.** [agones/site/main.go, lines 23–37](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/main.go#L23-L37). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `849d6b0038e9fa50bea6b0da5bb951477384153431c43bf3bb2b2e607766c0fa`.

> 
> func main() {
> 	port := os.Getenv("PORT")
> 	if port == "" {
> 		port = "8080"
> 		log.Printf("Defaulting to port %s", port)
> 	}
> 
> 	var configPath string
> 	switch len(os.Args) {
> 	case 1:
> 		configPath = "vanity.yaml"
> 	case 2:
> 		configPath = os.Args[1]
> 	default:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Account for 2 lexically identified declarations, including `main` (L24), `defaultHost` (L54). The complete list remains in `agones:site/main.go` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.


5. GO_ONLY_SUPERSEDED: Go typed-nil interfaces and nil slice/map/header identity. RUST_NATIVE: Option and native containers, with serde/protobuf presence only where an Agones contract requires it. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


6. GO_ONLY_SUPERSEDED: Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics. RUST_NATIVE: std::fmt, std::error::Error and established Rust logging/serialization facilities. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.


<a id="file-05508694eeef8a28"></a>


### File `site/tools.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `23ea3296bceeb83e97cdbd1561e85197bb1f9b97`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/site/tools.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1bf58a9a6caca27ae7.** [agones/site/tools.go, lines 22–23](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/tools.go#L22-L23). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9a35cb8ec694cae9646b8396a249ac9e4ea039c8b0448dce934c4b0f84b93a81`.

> 	_ "github.com/google/docsy/theme"
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.


**Derived implementation instructions.**


1. **AGONES_REQUIRED:** Agones example requests/resources and required build/report/version/schema/documentation outputs. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.


2. **RUST_NATIVE:** Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.


3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.


4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.


5. GO_ONLY_SUPERSEDED: Go goroutine scheduling, channel/select mechanics and channel representation. RUST_NATIVE: Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.


12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.


13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.


14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

