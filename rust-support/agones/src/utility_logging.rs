//! Source JSON logging and the inherited apimachinery error-backoff resource.
// Entry.Log/Trace, JSONFormatter and prefixFieldClashes follow pinned Logrus.
// Copyright (c) 2014 Simon Eskildsen, MIT; see
// ../../../vendor/github.com/sirupsen/logrus/LICENSE.
use std::{collections::BTreeMap, fmt, io::Write, sync::{Arc, Mutex, atomic::{AtomicU8, Ordering}}, time::Instant};
use cgp::prelude::*;
use chrono::{DateTime, Local};
use futures::future::BoxFuture;
use serde_json::Value;
use crate::{errors::BoxError, logfields::LogEntry};
use serde::Serialize;
use crate::{utility_error_slice::SourceRetainedError, utility_format_copy::SourceFmtValueCopy};

#[derive(Debug)]
pub enum SourceLogValue {
    Json(Value),
    HttpHeaders(crate::utility_http::Headers),
    Bytes(crate::utility_http::SourceByteString),
    Url(Option<crate::utility_http::SourceUrl>),
    Error(SourceRetainedError),
}
impl Clone for SourceLogValue {
    fn clone(&self) -> Self {
        match self { Self::Json(value) => Self::Json(value.clone()), Self::HttpHeaders(value) => Self::HttpHeaders(value.clone()),
            Self::Bytes(value) => Self::Bytes(value.clone()), Self::Url(value) => Self::Url(value.clone()),
            Self::Error(error) => Self::Error(error.source_fmt_copy()) }
    }
}
// LogEntry's native comparison is a descriptor convenience, not a Go Fields
// or error-interface == operation; it must never call Error during comparison.
impl PartialEq for SourceLogValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) { (Self::Json(a), Self::Json(b)) => a == b,
            (Self::HttpHeaders(a), Self::HttpHeaders(b)) => a == b, (Self::Bytes(a), Self::Bytes(b)) => a == b,
            (Self::Url(a), Self::Url(b)) => a == b, (Self::Error(a), Self::Error(b)) => a.native_same_owner(b), _ => false }
    }
}
impl SourceLogValue {
    /// JSONFormatter converts every error-valued field before field clashes
    /// or JSON encoding. This is a plain Error call, with no fmt panic recovery.
    fn json_field_copy(&self) -> Self {
        let Self::Error(error) = self else { return self.clone(); };
        let message = crate::utility_error_format::source_error_message(error)
            .unwrap_or_else(|| std::borrow::Cow::Owned(error.to_string().into_bytes()));
        Self::Bytes(crate::utility_http::SourceByteString(bytes::Bytes::copy_from_slice(&message)))
    }
}
impl Serialize for SourceLogValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self { Self::Json(value) => value.serialize(serializer), Self::HttpHeaders(value) => value.serialize(serializer),
            Self::Bytes(value) => value.serialize(serializer), Self::Url(value) => value.serialize(serializer),
            Self::Error(_) => self.json_field_copy().serialize(serializer) }
    }
}
impl LogEntry {
    pub fn with_url(&self, name: impl Into<String>, url: Option<crate::utility_http::SourceUrl>) -> Self {
        let mut entry = self.clone();
        entry.fields.insert(name.into(), SourceLogValue::Url(url));
        entry
    }
    pub fn with_bytes(&self, name: impl Into<String>, bytes: impl AsRef<[u8]>) -> Self {
        let mut entry = self.clone();
        entry.fields.insert(name.into(), SourceLogValue::Bytes(crate::utility_http::SourceByteString(bytes::Bytes::copy_from_slice(bytes.as_ref()))));
        entry
    }
    pub fn with_headers(&self, name: impl Into<String>, headers: crate::utility_http::Headers) -> Self {
        let mut entry = self.clone();
        entry.fields.insert(name.into(), SourceLogValue::HttpHeaders(headers));
        entry
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LogLevel { Panic = 0, Fatal = 1, Error = 2, Warn = 3, Info = 4, Debug = 5, Trace = 6 }
impl LogLevel {
    pub fn source_name(self) -> &'static str {
        match self { Self::Panic => "panic", Self::Fatal => "fatal", Self::Error => "error", Self::Warn => "warning",
            Self::Info => "info", Self::Debug => "debug", Self::Trace => "trace" }
    }
}

pub struct SourceLogger { level: AtomicU8, output: Mutex<Box<dyn Write + Send>> }
enum SourceLogWriteError { Format(BoxError), Output(std::io::Error) }
fn report_log_write_error(error: SourceLogWriteError) {
    match error { SourceLogWriteError::Format(error) => eprintln!("Failed to obtain reader, {error}"),
        SourceLogWriteError::Output(error) => eprintln!("Failed to write to log, {error}") }
}
impl fmt::Debug for SourceLogger {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.debug_struct("SourceLogger").field("level", &self.level.load(Ordering::Relaxed)).finish_non_exhaustive() }
}
impl SourceLogger {
    pub fn new(output: impl Write + Send + 'static) -> Self {
        Self { level: AtomicU8::new(LogLevel::Info as u8), output: Mutex::new(Box::new(output)) }
    }
    pub fn set_level(&self, level: LogLevel) { self.level.store(level as u8, Ordering::Release); }
    pub fn enabled(&self, level: LogLevel) -> bool { level as u8 <= self.level.load(Ordering::Acquire) }
    fn write(&self, level: LogLevel, fields: &BTreeMap<String, SourceLogValue>, message: &str, timestamp: &str,
        serialization_error: Option<&str>) {
        self.write_bytes(level, fields, message.as_bytes(), timestamp, serialization_error)
    }
    fn write_bytes(&self, level: LogLevel, fields: &BTreeMap<String, SourceLogValue>, message: &[u8], timestamp: &str,
        serialization_error: Option<&str>) {
        // Source Entry.write holds Logger.mu over Formatter.Format and Out.Write.
        // The message's Sprint ran before this lock; field Error calls run here.
        let mut output = self.output.lock().unwrap_or_else(|error| error.into_inner());
        let result = (|| -> Result<(), SourceLogWriteError> {
            let mut data: BTreeMap<_, _> = fields.iter().map(|(key, value)| (key.clone(), value.json_field_copy())).collect();
            // Preserve logrus's collision order and overwrite behavior.
            for name in ["time", "message", "severity", "logrus_error"] {
                if let Some(value) = data.remove(name) { data.insert(format!("fields.{name}"), value); }
            }
            data.insert("time".into(), SourceLogValue::Json(Value::String(timestamp.to_owned())));
            data.insert("message".into(), SourceLogValue::Bytes(crate::utility_http::SourceByteString(bytes::Bytes::copy_from_slice(message))));
            data.insert("severity".into(), SourceLogValue::Json(Value::String(level.source_name().into())));
            if let Some(error) = serialization_error { return Err(SourceLogWriteError::Format(format!("failed to marshal fields to JSON, {error}").into())); }
            let json = serde_json::to_string(&data).map_err(|error| SourceLogWriteError::Format(format!("failed to marshal fields to JSON, {error}").into()))?;
            // encoding/json's default HTML and JavaScript separator escaping is
            // part of the selected formatter, including strings nested in fields.
            let json = json.replace('&', "\\u0026").replace('<', "\\u003c").replace('>', "\\u003e")
                .replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029") + "\n";
            // logrus calls Writer.Write once and ignores a short successful count.
            output.write(json.as_bytes()).map_err(SourceLogWriteError::Output)?;
            Ok(())
        })();
        // Entry.write diagnoses both stages before its deferred unlock.
        if let Err(error) = result { report_log_write_error(error); }
    }
}

