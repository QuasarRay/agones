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

`SourceResponse` now carries immutable HTTP/1 fields and a body framing decision
from the same source response writer. Its existing H1 role encoder writes the
status line and supplied field block, then uses the existing connection I/O and
body buffers. The source's handler fields and automatically added fields retain
their separate ordering and normalization. Source chunk sizes use lowercase
hexadecimal. For source Content-Length responses the request writer owns the
attempted signed count, rejection and wrap behavior; the native encoder does not
truncate again or declare the source handler complete when that length is met.
Ordinary Hyper responses retain their original encoder and count policy.

Each final source response also carries `SourceResponseEnd`. The source request
phase publishes handler and request-body cleanup completion, with its final
connection-close decision; emergency phase disposal publishes close. The existing
H1 dispatcher and connection wait before reuse, including HEAD/no-body replies,
and preserve queued I/O. This is a per-response data signal using the existing
driver's waker, without a new task or lifecycle manager. Connection disposal
cancels its request children; ordinary H1 no-body disposal does not cancel an
otherwise still-running handler. H2 retains independent stream cancellation.

The source request and response share one `RequestFields` handle for the live
method, content length and current Body interface. The request phase separately
retains the original transport body used by `response.reqBody` cleanup and
bounded discard. Replacing Body does not transfer that cleanup to the new
reader. Continue-reader closed/EOF flags belong to that retained wrapper, and
the H1 Continue read and disable paths share their source lock/flush order. H2
keeps its separate read-before-pipe Continue state. Read outcomes retain bytes
together with EOF or another error. The existing static CGP body-I/O capability
calls one retained source reader receiver; it adds no registry or tasks.

The H1 source writer checks the current method when flushing bytes. A response
whose headers were selected for HEAD therefore keeps an unframed native body
alive until source completion, allowing a later method reassignment to affect
the source writer's output. An already selected chunked response still emits
its terminating chunk when the current method later becomes HEAD. The source
phase continues to own the reuse/close decision. Remaining work includes mutable
header alias profiles, parser and protocol-switch behavior, body/close/
abort/partial-I/O and H2 reset timing, exact diagnostics and selected native
profiles. These source paths remain uncompiled, unexecuted and unverified.

The request's URL is now a separate shared `SourceUrl` data owner. RequestFields
owns the current optional URL pointer, Host bytes and RequestURI bytes. Source
mux decisions read the current URL.Path/RawPath/Host/RawQuery; typed log entries
retain the URL pointer selected when the field was created. In-place mutation
is visible when those entries serialize, while later Request.URL replacement
does not retarget the saved pointer. URL JSON uses the selected Go1.26.5/1.27.1
struct declaration order, original-byte string escaping, null User pointers and
the empty exported-field shape of non-nil Userinfo. This is data/serialization
authoring under the existing CGP HTTP/log effects, without another effect or
lifecycle owner. Native URL/authority/parser equivalence, remaining request-field
copy/alias profiles and protocol schedules remain open; no execution or assurance
credit follows from these changes.

Source responses also bypass the normal response-metadata keep-alive heuristic:
their 1.0/1.1 version and Connection decision were already made by the source
writer, including extra fields that are not in the metadata map. Explicit native
shutdown remains authoritative and cannot be re-enabled by a source head. The
normal metadata heuristic remains in place for other Hyper responses.

Source close-delimited bodies also wait for request-phase completion before
physical close. Their native byte output remains unframed; the source phase's
final close decision owns EOF timing after handler and request-body cleanup.
