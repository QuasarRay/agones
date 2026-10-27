//! Retained Request.Body values, separate from the server's original reqBody.
// Source: Go io.ReadCloser, io.NopCloser, bytes.Reader and net/http's continue
// readers. Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::request_body::{RequestBody, ClosedBody};
use crate::errors::BoxError;
use bytes::Bytes;
use futures::future::BoxFuture;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::sync::Mutex;
use cgp::prelude::*;

#[derive(Debug)]
pub enum SourceReadError { Eof, Failure(BoxError) }
/// Go io.Reader can return n>0 with EOF or another error. Consume these bytes
/// before consulting error; an empty successful read is not EOF.
#[derive(Debug)]
pub struct SourceBodyRead { pub data: Bytes, pub error: Option<SourceReadError> }
impl SourceBodyRead {
    pub fn data(data: Bytes) -> Self { Self { data, error: None } }
    pub fn eof() -> Self { Self { data: Bytes::new(), error: Some(SourceReadError::Eof) } }
    pub fn failed(error: BoxError) -> Self { Self { data: Bytes::new(), error: Some(SourceReadError::Failure(error)) } }
}
/// One source io.ReadCloser receiver. This is request callback/interface data
/// retained by the static CGP HTTP provider, with no provider registry.
pub trait SourceReadCloser: Send + Sync {
    fn read(&self, limit: usize) -> BoxFuture<'_, SourceBodyRead>;
    fn close(&self) -> BoxFuture<'_, Result<(), BoxError>>;
}

#[derive(Clone)]
pub struct SourceBody(BodyValue);
#[derive(Clone)]
enum BodyValue {
    Nil,
    NoBody,
    Native(RequestBody),
    Expect(Arc<ExpectBody>),
    Http2(Arc<Http2Body>),
    Bytes(Arc<Mutex<Bytes>>),
    Reader(Arc<dyn SourceReadCloser>),
}
struct ExpectBody { native: RequestBody, continue_state: ContinueState, closed: AtomicBool, saw_eof: AtomicBool }
struct Http2Body { native: RequestBody, needs_continue: AtomicBool, sender: Mutex<Option<hyper::ext::InformationalSender>> }