pub struct SourceErrorBackoff { last: Mutex<Instant>, min_period: i64 }
impl Default for SourceErrorBackoff { fn default() -> Self { Self::new(1_000_000) } }
impl SourceErrorBackoff {
    pub fn new(min_period: i64) -> Self { Self { last: Mutex::new(Instant::now()), min_period } }
    pub fn min_period(&self) -> i64 { self.min_period }
    pub(crate) fn next_delay(&self, now: Instant, next_now: impl FnOnce() -> Instant) -> i64 {
        let mut last = self.last.lock().unwrap_or_else(|error| error.into_inner());
        let delta = if now >= *last { now.duration_since(*last).as_nanos().min(i64::MAX as u128) as i64 }
            else { let magnitude = last.duration_since(now).as_nanos();
                if magnitude >= (1u128 << 63) { i64::MIN } else { -(magnitude as i64) } };
        *last = next_now();
        // time.Time.Sub saturates; Duration subtraction then wraps. The CGP
        // delay provider implements Sleep's nonpositive immediate return.
        self.min_period.wrapping_sub(delta)
    }
}

#[cgp_getter]
pub trait HasSourceLogger { fn source_logger(&self) -> &Arc<SourceLogger>; }
#[cgp_getter]
pub trait HasSourceErrorBackoff { fn source_error_backoff(&self) -> &Arc<SourceErrorBackoff>; }
#[cgp_component(SourceLogClock)]
pub trait CanReadSourceLogClock { fn source_log_timestamp(&self) -> String; }
#[cgp_new_provider]
impl<Context> SourceLogClock<Context> for ReadLocalSourceLogClock {
    fn source_log_timestamp(_context: &Context) -> String { rfc3339_nano(Local::now()) }
}
fn rfc3339_nano(time: DateTime<Local>) -> String {
    let mut result = time.format("%Y-%m-%dT%H:%M:%S").to_string();
    if time.timestamp_subsec_nanos() != 0 {
        result.push('.'); result.push_str(format!("{:09}", time.timestamp_subsec_nanos()).trim_end_matches('0'));
    }
    if time.offset().local_minus_utc() == 0 { result.push('Z'); }
    else { result.push_str(&time.format("%:z").to_string()); }
    result
}

