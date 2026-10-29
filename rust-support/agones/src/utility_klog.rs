//! Source klog contextual/global logger routing and concrete klogger sink.
// klog/v2/{contextual,klog,klogr}.go, Kubernetes Authors, Apache-2.0.
// Header/caller/vmodule/native buffer/output/flag profiles remain required.
// Existing CGP owns effects; all callbacks here belong to actual source data.
use std::{io::Write as _, sync::{Arc, Mutex}};
use bytes::Bytes;
use cgp::prelude::*;
use derive_deftly::Deftly;
use crate::{utility_runtime::UtilityRuntimeContext,
    utility_logr::{SourceLogr, SourceLogSink, SourceCallDepthLogSink, add_int},
    utility_logr_context::{from_context, new_context},
    utility_error_handlers::{SourceErrorContext, SourceErrorArgument, SourceErrorArguments},
    utility_error_slice::SourceRetainedError,
    utility_format_copy::{SourceFmtValueCopy, SourceFmtSharedString},
    utility_http::{HeaderAllocationProfile, complete_source_collection},
    utility_klog_dispatch::{SourceKlogFormatter, SourceKlogFormatError},
    utility_declarations::derive_deftly_template_SourceLogSinkInterfaces};

pub type SourceKlogFlush = Arc<dyn Fn() + Send + Sync>;
pub type SourceKlogWriteBuffer = Arc<dyn Fn(&[u8]) + Send + Sync>;
pub type SourceKlogFilter = Arc<dyn Fn(Bytes, SourceErrorArguments) -> (Bytes, SourceErrorArguments) + Send + Sync>;
pub type SourceKlogVmodule = Arc<dyn Fn(&UtilityRuntimeContext, i64, i32) -> bool + Send + Sync>;
#[derive(Clone, Default)]
pub struct SourceKlogOptions {
    pub contextual_logger: bool, pub flush: Option<SourceKlogFlush>, pub write_klog_buffer: Option<SourceKlogWriteBuffer>,
}
pub enum SourceKlogLoggerOption {
    Nil, ContextualLogger(bool), FlushLogger(Option<SourceKlogFlush>), WriteKlogBuffer(Option<SourceKlogWriteBuffer>),
    Callback(Arc<dyn Fn(&SourceKlogState) + Send + Sync>),
}
#[derive(Clone)]
struct LogWriter { logger: SourceLogr, write_klog_buffer: Option<SourceKlogWriteBuffer> }
struct StateData {
    contextual_enabled: bool, options: SourceKlogOptions, writer: Option<LogWriter>,
    verbosity: i32, vmodule: Option<SourceKlogVmodule>, filter: Option<SourceKlogFilter>,
    slog_stack: Option<crate::utility_slog_values::SourceSlogStack>,
}
pub struct SourceKlogState { data: Mutex<StateData>, builtin: SourceLogr, profile: HeaderAllocationProfile }
pub fn new_klogr(profile: HeaderAllocationProfile) -> SourceLogr {
    SourceLogr::new(Some(Arc::new(SourceKlogger::empty(profile))), profile)
}
impl SourceKlogState {
    pub fn new(profile: HeaderAllocationProfile) -> Self {
        Self { data: Mutex::new(StateData { contextual_enabled: true, options: SourceKlogOptions::default(), writer: None,
            verbosity: 0, vmodule: None, filter: None, slog_stack: None }),
            builtin: new_klogr(profile), profile }
    }
    pub fn profile(&self) -> HeaderAllocationProfile { self.profile }
    pub fn enable_contextual_logging(&self, enabled: bool) { self.data.lock().unwrap_or_else(|e| e.into_inner()).contextual_enabled = enabled; }
    pub fn contextual_enabled(&self) -> bool { self.data.lock().unwrap_or_else(|e| e.into_inner()).contextual_enabled }
    pub fn from_context(&self, runtime: &UtilityRuntimeContext, context: &SourceErrorContext) -> SourceLogr {
        if self.contextual_enabled() { if let Ok(logger) = from_context(runtime, context) { return logger; } }
        self.background()
    }
    pub fn background(&self) -> SourceLogr {
        let selected = {
            let data = self.data.lock().unwrap_or_else(|e| e.into_inner());
            if data.options.contextual_logger { Some(data.writer.as_ref().map(|writer| writer.logger.source_fmt_copy())) }
            else { None }
        };
        match selected { Some(Some(logger)) => logger, Some(None) => crate::utility_error_nil::nil_error_receiver(),
            None => self.builtin.source_fmt_copy() }
    }
    pub fn todo(&self) -> SourceLogr { self.background() }
    pub fn logger_with_values(&self, logger: &SourceLogr, values: SourceErrorArguments) -> SourceLogr {
        if self.contextual_enabled() { logger.with_values(values) } else { logger.source_fmt_copy() }
    }
    pub fn logger_with_name(&self, logger: &SourceLogr, name: &[u8]) -> SourceLogr {
        if self.contextual_enabled() { logger.with_name(name) } else { logger.source_fmt_copy() }
    }
    pub fn new_context(&self, context: SourceErrorContext, logger: SourceLogr) -> SourceErrorContext {
        if self.contextual_enabled() { new_context(context, logger) } else { context }
    }
    pub fn set_logger(&self, logger: SourceLogr) { self.set_logger_with_options(logger, &[]); }
    pub fn set_logger_with_options(&self, logger: SourceLogr, options: &[SourceKlogLoggerOption]) {
        let previous = { let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::take(&mut data.options) };
        drop(previous);
        // Each option mutates the actual global option variable. No state
        // lock is held during a supplied option, allowing source re-entry.
        for option in options {
            match option {
                SourceKlogLoggerOption::Nil => crate::utility_error_nil::nil_error_receiver(),
                SourceKlogLoggerOption::ContextualLogger(value) => self.set_contextual_logger(*value),
                SourceKlogLoggerOption::FlushLogger(value) => drop(self.set_flush_logger(value.as_ref().map(Arc::clone))),
                SourceKlogLoggerOption::WriteKlogBuffer(value) => drop(self.set_write_klog_buffer(value.as_ref().map(Arc::clone))),
                SourceKlogLoggerOption::Callback(callback) => callback(self),
            }
        }
        let previous = {
            let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner());
            let writer = LogWriter { logger, write_klog_buffer: data.options.write_klog_buffer.as_ref().map(Arc::clone) };
            data.writer.replace(writer)
        };
        drop(previous);
    }
    pub fn set_contextual_logger(&self, enabled: bool) { self.data.lock().unwrap_or_else(|e| e.into_inner()).options.contextual_logger = enabled; }
    pub fn set_flush_logger(&self, value: Option<SourceKlogFlush>) -> Option<SourceKlogFlush> {
        let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.options.flush, value)
    }
    pub fn set_write_klog_buffer(&self, value: Option<SourceKlogWriteBuffer>) -> Option<SourceKlogWriteBuffer> {
        let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.options.write_klog_buffer, value)
    }
    pub fn clear_logger(&self) {
        let previous = { let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner());
            (data.writer.take(), std::mem::take(&mut data.options)) };
        drop(previous);
    }
    pub fn flush_backing_logger(&self) {
        let flush = self.data.lock().unwrap_or_else(|e| e.into_inner()).options.flush.as_ref().map(Arc::clone);
        if let Some(flush) = flush { flush(); }
    }
    pub fn set_verbosity(&self, level: i32) { self.data.lock().unwrap_or_else(|e| e.into_inner()).verbosity = level; }
    pub fn set_vmodule(&self, value: Option<SourceKlogVmodule>) -> Option<SourceKlogVmodule> {
        let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.vmodule, value)
    }
    pub fn set_filter(&self, value: Option<SourceKlogFilter>) -> Option<SourceKlogFilter> {
        let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.filter, value)
    }
    pub fn set_slog_stack(&self, value: Option<crate::utility_slog_values::SourceSlogStack>) -> Option<crate::utility_slog_values::SourceSlogStack> {
        let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.slog_stack, value)
    }
    pub fn enabled(&self, runtime: &UtilityRuntimeContext, depth: i64, level: i32) -> bool {
        let (verbosity, vmodule) = { let data = self.data.lock().unwrap_or_else(|e| e.into_inner());
            (data.verbosity, data.vmodule.as_ref().map(Arc::clone)) };
        if verbosity >= level { return true; }
        vmodule.is_some_and(|lookup| lookup(runtime, add_int(self.profile, depth, 2), level))
    }
    fn selected_writer(&self, level: Option<i32>) -> Option<LogWriter> {
        let data = self.data.lock().unwrap_or_else(|e| e.into_inner());
        data.writer.as_ref().map(|writer| LogWriter {
            logger: level.map_or_else(|| writer.logger.source_fmt_copy(), |level| writer.logger.v(i64::from(level))),
            write_klog_buffer: if level.is_some() { data.options.write_klog_buffer.as_ref().map(Arc::clone) }
                else { writer.write_klog_buffer.as_ref().map(Arc::clone) },
        })
    }
    pub fn info_s(&self, runtime: &UtilityRuntimeContext, depth: i64, level: i32, message: &[u8], values: SourceErrorArguments) {
        let enabled = self.enabled(runtime, depth, level);
        let writer = self.selected_writer(Some(level)); // newVerbose runs even when disabled
        if enabled { self.route(runtime, writer, SourceKlogRecord { severity: SourceKlogSeverity::Info,
            call_depth: depth, error: None, message: Bytes::copy_from_slice(message), values }); }
    }
    pub fn error_s(&self, runtime: &UtilityRuntimeContext, depth: i64, error: Option<SourceRetainedError>, message: &[u8], values: SourceErrorArguments) {
        self.route(runtime, self.selected_writer(None), SourceKlogRecord { severity: SourceKlogSeverity::Error,
            call_depth: depth, error, message: Bytes::copy_from_slice(message), values });
    }
    fn route(&self, runtime: &UtilityRuntimeContext, writer: Option<LogWriter>, mut record: SourceKlogRecord) {
        let filter = self.data.lock().unwrap_or_else(|e| e.into_inner()).filter.as_ref().map(Arc::clone);
        if let Some(filter) = filter { (record.message, record.values) = filter(record.message, record.values); }
        if let Some(writer) = writer {
            let logger = writer.logger.with_call_depth(add_int(self.profile, record.call_depth, 2));
            match record.severity { SourceKlogSeverity::Info => logger.info(runtime, &record.message, record.values),
                SourceKlogSeverity::Error => logger.error(runtime, record.error, &record.message, record.values) }
        } else {
            // errorS/infoS -> printS(depth+1) -> printDepth(depth+1).
            record.call_depth = add_int(self.profile, record.call_depth, 2);
            runtime.emit_source_klog_record(record);
        }
    }
}

