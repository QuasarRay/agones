//! Source JSON logging and the inherited apimachinery error-backoff resource.
use std::{collections::BTreeMap, fmt, io::Write, sync::{Arc, Mutex, atomic::{AtomicU8, Ordering}}, time::{Duration, Instant}};
use cgp::prelude::*;
use chrono::{DateTime, Local};
use futures::future::BoxFuture;
use serde_json::Value;
use crate::{errors::BoxError, logfields::LogEntry};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SourceLogValue {
    Json(Value),
    HttpHeaders(crate::utility_http::Headers),
    Bytes(crate::utility_http::SourceByteString),
    Url(Option<crate::utility_http::SourceUrl>),
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
impl fmt::Debug for SourceLogger {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.debug_struct("SourceLogger").field("level", &self.level.load(Ordering::Relaxed)).finish_non_exhaustive() }
}
impl SourceLogger {
    pub fn new(output: impl Write + Send + 'static) -> Self {
        Self { level: AtomicU8::new(LogLevel::Info as u8), output: Mutex::new(Box::new(output)) }
    }
    pub fn set_level(&self, level: LogLevel) { self.level.store(level as u8, Ordering::Release); }
    pub fn enabled(&self, level: LogLevel) -> bool { level as u8 <= self.level.load(Ordering::Acquire) }
    fn write(&self, level: LogLevel, fields: &BTreeMap<String, SourceLogValue>, message: &str, timestamp: &str) -> Result<(), BoxError> {
        let mut data = fields.clone();
        // Preserve logrus's collision order and overwrite behavior.
        for name in ["time", "message", "severity", "logrus_error"] {
            if let Some(value) = data.remove(name) { data.insert(format!("fields.{name}"), value); }
        }
        data.insert("time".into(), SourceLogValue::Json(Value::String(timestamp.to_owned())));
        data.insert("message".into(), SourceLogValue::Json(Value::String(message.to_owned())));
        data.insert("severity".into(), SourceLogValue::Json(Value::String(level.source_name().into())));
        let json = serde_json::to_string(&data)?;
        // encoding/json's default HTML and JavaScript separator escaping is
        // part of the selected formatter, including strings nested in fields.
        let json = json.replace('&', "\\u0026").replace('<', "\\u003c").replace('>', "\\u003e")
            .replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029") + "\n";
        // logrus calls Writer.Write once and ignores a short successful count.
        self.output.lock().unwrap_or_else(|error| error.into_inner()).write(json.as_bytes())?;
        Ok(())
    }
}

pub struct SourceErrorBackoff { last: Mutex<Instant> }
impl Default for SourceErrorBackoff { fn default() -> Self { Self { last: Mutex::new(Instant::now()) } } }
impl SourceErrorBackoff {
    fn next_delay(&self, now: Instant) -> Duration {
        let mut last = self.last.lock().unwrap_or_else(|error| error.into_inner());
        let delta = if now >= *last { now.duration_since(*last).as_nanos().min(i64::MAX as u128) as i64 }
            else { -(last.duration_since(now).as_nanos().min(i64::MAX as u128) as i64) };
        *last = Instant::now();
        // Go time.Duration subtraction wraps; nonpositive Sleep is immediate.
        Duration::from_nanos(1_000_000i64.wrapping_sub(delta).max(0) as u64)
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
        if let Some(error) = &logger.serialization_error {
            eprintln!("Failed to obtain reader, failed to marshal fields to JSON, {error}");
            return;
        }
        if let Err(error) = state.write(level, &logger.fields, message, &context.source_log_timestamp()) {
            eprintln!("Failed to write to log, {error}");
        }
    }
}

#[cgp_component(SourceRuntimeErrorHandler)]
pub trait CanHandleSourceRuntimeError: Sync {
    fn handle_source_runtime_error<'a>(&'a self, logger: Option<&'a LogEntry>, error: Option<&'a (dyn std::error::Error + Send + Sync)>) -> BoxFuture<'a, ()>;
}
#[cgp_new_provider]
impl<Context> SourceRuntimeErrorHandler<Context> for HandleSourceErrorAndBackoff
where Context: CanEmitSourceLog + HasSourceLogger + HasSourceErrorBackoff + Sync,
{
    fn handle_source_runtime_error<'a>(context: &'a Context, logger: Option<&'a LogEntry>, error: Option<&'a (dyn std::error::Error + Send + Sync)>) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if let Some(logger) = logger { context.emit_source_log(&logger.with_error(error), LogLevel::Error, ""); }
            if let Some(error) = error {
                let logger = LogEntry::from_logger(Arc::clone(context.source_logger()));
                context.emit_source_log(&logger, LogLevel::Error, &error.to_string());
                // The source starts its timestamp before locking and sleeps
                // after unlocking, so other callers do not hold this lock.
                let delay = context.source_error_backoff().next_delay(Instant::now());
                if !delay.is_zero() { tokio::time::sleep(delay).await; }
            }
        })
    }
}
