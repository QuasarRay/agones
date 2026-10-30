// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/https/https.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::Arc;
use crate::{errors::Errors, logfields::LogEntry, runtime,
    utility_web::{Headers, HttpHandler, ErrorHandler, HttpRequest, ResponseWriter, headers_json},
    utility_logging::{CanEmitSourceLog, LogLevel}, utility_runtime::UtilityRuntimeContext};

pub type ErrorHandlerFunc = ErrorHandler;
pub const sensitiveHeaders: &[&str] = &["Authorization", "Proxy-Authorization", "Cookie", "Set-Cookie", "X-Api-Key"];
const ERRORS: Errors = Errors::from_package("agones.dev/agones/pkg/util/https");

pub fn redactSensitiveHeaders(headers: &Headers) -> Headers {
    let mut redacted = headers.clone();
    // Mutate only the selected fields; retain every value of other headers.
    for name in sensitiveHeaders {
        if redacted.contains_key(*name) {
            redacted.insert(hyper::header::HeaderName::from_bytes(name.as_bytes()).expect("constant sensitive header"),
                hyper::header::HeaderValue::from_static("REDACTED"));
        }
    }
    redacted
}
pub fn LogRequest(logger: &LogEntry, request: &HttpRequest) -> LogEntry {
    logger.with_field("method", request.method().as_str()).with_field("url", request.uri().to_string())
        .with_field("host", request.host()).with_field("headers", headers_json(&redactSensitiveHeaders(&request.headers)))
        .with_field("requestURI", request.uri().to_string())
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
            let error = ERRORS.Wrap(Some(error), "error in default handler").expect("request read error is present");
            runtime::HandleError(context, Some(&LogRequest(logger, request)), Some(error.as_ref())).await;
            writer.http_error(&error.to_string(), 500).await;
        }
        Ok(body) => {
            context.emit_source_log(&LogRequest(logger, request).with_bytes("body", &body), LogLevel::Warn, "404");
            writer.http_error("404 page not found", 404).await;
        }
    }
}
