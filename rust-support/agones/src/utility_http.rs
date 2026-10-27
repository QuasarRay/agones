//! Native HTTP effects behind the source mux and response-writer contracts.
//! Request handlers stream through bounded channels into actual Hyper bodies;
//! a disconnected consumer is a write error, not a synthetic successful write.
use std::{collections::BTreeMap, error::Error, fmt, io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr}, pin::Pin, sync::{Arc, atomic::{AtomicBool, Ordering}}, task::{Context, Poll}, time::Duration};
use bytes::Bytes;
use futures::future::BoxFuture;
use http_body_util::BodyExt;
use hyper::{body::{Body, Frame, Incoming, SizeHint}, Request, Response};
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo, TokioTimer};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tokio::{io::{AsyncRead, AsyncWrite}, net::TcpListener, sync::{mpsc, oneshot}, task::JoinSet};
use tokio_util::sync::CancellationToken;
use cgp::prelude::*;
use crate::{errors::{BoxError, Errors}, utility_runtime::UtilityRuntimeContext, utility_scope::{CanSpawnUtilityTask, HasUtilityTaskScope, UtilityTaskScope}};

#[path = "utility_http_mux.rs"]
mod mux;
pub use mux::{SourceMux, MuxProfile};

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
    pub matched_pattern: Vec<u8>,
    path_values: BTreeMap<String, Vec<u8>>,
    /// The request owner cancels on handler return or transport/body disposal.
    /// Server Shutdown does not cancel an otherwise active request.
    pub cancelled: CancellationToken,
    body: Option<Incoming>,
    body_closed: bool,
    body_eof: bool,
}
impl HttpRequest {
    fn from_hyper(request: Request<Incoming>) -> Result<Self, ()> {
        let (parts, body) = request.into_parts();
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
        let raw_path_log = if raw_path == "*" || mux::escape_url_path(&url_path) == raw_path.as_bytes() { "" } else { raw_path };
        let url_log_value = json!({"Scheme":scheme,"Opaque":"","User":null,"Host":authority,"Path":path,
            "RawPath":raw_path_log,"OmitHost":false,"ForceQuery":parts.uri.query()==Some(""),"RawQuery":raw_query,"Fragment":"","RawFragment":""});
        Ok(Self { method: parts.method.to_string(), path, host, request_uri, url_log_value, headers,
            url_path, url_raw_path: raw_path_log.as_bytes().to_vec(), url_host: authority.to_owned(), url_raw_query: raw_query.as_bytes().to_vec(),
            protocol_at_least_11: parts.version != hyper::Version::HTTP_09 && parts.version != hyper::Version::HTTP_10,
            matched_pattern: Vec::new(), path_values: BTreeMap::new(),
            cancelled: CancellationToken::new(), body: Some(body), body_closed: false, body_eof: false })
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
        if self.body_eof { return Ok(None); }
        if let Some(body) = &mut self.body {
            while let Some(frame) = body.frame().await {
                if let Ok(bytes) = frame?.into_data() { return Ok(Some(bytes)); }
            }
        }
        self.body_eof = true;
        Ok(None)
    }
    /// Streaming decoders retain their own read-ahead across Decode calls.
    pub async fn read_body_chunk(&mut self) -> Result<Option<Bytes>, BoxError> {
        self.require_open_body()?;
        self.next_data().await
    }
    fn require_open_body(&self) -> Result<(), BoxError> {
        if self.body_closed { Err(Errors::default().New("http: invalid Read on closed Body")) } else { Ok(()) }
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
    pub fn close_body(&mut self) { self.body_closed = true; self.body.take(); }
}
impl Drop for HttpRequest {
    fn drop(&mut self) { self.cancelled.cancel(); }
}
struct ResponseHead { status: u16, headers: Headers }
pub struct ResponseWriter {
    pub headers: Headers,
    committed: Option<u16>,
    committed_headers: Option<Headers>,
    head: Option<oneshot::Sender<ResponseHead>>,
    data: mpsc::Sender<Bytes>,
    panicked: Arc<AtomicBool>,
    head_request: bool,
    written: u64,
    buffered: Vec<u8>,
    flushed: bool,
    finished: bool,
}
impl ResponseWriter {
    pub async fn write_header(&mut self, status: u16) -> Result<(), BoxError> {
        if self.committed.is_some() { return Ok(()); }
        if !(100..=999).contains(&status) { panic!("invalid WriteHeader code {status}"); }
        self.committed = Some(status);
        self.committed_headers = Some(self.headers.clone());
        Ok(())
    }
    async fn send_head(&mut self) -> Result<(), BoxError> {
        if self.committed.is_none() { self.write_header(200).await?; }
        if let Some(sender) = self.head.take() {
            sender.send(ResponseHead { status: self.committed.unwrap_or(200), headers: self.committed_headers.clone().unwrap_or_default() })
                .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "HTTP response consumer closed"))?;
        }
        Ok(())
    }
    pub async fn write(&mut self, bytes: &[u8]) -> Result<usize, BoxError> {
        if self.committed.is_none() {
            if !bytes.is_empty() && header(&self.headers, "Content-Type").is_none() && header(&self.headers, "Content-Encoding").is_none() {
                set_header(&mut self.headers, "Content-Type", sniff_content_type(bytes));
            }
            self.write_header(200).await?;
        }
        let status = self.committed.unwrap_or(200);
        if (100..=199).contains(&status) || status == 204 || status == 304 {
            return Err(Errors::default().New("http: request method or response status code does not allow body"));
        }
        if !bytes.is_empty() && !self.flushed {
            if let Some(headers) = &mut self.committed_headers {
                if header(headers, "Content-Type").is_none() && header(headers, "Content-Encoding").is_none() {
                    set_header(headers, "Content-Type", sniff_content_type(bytes));
                }
            }
        }
        let headers = self.committed_headers.as_ref().unwrap_or(&self.headers);
        if let Some(length) = header(headers, "Content-Length").and_then(|value| std::str::from_utf8(value).ok()).and_then(|value| value.parse::<u64>().ok()) {
            if self.written.saturating_add(bytes.len() as u64) > length {
                return Err(Errors::default().New("http: wrote more than the declared Content-Length"));
            }
        }
        self.written = self.written.saturating_add(bytes.len() as u64);
        if !self.flushed && self.buffered.len().saturating_add(bytes.len()) <= 2048 {
            self.buffered.extend_from_slice(bytes);
            return Ok(bytes.len());
        }
        self.flush().await?;
        if !self.head_request && !bytes.is_empty() {
            self.data.send(Bytes::copy_from_slice(bytes)).await
                .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "HTTP response consumer closed"))?;
        }
        Ok(bytes.len())
    }
    pub async fn flush(&mut self) -> Result<(), BoxError> {
        self.flushed = true;
        self.send_head().await?;
        if !self.head_request && !self.buffered.is_empty() {
            let bytes = Bytes::from(std::mem::take(&mut self.buffered));
            self.data.send(bytes).await.map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "HTTP response consumer closed"))?;
        } else { self.buffered.clear(); }
        Ok(())
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
    async fn finish(mut self) {
        if self.committed.is_none() { let _ = self.write_header(200).await; }
        if !self.flushed {
            let status = self.committed.unwrap_or(200);
            if let Some(headers) = &mut self.committed_headers {
                if status >= 200 && status != 204 && status != 304 && header(headers, "Content-Length").is_none() && header(headers, "Transfer-Encoding").is_none() {
                    set_header(headers, "Content-Length", self.written.to_string());
                }
            }
        }
        let _ = self.flush().await;
        self.finished = true;
    }
}
impl Drop for ResponseWriter {
    fn drop(&mut self) {
        // Async supervisors can drop a caught future after unwinding ended.
        // Cancellation is also an aborted response, even without a panic.
        if !self.finished { self.panicked.store(true, Ordering::Release); }
    }
}
fn sniff_content_type(bytes: &[u8]) -> &'static str {
    // The source utility responses set JSON explicitly or emit text. Retain
    // the default writer's binary/text distinction for other plain callbacks.
    if bytes.iter().take(512).any(|byte| *byte <= 0x08 || *byte == 0x0b || (0x0e..=0x1a).contains(byte) || (0x1c..=0x1f).contains(byte)) {
        "application/octet-stream"
    } else { "text/plain; charset=utf-8" }
}

