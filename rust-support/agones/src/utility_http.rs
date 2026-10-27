//! Native HTTP effects behind the source mux and response-writer contracts.
//! Request handlers stream through bounded channels into actual Hyper bodies;
//! a disconnected consumer is a write error, not a synthetic successful write.
use std::{collections::BTreeMap, error::Error, fmt, io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr}, pin::Pin, sync::{Arc, atomic::{AtomicBool, Ordering}}, task::{Context, Poll}, time::Duration};
use bytes::Bytes;
use futures::future::BoxFuture;
use hyper::{body::{Body, Frame, Incoming, SizeHint}, Request, Response};
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo, TokioTimer};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tokio::{io::{AsyncRead, AsyncWrite}, net::TcpListener, sync::{mpsc, oneshot}, task::JoinSet};
use tokio_util::sync::CancellationToken;
use cgp::prelude::*;
use crate::{errors::{BoxError, Errors}, utility_runtime::UtilityRuntimeContext, utility_scope::{CanSpawnUtilityTask, HasUtilityTaskScope, UtilityTaskScope}};

#[path = "utility_http_mux.rs"]
mod mux;
pub use mux::{SourceMux, MuxProfile};
#[path = "utility_http_sniff.rs"]
mod sniff;
use sniff::detect_content_type as sniff_content_type;
#[path = "utility_http_status.rs"]
mod status;
#[path = "utility_http_trailers.rs"]
mod trailers;
#[path = "utility_http_listener.rs"]
mod listener;
pub(crate) use listener::AcceptBackoff;
#[path = "utility_http_body.rs"]
mod request_body;
#[path = "utility_http_response.rs"]
mod response;
pub use response::ResponseWriter;

pub type Headers = BTreeMap<String, Arc<Vec<Vec<u8>>>>;
pub type HttpHandler = Arc<dyn for<'a> Fn(&'a UtilityRuntimeContext, &'a mut ResponseWriter, &'a mut HttpRequest) -> BoxFuture<'a, ()> + Send + Sync>;
pub type ErrorHandler = Arc<dyn for<'a> Fn(&'a UtilityRuntimeContext, &'a mut ResponseWriter, &'a mut HttpRequest) -> BoxFuture<'a, Result<(), BoxError>> + Send + Sync>;

pub fn canonical_header_key(name: &str) -> String {
    if !name.bytes().all(|value| value.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&value)) { return name.to_owned(); }
    let mut upper = true;
    name.bytes().map(|value| {
        let next = if upper { value.to_ascii_uppercase() } else { value.to_ascii_lowercase() };
        upper = value == b'-'; next as char
    }).collect()
}
pub fn set_header(headers: &mut Headers, name: &str, value: impl AsRef<[u8]>) {
    headers.insert(canonical_header_key(name), Arc::new(vec![value.as_ref().to_vec()]));
}
pub fn header<'a>(headers: &'a Headers, name: &str) -> Option<&'a [u8]> {
    headers.get(&canonical_header_key(name)).and_then(|values| values.first()).map(Vec::as_slice)
}
pub fn headers_json(headers: &Headers) -> Value {
    Value::Object(headers.iter().map(|(key, values)| (key.clone(), Value::Array(values.iter()
        .map(|value| Value::String(String::from_utf8_lossy(value).into_owned())).collect()))).collect())
}

