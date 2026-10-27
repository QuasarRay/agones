# Agones source HTTP adaptation

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

The same optional capability also exposes `SourceTrailers`, a single-publication
HTTP/1 trailer block attached to one response. The source response writer owns
declarations, final handler-header lookup and Go `Header.Write` encoding. After
publication it yields a normal trailers body frame. The existing H1 dispatcher
passes that frame to its existing connection encoder, which consumes the block
only for this response's chunked body. This preserves undeclared `Trailer:`
fields and the source's lack of a client-TE requirement, without applying that
policy to other Hyper responses. The packet contains header fields only; the
existing encoder emits the terminating chunk and blank line. Fixed-length,
close-delimited and bodyless responses do not emit trailer fields. No task,
executor, transport, global state or provider registry is introduced by this
data extension. HTTP/2 uses actual `Frame::trailers` and the existing h2 sender.

The source body carries its end-of-stream decision with each bounded frame,
including the empty-header and final-data cases. The connection phase owns the
graceful-close token used by source HTTP/2 `Connection: close`; its existing
driver performs graceful shutdown and owns the stream drain. Exact native
frame/HPACK, GOAWAY timing, header alias, partial I/O and error profiles remain
unverified implementation work. No compilation or behavior credit is implied.