struct ResponseBody { receiver: mpsc::Receiver<Bytes>, panicked: Arc<AtomicBool>, ended: bool, cancelled: CancellationToken }
impl Drop for ResponseBody {
    fn drop(&mut self) { self.cancelled.cancel(); }
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
            Poll::Ready(Some(bytes)) => Poll::Ready(Some(Ok(Frame::data(bytes)))),
            Poll::Ready(None) => { self.ended = true; Poll::Ready(None) }
            Poll::Pending => Poll::Pending,
        }
    }
    fn size_hint(&self) -> SizeHint { SizeHint::default() }
}

#[derive(Debug)]
pub struct ServerClosed;
impl fmt::Display for ServerClosed { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("http: Server closed") } }
impl Error for ServerClosed {}
pub fn is_server_closed(error: &(dyn Error + 'static)) -> bool { crate::errors::contains_error::<ServerClosed>(error) }

async fn request_service(context: UtilityRuntimeContext, mux: SourceMux, tasks: Arc<UtilityTaskScope>, incoming: Request<Incoming>) -> Result<Response<ResponseBody>, io::Error> {
    let version = incoming.version();
    let mut request = match HttpRequest::from_hyper(incoming) {
        Ok(request) => request,
        Err(()) => {
            let (data, receiver) = mpsc::channel(2);
            let _ = data.try_send(Bytes::from_static(b"400 Bad Request")); drop(data);
            let mut response = Response::new(ResponseBody { receiver, panicked: Arc::new(AtomicBool::new(false)),
                ended: false, cancelled: CancellationToken::new() });
            *response.status_mut() = hyper::StatusCode::BAD_REQUEST;
            response.headers_mut().insert(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static("text/plain; charset=utf-8"));
            if version != hyper::Version::HTTP_2 { response.headers_mut().insert(hyper::header::CONNECTION, hyper::header::HeaderValue::from_static("close")); }
            return Ok(response);
        }
    };
    // Hyper may drop the service future before the response head is ready.
    // Until ResponseBody owns cancellation, this guard covers that interval.
    struct PendingRequest(Option<CancellationToken>);
    impl Drop for PendingRequest {
        fn drop(&mut self) { if let Some(stop) = &self.0 { stop.cancel(); } }
    }
    let mut pending = PendingRequest(Some(request.cancelled.clone()));
    let (head_tx, head_rx) = oneshot::channel();
    let (data, receiver) = mpsc::channel(2);
    let panicked = Arc::new(AtomicBool::new(false));
    let mut writer = ResponseWriter { headers: BTreeMap::new(), committed: None, committed_headers: None, head: Some(head_tx), data,
        panicked: Arc::clone(&panicked), head_request: request.method == "HEAD", written: 0, buffered: Vec::new(), flushed: false, finished: false };
    tasks.spawn("HTTP request", Box::pin(async move {
        mux.dispatch(&context, &mut writer, &mut request).await;
        request.cancelled.cancel(); // net/http cancels when ServeHTTP returns.
        writer.finish().await;
        Ok(())
    })).map_err(io::Error::other)?;
    let head = head_rx.await.map_err(|_| io::Error::other("HTTP handler ended without a response"))?;
    let mut response = Response::new(ResponseBody { receiver, panicked, ended: false,
        cancelled: pending.0.take().expect("pending HTTP request owns cancellation") });
    *response.status_mut() = hyper::StatusCode::from_u16(head.status).map_err(io::Error::other)?;
    for (name, values) in head.headers {
        let Ok(name) = hyper::header::HeaderName::from_bytes(name.as_bytes()) else { continue; };
        for value in values.iter() {
            if let Ok(value) = hyper::header::HeaderValue::from_bytes(value) { response.headers_mut().append(name.clone(), value); }
        }
    }
    Ok(response)
}

#[derive(Clone, Copy)]
pub enum HttpProtocol { Http1, Http2 }

pub async fn serve_connection<IO>(io: IO, context: UtilityRuntimeContext, mux: SourceMux, stop: CancellationToken,
    owner_stop: CancellationToken, handlers: Arc<UtilityTaskScope>, read_header_timeout: Duration, protocol: HttpProtocol) -> Result<(), BoxError>
where IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let service = service_fn(move |request| request_service(context.clone(), mux.clone(), Arc::clone(&handlers), request));
    let mut builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
    builder.http1().timer(TokioTimer::new()).header_read_timeout(read_header_timeout);
    builder = match protocol { HttpProtocol::Http1 => builder.http1_only(), HttpProtocol::Http2 => builder.http2_only() };
    let connection = builder.serve_connection(TokioIo::new(io), service);
    tokio::pin!(connection);
    tokio::select! {
        result = &mut connection => result.map_err(|error| Errors::default().New(error.to_string())),
        _ = async { tokio::select! { _ = stop.cancelled() => (), _ = owner_stop.cancelled() => () } } => {
            connection.as_mut().graceful_shutdown();
            // Independent drain: an already cancelled stop token does not
            // cancel the in-flight response or close its connection early.
            connection.await.map_err(|error| Errors::default().New(error.to_string()))
        }
    }
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
    let result = loop {
        tokio::select! {
            _ = stop.cancelled() => break Err(Box::new(ServerClosed) as BoxError),
            _ = owner_stop.cancelled() => break Err(Box::new(ServerClosed) as BoxError),
            completed = connections.join_next(), if !connections.is_empty() => { let _ = completed; },
            accepted = listener.accept() => match accepted {
                Err(error) => break Err(Box::new(error) as BoxError),
                Ok((socket, _)) => {
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