#[cgp_component(SourceHttpClock)]
pub trait CanReadSourceHttpClock { fn source_http_date(&self) -> String; }
#[cgp_new_provider]
impl<Context> SourceHttpClock<Context> for ReadUtcSourceHttpClock {
    fn source_http_date(_context: &Context) -> String { chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string() }
}
#[cgp_component(SourceHttpDiagnostic)]
pub trait CanEmitSourceHttpDiagnostic { fn emit_source_http_diagnostic(&self, message: &str); }
#[cgp_new_provider]
impl<Context> SourceHttpDiagnostic<Context> for EmitNativeSourceHttpDiagnostic {
    fn emit_source_http_diagnostic(_context: &Context, message: &str) {
        let mut line = format!("{} {}", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"), message);
        if !line.ends_with('\n') { line.push('\n'); }
        let _ = std::io::Write::write(&mut std::io::stderr().lock(), line.as_bytes());
    }
}

#[cgp_component(SourceHttpDelay)]
pub trait CanWaitSourceHttpDelay: Sync {
    fn wait_source_http_delay(&self, delay: Duration) -> BoxFuture<'_, ()>;
}
#[cgp_new_provider]
impl<Context: Sync> SourceHttpDelay<Context> for WaitTokioSourceHttpDelay {
    fn wait_source_http_delay(_context: &Context, delay: Duration) -> BoxFuture<'_, ()> {
        Box::pin(tokio::time::sleep(delay))
    }
}

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub host: String,
    pub request_uri: String,
    pub url_log_value: Value,
    pub headers: Headers,
    /// Source Go strings can contain non-UTF8 bytes. Mux path decisions and
    /// captures use these URL bytes; `path` remains the existing text view.
    url_path: Vec<u8>,
    url_raw_path: Vec<u8>,
    url_host: String,
    url_raw_query: Vec<u8>,
    protocol_at_least_11: bool,
    protocol: hyper::Version,
    pub content_length: i64,
    informational: Option<hyper::ext::InformationalSender>,
    can_write_continue: Arc<AtomicBool>,
    wants10_keep_alive: bool,
    wants_close: bool,
    expect_continue: bool,
    pub matched_pattern: Vec<u8>,
    path_values: BTreeMap<String, Vec<u8>>,
    /// The request owner cancels on handler return or transport/body disposal.
    /// Server Shutdown does not cancel an otherwise active request.
    pub cancelled: CancellationToken,
    body: request_body::RequestBody,
}
impl HttpRequest {
    fn from_hyper(request: Request<Incoming>) -> Result<Self, ()> {
        let (parts, body) = request.into_parts();
        let content_length = body.size_hint().exact().and_then(|length| i64::try_from(length).ok()).unwrap_or(-1);
        let request_uri = parts.uri.to_string();
        let connect_authority = parts.method == hyper::Method::CONNECT && !request_uri.starts_with('/');
        let raw_path = if connect_authority { "" } else { parts.uri.path() };
        let url_path = mux::path_unescape(raw_path.as_bytes()).ok_or(())?;
        let path = String::from_utf8_lossy(&url_path).into_owned();
        let mut headers: Headers = BTreeMap::new();
        for name in parts.headers.keys() {
            let values = parts.headers.get_all(name).iter().map(|value| value.as_bytes().to_vec()).collect();
            headers.insert(canonical_header_key(name.as_str()), Arc::new(values));
        }
        let host = parts.uri.authority().map(|authority| authority.as_str().to_owned())
            .or_else(|| header(&headers, "Host").map(|value| String::from_utf8_lossy(value).into_owned())).unwrap_or_default();
        // net/http moves Host out of Request.Header into Request.Host.
        headers.remove("Host");
        let scheme = if connect_authority { "" } else { parts.uri.scheme_str().unwrap_or_default() };
        let authority = parts.uri.authority().map_or("", |value| value.as_str());
        let raw_query = parts.uri.query().unwrap_or_default();
        let can_write_continue = if parts.version == hyper::Version::HTTP_2 {
            let expected = headers.get("Expect").is_some_and(|values| values.iter().any(|value| header_has_comma_token(value, b"100-continue")));
            if expected { headers.remove("Expect"); } // bundled httpcommon.NewServerRequest
            expected
        } else {
            content_length != 0 && parts.version != hyper::Version::HTTP_09 && parts.version != hyper::Version::HTTP_10
                && header(&headers, "Expect").is_some_and(|value| header_has_token(value, b"100-continue"))
        };
        let informational = parts.extensions.get::<hyper::ext::InformationalSender>().cloned();
        let wants10_keep_alive = parts.version == hyper::Version::HTTP_10
            && header(&headers, "Connection").is_some_and(|value| header_has_token(value, b"keep-alive"));
        let wants_close = (parts.version == hyper::Version::HTTP_10 && !wants10_keep_alive) || parts.version == hyper::Version::HTTP_09
            || header(&headers, "Connection").is_some_and(|value| header_has_token(value, b"close"));
        let expect_continue = parts.version != hyper::Version::HTTP_2 && can_write_continue;
        let body = request_body::RequestBody::new(body, content_length, parts.version == hyper::Version::HTTP_2, wants_close, expect_continue);
        let raw_path_log = if raw_path == "*" || mux::escape_url_path(&url_path) == raw_path.as_bytes() { "" } else { raw_path };
        let url_log_value = json!({"Scheme":scheme,"Opaque":"","User":null,"Host":authority,"Path":path,
            "RawPath":raw_path_log,"OmitHost":false,"ForceQuery":parts.uri.query()==Some(""),"RawQuery":raw_query,"Fragment":"","RawFragment":""});
        Ok(Self { method: parts.method.to_string(), path, host, request_uri, url_log_value, headers,
            url_path, url_raw_path: raw_path_log.as_bytes().to_vec(), url_host: authority.to_owned(), url_raw_query: raw_query.as_bytes().to_vec(),
            protocol_at_least_11: parts.version != hyper::Version::HTTP_09 && parts.version != hyper::Version::HTTP_10,
            protocol: parts.version, content_length,
            informational, can_write_continue: Arc::new(AtomicBool::new(can_write_continue)),
            wants10_keep_alive, wants_close, expect_continue,
            matched_pattern: Vec::new(), path_values: BTreeMap::new(),
            cancelled: CancellationToken::new(), body })
    }
    pub fn path_value(&self, name: &str) -> &[u8] { self.path_values.get(name).map_or(&[], Vec::as_slice) }
    pub fn set_path_value(&mut self, name: &str, value: Vec<u8>) { self.path_values.insert(name.to_owned(), value); }
    pub fn escaped_path(&self) -> Vec<u8> {
        if !self.url_raw_path.is_empty() && mux::valid_encoded_path(&self.url_raw_path)
            && mux::path_unescape(&self.url_raw_path).as_deref() == Some(self.url_path.as_slice()) {
            return self.url_raw_path.clone();
        }
        if self.url_path == b"*" { return vec![b'*']; }
        mux::escape_url_path(&self.url_path)
    }
    pub async fn read_body(&mut self) -> Result<Vec<u8>, BoxError> {
        self.require_open_body()?;
        let mut bytes = Vec::new();
        while let Some(data) = self.next_data().await? { bytes.extend_from_slice(&data); }
        Ok(bytes)
    }
    async fn next_data(&mut self) -> Result<Option<Bytes>, BoxError> {
        self.next_data_limited(usize::MAX).await
    }
    async fn next_data_limited(&mut self, limit: usize) -> Result<Option<Bytes>, BoxError> {
        if self.can_write_continue.swap(false, Ordering::AcqRel) {
            let sender = self.informational.as_mut().ok_or_else(|| Errors::default().New("HTTP informational response driver is unavailable"))?;
            let mut response = Response::new(()); *response.status_mut() = hyper::StatusCode::CONTINUE;
            // The Go continue reader ignores the header write error and still
            // performs the body read, which owns any actual input failure.
            let _ = sender.send(response).await;
        }
        if limit == usize::MAX { self.body.read().await } else { self.body.read_up_to(limit).await }
    }
    /// Streaming decoders retain their own read-ahead across Decode calls.
    pub async fn read_body_chunk(&mut self) -> Result<Option<Bytes>, BoxError> {
        self.require_open_body()?;
        self.next_data().await
    }
    fn require_open_body(&self) -> Result<(), BoxError> {
        // H2 Read sends a needed Continue before consulting its pipe, even
        // after Close. The body owner checks that protocol's closed state.
        if self.body.is_http2() { Ok(()) } else { self.body.require_open() }
    }
    pub async fn decode_one_json<T: DeserializeOwned>(&mut self) -> Result<T, BoxError> {
        self.require_open_body()?;
        let mut bytes = Vec::new();
        loop {
            let result = { let mut input = serde_json::Deserializer::from_slice(&bytes).into_iter::<T>(); input.next() };
            match result {
                Some(Ok(value)) => return Ok(value), // Decode one value; trailing input is not another Decode.
                Some(Err(error)) if !error.is_eof() => return Err(Box::new(error)),
                result => {
                    if let Some(data) = self.next_data().await? { bytes.extend_from_slice(&data); }
                    else { return Err(match result {
                        Some(Err(error)) => Box::new(error) as BoxError,
                        _ => Errors::default().New("EOF"),
                    }); }
                }
            }
        }
    }
    pub async fn close_body(&mut self) -> Result<(), BoxError> { self.body.close().await }
    pub async fn body_trailers(&self) -> Headers { self.body.trailers().await }
}
impl Drop for HttpRequest {
    fn drop(&mut self) { self.cancelled.cancel(); }
}
struct ResponseHead { status: u16, headers: Headers, trailers: hyper::ext::SourceTrailers, end_stream: bool, source_http1: Option<hyper::ext::SourceResponse> }
struct ResponseFrame { frame: Frame<Bytes>, end_stream: bool }
fn header_has_token(value: &[u8], token: &[u8]) -> bool {
    if token.is_empty() || token.len() > value.len() { return false; }
    let boundary = |byte| matches!(byte, b' ' | b',' | b'\t');
    value.windows(token.len()).enumerate().any(|(start, part)|
        (start == 0 || boundary(value[start-1])) && (start+token.len() == value.len() || boundary(value[start+token.len()]))
        && part.eq_ignore_ascii_case(token))
}
fn header_has_comma_token(value: &[u8], token: &[u8]) -> bool {
    value.split(|byte| *byte == b',').any(|part| {
        let start = part.iter().position(|byte| !matches!(*byte, b' ' | b'\t')).unwrap_or(part.len());
        let end = part.iter().rposition(|byte| !matches!(*byte, b' ' | b'\t')).map_or(start, |index| index+1);
        part[start..end].eq_ignore_ascii_case(token)
    })
}
fn native_response_head(status: u16, headers: &Headers) -> Result<Response<()>, BoxError> {
    let mut response = Response::new(()); *response.status_mut() = hyper::StatusCode::from_u16(status)?;
    response.extensions_mut().insert(hyper::ext::ReasonPhrase::try_from(status::reason_phrase(status))?);
    for (name, values) in headers {
        let Ok(name) = hyper::header::HeaderName::from_bytes(name.as_bytes()) else { continue; };
        for value in values.iter() {
            if let Ok(value) = hyper::header::HeaderValue::from_bytes(value) { response.headers_mut().append(name.clone(), value); }
        }
    }
    Ok(response)
}
struct ResponseBody { receiver: mpsc::Receiver<ResponseFrame>, panicked: Arc<AtomicBool>, ended: bool, cancelled: CancellationToken, cancel_on_drop: bool }
impl Drop for ResponseBody {
    fn drop(&mut self) { if self.cancel_on_drop { self.cancelled.cancel(); } }
}
impl Body for ResponseBody {
    type Data = Bytes; type Error = io::Error;
    fn poll_frame(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Result<Frame<Bytes>, io::Error>>> {
        if self.ended { return Poll::Ready(None); }
        if self.panicked.load(Ordering::Acquire) {
            self.ended = true;
            return Poll::Ready(Some(Err(io::Error::other("HTTP handler panicked"))));
        }
        match self.receiver.poll_recv(context) {
            Poll::Ready(Some(message)) => {
                self.ended = message.end_stream;
                Poll::Ready(Some(Ok(message.frame)))
            },
            Poll::Ready(None) => { self.ended = true; Poll::Ready(None) }
            Poll::Pending => Poll::Pending,
        }
    }
    fn size_hint(&self) -> SizeHint { SizeHint::default() }
    fn is_end_stream(&self) -> bool { self.ended }
}

#[derive(Debug)]
pub struct ServerClosed;
impl fmt::Display for ServerClosed { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("http: Server closed") } }
impl Error for ServerClosed {}
pub fn is_server_closed(error: &(dyn Error + 'static)) -> bool { crate::errors::contains_error::<ServerClosed>(error) }

struct SourceRequestPhase(Option<hyper::ext::SourceResponseEnd>);
impl SourceRequestPhase {
    fn finish(mut self, close: bool) { if let Some(end) = self.0.take() { end.finish(close); } }
}
impl Drop for SourceRequestPhase {
    fn drop(&mut self) { if let Some(end) = self.0.take() { end.finish(true); } }
}
struct SourceConnectionClosed(CancellationToken);
impl Drop for SourceConnectionClosed { fn drop(&mut self) { self.0.cancel(); } }

async fn request_service(context: UtilityRuntimeContext, mux: SourceMux, tasks: Arc<UtilityTaskScope>, source_close: CancellationToken,
    server_stops: [CancellationToken; 2], connection_closed: CancellationToken, incoming: Request<Incoming>) -> Result<Response<ResponseBody>, io::Error> {
    let version = incoming.version();
    let mut request = match HttpRequest::from_hyper(incoming) {
        Ok(request) => request,
        Err(()) => {
            let (data, receiver) = mpsc::channel(2);
            let _ = data.try_send(ResponseFrame { frame: Frame::data(Bytes::from_static(b"400 Bad Request")), end_stream: true }); drop(data);
            let mut response = Response::new(ResponseBody { receiver, panicked: Arc::new(AtomicBool::new(false)),
                ended: false, cancelled: CancellationToken::new(), cancel_on_drop: version == hyper::Version::HTTP_2 });
            *response.status_mut() = hyper::StatusCode::BAD_REQUEST;
            response.headers_mut().insert(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static("text/plain; charset=utf-8"));
            if version != hyper::Version::HTTP_2 { response.headers_mut().insert(hyper::header::CONNECTION, hyper::header::HeaderValue::from_static("close")); }
            return Ok(response);
        }
    };
    request.cancelled = connection_closed.child_token();
    // Hyper may drop the service future before the response head is ready.
    // This guard covers that interval. Connection disposal owns H1 cancellation;
    // H2 stream/body disposal also cancels its own request independently.
    struct PendingRequest(Option<CancellationToken>);
    impl Drop for PendingRequest {
        fn drop(&mut self) { if let Some(stop) = &self.0 { stop.cancel(); } }
    }
    let mut pending = PendingRequest(Some(request.cancelled.clone()));
    let (head_tx, head_rx) = oneshot::channel();
    let (data, receiver) = mpsc::channel(2);
    let panicked = Arc::new(AtomicBool::new(false));
    let end = hyper::ext::SourceResponseEnd::new();
    let phase = SourceRequestPhase(Some(end.clone()));
    let mut writer = ResponseWriter::new(context.clone(), head_tx, data, Arc::clone(&panicked), source_close, server_stops, end, &request).map_err(io::Error::other)?;
    tasks.spawn("HTTP request", Box::pin(async move {
        let rejected_expectation = request.protocol != hyper::Version::HTTP_2 && header(&request.headers, "Expect")
            .is_some_and(|value| !value.is_empty() && !header_has_token(value, b"100-continue"));
        if rejected_expectation {
            set_header(&mut writer.headers, "Connection", "close"); let _ = writer.write_header(417).await;
        } else if request.method == "OPTIONS" && request.request_uri == "*" { global_options(&mut writer, &mut request).await; }
        else { mux.dispatch(&context, &mut writer, &mut request).await; }
        request.cancelled.cancel(); // net/http cancels when ServeHTTP returns.
        let body = request.body.clone();
        let close = writer.finish().await;
        let _ = body.close().await;
        phase.finish(close || body.did_early_close().await);
        Ok(())
    })).map_err(io::Error::other)?;
    let head = head_rx.await.map_err(|_| io::Error::other("HTTP handler ended without a response"))?;
    let ended = head.end_stream;
    let mut response = native_response_head(head.status, &head.headers).map_err(io::Error::other)?;
    response.extensions_mut().insert(head.trailers);
    if let Some(source) = head.source_http1 { response.extensions_mut().insert(source); }
    Ok(response.map(|()| ResponseBody { receiver, panicked, ended,
        cancelled: pending.0.take().expect("pending HTTP request owns cancellation"), cancel_on_drop: version == hyper::Version::HTTP_2 }))
}

async fn global_options(writer: &mut ResponseWriter, request: &mut HttpRequest) {
    set_header(&mut writer.headers, "Content-Length", "0");
    if request.content_length == 0 { return; }
    let mut remaining = 4 << 10;
    while let Ok(Some(bytes)) = request.next_data_limited(remaining + 1).await {
        if bytes.len() > remaining {
            // Source MaxBytesReader calls requestTooLarge on the HTTP/1
            // response, closing this connection after the otherwise empty 200.
            if request.protocol != hyper::Version::HTTP_2 { set_header(&mut writer.headers, "Connection", "close"); }
            return;
        }
        remaining -= bytes.len();
    }
}

#[derive(Clone, Copy)]
pub enum HttpProtocol { Http1, Http2 }

#[derive(Clone)]
struct SourceHttpExecutor { tasks: std::sync::Weak<UtilityTaskScope> }
impl<Future> hyper::rt::Executor<Future> for SourceHttpExecutor
where Future: std::future::Future + Send + 'static, Future::Output: Send + 'static,
{
    fn execute(&self, future: Future) {
        if let Some(tasks) = self.tasks.upgrade() {
            let stop = tasks.child_token();
            let child: BoxFuture<'static, Result<(), BoxError>> = Box::pin(async move {
                tokio::select! { _ = stop.cancelled() => (), _ = future => () }
                Ok(())
            });
            // Rejection drops the unstarted, resource-owning future. The weak
            // executor cannot retain its own scope through an H2 stream task.
            let _ = tasks.spawn("HTTP protocol stream", child);
        }
    }
}

