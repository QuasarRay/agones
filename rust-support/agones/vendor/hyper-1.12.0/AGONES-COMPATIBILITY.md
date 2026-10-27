# Agones informational response adaptation

Base: Hyper 1.12.0, exact crates.io source archive SHA-256
`2c3e324da4c95177d6291d4c8730197c0d1822f8a9766814a4a44fa5ab797c9c`.
The original source archive, file identities and modified payloads are recorded
in `verification/agones/roadmap-M0003-hyper-adoption.json` at the Agones root.
The upstream MIT license remains in `LICENSE`.

The opt-in `agones-informational` feature adds a per-request, bounded
`InformationalSender` extension. The existing H1 dispatcher owns its receiver,
encodes non-terminal headers without consuming the final response state, and
acknowledges after its normal I/O flush. The existing H2 stream owns its
receiver and calls the already pinned h2 0.4.20 informational-header API.
Driver/stream disposal drops pending acknowledgements and receivers. No worker,
executor, application manager or runtime provider registry is introduced here.

The Agones CGP HTTP provider enables the feature. Its response writer uses this
path for source `WriteHeader(1xx)` and deferred Continue handling; HTTP/1 101
continues through the ordinary upgrade path. Header commands do not clear the
source handler header map or commit its final response. H1 automatic Continue
can be disabled before an explicit 100 or a final header, as required by the
source. HTTP/2 retains the bundled Go reader's separate Continue state.

This adaptation was authored during IMPLEMENTATION_ONLY. It has not been
compiled, tested, linted, exercised on a socket, or granted parity/proof credit.
Root Cargo patch/lock source bookkeeping was authored without running Cargo.
