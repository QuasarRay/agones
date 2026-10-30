//! Native HTTP routing, request bodies and streaming responses.
//! Axum owns routing; Hyper owns HTTP framing and protocol state. CGP selects
//! server/request policy and the existing task scope owns application futures.
use std::{collections::BTreeMap, error::Error, fmt, future::Future, io,
    pin::Pin, sync::{Arc, RwLock, Weak}, task::{Context as TaskContext, Poll}, time::Duration};
use axum::{body::Body, extract::{Request, State}, routing::any, Router};
use bytes::Bytes;
use cgp::prelude::*;
use futures::{future::BoxFuture, FutureExt, Stream};
use http_body_util::BodyExt;
use hyper::{header::{HeaderMap, HeaderName, HeaderValue}, Method, Response, StatusCode, Uri};
use hyper::service::{service_fn, Service};
use hyper_util::{rt::{TokioIo, TokioTimer}, server::conn::auto, service::TowerToHyperService};
use serde::{de::DeserializeOwned, Serialize};
use tokio::{io::{AsyncRead, AsyncWrite}, net::TcpListener, sync::{mpsc, oneshot}, task::JoinSet};
use tokio_rustls::TlsAcceptor;
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, logfields::LogEntry,
    utility_logging::{CanEmitSourceLog, HasSourceLogger, LogLevel},
    utility_runtime::UtilityRuntimeContext, utility_scope::{CanSpawnUtilityTask, UtilityTaskScope}};

pub type HttpHandler = Arc<dyn for<'a> Fn(&'a UtilityRuntimeContext, &'a mut ResponseWriter, &'a mut HttpRequest) -> BoxFuture<'a, ()> + Send + Sync>;
pub type ErrorHandler = Arc<dyn for<'a> Fn(&'a UtilityRuntimeContext, &'a mut ResponseWriter, &'a mut HttpRequest) -> BoxFuture<'a, Result<(), BoxError>> + Send + Sync>;
pub use hyper::HeaderMap as Headers;

pub fn header(headers: &HeaderMap, name: &str) -> Option<Bytes> { headers.get(name).map(|value| Bytes::copy_from_slice(value.as_bytes())) }
pub fn set_header(headers: &mut HeaderMap, name: &str, value: &str) -> Result<(), BoxError> {
    headers.insert(HeaderName::from_bytes(name.as_bytes())?, HeaderValue::from_str(value)?); Ok(())
}
pub fn headers_json(headers: &HeaderMap) -> serde_json::Value {
    let values: BTreeMap<_, _> = headers.keys().map(|name| (name.as_str(), headers.get_all(name).iter()
        .map(|value| String::from_utf8_lossy(value.as_bytes()).into_owned()).collect::<Vec<_>>())).collect();
    serde_json::json!(values)
}

