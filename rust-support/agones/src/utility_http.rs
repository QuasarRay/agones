//! Native HTTP effects behind the source mux and response-writer contracts.
//! Request handlers stream through bounded channels into actual Hyper bodies;
//! a disconnected consumer is a write error, not a synthetic successful write.
use std::{collections::BTreeMap, error::Error, fmt, io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr}, pin::Pin, sync::{Arc, atomic::{AtomicBool, Ordering}}, task::{Context, Poll}, time::Duration};
use bytes::Bytes;
use futures::{future::BoxFuture, FutureExt};
use hyper::{body::{Body, Frame, Incoming, SizeHint}, Request, Response};
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo, TokioTimer};
use serde::de::DeserializeOwned;
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
#[path = "utility_http_headers.rs"]
mod headers;
pub use headers::{HeaderName, HeaderValues, HeaderAllocationProfile, SourceSliceAllocationError, SourceSliceAllocationLayout,
    Headers, SourceByteString, canonical_header_key, try_canonical_header_key, set_header, try_set_header, header, headers_json};
pub(crate) use headers::reserve_source_array;
#[path = "utility_http_allocation.rs"]
mod allocation;
pub use allocation::{SourceRuntimeAllocationError, SourceNilMapAssignment, NativeSourceCollectionContext,
    CanRaiseSourceCollectionFailure, SourceCollectionFailureRaiser, SourceCollectionFailureRaiserComponent,
    RaiseSourceCollectionFailure, complete_source_collection};
pub(crate) use allocation::{raise_source_allocation_failure, unwind_source_panic};
pub use crate::utility_panic::{SourceAbortHandler, ERR_ABORT_HANDLER as ErrAbortHandler};
#[path = "utility_http_panic.rs"]
mod panic_reporting;
pub use panic_reporting::{CanReportSourceHttpPanic, SourceHttpPanicReporter,
    SourceHttpPanicReporterComponent, ReportNativeSourceHttpPanic};
#[path = "utility_http_trailers.rs"]
mod trailers;
#[path = "utility_http_listener.rs"]
mod listener;
pub(crate) use listener::AcceptBackoff;
#[path = "utility_http_body.rs"]
mod request_body;
#[path = "utility_http_request_body.rs"]
mod request_body_value;
pub use request_body_value::{SourceBody, SourceBodyRead, SourceReadError, SourceReadCloser,
    SourceRequestBodyIo, SourceRequestBodyIoComponent, CanAccessSourceRequestBody, AccessSourceRequestBody};
#[path = "utility_http_request_fields.rs"]
mod request_fields;
pub use request_fields::RequestFields;
#[path = "utility_http_url.rs"]
mod url;
pub use url::{SourceUrl, SourceUrlFields, SourceUserinfo};
#[path = "utility_http_form.rs"]
mod form;
pub use form::{SourceValues, SourceQueryProfile, SourceQuerySettings, SourceQueryParse, SourceUrlEscapeError,
    parse_source_query, HasSourceQuerySettings, SourceQuerySettingsGetterComponent};
#[path = "utility_http_unicode.rs"]
mod unicode;
pub use unicode::{SourceUnicodeProfile, HasSourceUnicodeProfile, SourceUnicodeProfileGetterComponent,
    CanUppercaseSourceString, SourceUppercaseMapper, SourceUppercaseMapperComponent, MapSourceUppercase, source_to_upper};
#[path = "utility_http_response.rs"]
mod response;
pub use response::ResponseWriter;

