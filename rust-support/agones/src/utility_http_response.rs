//! Owned source response buffering, header snapshots and body accounting.
// Adapted from Go net/http server.go and transfer.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. The existing CGP request task owns this
// writer, its bounded body sender, pending head and clock/diagnostic effects.
use super::{Headers, HttpRequest, ResponseHead, ResponseFrame, header, set_header, sniff_content_type, native_response_head, trailers, CanReadSourceHttpClock, CanEmitSourceHttpDiagnostic};
use crate::{errors::{BoxError, Errors}, utility_runtime::UtilityRuntimeContext};
use bytes::Bytes;
use hyper::body::Frame;
use std::{collections::{BTreeMap, HashMap}, io, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use tokio::sync::{mpsc, oneshot};
#[path = "utility_http_framing.rs"]
mod framing;

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
    data: mpsc::Sender<ResponseFrame>,
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
    declared_trailers: Vec<Vec<u8>>,
    source_trailers: hyper::ext::SourceTrailers,
    source_close: tokio_util::sync::CancellationToken,
    request_body: super::request_body::RequestBody,
    protocol_at_least_11: bool,
    wants10_keep_alive: bool,
    wants_close: bool,
    expect_continue: bool,
    request_content_length: i64,
    server_stops: [tokio_util::sync::CancellationToken; 2],
    full_duplex: bool,
    source_end: hyper::ext::SourceResponseEnd,
    close_after_reply: bool,
}
impl ResponseWriter {
    pub(super) fn new(runtime: UtilityRuntimeContext, head: oneshot::Sender<ResponseHead>, data: mpsc::Sender<ResponseFrame>,
        panicked: Arc<AtomicBool>, source_close: tokio_util::sync::CancellationToken,
        server_stops: [tokio_util::sync::CancellationToken; 2], source_end: hyper::ext::SourceResponseEnd, request: &HttpRequest) -> Result<Self, BoxError> {
        let informational = request.informational.clone().ok_or_else(|| Errors::default().New("HTTP informational response driver is unavailable"))?;
        Ok(Self { headers: BTreeMap::new(), committed: None, committed_headers: None, head: Some(head), data, runtime,
            panicked, head_request: request.method == "HEAD", written: 0, content_length: None, buffered: Vec::new(), finished: false,
            http2: request.protocol == hyper::Version::HTTP_2, can_write_continue: Arc::clone(&request.can_write_continue), informational,
            handler_done: false, write_failure: None, declared_trailers: Vec::new(), source_trailers: hyper::ext::SourceTrailers::new(), source_close,
            request_body: request.body.clone(), protocol_at_least_11: request.protocol_at_least_11, wants10_keep_alive: request.wants10_keep_alive,
            wants_close: request.wants_close, expect_continue: request.expect_continue, request_content_length: request.content_length,
            server_stops, full_duplex: false, source_end, close_after_reply: false })
    }
    pub fn enable_full_duplex(&mut self) { self.full_duplex = true; }
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
            let mut response = native_response_head(status, &headers)?;
            if !self.http2 { response.extensions_mut().insert(hyper::ext::SourceResponse::new(
                trailers::encode_http1(&headers), hyper::ext::SourceBody::Empty, false)); }
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
        if self.http2 && self.handler_done { self.promote_undeclared_trailers(); }
        if let Some(sender) = self.head.take() {
            let status = self.committed.unwrap_or(200);
            let mut headers = self.committed_headers.clone().unwrap_or_default();
            let source_http1 = if self.http2 { None } else { Some(self.prepare_http1(&mut headers, status, first_data).await) };
            if self.http2 {
                if headers.contains_key("Connection") {
                    if header(&headers, "Connection") == Some(b"close") { self.source_close.cancel(); }
                    headers.remove("Connection");
                }
                if let Some(value) = header(&headers, "Content-Length").filter(|value| !value.is_empty()).map(<[u8]>::to_vec) {
                    headers.remove("Content-Length");
                    self.content_length = parse_h2_content_length(&value);
                    if self.content_length.is_some() { set_header(&mut headers, "Content-Length", value); }
                }
                if !headers.contains_key("Content-Length") && self.handler_done && body_allowed(status) && (!self.head_request || !first_data.is_empty()) {
                    set_header(&mut headers, "Content-Length", first_data.len().to_string());
                }
            }
            if self.http2 && body_allowed(status) {
                let encoded = header(&headers, "Content-Encoding").is_some_and(|value| !value.is_empty());
                if !encoded && !headers.contains_key("Content-Type") && !first_data.is_empty() {
                    set_header(&mut headers, "Content-Type", sniff_content_type(first_data));
                }
            }
            if self.http2 && !headers.contains_key("Date") { set_header(&mut headers, "Date", self.runtime.source_http_date()); }
            if let Some(values) = headers.get("Trailer") {
                for value in values.iter() { for name in trailers::elements(value) { self.declare_trailer(name); } }
            }
            headers.retain(|name, _| !name.starts_with("Trailer:"));
            let end_stream = self.http2 && (self.head_request || (self.handler_done && self.declared_trailers.is_empty() && first_data.is_empty()));
            sender.send(ResponseHead { status, headers, trailers: self.source_trailers.clone(), end_stream, source_http1 })
                .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "HTTP response consumer closed"))?;
        }
        Ok(())
    }
    async fn write_chunk(&mut self, bytes: &[u8]) -> Result<(), BoxError> {
        if let Some(error) = &self.write_failure { return Err(Box::new(error.clone())); }
        if let Err(error) = self.send_head(bytes).await { return Err(self.remember_failure(error)); }
        let end_stream = self.http2 && self.handler_done && self.final_trailers().is_empty();
        if !self.head_request && (!bytes.is_empty() || end_stream) {
            if self.data.send(ResponseFrame { frame: Frame::data(Bytes::copy_from_slice(bytes)), end_stream }).await.is_err() {
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
    fn declare_trailer(&mut self, name: &[u8]) {
        let name = trailers::canonical_name(name);
        if !trailers::valid_trailer(&name) {
            if self.http2 {
                self.runtime.emit_source_http_diagnostic(&format!("ignoring invalid trailer {:?}", String::from_utf8_lossy(&name)));
            }
            return;
        }
        // HTTP/1 retains repeated declarations; HTTP/2 declares each key once.
        if !self.http2 || !self.declared_trailers.contains(&name) { self.declared_trailers.push(name); }
    }
    fn promote_undeclared_trailers(&mut self) {
        // Range over unordered source-map data here. Distinct prefixed keys
        // can canonicalize to one key; their overwrite choice must not become
        // a prescribed lexicographic priority. Header wire order is sorted
        // separately by the source encoder.
        let promoted: HashMap<_, _> = self.headers.iter().filter_map(|(key, values)| key.strip_prefix("Trailer:")
            .map(|name| (name.to_owned(), Arc::clone(values)))).collect();
        for (name, values) in promoted {
            self.declare_trailer(name.as_bytes());
            // The source overwrites an existing unprefixed key even when its
            // declaration was rejected; only declared keys reach the encoder.
            let key = String::from_utf8(trailers::canonical_name(name.as_bytes())).expect("canonical UTF8 trailer key");
            self.headers.insert(key, values);
        }
        self.declared_trailers.sort();
    }
    fn final_trailers(&self) -> Headers {
        let mut result = Headers::new();
        if !self.http2 {
            for (key, values) in &self.headers {
                if let Some(key) = key.strip_prefix("Trailer:") { result.insert(key.to_owned(), Arc::clone(values)); }
            }
        }
        for key in &self.declared_trailers {
            let Ok(key) = std::str::from_utf8(key) else { continue; };
            if let Some(values) = self.headers.get(key) {
                if self.http2 { result.insert(key.to_owned(), Arc::clone(values)); }
                else { Arc::make_mut(result.entry(key.to_owned()).or_default()).extend(values.iter().cloned()); }
            }
        }
        result
    }
    pub(super) async fn finish(mut self) -> bool {
        self.handler_done = true;
        if self.committed.is_none() { let _ = self.write_header(200).await; }
        let flushed = self.flush_buffer().await.is_ok();
        if flushed && !self.head_request {
            let trailers = self.final_trailers();
            if self.http2 {
                // Key presence, including a nil/empty value slice, determines
                // whether Go ends with a trailers HEADERS frame.
                if !trailers.is_empty() { let _ = self.data.send(ResponseFrame { frame: Frame::trailers(trailers::encode_http2(&trailers)), end_stream: true }).await; }
            } else if self.source_trailers.finish_http1(trailers::encode_http1(&trailers)).is_ok() {
                let _ = self.data.send(ResponseFrame { frame: Frame::trailers(hyper::HeaderMap::new()), end_stream: true }).await;
            }
        }
        self.finished = true;
        self.close_after_reply || self.write_failure.is_some() || (!self.head_request && body_allowed(self.committed.unwrap_or(200))
            && self.content_length.is_some_and(|length| self.written != length))
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