pub async fn serve_connection<IO>(io: IO, context: UtilityRuntimeContext, mux: SourceMux, stop: CancellationToken,
    owner_stop: CancellationToken, handlers: Arc<UtilityTaskScope>, read_header_timeout: Duration, protocol: HttpProtocol) -> Result<(), BoxError>
where IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let source_shutdown = CancellationToken::new();
    let source_close = source_shutdown.clone();
    let connection_closed = CancellationToken::new();
    let _closed_guard = SourceConnectionClosed(connection_closed.clone());
    let request_connection = connection_closed.clone();
    let server_stop = stop.clone(); let server_owner_stop = owner_stop.clone();
    let service = service_fn(move |request| request_service(context.clone(), mux.clone(), Arc::clone(&handlers), source_close.clone(),
        [server_stop.clone(), server_owner_stop.clone()], request_connection.clone(), request));
    let protocol_tasks = Arc::new(UtilityTaskScope::default());
    let executor = SourceHttpExecutor { tasks: Arc::downgrade(&protocol_tasks) };
    let mut builder = hyper_util::server::conn::auto::Builder::new(executor);
    builder.http1().timer(TokioTimer::new()).header_read_timeout(read_header_timeout).auto_date_header(false).title_case_headers(true);
    builder.http2().auto_date_header(false);
    builder = match protocol { HttpProtocol::Http1 => builder.http1_only(), HttpProtocol::Http2 => builder.http2_only() };
    let connection = builder.serve_connection(TokioIo::new(io), service);
    tokio::pin!(connection);
    let result = tokio::select! {
        result = &mut connection => result.map_err(|error| Errors::default().New(error.to_string())),
        _ = async { tokio::select! { _ = stop.cancelled() => (), _ = owner_stop.cancelled() => (), _ = source_shutdown.cancelled() => () } } => {
            connection.as_mut().graceful_shutdown();
            // Independent drain: an already cancelled stop token does not
            // cancel the in-flight response or close its connection early.
            connection.await.map_err(|error| Errors::default().New(error.to_string()))
        }
    };
    // Also own Hyper's H2 stream/upgrade futures, rather than relying on an
    // executor that discards their JoinHandles. Normal and error exits drain
    // this same connection phase; emergency Drop aborts its owned children.
    connection_closed.cancel();
    let _ = protocol_tasks.cancel_and_join().await;
    result
}