#[cgp_component(SourceLogEmitter)]
pub trait CanEmitSourceLog {
    fn emit_source_log(&self, logger: &LogEntry, level: LogLevel, message: &str);
}
#[cgp_new_provider]
impl<Context> SourceLogEmitter<Context> for EmitSourceJsonLog
where Context: HasSourceLogger + CanReadSourceLogClock,
{
    fn emit_source_log(context: &Context, logger: &LogEntry, level: LogLevel, message: &str) {
        let state = logger.logger.as_ref().unwrap_or_else(|| context.source_logger());
        if !state.enabled(level) { return; }
        state.write(level, &logger.fields, message, &context.source_log_timestamp(), logger.serialization_error.as_deref());
    }
}

/// Logrus Entry.Trace(err) checks the level before fmt.Sprint(err). Preserve
/// that lazy method call and source panic/nil formatting in the CGP provider.
#[cgp_component(SourceErrorLogEmitter)]
pub trait CanEmitSourceErrorLog {
    fn emit_source_error_log(&self, logger: &LogEntry, level: LogLevel, prefix: &[u8], error: &(dyn std::error::Error + Send + Sync + 'static));
}
#[cgp_new_provider]
impl<Context> SourceErrorLogEmitter<Context> for EmitSourceJsonErrorLog
where Context: HasSourceLogger + CanReadSourceLogClock + crate::utility_http::HasSourceUnicodeProfile
    + crate::utility_http::CanRaiseSourceCollectionFailure,
{
    fn emit_source_error_log(context: &Context, logger: &LogEntry, level: LogLevel, prefix: &[u8], error: &(dyn std::error::Error + Send + Sync + 'static)) {
        let state = logger.logger.as_ref().unwrap_or_else(|| context.source_logger());
        if !state.enabled(level) { return; }
        let mut message = Vec::new();
        // Sprint's leading string argument suppresses the implicit separator.
        crate::utility_http::complete_source_collection(context, crate::utility_go_quote::append(&mut message, prefix));
        crate::utility_http::complete_source_collection(context,
            crate::utility_format_arguments::write_source_error_operand(error, 'v', *context.source_unicode_profile(),
                crate::utility_format_print::SourcePrintfMode::Sprintf, &mut message));
        // Entry.log continues after Sprint even if a value method changes the
        // level. Avoid a second enabled check through the string emitter.
        state.write_bytes(level, &logger.fields, &message, &context.source_log_timestamp(), logger.serialization_error.as_deref());
    }
}

#[cgp_component(SourceRuntimeErrorHandler)]
pub trait CanHandleSourceRuntimeError: Sync {
    fn handle_source_runtime_error<'a>(&'a self, logger: Option<&'a LogEntry>, error: Option<&'a SourceRetainedError>) -> BoxFuture<'a, ()>;
}
#[cgp_new_provider]
impl<Context> SourceRuntimeErrorHandler<Context> for HandleSourceErrorHandlers
where Context: CanEmitSourceLog + crate::utility_error_handlers::CanDispatchSourceRuntimeError + Sync,
{
    fn handle_source_runtime_error<'a>(context: &'a Context, logger: Option<&'a LogEntry>, error: Option<&'a SourceRetainedError>) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if let Some(logger) = logger {
                context.emit_source_log(&logger.with_retained_error(error.map(SourceFmtValueCopy::source_fmt_copy)), LogLevel::Error, "");
            }
            if let Some(error) = error {
                context.dispatch_source_runtime_error(crate::utility_error_handlers::SourceErrorReport {
                    context: crate::utility_error_handlers::SourceErrorContext::background(),
                    error: Some(error.source_fmt_copy()), message: bytes::Bytes::from_static(b"Unhandled Error"),
                    keys_and_values: crate::utility_error_handlers::SourceErrorArguments::nil(),
                }).await;
            }
        })
    }
}
