//! One request-body owner shared by the source request and response writer.
// Adapted from Go net/http/{server,transfer,h2_bundle}.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{Headers, canonical_header_key};
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
    expect_saw_eof: AtomicBool,
    expect_closed: AtomicBool,
    expect_continue: bool,
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
struct ClosedBody { http2: bool }
impl fmt::Display for ClosedBody {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(if self.http2 { "body closed by handler" } else { "http: invalid Read on closed Body" })
    }
}
impl Error for ClosedBody {}

impl RequestBody {
    pub(super) fn new(incoming: Incoming, content_length: i64, http2: bool, closing: bool, expect_continue: bool) -> Self {
        let empty = if http2 { incoming.is_end_stream() } else { content_length == 0 };
        Self(Arc::new(Owner { state: Mutex::new(State { incoming: Some(incoming), pending: Bytes::new(),
            remaining: u64::try_from(content_length).ok(), early_close: false, trailers: Headers::new() }),
            closed: CancellationToken::new(), saw_eof: AtomicBool::new(!http2 && empty),
            expect_saw_eof: AtomicBool::new(false), expect_closed: AtomicBool::new(false), expect_continue,
            http2, empty, closing, chunked: !http2 && content_length < 0 }))
    }
    pub(super) fn require_open(&self) -> Result<(), BoxError> {
        if self.0.expect_continue && self.0.expect_closed.load(Ordering::Acquire) {
            return Err(Box::new(ClosedBody { http2: false }));
        }
        // NoBody remains readable after Close. The H2 reader checks its nil
        // pipe/sawEOF before reading the pipe's closed-body error.
        if self.0.closed.is_cancelled() && !self.0.empty && !(self.0.http2 && self.0.saw_eof.load(Ordering::Acquire)) {
            Err(Box::new(ClosedBody { http2: self.0.http2 }))
        } else { Ok(()) }
    }
    pub(super) fn is_http2(&self) -> bool { self.0.http2 }
    pub(super) fn expect_saw_eof(&self) -> bool { self.0.expect_saw_eof.load(Ordering::Acquire) }
    pub(super) async fn read(&self) -> Result<Option<Bytes>, BoxError> {
        self.read_up_to(usize::MAX).await
    }
    pub(super) async fn read_up_to(&self, limit: usize) -> Result<Option<Bytes>, BoxError> {
        let result = self.read_limited(limit).await;
        if result.is_ok() && self.0.saw_eof.load(Ordering::Acquire) { self.0.expect_saw_eof.store(true, Ordering::Release); }
        result
    }
    async fn read_limited(&self, limit: usize) -> Result<Option<Bytes>, BoxError> {
        self.require_open()?;
        let mut state = self.0.state.lock().await;
        self.require_open()?;
        self.read_locked(&mut state, limit).await
    }
    async fn read_locked(&self, state: &mut State, limit: usize) -> Result<Option<Bytes>, BoxError> {
        if self.0.empty || self.0.saw_eof.load(Ordering::Acquire) { return Ok(None); }
        loop {
            if !state.pending.is_empty() {
                let data = state.pending.split_to(limit.min(state.pending.len()));
                if let Some(remaining) = &mut state.remaining {
                    *remaining = remaining.saturating_sub(data.len() as u64);
                    // A Go H1 LimitedReader reports EOF with its last data.
                    // H2 pipe EOF remains a separate subsequent read.
                    if *remaining == 0 && !self.0.http2 { self.0.saw_eof.store(true, Ordering::Release); }
                }
                return Ok(Some(data));
            }
            let Some(incoming) = state.incoming.as_mut() else { return Ok(None); };
            let frame = if self.0.http2 {
                tokio::select! {
                    frame = incoming.frame() => frame,
                    _ = self.0.closed.cancelled() => return Err(Box::new(ClosedBody { http2: true })),
                }
            } else { incoming.frame().await };
            let Some(frame) = frame else { self.0.saw_eof.store(true, Ordering::Release); return Ok(None); };
            let frame = frame?;
            match frame.into_data() {
                Ok(data) => state.pending = data,
                Err(frame) => if let Ok(trailers) = frame.into_trailers() {
                    for name in trailers.keys() {
                        let values = trailers.get_all(name).iter().map(|value| value.as_bytes().to_vec()).collect();
                        state.trailers.insert(canonical_header_key(name.as_str()), Arc::new(values));
                    }
                },
            }
        }
    }
    pub(super) async fn snapshot(&self) -> BodySnapshot {
        let state = self.0.state.lock().await;
        BodySnapshot { closed: self.0.closed.is_cancelled(), saw_eof: self.0.saw_eof.load(Ordering::Acquire), remaining: state.remaining }
    }
    pub(super) async fn discard_before_response(&self) -> Discard {
        let snapshot = self.snapshot().await;
        if snapshot.closed { return if snapshot.saw_eof { Discard::Reusable } else { Discard::Close }; }
        if snapshot.remaining.is_some_and(|remaining| remaining >= MAX_POST_HANDLER_READ as u64) { return Discard::TooLarge; }
        let mut remaining = MAX_POST_HANDLER_READ + 1;
        while remaining != 0 {
            match self.read_limited(remaining).await {
                Ok(Some(bytes)) => remaining -= bytes.len(),
                Ok(None) => return if self.close().await.is_ok() { Discard::Reusable } else { Discard::Close },
                Err(error) if error.downcast_ref::<ClosedBody>().is_some() => return Discard::Reusable,
                Err(_) => return Discard::Close, // source closes on discard errors
            }
        }
        Discard::TooLarge
    }
    pub(super) async fn close(&self) -> Result<(), BoxError> {
        if self.0.expect_continue { self.0.expect_closed.store(true, Ordering::Release); }
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
                    match self.read_locked(&mut state, remaining).await {
                        Ok(Some(bytes)) => remaining -= bytes.len(),
                        Ok(None) => break,
                        Err(error) => { result = Err(error); break; }
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