pub async fn serve_plain(context: UtilityRuntimeContext, mux: SourceMux, port: &str, stop: CancellationToken, read_header_timeout: Duration) -> Result<(), BoxError> {
    if stop.is_cancelled() { return Err(Box::new(ServerClosed)); }
    let owner_stop = context.utility_child_token()?;
    let listener = bind_source_listener(port).await?;
    serve_listener(listener, context, mux, stop, owner_stop, read_header_timeout).await
}

/// Empty-host net.Listen("tcp", ":port") prefers a dual-stack IPv6 socket,
/// with IPv4 available on hosts without IPv6. Port spelling remains source
/// input; named-service resolution/error text is accounted for separately.
pub async fn bind_source_listener(port: &str) -> Result<TcpListener, BoxError> {
    let port: u16 = if port.is_empty() { 0 } else { port.parse()? };
    match TcpListener::bind(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), port)).await {
        Ok(listener) => Ok(listener),
        Err(error) if error.kind() == io::ErrorKind::Unsupported || error.raw_os_error() == Some(libc::EAFNOSUPPORT) || error.raw_os_error() == Some(libc::EPROTONOSUPPORT) =>
            Ok(TcpListener::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port)).await?),
        Err(error) => Err(Box::new(error)),
    }
}

/// The source Shutdown goroutine outlives ListenAndServe's return, including
/// permanent accept errors. Transfer its connection/request drain to the
/// application owner; a closing owner receives and awaits the rejected future.
pub async fn retain_connection_drain(context: &UtilityRuntimeContext, mut connections: JoinSet<Result<(), BoxError>>,
    handlers: Arc<UtilityTaskScope>, owner_stop: CancellationToken)
{
    let drain: BoxFuture<'static, Result<(), BoxError>> = Box::pin(async move {
        while let Some(_result) = connections.join_next().await {}
        let _ = handlers.cancel_and_join().await;
        Ok(())
    });
    let rejected = match context.utility_task_scope().upgrade() {
        Some(scope) => scope.spawn_or_return("HTTP server connection drain", drain).err(),
        None => Some(drain),
    };
    if let Some(drain) = rejected { owner_stop.cancel(); let _ = drain.await; }
}