pub struct HttpRequest { method: Method, uri: Uri, path: Bytes, pub headers: HeaderMap, body: Body, pub cancelled: CancellationToken }
impl HttpRequest {
    fn new(request: Request, cancelled: CancellationToken) -> Self {
        let (parts, body) = request.into_parts();
        let path = Bytes::from(percent_encoding::percent_decode_str(parts.uri.path()).collect::<Vec<_>>());
        Self { method: parts.method, uri: parts.uri, path, headers: parts.headers, body, cancelled }
    }
    pub fn method(&self) -> &Method { &self.method }
    pub fn path(&self) -> Bytes { self.path.clone() }
    pub fn uri(&self) -> &Uri { &self.uri }
    pub fn host(&self) -> &str {
        self.uri.authority().map(|value| value.as_str()).or_else(|| self.headers.get(hyper::header::HOST).and_then(|value| value.to_str().ok())).unwrap_or_default()
    }
    pub async fn read_body(&mut self) -> Result<Bytes, BoxError> {
        let body = std::mem::replace(&mut self.body, Body::empty());
        tokio::select! {
            result = body.collect() => Ok(result?.to_bytes()),
            _ = self.cancelled.cancelled() => Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, "HTTP request cancelled")) as BoxError),
        }
    }
    pub async fn decode_one_json<T: DeserializeOwned>(&mut self) -> Result<T, BoxError> {
        let bytes = self.read_body().await?;
        let mut decoder = serde_json::Deserializer::from_slice(&bytes);
        Ok(T::deserialize(&mut decoder)?)
    }
}
impl Drop for HttpRequest { fn drop(&mut self) { self.cancelled.cancel(); } }
struct ResponseHead { status: StatusCode, headers: HeaderMap }
pub struct ResponseWriter {
    pub headers: HeaderMap, head: Option<oneshot::Sender<ResponseHead>>,
    body: mpsc::Sender<Result<Bytes, io::Error>>, cancelled: CancellationToken,
}
impl ResponseWriter {
    fn commit(&mut self, status: StatusCode) -> Result<(), BoxError> {
        if let Some(head) = self.head.take() {
            head.send(ResponseHead { status, headers: self.headers.clone() })
                .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "HTTP response receiver closed"))?;
        }
        Ok(())
    }
    pub async fn write_header(&mut self, status: u16) -> Result<(), BoxError> { self.commit(StatusCode::from_u16(status)?) }
    pub async fn write(&mut self, value: &[u8]) -> Result<usize, BoxError> {
        self.commit(StatusCode::OK)?;
        tokio::select! {
            result = self.body.send(Ok(Bytes::copy_from_slice(value))) => {
                result.map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "HTTP response body closed"))?;
                Ok(value.len())
            }
            _ = self.cancelled.cancelled() => Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, "HTTP request cancelled")) as BoxError),
        }
    }
    pub async fn encode_json<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), BoxError> {
        if !self.headers.contains_key(hyper::header::CONTENT_TYPE) { set_header(&mut self.headers, "content-type", "application/json")?; }
        let mut bytes = serde_json::to_vec(value)?; bytes.push(b'\n'); self.write(&bytes).await?; Ok(())
    }
    pub async fn http_error(&mut self, message: &str, status: u16) {
        self.headers.remove(hyper::header::CONTENT_LENGTH);
        let _ = set_header(&mut self.headers, "content-type", "text/plain; charset=utf-8");
        let _ = self.write_header(status).await;
        let _ = self.write(format!("{message}\n").as_bytes()).await;
    }
    pub async fn flush(&mut self) -> Result<(), BoxError> { self.commit(StatusCode::OK) }
    async fn fail(&mut self) {
        if self.head.is_some() { self.http_error("internal server error", 500).await; }
        else { let _ = self.body.send(Err(io::Error::other("HTTP handler panicked"))).await; }
    }
}
struct ResponseStream { receiver: mpsc::Receiver<Result<Bytes, io::Error>>, cancelled: CancellationToken }
impl Stream for ResponseStream {
    type Item = Result<Bytes, io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, context: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> { self.receiver.poll_recv(context) }
}
impl Drop for ResponseStream { fn drop(&mut self) { self.cancelled.cancel(); } }

#[cgp_component(NativeHttpHandlerRunner)]
pub trait CanRunNativeHttpHandler {
    fn run_native_http_handler<'a>(&'a self, handler: &'a HttpHandler, writer: &'a mut ResponseWriter, request: &'a mut HttpRequest) -> BoxFuture<'a, ()>;
}
#[cgp_new_provider]
impl NativeHttpHandlerRunner<UtilityRuntimeContext> for RunRustHttpHandler {
    fn run_native_http_handler<'a>(context: &'a UtilityRuntimeContext, handler: &'a HttpHandler, writer: &'a mut ResponseWriter, request: &'a mut HttpRequest) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if std::panic::AssertUnwindSafe(handler(context, writer, request)).catch_unwind().await.is_err() {
                let logger = LogEntry::from_logger(Arc::clone(context.source_logger())).with_field("path", request.uri().path());
                context.emit_source_log(&logger, LogLevel::Error, "HTTP handler panicked");
                writer.fail().await;
            }
        })
    }
}
async fn dispatch(context: UtilityRuntimeContext, handler: HttpHandler, request: Request) -> Response<Body> {
    let cancelled = match context.utility_child_token() {
        Ok(token) if !token.is_cancelled() => token,
        _ => return Response::builder().status(StatusCode::SERVICE_UNAVAILABLE).body(Body::from("HTTP owner is closing\n")).expect("constant HTTP response"),
    };
    let pending = cancelled.clone().drop_guard();
    let mut request = HttpRequest::new(request, cancelled.clone());
    let (head, ready) = oneshot::channel();
    let (body, receiver) = mpsc::channel(2);
    let mut writer = ResponseWriter { headers: HeaderMap::new(), head: Some(head), body, cancelled: cancelled.clone() };
    let runtime = context.clone();
    if context.spawn_utility_task("native HTTP request", Box::pin(async move {
        runtime.run_native_http_handler(&handler, &mut writer, &mut request).await;
        let _ = writer.commit(StatusCode::OK);
        Ok(())
    })).is_err() {
        return Response::builder().status(StatusCode::SERVICE_UNAVAILABLE).body(Body::from("HTTP owner is closing\n")).expect("constant HTTP response");
    }
    match ready.await {
        Ok(head) => {
            pending.disarm();
            let mut response = Response::new(Body::from_stream(ResponseStream { receiver, cancelled }));
            *response.status_mut() = head.status; *response.headers_mut() = head.headers; response
        }
        Err(_) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from("HTTP handler ended without a response\n")).expect("constant HTTP response"),
    }
}

