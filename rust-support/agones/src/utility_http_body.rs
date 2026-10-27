//! Original transport body, retained separately from mutable Request.Body.
// Adapted from Go net/http/{server,transfer,h2_bundle}.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{Headers, HeaderValues, canonical_header_key, SourceBodyRead, SourceReadError};
use crate::errors::BoxError;
use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::body::{Body, Incoming};
use std::{error::Error, fmt, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

pub(super) const MAX_POST_HANDLER_READ: usize = 256 << 10;
#[derive(Clone)]
pub(super) struct RequestBody(Arc<Owner>);
struct Owner {
    state: Mutex<State>,
    closed: CancellationToken,
    saw_eof: AtomicBool,
    http2: bool,
    empty: bool,
    closing: bool,
    chunked: bool,
}
struct State {
    incoming: Option<Incoming>,
    pending: Bytes,
    remaining: Option<u64>,
    early_close: bool,
    trailers: Headers,
}
pub(super) struct BodySnapshot { pub closed: bool, pub saw_eof: bool, pub remaining: Option<u64> }
pub(super) enum Discard { Reusable, Close, TooLarge }

#[derive(Debug)]
pub(super) struct ClosedBody { pub http2: bool }
impl fmt::Display for ClosedBody {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(if self.http2 { "body closed by handler" } else { "http: invalid Read on closed Body" })
    }
}
impl Error for ClosedBody {}