pub type HttpHandler = Arc<dyn for<'a> Fn(&'a UtilityRuntimeContext, &'a mut ResponseWriter, &'a mut HttpRequest) -> BoxFuture<'a, ()> + Send + Sync>;
pub type ErrorHandler = Arc<dyn for<'a> Fn(&'a UtilityRuntimeContext, &'a mut ResponseWriter, &'a mut HttpRequest) -> BoxFuture<'a, Result<(), BoxError>> + Send + Sync>;

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
    fields: RequestFields,
    pub headers: Headers,
    protocol_at_least_11: bool,
    protocol: hyper::Version,
    informational: Option<hyper::ext::InformationalSender>,
    continue_state: request_body_value::ContinueState,
    wants10_keep_alive: bool,
    wants_close: bool,
    pub matched_pattern: Vec<u8>,
    path_values: BTreeMap<String, Vec<u8>>,
    /// The request owner cancels on handler return or transport/body disposal.
    /// Server Shutdown does not cancel an otherwise active request.
    pub cancelled: CancellationToken,
    original_body: request_body::RequestBody,
    runtime: UtilityRuntimeContext,
}
impl HttpRequest {
    fn from_hyper(runtime: UtilityRuntimeContext, request: Request<Incoming>) -> Result<Self, ()> {
        let (parts, body) = request.into_parts();
        let content_length = body.size_hint().exact().and_then(|length| i64::try_from(length).ok()).unwrap_or(-1);
        let request_uri = parts.uri.to_string();
        let connect_authority = parts.method == hyper::Method::CONNECT && !request_uri.starts_with('/');
        let raw_path = if connect_authority { "" } else { parts.uri.path() };
        let url_path = mux::path_unescape(raw_path.as_bytes()).ok_or(())?;
        let headers = Headers::new();
        for name in parts.headers.keys() {
            let values = parts.headers.get_all(name).iter().map(|value| Bytes::copy_from_slice(value.as_bytes()));
            headers.insert(canonical_header_key(name.as_str()), HeaderValues::parsed(values, headers.allocation_profile()));
        }
        if parts.version == hyper::Version::HTTP_2 {
            if let Some(cookies) = headers.get("Cookie").filter(|values| values.len() > 1) {
                let mut joined = Vec::new();
                for (index, value) in cookies.iter().enumerate() { if index != 0 { joined.extend_from_slice(b"; "); } joined.extend_from_slice(&value); }
                headers.insert("Cookie", HeaderValues::from_values([Bytes::from(joined)]));
            }
        }
        let initial_trailers = trailers::request_trailers(&headers, parts.version == hyper::Version::HTTP_2,
            parts.version != hyper::Version::HTTP_2 && content_length < 0)?;
        let host = parts.uri.authority().map(|authority| Bytes::copy_from_slice(authority.as_str().as_bytes()))
            .or_else(|| header(&headers, "Host")).unwrap_or_default();
        // net/http moves Host out of Request.Header into Request.Host.
        headers.remove("Host");
        let scheme = if connect_authority { "" } else { parts.uri.scheme_str().unwrap_or_default() };
        let authority = parts.uri.authority().map_or("", |value| value.as_str());
        let raw_query = parts.uri.query().unwrap_or_default();
        let can_write_continue = if parts.version == hyper::Version::HTTP_2 {
            let expected = headers.get("Expect").is_some_and(|values| values.iter().any(|value| header_has_comma_token(&value, b"100-continue")));
            if expected { headers.remove("Expect"); } // bundled httpcommon.NewServerRequest
            expected
        } else {
            content_length != 0 && parts.version != hyper::Version::HTTP_09 && parts.version != hyper::Version::HTTP_10
                && header(&headers, "Expect").is_some_and(|value| header_has_token(&value, b"100-continue"))
        };
        let informational = parts.extensions.get::<hyper::ext::InformationalSender>().cloned();
        let wants10_keep_alive = parts.version == hyper::Version::HTTP_10
            && header(&headers, "Connection").is_some_and(|value| header_has_token(&value, b"keep-alive"));
        let wants_close = (parts.version == hyper::Version::HTTP_10 && !wants10_keep_alive) || parts.version == hyper::Version::HTTP_09
            || header(&headers, "Connection").is_some_and(|value| header_has_token(&value, b"close"));
        let continue_state = request_body_value::ContinueState::new(parts.version != hyper::Version::HTTP_2 && can_write_continue, informational.clone());
        let original_body = request_body::RequestBody::new(body, content_length, parts.version == hyper::Version::HTTP_2, wants_close, initial_trailers);
        let body = SourceBody::from_native(original_body.clone(), can_write_continue, continue_state.clone(), informational.clone());
        let raw_path_hint = if raw_path == "*" || mux::escape_url_path(&url_path) == raw_path.as_bytes() { "" } else { raw_path };
        let url = SourceUrl::new(SourceUrlFields { scheme: Bytes::copy_from_slice(scheme.as_bytes()), host: Bytes::copy_from_slice(authority.as_bytes()),
            path: Bytes::from(url_path), raw_path: Bytes::copy_from_slice(raw_path_hint.as_bytes()), raw_query: Bytes::copy_from_slice(raw_query.as_bytes()),
            force_query: parts.uri.query() == Some(""), ..Default::default() });
        let fields = RequestFields::new(Bytes::copy_from_slice(parts.method.as_str().as_bytes()), content_length, body,
            Some(url), host, Bytes::from(request_uri));
        Ok(Self { fields, headers,
            protocol_at_least_11: parts.version != hyper::Version::HTTP_09 && parts.version != hyper::Version::HTTP_10,
            protocol: parts.version, informational, continue_state,
            wants10_keep_alive, wants_close,
            matched_pattern: Vec::new(), path_values: BTreeMap::new(),
            cancelled: CancellationToken::new(), original_body, runtime })
    }
    /// Retain this Request's field owner without allowing replacement of the
    /// owner itself. Its setters mutate fields seen by the response writer.
    pub fn fields(&self) -> RequestFields { self.fields.clone() }
    pub fn method(&self) -> Bytes { self.fields.method() }
    pub fn set_method(&self, method: impl AsRef<[u8]>) { self.fields.set_method(method); }
    pub fn content_length(&self) -> i64 { self.fields.content_length() }
    pub fn set_content_length(&self, length: i64) { self.fields.set_content_length(length); }
    pub fn body(&self) -> SourceBody { self.fields.body() }
    pub fn set_body(&self, body: SourceBody) { self.fields.set_body(body); }
    pub fn url(&self) -> Option<SourceUrl> { self.fields.url() }
    pub fn set_url(&self, url: Option<SourceUrl>) { self.fields.set_url(url); }
    pub fn require_url(&self) -> SourceUrl { self.url().expect("invalid memory address or nil pointer dereference: Request.URL") }
    pub fn host(&self) -> Bytes { self.fields.host() }
    pub fn set_host(&self, host: impl AsRef<[u8]>) { self.fields.set_host(host); }
    pub fn request_uri(&self) -> Bytes { self.fields.request_uri() }
    pub fn set_request_uri(&self, uri: impl AsRef<[u8]>) { self.fields.set_request_uri(uri); }
    pub fn path(&self) -> Bytes { self.require_url().path() }
    pub fn form(&self) -> SourceValues { self.fields.form() }
    pub fn set_form(&self, values: SourceValues) { self.fields.set_form(values); }
    pub fn post_form(&self) -> SourceValues { self.fields.post_form() }
    pub fn set_post_form(&self, values: SourceValues) { self.fields.set_post_form(values); }
    pub(crate) fn runtime(&self) -> &UtilityRuntimeContext { &self.runtime }
    pub fn path_value(&self, name: &str) -> &[u8] { self.path_values.get(name).map_or(&[], Vec::as_slice) }
    pub fn set_path_value(&mut self, name: &str, value: Vec<u8>) { self.path_values.insert(name.to_owned(), value); }
    pub fn escaped_path(&self) -> Vec<u8> {
        self.require_url().escaped_path()
    }
    pub async fn read_body(&mut self) -> Result<Vec<u8>, BoxError> {
        let read = self.read_body_all().await;
        match read.error { Some(SourceReadError::Failure(error)) => Err(error), _ => Ok(read.data.to_vec()) }
    }
    /// io.ReadAll's buffer sizes are observable to a source io.Reader. Keep a
    /// logical source capacity independently of the Rust Vec allocation.
    /// Partial bytes survive a non-EOF error in this low-level result.
    pub async fn read_body_all(&self) -> SourceBodyRead {
        self.read_body_all_limited(None).await
    }
    /// The optional limit represents io.LimitReader wrapping the retained
    /// body; its exhaustion is EOF without another call to that body.
    pub async fn read_body_all_limited(&self, limit: Option<usize>) -> SourceBodyRead {
        let mut bytes = Vec::new();
        let profile = self.headers.allocation_profile();
        let mut capacity = 512;
        // io.ReadAll retains the initial Reader interface even if an alias
        // assigns another Body to the enclosing Request during a Read call.
        let body = self.body();
        loop {
            let read = match limit.map(|limit| limit - bytes.len()) {
                Some(0) => SourceBodyRead::eof(),
                remaining => self.read_body_handle_up_to(body.clone(), remaining.map_or(capacity - bytes.len(), |remaining| remaining.min(capacity - bytes.len()))).await,
            };
            bytes.extend_from_slice(&read.data);
            match read.error {
                Some(SourceReadError::Eof) => return SourceBodyRead::data(Bytes::from(bytes)),
                Some(error) => return SourceBodyRead { data: Bytes::from(bytes), error: Some(error) },
                None => (),
            }
            if bytes.len() == capacity { capacity = profile.grow_bytes(capacity, capacity + 1); }
        }
    }
    pub async fn read_body_up_to(&self, limit: usize) -> SourceBodyRead {
        self.read_body_handle_up_to(self.body(), limit).await
    }
    pub async fn read_body_handle_up_to(&self, body: SourceBody, limit: usize) -> SourceBodyRead {
        self.runtime.read_source_request_body(body, limit).await
    }
    pub async fn decode_one_json<T: DeserializeOwned>(&mut self) -> Result<T, BoxError> {
        let mut decoder = crate::utility_gateway_framing::SourceJsonDecoder::new(self.body(), self.headers.allocation_profile());
        let raw = decoder.read_raw(&self.runtime).await?.ok_or_else(|| Errors::default().New("EOF"))?;
        // This is one Decode call. The source's read-ahead is discarded with
        // that decoder; trailing values are not decoded or validated here.
        Ok(serde_json::from_str(raw.get())?)
    }
    pub async fn close_body(&mut self) -> Result<(), BoxError> { self.runtime.close_source_request_body(self.body()).await }
    pub async fn body_trailers(&self) -> Headers { self.original_body.trailers().await }
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
            if let Ok(value) = hyper::header::HeaderValue::from_bytes(&value) { response.headers_mut().append(name.clone(), value); }
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
    server_stops: [CancellationToken; 2], connection_closed: CancellationToken, remote_addr: Arc<str>, incoming: Request<Incoming>) -> Result<Response<ResponseBody>, io::Error> {
    let version = incoming.version();
    let mut request = match HttpRequest::from_hyper(context.clone(), incoming) {
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
        let handled = std::panic::AssertUnwindSafe(async {
            let rejected_expectation = request.protocol != hyper::Version::HTTP_2 && header(&request.headers, "Expect")
                .is_some_and(|value| !value.is_empty() && !header_has_token(&value, b"100-continue"));
            if rejected_expectation {
                set_header(&mut writer.headers, "Connection", "close"); let _ = writer.write_header(417).await;
            } else if request.method().as_ref() == b"OPTIONS" && request.request_uri().as_ref() == b"*" { global_options(&mut writer, &mut request).await; }
            else { mux.dispatch(&context, &mut writer, &mut request).await; }
        }).catch_unwind().await;
        if let Err(payload) = handled {
            let panic = crate::utility_panic::SourceRecoveredPanic::new(payload);
            if request.protocol == hyper::Version::HTTP_2 {
                // H2 cancellation precedes reporting. Dropping an unfinished
                // writer marks Body failure and wakes its existing consumer;
                // the existing Hyper driver owns the actual stream reset.
                request.cancelled.cancel();
                drop(writer);
                context.report_source_http_panic(true, &remote_addr, &panic);
            } else {
                // H1 reports before its deferred cancellation and original
                // reqBody cleanup, without committing a successful response.
                context.report_source_http_panic(false, &remote_addr, &panic);
                request.cancelled.cancel();
                request.continue_state.disable().await;
                drop(writer);
                let _ = request.original_body.close().await;
            }
            phase.finish(true);
            return Ok(());
        }
        request.cancelled.cancel(); // net/http cancels when ServeHTTP returns.
        let close = writer.finish().await;
        // finishRequest closes the body captured before the expectation
        // wrapper was installed, even when the handler replaces r.Body.
        let _ = request.original_body.close().await;
        let current = request.body();
        let early_close = match current.native_http1() { Some(body) => body.did_early_close().await, None => false };
        phase.finish(close || early_close);
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
    if request.content_length() == 0 { return; }
    let mut remaining = 4 << 10;
    loop {
        let read = request.read_body_up_to(remaining + 1).await;
        if read.data.len() > remaining {
            // Source MaxBytesReader calls requestTooLarge on the HTTP/1
            // response, closing this connection after the otherwise empty 200.
            if request.protocol != hyper::Version::HTTP_2 { set_header(&mut writer.headers, "Connection", "close"); }
            return;
        }
        remaining -= read.data.len();
        if read.error.is_some() { return; }
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
    serve_connection_from(io, context, mux, stop, owner_stop, handlers, read_header_timeout, protocol, String::new()).await
}

/// Native listeners supply the accepted peer; an address-less stream retains
/// the source empty RemoteAddr projection through serve_connection above.
pub async fn serve_connection_from<IO>(io: IO, context: UtilityRuntimeContext, mux: SourceMux, stop: CancellationToken,
    owner_stop: CancellationToken, handlers: Arc<UtilityTaskScope>, read_header_timeout: Duration, protocol: HttpProtocol,
    remote_addr: String) -> Result<(), BoxError>
where IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let source_shutdown = CancellationToken::new();
    let source_close = source_shutdown.clone();
    let connection_closed = CancellationToken::new();
    let _closed_guard = SourceConnectionClosed(connection_closed.clone());
    let request_connection = connection_closed.clone();
    let remote_addr: Arc<str> = remote_addr.into();
    let server_stop = stop.clone(); let server_owner_stop = owner_stop.clone();
    let service = service_fn(move |request| request_service(context.clone(), mux.clone(), Arc::clone(&handlers), source_close.clone(),
        [server_stop.clone(), server_owner_stop.clone()], request_connection.clone(), Arc::clone(&remote_addr), request));
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
                Ok((socket, peer)) => {
                    accept_backoff.accepted();
                    let (context, mux, stop, owner_stop, handlers) = (context.clone(), mux.clone(), stop.clone(), owner_stop.clone(), Arc::clone(&handlers));
                    connections.spawn(async move { serve_connection_from(socket, context, mux, stop, owner_stop, handlers, read_header_timeout, HttpProtocol::Http1, peer.to_string()).await });
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
