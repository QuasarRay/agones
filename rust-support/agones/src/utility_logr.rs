//! Concrete logr.Logger value and per-sink optional method sets.
// Pinned github.com/go-logr/logr/{logr,discard}.go; logr Authors,
// Apache-2.0. Sink references are actual compiled source values, not plugins,
// a provider registry, a Go object interpreter or a lifecycle manager.
use std::sync::Arc;
use crate::{utility_runtime::UtilityRuntimeContext, utility_error_handlers::SourceErrorArguments,
    utility_error_slice::SourceRetainedError, utility_format_copy::SourceFmtValueCopy,
    utility_http::HeaderAllocationProfile};

pub(crate) fn add_int(profile: HeaderAllocationProfile, left: i64, right: i64) -> i64 {
    let value = left.wrapping_add(right);
    if profile.word_bits() == 32 { i64::from(value as i32) } else { value }
}
#[derive(Clone, Default)]
pub struct SourceLogrHelper { pub function: Option<Arc<dyn Fn() + Send + Sync>> }
impl SourceLogrHelper {
    pub fn noop() -> Self { Self { function: Some(Arc::new(|| {})) } }
    pub fn call(&self) { match &self.function { Some(function) => function(), None => crate::utility_error_nil::nil_error_receiver() } }
}
pub trait SourceCallDepthLogSink {
    fn source_with_call_depth(&self, depth: i64) -> Option<Arc<dyn SourceLogSink>>;
}
pub trait SourceCallStackHelperLogSink { fn source_call_stack_helper(&self) -> SourceLogrHelper; }
#[derive(Default)]
pub struct SourceLogSinkInterfaces<'a> {
    pub call_depth: Option<&'a dyn SourceCallDepthLogSink>,
    pub call_stack_helper: Option<&'a dyn SourceCallStackHelperLogSink>,
}
pub trait SourceLogSinkMethodViews {
    fn source_sink_methods(&self) -> SourceLogSinkInterfaces<'_> { SourceLogSinkInterfaces::default() }
}
pub trait SourceLogSink: SourceLogSinkMethodViews + Send + Sync {
    fn source_init(&self, call_depth: i64);
    fn source_enabled(&self, runtime: &UtilityRuntimeContext, level: i64) -> bool;
    fn source_info(&self, runtime: &UtilityRuntimeContext, level: i64, message: &[u8], values: SourceErrorArguments);
    fn source_error(&self, runtime: &UtilityRuntimeContext, error: Option<SourceRetainedError>, message: &[u8], values: SourceErrorArguments);
    fn source_with_values(&self, values: SourceErrorArguments) -> Option<Arc<dyn SourceLogSink>>;
    fn source_with_name(&self, name: &[u8]) -> Option<Arc<dyn SourceLogSink>>;
}
pub struct SourceLogr {
    sink: Option<Arc<dyn SourceLogSink>>, level: i64, profile: HeaderAllocationProfile,
}
impl SourceFmtValueCopy for SourceLogr {
    fn source_fmt_copy(&self) -> Self { Self { sink: self.sink.as_ref().map(Arc::clone), level: self.level, profile: self.profile } }
}
impl Clone for SourceLogr { fn clone(&self) -> Self { self.source_fmt_copy() } }
impl Default for SourceLogr {
    fn default() -> Self {
        let profile = if usize::BITS == 32 { HeaderAllocationProfile::GO126_32 } else { HeaderAllocationProfile::GO126_64 };
        Self::discard(profile)
    }
}
impl SourceLogr {
    pub fn new(sink: Option<Arc<dyn SourceLogSink>>, profile: HeaderAllocationProfile) -> Self {
        let logger = Self { sink, level: 0, profile };
        if let Some(sink) = &logger.sink { sink.source_init(1); }
        logger
    }
    pub fn discard(profile: HeaderAllocationProfile) -> Self { Self::new(None, profile) }
    pub fn sink(&self) -> Option<Arc<dyn SourceLogSink>> { self.sink.as_ref().map(Arc::clone) }
    pub fn with_sink(&self, sink: Option<Arc<dyn SourceLogSink>>) -> Self {
        Self { sink, level: self.level, profile: self.profile } // no Init
    }
    pub fn is_zero(&self) -> bool { self.sink.is_none() }
    pub fn get_v(&self) -> i64 { self.level }
    pub fn enabled(&self, runtime: &UtilityRuntimeContext) -> bool {
        self.sink.as_ref().is_some_and(|sink| sink.source_enabled(runtime, self.level))
    }
    pub fn info(&self, runtime: &UtilityRuntimeContext, message: &[u8], values: SourceErrorArguments) {
        if let Some(sink) = &self.sink {
            if sink.source_enabled(runtime, self.level) {
                if let Some(helper) = sink.source_sink_methods().call_stack_helper { helper.source_call_stack_helper().call(); }
                sink.source_info(runtime, self.level, message, values);
            }
        }
    }
    pub fn error(&self, runtime: &UtilityRuntimeContext, error: Option<SourceRetainedError>, message: &[u8], values: SourceErrorArguments) {
        if let Some(sink) = &self.sink {
            if let Some(helper) = sink.source_sink_methods().call_stack_helper { helper.source_call_stack_helper().call(); }
            sink.source_error(runtime, error, message, values);
        }
    }
    pub fn v(&self, level: i64) -> Self {
        let level = if self.profile.word_bits() == 32 { i64::from(level as i32) } else { level };
        let mut logger = self.source_fmt_copy();
        if logger.sink.is_some() { logger.level = add_int(logger.profile, logger.level, level.max(0)); }
        logger
    }
    pub fn with_values(&self, values: SourceErrorArguments) -> Self {
        match &self.sink { Some(sink) => self.with_sink(sink.source_with_values(values)), None => self.source_fmt_copy() }
    }
    pub fn with_name(&self, name: &[u8]) -> Self {
        match &self.sink { Some(sink) => self.with_sink(sink.source_with_name(name)), None => self.source_fmt_copy() }
    }
    pub fn with_call_depth(&self, depth: i64) -> Self {
        let depth = if self.profile.word_bits() == 32 { i64::from(depth as i32) } else { depth };
        if let Some(method) = self.sink.as_ref().and_then(|sink| sink.source_sink_methods().call_depth) {
            return self.with_sink(method.source_with_call_depth(depth));
        }
        self.source_fmt_copy()
    }
    pub fn with_call_stack_helper(&self) -> (SourceLogrHelper, Self) {
        if self.sink.is_none() { return (SourceLogrHelper::noop(), self.source_fmt_copy()); }
        let logger = self.with_call_depth(1);
        // GetCallStackHelper is looked up on the newly returned sink.
        let helper = logger.sink.as_ref().and_then(|sink| sink.source_sink_methods().call_stack_helper)
            .map_or_else(SourceLogrHelper::noop, |helper| helper.source_call_stack_helper());
        (helper, logger)
    }
}