/// H1 Read and disableWriteContinue share a lock. The flag is cleared after
/// the 100 response is flushed, matching the source synchronization order.
#[derive(Clone)]
pub(super) struct ContinueState(Arc<ContinueOwner>);
struct ContinueOwner { available: AtomicBool, sender: Mutex<Option<hyper::ext::InformationalSender>> }
impl ContinueState {
    pub(super) fn new(available: bool, sender: Option<hyper::ext::InformationalSender>) -> Self {
        Self(Arc::new(ContinueOwner { available: AtomicBool::new(available), sender: Mutex::new(sender) }))
    }
    async fn read_continue(&self) {
        if !self.0.available.load(Ordering::Acquire) { return; }
        let mut sender = self.0.sender.lock().await;
        if self.0.available.load(Ordering::Acquire) {
            if let Some(sender) = sender.as_mut() { send_continue(sender).await; }
            self.0.available.store(false, Ordering::Release);
        }
    }
    pub(super) async fn disable(&self) {
        let mut sender = self.0.sender.lock().await;
        self.0.available.store(false, Ordering::Release);
        if let Some(sender) = sender.as_mut() { let _ = sender.disable_continue().await; }
    }
}
async fn send_continue(sender: &mut hyper::ext::InformationalSender) {
    let mut response = hyper::Response::new(()); *response.status_mut() = hyper::StatusCode::CONTINUE;
    // Both Go implementations ignore the write result and then read the body.
    let _ = sender.send(response).await;
}
impl SourceBody {
    pub fn nil() -> Self { Self(BodyValue::Nil) }
    pub fn no_body() -> Self { Self(BodyValue::NoBody) }
    /// io.NopCloser(bytes.NewReader(data)): the last data read has nil error,
    /// and Close does not prevent later reads through a retained alias.
    pub fn from_bytes(data: impl Into<Bytes>) -> Self { Self(BodyValue::Bytes(Arc::new(Mutex::new(data.into())))) }
    pub fn from_reader(reader: Arc<dyn SourceReadCloser>) -> Self { Self(BodyValue::Reader(reader)) }
    pub(super) fn from_native(native: RequestBody, expected: bool, continue_state: ContinueState,
        sender: Option<hyper::ext::InformationalSender>) -> Self {
        Self(if native.is_http2() {
            BodyValue::Http2(Arc::new(Http2Body { native, needs_continue: AtomicBool::new(expected), sender: Mutex::new(sender) }))
        } else if expected {
            BodyValue::Expect(Arc::new(ExpectBody { native, continue_state, closed: AtomicBool::new(false), saw_eof: AtomicBool::new(false) }))
        } else if native.is_no_body() { BodyValue::NoBody } else { BodyValue::Native(native) })
    }
    pub(super) fn has_unread_expectation(&self) -> bool {
        matches!(&self.0, BodyValue::Expect(body) if !body.saw_eof.load(Ordering::Acquire))
    }
    pub(super) fn is_expect_reader(&self) -> bool { matches!(&self.0, BodyValue::Expect(_)) }
    pub(super) fn native_http1(&self) -> Option<&RequestBody> { if let BodyValue::Native(body) = &self.0 { Some(body) } else { None } }
    async fn read(&self, limit: usize) -> SourceBodyRead {
        let read = match &self.0 {
            BodyValue::Nil => panic!("invalid memory address or nil pointer dereference: Request.Body.Read"),
            BodyValue::NoBody => SourceBodyRead::eof(),
            BodyValue::Native(body) => body.read_up_to(limit).await,
            BodyValue::Expect(body) => {
                if body.closed.load(Ordering::Acquire) { return SourceBodyRead::failed(Box::new(ClosedBody { http2: false })); }
                body.continue_state.read_continue().await;
                let read = body.native.read_up_to(limit).await;
                if matches!(&read.error, Some(SourceReadError::Eof)) { body.saw_eof.store(true, Ordering::Release); }
                read
            }
            BodyValue::Http2(body) => {
                if body.needs_continue.swap(false, Ordering::AcqRel) {
                    if let Some(sender) = body.sender.lock().await.as_mut() { send_continue(sender).await; }
                }
                body.native.read_up_to(limit).await
            }
            BodyValue::Bytes(bytes) => {
                let mut bytes = bytes.lock().await;
                if bytes.is_empty() { SourceBodyRead::eof() } else { let size = limit.min(bytes.len()); SourceBodyRead::data(bytes.split_to(size)) }
            }
            BodyValue::Reader(reader) => reader.read(limit).await,
        };
        assert!(read.data.len() <= limit, "io.Reader returned more bytes than the supplied buffer");
        read
    }
    async fn close(&self) -> Result<(), BoxError> {
        match &self.0 {
            BodyValue::Nil => panic!("invalid memory address or nil pointer dereference: Request.Body.Close"),
            BodyValue::NoBody | BodyValue::Bytes(_) => Ok(()),
            BodyValue::Native(body) => body.close().await,
            BodyValue::Expect(body) => { body.closed.store(true, Ordering::Release); body.native.close().await },
            BodyValue::Http2(body) => body.native.close().await,
            BodyValue::Reader(reader) => reader.close().await,
        }
    }
}

#[cgp_component(SourceRequestBodyIo)]
pub trait CanAccessSourceRequestBody: Sync {
    fn read_source_request_body(&self, body: SourceBody, limit: usize) -> BoxFuture<'_, SourceBodyRead>;
    fn close_source_request_body(&self, body: SourceBody) -> BoxFuture<'_, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl<Context: Sync> SourceRequestBodyIo<Context> for AccessSourceRequestBody {
    fn read_source_request_body(_context: &Context, body: SourceBody, limit: usize) -> BoxFuture<'_, SourceBodyRead> {
        Box::pin(async move { body.read(limit).await })
    }
    fn close_source_request_body(_context: &Context, body: SourceBody) -> BoxFuture<'_, Result<(), BoxError>> {
        Box::pin(async move { body.close().await })
    }
}