impl RequestBody {
    pub(super) fn new(incoming: Incoming, content_length: i64, http2: bool, closing: bool, trailers: Headers) -> Self {
        let empty = if http2 { incoming.is_end_stream() } else { content_length == 0 };
        Self(Arc::new(Owner { state: Mutex::new(State { incoming: Some(incoming), pending: Bytes::new(),
            remaining: u64::try_from(content_length).ok(), early_close: false, trailers }),
            closed: CancellationToken::new(), saw_eof: AtomicBool::new(!http2 && empty),
            http2, empty, closing, chunked: !http2 && content_length < 0 }))
    }
    fn require_open(&self) -> Result<(), BoxError> {
        // NoBody remains readable after Close. The H2 reader checks its nil
        // pipe/sawEOF before reading the pipe's closed-body error.
        if self.0.closed.is_cancelled() && !self.0.empty && !(self.0.http2 && self.0.saw_eof.load(Ordering::Acquire)) {
            Err(Box::new(ClosedBody { http2: self.0.http2 }))
        } else { Ok(()) }
    }
    pub(super) fn is_http2(&self) -> bool { self.0.http2 }
    pub(super) fn is_no_body(&self) -> bool { !self.0.http2 && self.0.empty }
    pub(super) async fn read_up_to(&self, limit: usize) -> SourceBodyRead {
        if let Err(error) = self.require_open() { return SourceBodyRead::failed(error); }
        let mut state = self.0.state.lock().await;
        if let Err(error) = self.require_open() { return SourceBodyRead::failed(error); }
        self.read_locked(&mut state, limit).await
    }
    async fn read_locked(&self, state: &mut State, limit: usize) -> SourceBodyRead {
        if self.0.empty || self.0.saw_eof.load(Ordering::Acquire) { return SourceBodyRead::eof(); }
        if limit == 0 { return SourceBodyRead::data(Bytes::new()); }
        loop {
            if !state.pending.is_empty() {
                let data = state.pending.split_to(limit.min(state.pending.len()));
                let mut eof = false;
                if let Some(remaining) = &mut state.remaining {
                    *remaining = remaining.saturating_sub(data.len() as u64);
                    // A Go H1 LimitedReader reports EOF with its last data.
                    // H2 pipe EOF remains a separate subsequent read.
                    eof = *remaining == 0 && !self.0.http2;
                    if eof { self.0.saw_eof.store(true, Ordering::Release); }
                }
                // Pair data and error under the body lock. Close cannot change
                // the EOF result observed by an expectContinueReader alias.
                return SourceBodyRead { data, error: eof.then_some(SourceReadError::Eof) };
            }
            let Some(incoming) = state.incoming.as_mut() else { return SourceBodyRead::eof(); };
            let frame = if self.0.http2 {
                tokio::select! {
                    frame = incoming.frame() => frame,
                    _ = self.0.closed.cancelled() => return SourceBodyRead::failed(Box::new(ClosedBody { http2: true })),
                }
            } else { incoming.frame().await };
            let Some(frame) = frame else { self.0.saw_eof.store(true, Ordering::Release); return SourceBodyRead::eof(); };
            let frame = match frame { Ok(frame) => frame, Err(error) => return SourceBodyRead::failed(Box::new(error)) };
            match frame.into_data() {
                Ok(data) => state.pending = data,
                Err(frame) => if let Ok(trailers) = frame.into_trailers() {
                    if !trailers.is_empty() && state.trailers.is_nil() { state.trailers = Headers::with_profile(state.trailers.allocation_profile()); }
                    for name in trailers.keys() {
                        let values = trailers.get_all(name).iter().map(|value| Bytes::copy_from_slice(value.as_bytes()));
                        state.trailers.insert(canonical_header_key(name.as_str()), HeaderValues::parsed(values, state.trailers.allocation_profile()));
                    }
                },
            }
        }
    }
    pub(super) async fn snapshot(&self) -> BodySnapshot {
        let state = self.0.state.lock().await;
        BodySnapshot { closed: self.0.closed.is_cancelled(), saw_eof: self.0.saw_eof.load(Ordering::Acquire), remaining: state.remaining }
    }
    /// io.CopyN reads w.reqBody after the writer inspects current Request.Body.
    pub(super) async fn discard_original(&self) -> Discard {
        let mut remaining = MAX_POST_HANDLER_READ + 1;
        loop {
            let read = self.read_up_to(remaining).await;
            remaining -= read.data.len();
            // CopyN returns nil once n bytes were copied, including n+EOF.
            if remaining == 0 { return Discard::TooLarge; }
            match read.error {
                None => (),
                Some(SourceReadError::Eof) => return if self.close().await.is_ok() { Discard::Reusable } else { Discard::Close },
                Some(SourceReadError::Failure(error)) if error.downcast_ref::<ClosedBody>().is_some() => return Discard::Reusable,
                Some(SourceReadError::Failure(_)) => return Discard::Close,
            }
        }
    }
    pub(super) async fn close(&self) -> Result<(), BoxError> {
        if self.0.http2 {
            // Break the reader before waiting for its mutex, so Close wakes a
            // blocked read instead of waiting for additional remote data.
            self.0.closed.cancel();
            self.0.state.lock().await.incoming.take();
            return Ok(());
        }
        let mut state = self.0.state.lock().await;
        if self.0.closed.is_cancelled() || self.0.empty { return Ok(()); }
        let mut result = Ok(());
        if !self.0.saw_eof.load(Ordering::Acquire) && !(self.0.closing && !self.0.chunked) {
            if state.remaining.is_some_and(|remaining| remaining > MAX_POST_HANDLER_READ as u64) { state.early_close = true; }
            else {
                let mut remaining = MAX_POST_HANDLER_READ;
                while remaining != 0 {
                    let read = self.read_locked(&mut state, remaining).await;
                    remaining -= read.data.len();
                    match read.error {
                        None => (),
                        Some(SourceReadError::Eof) => break,
                        Some(SourceReadError::Failure(error)) => { result = Err(error); break; }
                    }
                }
                if remaining == 0 { state.early_close = true; }
            }
        }
        self.0.closed.cancel(); state.incoming.take(); result
    }
    pub(super) async fn trailers(&self) -> Headers { self.0.state.lock().await.trailers.clone() }
    pub(super) async fn did_early_close(&self) -> bool { self.0.state.lock().await.early_close }
}
