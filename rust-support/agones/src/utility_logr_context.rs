//! Selected Go1.21+ logr context key and typed lookup control.
// logr/context{,_slog}.go, logr Authors, Apache-2.0. This owns only the real
// logr key binding; full Go context Deadline/Err/other keys remain required.
use std::{borrow::Cow, error::Error, fmt, sync::Arc};
use derive_deftly::Deftly;
use crate::{utility_logr::SourceLogr, utility_runtime::UtilityRuntimeContext,
    utility_error_handlers::{SourceErrorContext, SourceErrorArgument},
    utility_format_copy::{SourceFmtValueCopy, SourceFmtSharedString},
    utility_format_arguments::SourcePrintfValue,
    utility_format_methods::{SourceErrorMethod, SourcePrintfRecursion, SourcePrintfPanic},
    utility_format_collections::{SourcePrintfStructFields, write_source_struct},
    utility_format_scan::SourcePrintfResolvedCounts, utility_format_print::SourcePrintfMode,
    utility_klog_dispatch::SourceKlogFormatError,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError, NativeSourceCollectionContext, complete_source_collection},
    utility_declarations::{derive_deftly_template_SourceErrorOrigin, derive_deftly_template_SourcePrintfInterfaces}};

/// The actual compiled slog.Logger Handler -> FromSlogHandler binding is
/// retained per logger. Full native slog handler/record conversion remains a
/// required binding, not an inferred logger name or an object interpreter.
pub struct SourceContextSlogLogger {
    pub from_handler: Arc<dyn Fn(&UtilityRuntimeContext) -> SourceLogr + Send + Sync>,
}
pub enum SourceLogrContextValue {
    Nil, Logger(SourceLogr), Slog(Option<Arc<SourceContextSlogLogger>>), Other(SourceErrorArgument),
}
impl SourceFmtValueCopy for SourceLogrContextValue {
    fn source_fmt_copy(&self) -> Self { match self {
        Self::Nil => Self::Nil, Self::Logger(logger) => Self::Logger(logger.source_fmt_copy()),
        Self::Slog(logger) => Self::Slog(logger.as_ref().map(Arc::clone)), Self::Other(value) => Self::Other(value.source_fmt_copy()),
    } }
}
pub type SourceLogrContextLookup = Arc<dyn Fn() -> SourceLogrContextValue + Send + Sync>;
pub fn from_context(runtime: &UtilityRuntimeContext, context: &SourceErrorContext) -> Result<SourceLogr, SourceLogrNotFoundError> {
    match context.logr_value() {
        SourceLogrContextValue::Nil => Err(SourceLogrNotFoundError),
        SourceLogrContextValue::Logger(logger) => Ok(logger),
        SourceLogrContextValue::Slog(Some(logger)) => Ok((logger.from_handler)(runtime)),
        SourceLogrContextValue::Slog(None) => crate::utility_error_nil::nil_error_receiver(),
        SourceLogrContextValue::Other(value) => {
            if value.is_nil() { return Err(SourceLogrNotFoundError); }
            let mut message = b"unexpected value type for logr context key: ".to_vec();
            match value.try_write('T', &SourcePrintfResolvedCounts::default(), runtime.source_unicode_profile, &mut message) {
                Some(result) => complete_source_collection(&NativeSourceCollectionContext, result),
                None => std::panic::panic_any(SourceKlogFormatError::Unbound {
                    source_type: None, operation: "logr context unexpected value type" }),
            }
            SourcePrintfPanic::raise(SourceFmtSharedString(Arc::from(message)))
        }
    }
}
pub fn new_context(context: SourceErrorContext, logger: SourceLogr) -> SourceErrorContext {
    context.with_logr_value(SourceLogrContextValue::Logger(logger))
}
pub fn new_context_with_slog_logger(context: SourceErrorContext, logger: Option<Arc<SourceContextSlogLogger>>) -> SourceErrorContext {
    context.with_logr_value(SourceLogrContextValue::Slog(logger))
}
pub fn from_context_or_discard(runtime: &UtilityRuntimeContext, context: &SourceErrorContext) -> SourceLogr {
    from_context(runtime, context).unwrap_or_else(|_| SourceLogr::discard(runtime.source_klog_state.profile()))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces)]
#[deftly(source_package = "github.com/go-logr/logr", source_struct = "notFoundError", source_log_type = "logr.notFoundError", source_error_method)]
pub struct SourceLogrNotFoundError;
impl SourceFmtValueCopy for SourceLogrNotFoundError { fn source_fmt_copy(&self) -> Self { *self } }
impl SourceLogrNotFoundError { pub fn is_not_found(&self) -> bool { true } }
impl SourceErrorMethod for SourceLogrNotFoundError {
    fn source_error_message(&self) -> Cow<'_, [u8]> { Cow::Borrowed(b"no logr.Logger was present") }
}
impl fmt::Display for SourceLogrNotFoundError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result { output.write_str("no logr.Logger was present") }
}
impl Error for SourceLogrNotFoundError {}
impl SourcePrintfValue for SourceLogrNotFoundError {
    fn source_type(&self) -> &str { "logr.notFoundError" }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { Some(self) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl SourcePrintfStructFields for SourceLogrNotFoundError {
    fn source_field_count(&self) -> usize { 0 }
    fn source_field_name(&self, _: usize) -> &str { unreachable!("empty source notFoundError") }
    fn source_field_interfaceable(&self, _: usize) -> bool { false }
    fn source_write_field(&self, _: usize, _: char, _: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, _: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { unreachable!("empty source notFoundError") }
}
