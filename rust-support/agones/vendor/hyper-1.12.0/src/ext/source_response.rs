//! Immutable wire decisions supplied by the Agones source response owner.
use bytes::Bytes;
use std::{fmt, sync::{Arc, atomic::{AtomicU8, Ordering}}, task::{Context, Poll}};
use atomic_waker::AtomicWaker;

/// Completion of one source handler and its request-body cleanup.
///
/// The existing HTTP/1 dispatcher waits for this signal before dispatching
/// another request. Its connection also consumes the final close decision.
#[derive(Clone)]
pub struct SourceResponseEnd(Arc<End>);
struct End { flags: AtomicU8, waker: AtomicWaker }
impl fmt::Debug for SourceResponseEnd {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("SourceResponseEnd") }
}
impl SourceResponseEnd {
    /// Creates a pending completion owned by the current source request phase.
    pub fn new() -> Self { Self(Arc::new(End { flags: AtomicU8::new(0), waker: AtomicWaker::new() })) }
    /// Publishes handler/cleanup completion once and wakes the native driver.
    pub fn finish(&self, close: bool) {
        let flags = if close { 3 } else { 1 };
        if self.0.flags.compare_exchange(0, flags, Ordering::AcqRel, Ordering::Acquire).is_ok() { self.0.waker.wake(); }
    }
    pub(crate) fn poll(&self, cx: &mut Context<'_>) -> Poll<bool> {
        let flags = self.0.flags.load(Ordering::Acquire);
        if flags & 1 != 0 { return Poll::Ready(flags & 2 != 0); }
        self.0.waker.register(cx.waker());
        let flags = self.0.flags.load(Ordering::Acquire);
        if flags & 1 != 0 { Poll::Ready(flags & 2 != 0) } else { Poll::Pending }
    }
}
impl Default for SourceResponseEnd { fn default() -> Self { Self::new() } }

/// The source writer's HTTP/1 response body framing decision.
#[derive(Clone, Copy, Debug)]
pub enum SourceBody {
    /// HEAD, informational, 204 or 304: no response body on the wire.
    Empty,
    /// A source-valid, explicit or inferred Content-Length.
    Length(u64),
    /// HTTP/1.1 chunk framing, including the final source trailer block.
    Chunked,
    /// An identity or HTTP/1.0 response terminated by connection close.
    CloseDelimited,
}

/// Source-encoded HTTP/1 fields and framing for one response head.
///
/// The source request/response owner computes these decisions before yielding
/// its final head. The existing connection writes them and continues owning
/// body I/O. `fields` contains header lines without a status line or final
/// blank line. No new transport, task or lifecycle owner is introduced here.
#[derive(Clone, Debug)]
pub struct SourceResponse {
    pub(crate) fields: Bytes,
    pub(crate) body: SourceBody,
    pub(crate) close: bool,
    pub(crate) end: Option<SourceResponseEnd>,
}
impl SourceResponse {
    /// Carries source-normalized fields and the selected body/close policy.
    pub fn new(fields: Bytes, body: SourceBody, close: bool) -> Self { Self { fields, body, close, end: None } }
    /// Attaches completion of the source handler and request cleanup phase.
    pub fn with_end(mut self, end: SourceResponseEnd) -> Self { self.end = Some(end); self }
}
