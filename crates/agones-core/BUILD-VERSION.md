# Agones build version

`pkg.Version` in the original source is a build identity, with an unconfigured
value of `dev`. It is independent of the Rust package's release version.

Set `AGONES_BUILD_VERSION` when invoking Cargo to provide the equivalent of the
source build's `-X agones.dev/agones/pkg.Version=...`. An explicit empty value is
preserved. Normal Cargo builds track this environment input and the shared
annotation conversion source. The build script uses the existing pinned Genco
dependency and writes only into `OUT_DIR`; it performs no downloads or external
commands.

The workspace supplies the same `dev` default through Cargo configuration, so
unchanged ordinary builds stay cacheable. Cargo can conflate absent and
non-UTF-8 environment values in its fingerprint. For either profile the build
script requests a recheck on every invocation, including when this crate is
built from outside the workspace. Generated files are rewritten only when their
contents change. This preserves raw-byte changes and restoration to the default;
it can add build work for those unusual profiles.

`AGONES_BUILD_VERSION_BYTES` retains the original bytes. `AGONES_BUILD_VERSION`
and the compatibility alias `DEFAULT_SDK_VERSION` expose the decoded annotation
value. On the pinned Linux host the build script preserves raw OS bytes;
non-UTF-8 build environments on other hosts are not certified by this evidence.

An application can compose `HasAgonesBuildVersion` and
`CanGetAgonesVersionAnnotation` with its own CGP context. The shared
`EncodeAgonesVersionAnnotation` provider uses Go JSON's string observation:
valid UTF-8 is preserved, and each invalid UTF-8 byte becomes one U+FFFD. This is
the decoded metadata value; JSON escape spelling is not a second build identity.

GameServer and Fleet defaulting, creation admission, and actual Rusternetes Pod
construction consume this capability. Default entry points use the compiled
build identity. The explicit context entry points permit application composition
without reading process configuration during each mutation. Source overwrite
rules apply even when an existing resource/template annotation differs.