async fn serve_listener(listener: TcpListener, context: UtilityRuntimeContext, mux: SourceMux, stop: CancellationToken,
    owner_stop: CancellationToken, read_header_timeout: Duration) -> Result<(), BoxError> {
    let handlers = Arc::new(UtilityTaskScope::default());
    let mut connections = JoinSet::new();
    let mut accept_backoff = AcceptBackoff::default();
    let result = loop {
        tokio::select! {
            _ = stop.cancelled() => break Err(Box::new(ServerClosed) as BoxError),
            _ = owner_stop.cancelled() => break Err(Box::new(ServerClosed) as BoxError),
            completed = connections.join_next(), if !connections.is_empty() => { let _ = completed; },
            accepted = listener.accept() => match accepted {
                Err(error) => {
                    if stop.is_cancelled() || owner_stop.is_cancelled() { break Err(Box::new(ServerClosed) as BoxError); }
                    if accept_backoff.retry(&context, &error).await { continue; }
                    break Err(Box::new(error) as BoxError);
                }
                Ok((socket, _)) => {
                    accept_backoff.accepted();
                    let (context, mux, stop, owner_stop, handlers) = (context.clone(), mux.clone(), stop.clone(), owner_stop.clone(), Arc::clone(&handlers));
                    connections.spawn(async move { serve_connection(socket, context, mux, stop, owner_stop, handlers, read_header_timeout, HttpProtocol::Http1).await });
                }
            }
        }
    };
    drop(listener);
    retain_connection_drain(&context, connections, handlers, owner_stop).await;
    result
}

#[cgp_component(HttpServerRunner)]
pub trait CanRunHttpServer: Sync {
    fn run_http_server<'a>(&'a self, mux: &'a SourceMux, port: &'a str, stop: CancellationToken, read_header_timeout: Duration) -> BoxFuture<'a, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl HttpServerRunner<UtilityRuntimeContext> for RunNativeHttpServer {
    fn run_http_server<'a>(context: &'a UtilityRuntimeContext, mux: &'a SourceMux, port: &'a str, stop: CancellationToken, read_header_timeout: Duration) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(serve_plain(context.clone(), mux.clone(), port, stop, read_header_timeout))
    }
}
