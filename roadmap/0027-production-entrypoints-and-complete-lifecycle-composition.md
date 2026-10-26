<a id="m0027"></a>


# M0027 — Production entrypoints and complete lifecycle composition


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0027-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0023](0023-allocation-processing-and-multi-cluster-contracts.md), [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0013](0013-cloud-hooks-through-the-permitted-rusternetes-boundary.md), [M0021](0021-fleetautoscaler-policies-and-external-decisions.md), [M0015](0015-fleet-reconciliation-and-rollout-policy.md), [M0001](0001-source-mirror-version-and-shared-contracts.md), [M0019](0019-gameserver-and-gameserverset-lifecycle.md), [M0017](0017-telemetry-and-metric-semantics.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0011](0011-port-ownership-and-allocation-policy.md), [M0025](0025-sdk-server-streaming-health-and-mutation-behavior.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-11dfff1da301d4c4cc.** [agones/site/content/en/docs/Advanced/system-diagram.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/system-diagram.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e2b49fd915fa445c1beace0f69c29595277d175b4ec085cf64ebf1c0d2a306e`.

> 
> ![System Diagram](../../../diagrams/system-diagram.dot.png)
> 
> # Agones Control Plane 
> 
> The Agones Control Plane consists of 4 `Deployments`:
> ```
> NAME                READY   UP-TO-DATE   AVAILABLE   AGE
> agones-allocator    3/3     3            3           40d
> agones-controller   2/2     2            2           40d
> agones-extensions   2/2     2            2           40d
> agones-ping         2/2     2            2           40d
> ```


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Implement controller, allocator, extensions, ping, processor and SDK-server command surfaces, flags/environment precedence, startup errors, readiness, logging and shutdown.

2. The entrypoint only constructs the declared production context and invokes the CGP lifecycle consumer. Derive-deftly-generated wiring must include every actual child service/resource; no detached manager bypasses it.

3. Preserve authentication, mTLS, request limits, remote allocation behavior, health endpoints and cancellation/drain deadlines. Do not run any command as a smoke test before the gate.


## Architecture, readability and authored outputs


1. Own 10 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:cmd/allocator` -> module `cmd::allocator`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-11dfff1da301d4c4cc.** [agones/site/content/en/docs/Advanced/system-diagram.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/system-diagram.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e2b49fd915fa445c1beace0f69c29595277d175b4ec085cf64ebf1c0d2a306e`.

> 
> ![System Diagram](../../../diagrams/system-diagram.dot.png)
> 
> # Agones Control Plane 
> 
> The Agones Control Plane consists of 4 `Deployments`:
> ```
> NAME                READY   UP-TO-DATE   AVAILABLE   AGE
> agones-allocator    3/3     3            3           40d
> agones-controller   2/2     2            2           40d
> agones-extensions   2/2     2            2           40d
> agones-ping         2/2     2            2           40d
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-4abdbcf7468816e8"></a>


### File `cmd/allocator/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4a420f16ff7c2cb5a6e19c52a322f263b41aed78`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/allocator/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0f9ca3f6b1343cd476.** [agones/cmd/allocator/main.go, lines 105–119](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/allocator/main.go#L105-L119). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `190e43f3d8364ec13feab11632547c69592c94a5d1833956fbdab4986c6032c7`.

> 
> func parseEnvFlags() config {
> 	viper.SetDefault(httpPortFlag, -1)
> 	viper.SetDefault(grpcPortFlag, -1)
> 	viper.SetDefault(apiServerSustainedQPSFlag, 400)
> 	viper.SetDefault(apiServerBurstQPSFlag, 500)
> 	viper.SetDefault(enablePrometheusMetricsFlag, true)
> 	viper.SetDefault(enableStackdriverMetricsFlag, false)
> 	viper.SetDefault(projectIDFlag, "")
> 	viper.SetDefault(stackdriverLabels, "")
> 	viper.SetDefault(mTLSDisabledFlag, false)
> 	viper.SetDefault(tlsDisabledFlag, false)
> 	viper.SetDefault(remoteAllocationTimeoutFlag, 10*time.Second)
> 	viper.SetDefault(totalRemoteAllocationTimeoutFlag, 30*time.Second)
> 	viper.SetDefault(logLevelFlag, "Info")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 21 lexically identified declarations, including `parseEnvFlags` (L106), `config` (L198), `grpcHandlerFunc` (L223), `main` (L234), `validPort` (L398), `runMux` (L403), `runREST` (L417), `runHTTP` (L426), `runGRPC` (L468), `newProcessorServiceHandler` (L497), `newServiceHandler` (L528), `readTLSCert` (L584), `getMuxServerOptions` (L595), `getGRPCServerOptions` (L615), `getTLSCert` (L647), `verifyClientConnection` (L661), `verifyClientCertificate` (L676), `getClients` (L710), `getCACertPool` (L734), `serviceHandler` (L765), `Allocate` (L785). The full declaration list is recorded under `agones:cmd/allocator/main.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg`, `agones.dev/agones/pkg/allocation/converters`, `agones.dev/agones/pkg/allocation/go`, `agones.dev/agones/pkg/apis/allocation/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/gameserverallocations`, `agones.dev/agones/pkg/gameserverallocations/processor`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/metrics`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/fswatch`, `agones.dev/agones/pkg/util/httpserver`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/signals`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve source panic/recover boundaries; do not use an unsupported panic as a translation placeholder.

7. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

8. Preserve source floating width, narrowing, exceptional values and relevant bits.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-84fa5728b882471e"></a>


### File `cmd/allocator/metrics.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `207b64265bf400aa3028400acb66e6f1c8ce0c71`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/allocator/metrics.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a537ef2b5b4fcf3644.** [agones/cmd/allocator/metrics.go, lines 20–29](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/allocator/metrics.go#L20-L29). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4796e804ecfb00454f34917157df31ce7dad67fdb59f108798a981346fa45127`.

> 
> func init() {
> 	registerMetricViews()
> }
> 
> func registerMetricViews() {
> 	if err := view.Register(ocgrpc.DefaultServerViews...); err != nil {
> 		logger.WithError(err).Error("could not register view")
> 	}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 2 lexically identified declarations, including `init` (L21), `registerMetricViews` (L25). The full declaration list is recorded under `agones:cmd/allocator/metrics.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:cmd/controller` -> module `cmd::controller`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-11dfff1da301d4c4cc.** [agones/site/content/en/docs/Advanced/system-diagram.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/system-diagram.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e2b49fd915fa445c1beace0f69c29595277d175b4ec085cf64ebf1c0d2a306e`.

> 
> ![System Diagram](../../../diagrams/system-diagram.dot.png)
> 
> # Agones Control Plane 
> 
> The Agones Control Plane consists of 4 `Deployments`:
> ```
> NAME                READY   UP-TO-DATE   AVAILABLE   AGE
> agones-allocator    3/3     3            3           40d
> agones-controller   2/2     2            2           40d
> agones-extensions   2/2     2            2           40d
> agones-ping         2/2     2            2           40d
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-d20cee51ccf691f5"></a>


### File `cmd/controller/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4c83c67fe9f9ea3fa9e8e44189a7d607ed479e6f`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/controller/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9dbdbb179fc83216dd.** [agones/cmd/controller/main.go, lines 102–116](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/controller/main.go#L102-L116). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7bfb26ebd6ca8f6c27cd93311ba89fb1860f7598805e3d7e0a602f3d4b3eab55`.

> 
> func setupLogging(logDir string, logSizeLimitMB int) {
> 	logFileName := filepath.Join(logDir, "agones-controller-"+time.Now().Format("20060102_150405")+".log")
> 
> 	const maxLogSizeMB = 100
> 	maxBackups := (logSizeLimitMB - maxLogSizeMB) / maxLogSizeMB
> 	logger.WithField("filename", logFileName).WithField("numbackups", maxBackups).Info("logging to file")
> 	logrus.SetOutput(
> 		io.MultiWriter(
> 			logrus.StandardLogger().Out,
> 			&lumberjack.Logger{
> 				Filename:   logFileName,
> 				MaxSize:    maxLogSizeMB,
> 				MaxBackups: maxBackups,
> 			},


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 13 lexically identified declarations, including `setupLogging` (L103), `main` (L122), `parseEnvFlags` (L247), `parsePortRanges` (L444), `parseSidecarSecurityContext` (L469), `config` (L485), `validate` (L521), `validateResource` (L540), `validatePorts` (L555), `validatePortRange` (L591), `overlaps` (L606), `runner` (L610), `whenLeader` (L614). The full declaration list is recorded under `agones:cmd/controller/main.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/cloudproduct`, `agones.dev/agones/pkg/fleetautoscalers`, `agones.dev/agones/pkg/fleets`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/gameserversets`, `agones.dev/agones/pkg/metrics`, `agones.dev/agones/pkg/portallocator`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/httpserver`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/signals`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

8. Preserve source floating width, narrowing, exceptional values and relevant bits.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-49567286abbbb4c8"></a>


### File `cmd/controller/pprof.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `696f3b914ef8c13149ff3da865c21fd06c9432c4`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/controller/pprof.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e11526f1d438afa3ff.** [agones/cmd/controller/pprof.go, lines 26–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/controller/pprof.go#L26-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2b08cfe6d0e469ea821ea08ec6aba6df0eed94a5d8c272167267421790f113df`.

> 
> func init() {
> 	go func() {
> 		logrus.WithError(http.ListenAndServe(":6060", nil)).Info("Closed pprof server")
> 	}()
> 	logrus.Info("*** PPROF PROFILER STARTED on :6060 ***")
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `init` (L27). The full declaration list is recorded under `agones:cmd/controller/pprof.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:cmd/extensions` -> module `cmd::extensions`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-11dfff1da301d4c4cc.** [agones/site/content/en/docs/Advanced/system-diagram.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/system-diagram.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e2b49fd915fa445c1beace0f69c29595277d175b4ec085cf64ebf1c0d2a306e`.

> 
> ![System Diagram](../../../diagrams/system-diagram.dot.png)
> 
> # Agones Control Plane 
> 
> The Agones Control Plane consists of 4 `Deployments`:
> ```
> NAME                READY   UP-TO-DATE   AVAILABLE   AGE
> agones-allocator    3/3     3            3           40d
> agones-controller   2/2     2            2           40d
> agones-extensions   2/2     2            2           40d
> agones-ping         2/2     2            2           40d
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-cfb9741c203cbe26"></a>


### File `cmd/extensions/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `195831d94832c83a63e85681277811075fbbd8c7`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/extensions/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7b1b9c7c9dec13bf9c.** [agones/cmd/extensions/main.go, lines 84–98](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/extensions/main.go#L84-L98). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5c2b758afb5ad9dd7beb48c4585c54ab5fb2f37f96373541efbeb369712d6156`.

> 
> func setupLogging(logDir string, logSizeLimitMB int) {
> 	logFileName := filepath.Join(logDir, "agones-extensions-"+time.Now().Format("20060102_150405")+".log")
> 
> 	const maxLogSizeMB = 100
> 	maxBackups := (logSizeLimitMB - maxLogSizeMB) / maxLogSizeMB
> 	logger.WithField("filename", logFileName).WithField("numbackups", maxBackups).Info("logging to file")
> 	logrus.SetOutput(
> 		io.MultiWriter(
> 			logrus.StandardLogger().Out,
> 			&lumberjack.Logger{
> 				Filename:   logFileName,
> 				MaxSize:    maxLogSizeMB,
> 				MaxBackups: maxBackups,
> 			},


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 5 lexically identified declarations, including `setupLogging` (L85), `main` (L104), `parseEnvFlags` (L256), `config` (L371), `runner` (L398). The full declaration list is recorded under `agones:cmd/extensions/main.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/cloudproduct`, `agones.dev/agones/pkg/fleetautoscalers`, `agones.dev/agones/pkg/fleets`, `agones.dev/agones/pkg/gameserverallocations`, `agones.dev/agones/pkg/gameserverallocations/processor`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/gameserversets`, `agones.dev/agones/pkg/metrics`, `agones.dev/agones/pkg/util/apiserver`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/https`, `agones.dev/agones/pkg/util/httpserver`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/signals`, `agones.dev/agones/pkg/util/webhooks`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Preserve source floating width, narrowing, exceptional values and relevant bits.

8. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-75d6c683f4631366"></a>


### File `cmd/extensions/pprof.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `696f3b914ef8c13149ff3da865c21fd06c9432c4`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/extensions/pprof.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4bd93beaf1382dc6a7.** [agones/cmd/extensions/pprof.go, lines 26–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/extensions/pprof.go#L26-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2b08cfe6d0e469ea821ea08ec6aba6df0eed94a5d8c272167267421790f113df`.

> 
> func init() {
> 	go func() {
> 		logrus.WithError(http.ListenAndServe(":6060", nil)).Info("Closed pprof server")
> 	}()
> 	logrus.Info("*** PPROF PROFILER STARTED on :6060 ***")
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `init` (L27). The full declaration list is recorded under `agones:cmd/extensions/pprof.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:cmd/ping` -> module `cmd::ping`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-11dfff1da301d4c4cc.** [agones/site/content/en/docs/Advanced/system-diagram.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/system-diagram.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e2b49fd915fa445c1beace0f69c29595277d175b4ec085cf64ebf1c0d2a306e`.

> 
> ![System Diagram](../../../diagrams/system-diagram.dot.png)
> 
> # Agones Control Plane 
> 
> The Agones Control Plane consists of 4 `Deployments`:
> ```
> NAME                READY   UP-TO-DATE   AVAILABLE   AGE
> agones-allocator    3/3     3            3           40d
> agones-controller   2/2     2            2           40d
> agones-extensions   2/2     2            2           40d
> agones-ping         2/2     2            2           40d
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-0e57ecb74d906ffd"></a>


### File `cmd/ping/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2b1ebbaa369f52e7aa82cb89221b792287bd6fa6`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/ping/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-433c438d0809ac80fc.** [agones/cmd/ping/main.go, lines 47–61](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/ping/main.go#L47-L61). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1a560f63d8fa15b6b993fdd7031eaebdab3b93eda335bf263aa61c8d638401bf`.

> 
> func main() {
> 	ctlConf := parseEnvFlags()
> 	if err := ctlConf.validate(); err != nil {
> 		logger.WithError(err).Fatal("Could not create controller from environment or flags")
> 	}
> 
> 	logger.WithField("version", pkg.Version).WithField("featureGates", runtime.EncodeFeatures()).
> 		WithField("ctlConf", ctlConf).Info("Starting ping...")
> 
> 	ctx, _ := signals.NewSigKillContext()
> 
> 	udpSrv := serveUDP(ctx, ctlConf)
> 	defer udpSrv.close()


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `main` (L48), `serveUDP` (L72), `serveHTTP` (L79), `config` (L117), `validate` (L123), `parseEnvFlags` (L131). The full declaration list is recorded under `agones:cmd/ping/main.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/signals`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Preserve source floating width, narrowing, exceptional values and relevant bits.

8. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-43b656496e604337"></a>


### File `cmd/ping/udp.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e0c029e89cd7245ea16c199a852ccd800a8a88db`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/ping/udp.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-05eff3825587bace40.** [agones/cmd/ping/udp.go, lines 35–49](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/ping/udp.go#L35-L49). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `67399cb61bc05768c2dd60123d8ec6e4f638169168734342f70e69b6c87a1c86`.

> 
> // udpServer is a rate limited udp server that echos
> // udp packets back to senders
> type udpServer struct {
> 	logger      *logrus.Entry
> 	conn        net.PacketConn
> 	rateLimit   rate.Limit
> 	rateBurst   int
> 	clock       clock.WithTickerAndDelayedExecution
> 	limitsMutex sync.Mutex
> 	limits      map[string]*visitor
> 	healthMutex sync.RWMutex
> 	health      bool
> 	errs        *errors.Errors
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 11 lexically identified declarations, including `udpServer` (L38), `visitor` (L53), `newUDPServer` (L60), `run` (L74), `cleanUp` (L94), `readWriteLoop` (L106), `rateLimitedEchoResponse` (L131), `close` (L154), `healthy` (L161), `unhealthy` (L168), `Health` (L176). The full declaration list is recorded under `agones:cmd/ping/udp.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.

7. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

8. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

9. Preserve source floating width, narrowing, exceptional values and relevant bits.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:cmd/processor` -> module `cmd::processor`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-11dfff1da301d4c4cc.** [agones/site/content/en/docs/Advanced/system-diagram.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/system-diagram.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e2b49fd915fa445c1beace0f69c29595277d175b4ec085cf64ebf1c0d2a306e`.

> 
> ![System Diagram](../../../diagrams/system-diagram.dot.png)
> 
> # Agones Control Plane 
> 
> The Agones Control Plane consists of 4 `Deployments`:
> ```
> NAME                READY   UP-TO-DATE   AVAILABLE   AGE
> agones-allocator    3/3     3            3           40d
> agones-controller   2/2     2            2           40d
> agones-extensions   2/2     2            2           40d
> agones-ping         2/2     2            2           40d
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-9713040d26a3f167"></a>


### File `cmd/processor/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b16add4dab35fefd714f9e800ee3425db135b332`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/processor/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-04eac52978ce1f9c2f.** [agones/cmd/processor/main.go, lines 83–97](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/processor/main.go#L83-L97). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5e58e1d9fcacc1d017f5121acfb913ec3e0aec92ec29c89433092a80223ceb0e`.

> 
> type processorConfig struct {
> 	GCPProjectID                 string
> 	LogLevel                     string
> 	PodNamespace                 string
> 	StackdriverLabels            string
> 	APIServerBurstQPS            int
> 	APIServerSustainedQPS        int
> 	GRPCPort                     int
> 	HTTPUnallocatedStatusCode    int
> 	LeaderElection               bool
> 	PrometheusMetrics            bool
> 	Stackdriver                  bool
> 	AllocationBatchWaitTime      time.Duration
> 	MaxListItems                 int64


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `processorConfig` (L84), `parseEnvFlags` (L106), `main` (L195), `whenLeader` (L283), `runGRPC` (L338), `getClients` (L367). The full declaration list is recorded under `agones:cmd/processor/main.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg`, `agones.dev/agones/pkg/allocation/go`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/gameserverallocations`, `agones.dev/agones/pkg/gameserverallocations/processor`, `agones.dev/agones/pkg/gameservers`, `agones.dev/agones/pkg/metrics`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/httpserver`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/signals`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Preserve source floating width, narrowing, exceptional values and relevant bits.

8. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:cmd/sdk-server` -> module `cmd::sdk-server`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-11dfff1da301d4c4cc.** [agones/site/content/en/docs/Advanced/system-diagram.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/system-diagram.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7e2b49fd915fa445c1beace0f69c29595277d175b4ec085cf64ebf1c0d2a306e`.

> 
> ![System Diagram](../../../diagrams/system-diagram.dot.png)
> 
> # Agones Control Plane 
> 
> The Agones Control Plane consists of 4 `Deployments`:
> ```
> NAME                READY   UP-TO-DATE   AVAILABLE   AGE
> agones-allocator    3/3     3            3           40d
> agones-controller   2/2     2            2           40d
> agones-extensions   2/2     2            2           40d
> agones-ping         2/2     2            2           40d
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-644eacb570e1fe68"></a>


### File `cmd/sdk-server/main.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `415f88ce6b22cbfbb63ba1f678ef0af3d22a4530`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/cmd/sdk-server/main.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c9de7f0339fc7bea7a.** [agones/cmd/sdk-server/main.go, lines 84–98](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cmd/sdk-server/main.go#L84-L98). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8113b4b4c2afe299e39c4f5856540d9e121e4280ce023b4dcaa640e028d6cddd`.

> 
> func main() {
> 	ctlConf := parseEnvFlags()
> 	logLevel, err := logrus.ParseLevel(ctlConf.LogLevel)
> 	if err != nil {
> 		logrus.WithError(err).Warn("Invalid LOG_LEVEL value. Defaulting to 'info'.")
> 		logLevel = logrus.InfoLevel
> 	}
> 	logger.Logger.SetLevel(logLevel)
> 	logger.WithField("version", pkg.Version).WithField("featureGates", runtime.EncodeFeatures()).
> 		WithField("ctlConf", ctlConf).Info("Starting sdk sidecar")
> 
> 	if ctlConf.MaxListItems <= 0 {
> 		logger.Fatalf("%s must be greater than 0", maxListItemsFlag)
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `main` (L85), `registerLocal` (L203), `registerTestSdkServer` (L233), `runGrpc` (L254), `runGateway` (L267), `parseEnvFlags` (L297), `config` (L380), `healthCheckWrapper` (L402). The full declaration list is recorded under `agones:cmd/sdk-server/main.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/sdk`, `agones.dev/agones/pkg/sdk/alpha`, `agones.dev/agones/pkg/sdk/beta`, `agones.dev/agones/pkg/sdkserver`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/signals`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