struct KloggerData { call_depth: i64, has_prefix: bool, values: SourceErrorArguments }
impl SourceFmtValueCopy for KloggerData {
    fn source_fmt_copy(&self) -> Self { Self { call_depth: self.call_depth, has_prefix: self.has_prefix, values: self.values.source_fmt_copy() } }
}
#[derive(Deftly)]
#[derive_deftly(SourceLogSinkInterfaces)]
#[deftly(source_logr_call_depth)]
pub struct SourceKlogger { data: Mutex<KloggerData>, profile: HeaderAllocationProfile }
impl SourceKlogger {
    fn empty(profile: HeaderAllocationProfile) -> Self {
        Self { data: Mutex::new(KloggerData { call_depth: 0, has_prefix: false, values: SourceErrorArguments::nil_with_profile(profile) }), profile }
    }
    fn snapshot(&self) -> KloggerData { self.data.lock().unwrap_or_else(|e| e.into_inner()).source_fmt_copy() }
    fn copied(&self, data: KloggerData) -> Option<Arc<dyn SourceLogSink>> { Some(Arc::new(Self { data: Mutex::new(data), profile: self.profile })) }
}
impl SourceLogSink for SourceKlogger {
    fn source_init(&self, call_depth: i64) { let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner());
        data.call_depth = add_int(self.profile, data.call_depth, call_depth); }
    fn source_enabled(&self, runtime: &UtilityRuntimeContext, level: i64) -> bool {
        runtime.source_klog_state.enabled(runtime, add_int(self.profile, self.snapshot().call_depth, 1), level as i32)
    }
    fn source_info(&self, runtime: &UtilityRuntimeContext, level: i64, message: &[u8], values: SourceErrorArguments) {
        let data = self.snapshot(); let values = crate::utility_klog_values::with_values(&data.values, &values, self.profile);
        runtime.source_klog_state.info_s(runtime, add_int(self.profile, data.call_depth, 1), level as i32, message, values);
    }
    fn source_error(&self, runtime: &UtilityRuntimeContext, error: Option<SourceRetainedError>, message: &[u8], values: SourceErrorArguments) {
        let data = self.snapshot(); let values = crate::utility_klog_values::with_values(&data.values, &values, self.profile);
        runtime.source_klog_state.error_s(runtime, add_int(self.profile, data.call_depth, 1), error, message, values);
    }
    fn source_with_values(&self, values: SourceErrorArguments) -> Option<Arc<dyn SourceLogSink>> {
        let mut data = self.snapshot(); data.values = crate::utility_klog_values::with_values(&data.values, &values, self.profile); self.copied(data)
    }
    fn source_with_name(&self, name: &[u8]) -> Option<Arc<dyn SourceLogSink>> {
        let mut data = self.snapshot();
        let capacity = if data.has_prefix { data.values.len() as i64 } else { add_int(self.profile, data.values.len() as i64, 2) };
        let mut values = SourceErrorArguments::make(0, capacity, self.profile);
        if data.has_prefix {
            for index in 0..data.values.len() { values.append(data.values.get(index)); }
            let previous = values.get(1);
            let mut prefix = crate::utility_klog_values::string_bytes(&previous).unwrap_or(b"").to_vec();
            complete_source_collection(&crate::utility_http::NativeSourceCollectionContext, crate::utility_go_quote::append(&mut prefix, b"."));
            complete_source_collection(&crate::utility_http::NativeSourceCollectionContext, crate::utility_go_quote::append(&mut prefix, name));
            values.set(1, SourceErrorArgument::from_value(SourceFmtSharedString(Arc::from(prefix))));
        } else {
            values.append(SourceErrorArgument::from_value("logger"));
            values.append(SourceErrorArgument::from_value(SourceFmtSharedString(Arc::from(name))));
            for index in 0..data.values.len() { values.append(data.values.get(index)); }
            data.has_prefix = true;
        }
        data.values = values; self.copied(data)
    }
}
impl SourceCallDepthLogSink for SourceKlogger {
    fn source_with_call_depth(&self, depth: i64) -> Option<Arc<dyn SourceLogSink>> {
        let mut data = self.snapshot(); data.call_depth = add_int(self.profile, data.call_depth, depth); self.copied(data)
    }
}
#[derive(Clone, Copy)]
pub enum SourceKlogSeverity { Info, Error }
pub struct SourceKlogRecord {
    pub severity: SourceKlogSeverity, pub call_depth: i64, pub error: Option<SourceRetainedError>,
    pub message: Bytes, pub values: SourceErrorArguments,
}
#[cgp_component(SourceKlogRecordEmitter)]
pub trait CanEmitSourceKlogRecord { fn emit_source_klog_record(&self, record: SourceKlogRecord); }
#[cgp_new_provider]
impl SourceKlogRecordEmitter<UtilityRuntimeContext> for EmitNativeSourceKlogRecord {
    fn emit_source_klog_record(runtime: &UtilityRuntimeContext, record: SourceKlogRecord) {
        let profile = runtime.source_unicode_profile;
        let mut formatter = SourceKlogFormatter::new(profile);
        formatter.slog_stack = runtime.source_klog_state.data.lock().unwrap_or_else(|e| e.into_inner()).slog_stack.as_ref().map(Arc::clone);
        let formatted = format_record_body(record, &formatter, runtime.source_klog_state.profile());
        let bytes = match formatted { Ok(bytes) => bytes,
            Err(SourceKlogFormatError::Allocation(error)) => complete_source_collection(runtime, Err(error)),
            Err(error) => std::panic::panic_any(error) };
        // Existing native stderr projection. Complete severity/header/caller,
        // configured writers, flags, buffer pools and failure policy remain.
        let _ = std::io::stderr().lock().write_all(&bytes);
    }
}
fn format_record_body(record: SourceKlogRecord, formatter: &SourceKlogFormatter, profile: HeaderAllocationProfile)
    -> Result<Vec<u8>, SourceKlogFormatError>
{
    let mut output = Vec::new();
    crate::utility_go_quote::append(&mut output, crate::utility_go_quote::try_source_go_quote(&record.message, formatter.profile)?.as_bytes())?;
    let mut errors = SourceErrorArguments::nil_with_profile(profile);
    if let Some(error) = record.error {
        errors = SourceErrorArguments::make(2, 2, profile);
        errors.set(0, SourceErrorArgument::from_value("err")); errors.set(1, SourceErrorArgument::from_error(Some(error)));
    }
    formatter.format_kvs(&mut output, &[errors, record.values])?;
    if output.last() != Some(&b'\n') { crate::utility_go_quote::append(&mut output, b"\n")?; }
    Ok(output)
}
