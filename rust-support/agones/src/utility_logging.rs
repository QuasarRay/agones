//! Agones structured logs and error-loop backoff using Rust facilities.
//! The application context owns the sink, filter and backoff. Formatting and
//! serialization use Display, serde_json and chrono; no Go formatter is involved.
use std::{collections::BTreeMap, error::Error, fmt, io::Write,
    sync::{Arc, Mutex, atomic::{AtomicU8, Ordering}}, time::{Duration, Instant}};
use cgp::prelude::*;
use chrono::{Local, SecondsFormat};
use futures::future::BoxFuture;
use serde::Serialize;
use serde_json::Value;
use crate::{logfields::LogEntry, utility_scope::HasUtilityTaskScope};

impl LogEntry {
    // These are temporary HTTP-boundary adapters. Their producers remain in
    // the HTTP adoption queue; the logging sink stores only serde_json values.
    pub fn with_url(&self, name: impl Into<String>, url: Option<crate::utility_http::SourceUrl>) -> Self {
        self.with_serializable_field(name, &url)
    }
    pub fn with_bytes(&self, name: impl Into<String>, bytes: impl AsRef<[u8]>) -> Self {
        self.with_field(name, String::from_utf8_lossy(bytes.as_ref()).into_owned())
    }
    pub fn with_headers(&self, name: impl Into<String>, headers: crate::utility_http::Headers) -> Self {
        self.with_serializable_field(name, &headers)
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
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.debug_struct("SourceLogger").field("level", &self.level.load(Ordering::Relaxed)).finish_non_exhaustive()
    }
}

#[derive(Serialize)]
struct JsonLogRecord<'a> {
    time: &'a str,
    severity: &'a str,
    message: &'a str,
    #[serde(flatten)]
    attributes: BTreeMap<&'a str, &'a Value>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    fields: BTreeMap<&'a str, &'a Value>,
}
impl SourceLogger {
    pub fn new(output: impl Write + Send + 'static) -> Self {
        Self { level: AtomicU8::new(LogLevel::Info as u8), output: Mutex::new(Box::new(output)) }
    }
    pub fn set_level(&self, level: LogLevel) { self.level.store(level as u8, Ordering::Release); }
    pub fn enabled(&self, level: LogLevel) -> bool { level as u8 <= self.level.load(Ordering::Acquire) }
    fn write(&self, level: LogLevel, fields: &BTreeMap<String, Value>, message: &str, timestamp: &str) {
        // Keep the Agones metadata keys stable. User attributes cannot replace
        // them; colliding attributes are retained in the nested fields object.
        let (collisions, attributes) = fields.iter().map(|(key, value)| (key.as_str(), value))
            .partition(|(key, _)| matches!(*key, "time" | "severity" | "message" | "fields"));
        let record = JsonLogRecord { time: timestamp, severity: level.source_name(), message,
            attributes, fields: collisions };
        let encoded = serde_json::to_vec(&record).map(|mut bytes| { bytes.push(b'\n'); bytes });
        match encoded {
            Ok(bytes) => {
                let mut output = self.output.lock().unwrap_or_else(|error| error.into_inner());
                if let Err(error) = output.write_all(&bytes) { eprintln!("Agones log output failed: {error}"); }
            }
            Err(error) => eprintln!("Agones log serialization failed: {error}"),
        }
    }
}

/// Agones retains the inherited one-millisecond error-loop delay, using a
/// monotonic Rust clock and saturating Duration arithmetic. Sleeping never
/// holds the state lock and can be interrupted by the parent lifecycle.
pub struct SourceErrorBackoff { last: Mutex<Instant>, min_period: Duration }
impl Default for SourceErrorBackoff { fn default() -> Self { Self::new(Duration::from_millis(1)) } }
impl SourceErrorBackoff {
    pub fn new(min_period: Duration) -> Self { Self { last: Mutex::new(Instant::now()), min_period } }
    pub fn min_period(&self) -> Duration { self.min_period }
    pub(crate) fn next_delay(&self, now: Instant) -> Duration {
        let mut last = self.last.lock().unwrap_or_else(|error| error.into_inner());
        let elapsed = now.saturating_duration_since(*last);
        *last = now;
        self.min_period.saturating_sub(elapsed)
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
    fn source_log_timestamp(_context: &Context) -> String { Local::now().to_rfc3339_opts(SecondsFormat::AutoSi, true) }
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
        if state.enabled(level) { state.write(level, &logger.fields, message, &context.source_log_timestamp()); }
    }
}

#[cgp_component(SourceErrorLogEmitter)]
pub trait CanEmitSourceErrorLog {
    fn emit_source_error_log(&self, logger: &LogEntry, level: LogLevel, prefix: &str, error: &(dyn Error + Send + Sync + 'static));
}
#[cgp_new_provider]
impl<Context> SourceErrorLogEmitter<Context> for EmitSourceJsonErrorLog
where Context: HasSourceLogger + CanReadSourceLogClock,
{
    fn emit_source_error_log(context: &Context, logger: &LogEntry, level: LogLevel, prefix: &str, error: &(dyn Error + Send + Sync + 'static)) {
        let state = logger.logger.as_ref().unwrap_or_else(|| context.source_logger());
        if state.enabled(level) {
            state.write(level, &logger.fields, &format!("{prefix}{error}"), &context.source_log_timestamp());
        }
    }
}

#[cgp_component(SourceRuntimeErrorHandler)]
pub trait CanHandleSourceRuntimeError: Sync {
    fn handle_source_runtime_error<'a>(&'a self, logger: Option<&'a LogEntry>, error: Option<&'a (dyn Error + Send + Sync + 'static)>) -> BoxFuture<'a, ()>;
}
#[cgp_new_provider]
impl<Context> SourceRuntimeErrorHandler<Context> for HandleNativeRuntimeError
where Context: CanEmitSourceLog + CanEmitSourceErrorLog + HasSourceLogger + HasSourceErrorBackoff + HasUtilityTaskScope + Sync,
{
    fn handle_source_runtime_error<'a>(context: &'a Context, logger: Option<&'a LogEntry>, error: Option<&'a (dyn Error + Send + Sync + 'static)>) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if let Some(logger) = logger { context.emit_source_log(&logger.with_error(error), LogLevel::Error, ""); }
            let Some(error) = error else { return; };
            let logger = LogEntry::from_logger(Arc::clone(context.source_logger()));
            context.emit_source_error_log(&logger, LogLevel::Error, "", error);
            let delay = context.source_error_backoff().next_delay(Instant::now());
            if delay.is_zero() { return; }
            let Some(scope) = context.utility_task_scope().upgrade() else { return; };
            let stop = scope.child_token();
            drop(scope);
            tokio::select! {
                _ = tokio::time::sleep(delay) => (),
                _ = stop.cancelled() => (),
            }
        })
    }
}
