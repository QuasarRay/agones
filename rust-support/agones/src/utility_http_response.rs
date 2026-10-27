//! Owned source response buffering, header snapshots and body accounting.
// Adapted from Go net/http server.go and transfer.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. The existing CGP request task owns this
// writer, its bounded body sender, pending head and clock/diagnostic effects.
use super::{Headers, HttpRequest, ResponseHead, header, set_header, sniff_content_type, native_response_head, CanReadSourceHttpClock, CanEmitSourceHttpDiagnostic};
use crate::{errors::{BoxError, Errors}, utility_runtime::UtilityRuntimeContext};
use bytes::Bytes;
use std::{collections::BTreeMap, io, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use tokio::sync::{mpsc, oneshot};

const BUFFER_SIZE: usize = 2048;
#[derive(Clone, Debug)]
struct WriteFailure(Arc<dyn std::error::Error + Send + Sync>);
impl std::fmt::Display for WriteFailure { fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.0.as_ref(), out) } }
impl std::error::Error for WriteFailure { fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { Some(self.0.as_ref()) } }
pub struct ResponseWriter {
    pub headers: Headers,
    committed: Option<u16>,
    committed_headers: Option<Headers>,
    head: Option<oneshot::Sender<ResponseHead>>,
    data: mpsc::Sender<Bytes>,
    runtime: UtilityRuntimeContext,
    panicked: Arc<AtomicBool>,
    head_request: bool,
    written: i64,
    content_length: Option<i64>,
    buffered: Vec<u8>,
    finished: bool,
    http2: bool,
    can_write_continue: Arc<AtomicBool>,
    informational: hyper::ext::InformationalSender,
    handler_done: bool,
    write_failure: Option<WriteFailure>,
}
impl ResponseWriter {
    pub(super) fn new(runtime: UtilityRuntimeContext, head: oneshot::Sender<ResponseHead>, data: mpsc::Sender<Bytes>,
        panicked: Arc<AtomicBool>, request: &HttpRequest) -> Result<Self, BoxError> {
        let informational = request.informational.clone().ok_or_else(|| Errors::default().New("HTTP informational response driver is unavailable"))?;
        Ok(Self { headers: BTreeMap::new(), committed: None, committed_headers: None, head: Some(head), data, runtime,
            panicked, head_request: request.method == "HEAD", written: 0, content_length: None, buffered: Vec::new(), finished: false,
            http2: request.protocol == hyper::Version::HTTP_2, can_write_continue: Arc::clone(&request.can_write_continue), informational,
            handler_done: false, write_failure: None })
    }
    pub async fn write_header(&mut self, status: u16) -> Result<(), BoxError> {
        if self.committed.is_some() { return Ok(()); }
        if !(100..=999).contains(&status) { panic!("invalid WriteHeader code {status}"); }
        if !self.http2 && (status < 101 || status > 199) {
            self.can_write_continue.store(false, Ordering::Release);
            // Source WriteHeader has no error result; it ignores transport
            // write/flush errors. The connection owner retains that failure.
            let _ = self.informational.disable_continue().await;
        }
        if (100..=199).contains(&status) && (status != 101 || self.http2) {
            let mut headers = self.headers.clone(); headers.remove("Content-Length"); headers.remove("Transfer-Encoding");
            let response = native_response_head(status, &headers)?;
            let _ = self.informational.send(response).await;
            return Ok(()); // 1xx does not commit or clear the final header map
        }
        self.committed = Some(status);
        // Source clones the called Header map before parsing Content-Length.
        // Mutations of its map/slices after this point cannot alter this copy.
        self.committed_headers = Some(self.headers.clone());
        if let Some(value) = header(&self.headers, "Content-Length").filter(|value| !self.http2 && !value.is_empty()) {
            match parse_content_length(value) {
                Some(length) => self.content_length = Some(length),
                None => {
                    self.runtime.emit_source_http_diagnostic(&format!("http: invalid Content-Length of {:?}", String::from_utf8_lossy(value)));
                    self.headers.remove("Content-Length");
                }
            }
        }
        Ok(())
    }
    async fn send_head(&mut self, first_data: &[u8]) -> Result<(), BoxError> {
        if self.committed.is_none() { self.write_header(200).await?; }
        if let Some(sender) = self.head.take() {
            let status = self.committed.unwrap_or(200);
            let mut headers = self.committed_headers.clone().unwrap_or_default();
            let has_transfer_encoding = header(&headers, "Transfer-Encoding").is_some_and(|value| !value.is_empty());
            if self.http2 {
                if let Some(value) = header(&headers, "Content-Length").filter(|value| !value.is_empty()).map(<[u8]>::to_vec) {
                    headers.remove("Content-Length");
                    self.content_length = parse_h2_content_length(&value);
                    if self.content_length.is_some() { set_header(&mut headers, "Content-Length", value); }
                }
                if !headers.contains_key("Content-Length") && self.handler_done && body_allowed(status) && (!self.head_request || !first_data.is_empty()) {
                    set_header(&mut headers, "Content-Length", first_data.len().to_string());
                }
            }
            if body_allowed(status) {
                let encoded = header(&headers, "Content-Encoding").is_some_and(|value| !value.is_empty());
                if !encoded && !headers.contains_key("Content-Type") && (self.http2 || !has_transfer_encoding) && !first_data.is_empty() {
                    set_header(&mut headers, "Content-Type", sniff_content_type(first_data));
                }
            } else if !self.http2 {
                headers.remove("Content-Length"); headers.remove("Transfer-Encoding");
                if status == 304 { headers.remove("Content-Type"); }
            }
            if self.head_request { headers.remove("Transfer-Encoding"); }
            if !headers.contains_key("Date") { set_header(&mut headers, "Date", self.runtime.source_http_date()); }
            // Fake Trailer:foo keys are not HTTP header names. Their eventual
            // trailer emission remains a distinct transport responsibility.
            headers.retain(|name, _| !name.starts_with("Trailer:"));
            sender.send(ResponseHead { status, headers })
                .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "HTTP response consumer closed"))?;
        }
        Ok(())
    }
    async fn write_chunk(&mut self, bytes: &[u8]) -> Result<(), BoxError> {
        if let Some(error) = &self.write_failure { return Err(Box::new(error.clone())); }
        if let Err(error) = self.send_head(bytes).await { return Err(self.remember_failure(error)); }
        if !self.head_request && !bytes.is_empty() {
            if self.data.send(Bytes::copy_from_slice(bytes)).await.is_err() {
                return Err(self.remember_failure(Box::new(io::Error::new(io::ErrorKind::BrokenPipe, "HTTP response consumer closed"))));
            }
        }
        Ok(())
    }
    async fn flush_buffer(&mut self) -> Result<(), BoxError> {
        let bytes = std::mem::take(&mut self.buffered);
        match self.write_chunk(&bytes).await { Ok(()) => Ok(()), Err(error) => { self.buffered = bytes; Err(error) } }
    }
    fn remember_failure(&mut self, error: BoxError) -> BoxError {
        let shared = WriteFailure(Arc::from(error)); self.write_failure = Some(shared.clone()); Box::new(shared)
    }
    pub async fn write(&mut self, bytes: &[u8]) -> Result<usize, BoxError> {
        if self.committed.is_none() { self.write_header(200).await?; }
        // HTTP/1 accepts an empty slice before checking body permission or
        // updating the counter. The bundled HTTP/2 writer checks permission
        // and its previously attempted length even for an empty slice.
        if bytes.is_empty() && !self.http2 { return Ok(0); }
        if !body_allowed(self.committed.unwrap_or(200)) {
            return Err(Errors::default().New("http: request method or response status code does not allow body"));
        }
        // The source int64 counter includes attempts rejected by the declared
        // length. A later smaller write must not erase the previous overrun.
        self.written = self.written.wrapping_add(bytes.len() as i64);
        if self.content_length.is_some_and(|length| (!self.http2 || length != 0) && self.written > length) {
            return Err(Errors::default().New(if self.http2 { "http2: handler wrote more than declared Content-Length" } else { "http: wrote more than the declared Content-Length" }));
        }
        if let Some(error) = &self.write_failure { return Err(Box::new(error.clone())); }
        let mut pending = bytes;
        let capacity = if self.http2 { 4096 } else { BUFFER_SIZE };
        while pending.len() > capacity - self.buffered.len() {
            if self.buffered.is_empty() {
                // bufio.Writer directly writes a large slice when empty.
                // The first chunk can therefore exceed BUFFER_SIZE.
                self.write_chunk(pending).await?; pending = &[];
            } else {
                let available = capacity - self.buffered.len();
                self.buffered.extend_from_slice(&pending[..available]); pending = &pending[available..];
                self.flush_buffer().await?;
            }
        }
        self.buffered.extend_from_slice(pending);
        Ok(bytes.len())
    }
    pub async fn flush(&mut self) -> Result<(), BoxError> {
        if self.committed.is_none() { self.write_header(200).await?; }
        self.flush_buffer().await
    }
    pub async fn encode_json<T: serde::Serialize>(&mut self, value: &T) -> Result<(), BoxError> {
        let bytes = serde_json::to_string(value)?.replace('&', "\\u0026").replace('<', "\\u003c").replace('>', "\\u003e")
            .replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029") + "\n";
        self.write(bytes.as_bytes()).await?; Ok(())
    }
    pub async fn http_error(&mut self, message: &str, status: u16) {
        self.headers.remove("Content-Length");
        set_header(&mut self.headers, "Content-Type", "text/plain; charset=utf-8");
        set_header(&mut self.headers, "X-Content-Type-Options", "nosniff");
        let _ = self.write_header(status).await;
        let _ = self.write(format!("{message}\n").as_bytes()).await;
    }
    pub(super) async fn finish(mut self) {
        self.handler_done = true;
        if self.committed.is_none() { let _ = self.write_header(200).await; }
        if self.head.is_some() && !self.http2 {
            let status = self.committed.unwrap_or(200);
            if let Some(headers) = &mut self.committed_headers {
                let trailers = headers.get("Trailer").is_some_and(|values| !values.is_empty()) || headers.keys().any(|name| name.starts_with("Trailer:"));
                let transfer_encoding = header(headers, "Transfer-Encoding").is_some_and(|value| !value.is_empty());
                if !trailers && !transfer_encoding && body_allowed(status) && !headers.contains_key("Content-Length")
                    && (!self.head_request || !self.buffered.is_empty()) {
                    self.content_length = Some(self.buffered.len() as i64);
                    set_header(headers, "Content-Length", self.buffered.len().to_string());
                }
            }
        }
        let _ = self.flush_buffer().await;
        self.finished = true;
    }
}
impl Drop for ResponseWriter {
    fn drop(&mut self) {
        if !self.finished { self.panicked.store(true, Ordering::Release); }
    }
}
fn body_allowed(status: u16) -> bool { !(100..=199).contains(&status) && status != 204 && status != 304 }
fn parse_content_length(bytes: &[u8]) -> Option<i64> {
    // strconv.ParseInt(base=10, bitSize=64), then v>=0. No whitespace or
    // underscores, but a leading sign and negative zero are source-valid.
    let digits = match bytes.first() { Some(b'+' | b'-') => &bytes[1..], _ => bytes };
    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) { return None; }
    std::str::from_utf8(bytes).ok()?.parse::<i64>().ok().filter(|value| *value >= 0)
}
fn parse_h2_content_length(bytes: &[u8]) -> Option<i64> {
    if bytes.is_empty() || !bytes.iter().all(u8::is_ascii_digit) { return None; }
    std::str::from_utf8(bytes).ok()?.parse::<u64>().ok().and_then(|value| i64::try_from(value).ok())
}