#[derive(Clone)]
pub struct SourceMux { routes: Arc<RwLock<Router<UtilityRuntimeContext>>> }
impl Default for SourceMux { fn default() -> Self { Self { routes: Arc::new(RwLock::new(Router::new())) } } }
impl SourceMux {
    pub fn handle(&self, path: &str, handler: HttpHandler) {
        let endpoint = move |State(context): State<UtilityRuntimeContext>, request: Request| {
            let handler = Arc::clone(&handler); async move { dispatch(context, handler, request).await }
        };
        let mut routes = self.routes.write().unwrap_or_else(|error| error.into_inner());
        *routes = if path == "/" { routes.clone().fallback(endpoint) }
            else if path.ends_with('/') { routes.clone().route(path, any(endpoint.clone())).route(&format!("{path}*rest"), any(endpoint)) }
            else { routes.clone().route(path, any(endpoint)) };
    }
    fn service(&self, context: UtilityRuntimeContext) -> Router { self.routes.read().unwrap_or_else(|error| error.into_inner()).clone().with_state(context) }
}
#[derive(Debug)]
pub struct ServerClosed;
impl fmt::Display for ServerClosed { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("HTTP server closed") } }
impl Error for ServerClosed {}
pub fn is_direct_server_closed(error: &(dyn Error + 'static)) -> bool { error.is::<ServerClosed>() }
pub fn is_server_closed(error: &(dyn Error + 'static)) -> bool { crate::errors::contains_error::<ServerClosed>(error) }

#[derive(Clone)]
struct ProtocolExecutor(Weak<UtilityTaskScope>);
impl<F> hyper::rt::Executor<F> for ProtocolExecutor where F: Future<Output = ()> + Send + 'static {
    fn execute(&self, future: F) {
        if let Some(scope) = self.0.upgrade() { let _ = scope.spawn("native HTTP protocol", Box::pin(async move { future.await; Ok(()) })); }
    }
}
trait NativeIo: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> NativeIo for T {}
async fn serve_connection(stream: Box<dyn NativeIo>, context: UtilityRuntimeContext, mux: SourceMux,
    stop: CancellationToken, owner_stop: CancellationToken, timeout: Duration) -> Result<(), BoxError> {
    let protocols = Arc::new(UtilityTaskScope::reporting());
    let result = {
        let service = service_fn(move |request| {
            let service = TowerToHyperService::new(mux.service(context.clone()));
            async move { service.call(request).await }
        });
        let mut builder = auto::Builder::new(ProtocolExecutor(Arc::downgrade(&protocols)));
        builder.http1().timer(TokioTimer::new()).header_read_timeout(timeout);
        builder.http2().timer(TokioTimer::new());
        let connection = builder.serve_connection_with_upgrades(TokioIo::new(stream), service);
        tokio::pin!(connection);
        tokio::select! {
            result = &mut connection => result,
            _ = async { tokio::select! { _ = stop.cancelled() => (), _ = owner_stop.cancelled() => () } } => {
                connection.as_mut().graceful_shutdown(); connection.await
            }
        }
    };
    let mut child_errors = protocols.cancel_and_join().await;
    result?;
    if let Some((_, error)) = child_errors.pop() { return Err(error); }
    Ok(())
}
pub async fn bind_listener(port: &str) -> Result<TcpListener, BoxError> {
    let port: u16 = port.parse().map_err(|_| Errors::default().New(format!("invalid HTTP port: {port}")))?;
    fn socket(port: u16, ipv6: bool) -> io::Result<std::net::TcpListener> {
        let domain = if ipv6 { socket2::Domain::IPV6 } else { socket2::Domain::IPV4 };
        let socket = socket2::Socket::new(domain, socket2::Type::STREAM, Some(socket2::Protocol::TCP))?;
        socket.set_reuse_address(true)?;
        socket.set_nonblocking(true)?;
        let address = if ipv6 {
            socket.set_only_v6(false)?;
            std::net::SocketAddr::from((std::net::Ipv6Addr::UNSPECIFIED, port))
        } else { std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, port)) };
        socket.bind(&address.into())?;
        socket.listen(1024)?;
        Ok(socket.into())
    }
    let listener = match socket(port, true) {
        Ok(listener) => listener,
        Err(error) if error.kind() == io::ErrorKind::AddrInUse => return Err(Box::new(error)),
        Err(_) => socket(port, false)?,
    };
    Ok(TcpListener::from_std(listener)?)
}
pub async fn serve_listener(context: UtilityRuntimeContext, mux: SourceMux, listener: TcpListener,
    stop: CancellationToken, read_header_timeout: Duration, tls: Option<TlsAcceptor>) -> Result<(), BoxError> {
    let owner_stop = context.utility_child_token()?;
    let connection_stop = stop.child_token();
    let mut connections = JoinSet::new();
    let result = loop {
        tokio::select! {
            _ = stop.cancelled() => break Err(Box::new(ServerClosed) as BoxError),
            _ = owner_stop.cancelled() => break Err(Box::new(ServerClosed) as BoxError),
            done = connections.join_next(), if !connections.is_empty() => {
                if let Some(result) = done {
                    match result {
                        Ok(Ok(())) => (),
                        Ok(Err(error)) => connection_log(&context, LogLevel::Debug, error.as_ref(), "HTTP connection ended"),
                        Err(error) => connection_log(&context, LogLevel::Error, &error, "HTTP connection task failed"),
                    }
                }
            }
            accepted = listener.accept() => match accepted {
                Err(error) if matches!(error.kind(), io::ErrorKind::Interrupted | io::ErrorKind::ConnectionAborted | io::ErrorKind::ConnectionReset) => continue,
                Err(error) => break Err(Box::new(error) as BoxError),
                Ok((socket, peer)) => {
                    let (runtime, mux, stop, owner_stop, tls) = (context.clone(), mux.clone(), connection_stop.clone(), owner_stop.clone(), tls.clone());
                    connections.spawn(async move {
                        let connection = async move {
                        let stream: Box<dyn NativeIo> = if let Some(acceptor) = tls {
                            let accepted = tokio::select! {
                                result = tokio::time::timeout(read_header_timeout, acceptor.accept(socket)) => result??,
                                _ = stop.cancelled() => return Ok(()),
                                _ = owner_stop.cancelled() => return Ok(()),
                            };
                            Box::new(accepted)
                        } else { Box::new(socket) };
                        serve_connection(stream, runtime, mux, stop, owner_stop, read_header_timeout).await
                        }.await;
                        connection.map_err(|error| Errors::default().Wrap(Some(error), &format!("HTTP connection from {peer}"))
                            .expect("connection error is present"))
                    });
                }
            }
        }
    };
    drop(listener);
    connection_stop.cancel();
    if result.as_ref().err().is_some_and(|error| !is_server_closed(error.as_ref())) { connections.abort_all(); }
    // Listener shutdown returns to its caller while the same CGP owner retains
    // and joins connection drain. Rejected ownership is drained in this future.
    let runtime = context.clone();
    let drain = Box::pin(async move {
        while let Some(done) = connections.join_next().await {
            match done {
                Ok(Ok(())) => (),
                Ok(Err(error)) => connection_log(&runtime, LogLevel::Debug, error.as_ref(), "HTTP connection drain ended"),
                Err(error) if error.is_cancelled() => (),
                Err(error) => connection_log(&runtime, LogLevel::Error, &error, "HTTP connection drain failed"),
            }
        }
        Ok(())
    });
    if let Err(drain) = context.spawn_utility_task_or_return("native HTTP connection drain", drain) { drain.await?; }
    result
}
fn connection_log(context: &UtilityRuntimeContext, level: LogLevel, error: &(dyn Error + Send + Sync + 'static), message: &str) {
    let logger = LogEntry::from_logger(Arc::clone(context.source_logger())).with_error(Some(error));
    context.emit_source_log(&logger, level, message);
}
#[cgp_component(NativeHttpServerRunner)]
pub trait CanRunHttpServer: Sync {
    fn run_http_server<'a>(&'a self, mux: &'a SourceMux, port: &'a str, stop: CancellationToken, timeout: Duration) -> BoxFuture<'a, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl NativeHttpServerRunner<UtilityRuntimeContext> for RunRustHttpServer {
    fn run_http_server<'a>(context: &'a UtilityRuntimeContext, mux: &'a SourceMux, port: &'a str, stop: CancellationToken, timeout: Duration) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            if stop.is_cancelled() || context.utility_child_token()?.is_cancelled() { return Err(Box::new(ServerClosed) as BoxError); }
            let listener = bind_listener(port).await?;
            serve_listener(context.clone(), mux.clone(), listener, stop, timeout, None).await
        })
    }
}
