//! Shared source error-interface ownership over an original native Box.
// Source contracts: Go error interfaces, fmt/errors.go and reflect copyVal.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
// This is per-error interface data, not a runtime type/provider registry.
use std::{borrow::Cow, error::Error, fmt, sync::Arc};
use crate::{
    errors::BoxError, runtime::SourceLogType,
    utility_format_arguments::{SourcePrintfError, SourcePrintfValue},
    utility_format_adapters::SourcePrintfOwnedError,
    utility_format_copy::SourceFmtValueCopy,
    utility_format_methods::{SourceErrorMethod, SourcePrintfMethodViews, SourcePrintfInterfaces, SourcePrintfRecursion},
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts},
    utility_format_print::SourcePrintfMode,
    utility_panic::SourceRuntimeError,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

type RuntimeView<Value> = for<'a> fn(&'a Value) -> &'a dyn SourceRuntimeError;
trait SharedErrorData: Send + Sync {
    fn original(&self) -> &(dyn Error + Send + Sync + 'static);
    fn source_type(&self) -> &'static str;
    fn methods(&self) -> SourcePrintfInterfaces<'_>;
    fn message(&self) -> Cow<'_, [u8]>;
    fn integer(&self) -> SourcePrintfInteger;
    fn nil_pointer(&self) -> bool;
    fn runtime_error(&self) -> Option<&dyn SourceRuntimeError>;
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>;
}
struct SharedFormatValue<Value> {
    value: Value,
    source_type: &'static str,
    runtime: Option<RuntimeView<Value>>,
}
impl<Value: SourcePrintfValue + SourceErrorMethod + Send + Sync> SharedErrorData for SharedFormatValue<Value> {
    fn original(&self) -> &(dyn Error + Send + Sync + 'static) {
        self.value.source_error_view().expect("shared source error retains its original error view")
    }
    fn source_type(&self) -> &'static str { self.source_type }
    fn methods(&self) -> SourcePrintfInterfaces<'_> { self.value.source_methods() }
    fn message(&self) -> Cow<'_, [u8]> { self.value.source_error_message() }
    fn integer(&self) -> SourcePrintfInteger { self.value.source_integer() }
    fn nil_pointer(&self) -> bool { self.value.source_is_nil_pointer() }
    fn runtime_error(&self) -> Option<&dyn SourceRuntimeError> { self.runtime.map(|view| view(&self.value)) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { self.value.write_source(verb, state, profile, mode, recursion, output) }
}

/// Copies retain this interface's data owner and the original error Box.
/// Erasure may allocate a native interface carrier; it neither moves nor
/// clones/reboxes the original error payload. Full Go header/GC/heap/ABI and
/// method-set/configuration identities remain required native profiles.
pub struct SourceSharedError { data: Arc<dyn SharedErrorData> }
impl SourceSharedError {
    pub fn canonical(error: BoxError) -> Result<Self, BoxError> {
        let value = SourcePrintfError::canonical(error)?;
        let source_type = value.source_type_name();
        Ok(Self { data: Arc::new(SharedFormatValue { value, source_type, runtime: None }) })
    }
    pub fn from_original<Value>(error: Box<Value>) -> Self
    where Value: Error + SourcePrintfValue + SourceErrorMethod + SourceLogType + Send + Sync + 'static,
    {
        Self { data: Arc::new(SharedFormatValue { value: SourcePrintfOwnedError::new(error),
            source_type: Value::SOURCE_LOG_TYPE, runtime: None }) }
    }
    pub fn from_runtime_original<Value>(error: Box<Value>) -> Self
    where Value: SourceRuntimeError + SourcePrintfValue + SourceErrorMethod + SourceLogType + Send + Sync + 'static,
    {
        Self { data: Arc::new(SharedFormatValue { value: SourcePrintfOwnedError::new(error),
            source_type: Value::SOURCE_LOG_TYPE,
            runtime: Some(|value: &SourcePrintfOwnedError<Value>| value.original() as &dyn SourceRuntimeError) }) }
    }
    pub fn original(&self) -> &(dyn Error + Send + Sync + 'static) { self.data.original() }
    pub fn source_type_name(&self) -> &'static str { self.data.source_type() }
    pub fn runtime_error(&self) -> Option<&dyn SourceRuntimeError> { self.data.runtime_error() }
    pub fn into_error(self) -> BoxError { Box::new(self) }
}
impl SourceFmtValueCopy for SourceSharedError {
    fn source_fmt_copy(&self) -> Self { Self { data: Arc::clone(&self.data) } }
}
impl SourcePrintfMethodViews for SourceSharedError {
    fn source_methods(&self) -> SourcePrintfInterfaces<'_> { self.data.methods() }
}
impl SourceErrorMethod for SourceSharedError {
    fn source_error_message(&self) -> Cow<'_, [u8]> { self.data.message() }
}
impl SourcePrintfValue for SourceSharedError {
    fn source_type(&self) -> &str { self.data.source_type() }
    fn source_integer(&self) -> SourcePrintfInteger { self.data.integer() }
    fn source_is_nil_pointer(&self) -> bool { self.data.nil_pointer() }
    // Keep the per-error carrier so the runtime marker/type view survives an
    // erased error boundary. Consumers resolve the original concrete data.
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { Some(self) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { self.data.write_source(verb, state, profile, mode, recursion, output) }
    fn into_source_error(self) -> Option<BoxError> { Some(self.into_error()) }
}
impl fmt::Debug for SourceSharedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.original(), out)
    }
}
impl fmt::Display for SourceSharedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(&String::from_utf8_lossy(&self.data.message()))
    }
}
impl Error for SourceSharedError {
    // A copied interface is the same error value, not an extra Unwrap edge.
    fn source(&self) -> Option<&(dyn Error + 'static)> { self.original().source() }
}

/// Resolve native interface carriers without traversing a source Unwrap edge.
pub fn source_error_original<'a>(mut error: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static) {
    while let Some(shared) = error.downcast_ref::<SourceSharedError>() { error = shared.original(); }
    error
}
