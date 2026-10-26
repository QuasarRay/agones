# Native CPG build support

This package owns the Rust equivalents of the pinned CPG build declarations.
The original `.sbt` and `.bzl` files remain in the literal `cpg-rs` mirror;
the pristine source tree and its license remain under
`third_party/first-generation/cpg`. Source pin:
`301c4010d28ecdac05bc2ed44310f56446459465` (1.7.79).

All of this implementation is **UNVERIFIED**. The combined roadmap's global
implementation gate controls execution. These entrypoints must not be built
or run while that gate is closed.

`NativeBuildContext` owns one absolute source root, the two distinct `Versions`
initialization scopes, open walks and readers, and synchronous child effects.
Its declaration uses the `NativeBuildProviders` derive-deftly template to
select CGP providers. Schema construction, protocol export and bindings use
that same parent context. They do not create an independent graph or
application manager. `cpg-rs` retains the sole Grafeo graph/persistence owner.

The flow is input -> fixed build operation -> owned effect -> artifact report.
There is no runtime provider registry. The six names read from `plugins.sbt`
are source dependency identities; a closed enum maps them to native operations:

| Retained facility | Native production consumer |
| --- | --- |
| sbt-protobuf | Pure Rust protocol/binding generator; M0008 owns complete source tool and input adoption |
| sbt-scalafmt | Format an explicitly owned source snapshot, or check the live native workspace after the gate |
| sbt-ci-release-early | Prepare a local payload/attribution bundle; final release delivery remains a later obligation |
| sbt-dynver | Separate the pinned source release from the native crate and native revision identities |
| sbt-native-packager | Generate domain prerequisites, then stage a supplied compiled native executable and resources |
| sbt-flatgraph | Construct the native schema, retain its content identity and emit Rust through existing Genco generation |

The authored CLI shapes are:

```text
cpg-build load <source-root>
cpg-build generate-protobuf <source-root>
cpg-build task <source-root> <native-operation> <request.json>
cpg-build facility <source-root> <retained-facility-name> <request.json>
cpg-build source-task <source-root> <source-task-or-label> <request.json>
```

The CLI reads an explicit `BuildRequest`; it never guesses another checkout,
silently replaces a manifest, or installs a tool. `source-task` enters the sbt
onLoad lifecycle for sbt names. The Git-LFS predicate must succeed before the
previous hook is invoked, and either failure propagates. A host that has a
previous state/hook uses `run_source_task_with_previous`; the one-shot CLI's
previous hook is the identity function. Bazel labels have no sbt onLoad hook.

`SourceBuildTask::parse` is the finite production adapter table. It keeps the
original project and target spellings while sharing one native library owner.
`RuleFactory` preserves all seven Java/Scala exports, including the imported
artifact and harness kinds, the original compiler options and empty runtime
dependency lists. `NativeTargetExecutor` owns Cargo children. Original source
tests are accounted for by M0036; existing native tests cannot be substituted
for those source identities. The retained Bazel `BUILD` files remain exact
reference/configuration inputs, including their archive workarounds and labels.

The library compiler and Rustdoc consume the current registered generated
directory, `cpg-rs/generated`. Requests for those operations must select that
same directory for domain and binding output. The source `generatedSrcDir`
setting resolves to `domainClasses/src/main/generated`; M0010 owns generation
and registration of that complete literal hierarchy. M0002 does not claim
that the existing flattened output satisfies M0010. Likewise, M0006 retains
full schema construction/body adoption, and M0008 retains source launcher,
protobuf input/tool and CLI semantics. No source/body counts have been removed
from those milestones.

`schema/generateProtobuf` preserves its source cache rule: it hashes the
source roots in traversal order, accepts a directory-only cache hit, invokes
Protogen at the source's hard-coded output path after a miss, and writes the
cache only after successful emission. An overridden source directory/target
is distinct from that hard-coded invocation path. The lazy discarded deletion
iterator retains both branch bodies without eagerly deleting the directory.

The retained build runtime profile uses JDK 21's UTF-8 default. Scala's
`Source.fromFile` reports malformed input; sbt `IO.read` replaces malformed
input. Versions are initialized once per defining scope, in declaration order;
failure is cached and later access receives the failed-class error. A project
reload can retain the parent meta-build scope. Native build teardown owns any
source reader/walk left open, while explicit source close failures propagate.
The source API record identifies the exact Scala/sbt/JDK inputs used to author
these boundaries. This small build adapter does not provide a general JVM,
classloader, charset registry or Java stack-trace runtime.

Package and formatting operations require an explicit `PackageManifest` with
the entire declared destination set, source identities, payload hashes,
permissions, timestamp and license/NOTICE paths. The producer consumes those
files and preserves relative hierarchy. It rejects existing destinations and
changed payload bytes. Formatting works on the resulting owned snapshot so
the live generated implementation keeps its emitter ownership. A bundle is
labelled UNVERIFIED; declared-payload closure is distinct from final roadmap
scope acceptance. Final release assembly must supply the full required scope.

Source/body ownership and downstream boundaries are recorded in
`verification/agones/roadmap-adoption-M0002.json` and
`verification/agones/roadmap-M0002-body-ownership.json`. Buildability, behavior,
source compatibility and assurance remain open until the global gate permits
the required checks.
