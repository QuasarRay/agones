// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/https/https.go. Copyright Contributors to Agones a Series of LF
// Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::Arc;
use comprende::c;
use crate::{errors::Errors, logfields::LogEntry, runtime,
    utility_http::{canonical_header_key, Headers, HeaderValues, HttpHandler, ErrorHandler, HttpRequest, ResponseWriter},
    utility_logging::{CanEmitSourceLog, LogLevel}, utility_runtime::UtilityRuntimeContext};

pub type ErrorHandlerFunc = ErrorHandler;
pub const sensitiveHeaders: &[&str] = &["Authorization", "Proxy-Authorization", "Cookie", "Set-Cookie", "X-Api-Key"];
const ERRORS: Errors = Errors::from_package("agones.dev/agones/pkg/util/https");

pub fn redactSensitiveHeaders(headers: &Headers) -> Headers {
    let redacted = Headers::with_profile(headers.allocation_profile());
    c!(redacted.insert(name.clone(), if sensitiveHeaders.iter().any(|sensitive| canonical_header_key(&name).as_bytes() == sensitive.as_bytes()) {
        HeaderValues::from_values([bytes::Bytes::from_static(b"REDACTED")])
    } else { values.clone() }); for (name, values) in headers);
    redacted
}

pub fn LogRequest(logger: &LogEntry, request: &HttpRequest) -> LogEntry {
    logger.with_field("method", request.method.as_str()).with_field("url", request.url_log_value.clone())
        .with_field("host", request.host.as_str()).with_headers("headers", redactSensitiveHeaders(&request.headers))
        .with_field("requestURI", request.request_uri.as_str())
}

pub fn ErrorHTTPHandler(logger: LogEntry, handler: ErrorHandlerFunc) -> HttpHandler {
    Arc::new(move |context, writer, request| {
        let (logger, handler) = (logger.clone(), Arc::clone(&handler));
        Box::pin(async move {
            if let Err(error) = handler(context, writer, request).await {
                runtime::HandleError(context, Some(&LogRequest(&logger, request)), Some(error.as_ref())).await;
                writer.http_error(&error.to_string(), 500).await;
            }
        })
    })
}

pub async fn FourZeroFour(context: &UtilityRuntimeContext, logger: &LogEntry, writer: &mut ResponseWriter, request: &mut HttpRequest) {
    match request.read_body().await {
        Err(error) => {
            let error = ERRORS.Wrap(Some(error), "error in default handler").expect("source non-nil error");
            runtime::HandleError(context, Some(&LogRequest(logger, request)), Some(error.as_ref())).await;
            writer.http_error(&error.to_string(), 500).await;
            // The source installs its deferred close only after ReadAll succeeds.
        }
        Ok(body) => {
            context.emit_source_log(&LogRequest(logger, request).with_field("body", String::from_utf8_lossy(&body).into_owned()), LogLevel::Warn, "404");
            writer.http_error("404 page not found", 404).await;
            let _ = request.close_body().await;
        }
    }
}
