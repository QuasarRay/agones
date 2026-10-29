//! Actual traceError method and private-field formatting over its one owner.
// Agones workerqueue.go, 6eb4a02588d8e7b581de187f3bc849733a78dcb2;
// Contributors to Agones a Series of LF Projects, LLC, Apache-2.0.
// Formatting follows Go fmt/print.go, go1.26.5; The Go Authors,
// BSD-3-Clause, see ../GO-LICENSE.
use std::{borrow::Cow, error::Error};
use derive_deftly::Deftly;
use crate::{source::workerqueue::traceError, errors::BoxError,
    utility_format_methods::{SourceErrorMethod, SourcePrintfRecursion},
    utility_format_arguments::{SourcePrintfValue, write_source_error_value},
    utility_format_collections::{SourcePrintfStructFields, write_source_struct},
    utility_format_pointer::write_source_address,
    utility_format_scan::SourcePrintfResolvedCounts, utility_format_print::SourcePrintfMode,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError}, utility_go_quote::append,
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces};

impl SourceErrorMethod for traceError {
    fn source_error_message(&self) -> Cow<'_, [u8]> {
        let Some(error) = self.source_inner_error() else { return Cow::Borrowed(b"<nil>"); };
        crate::utility_error_format::source_error_message(error)
            .unwrap_or_else(|| Cow::Owned(error.to_string().into_bytes()))
    }
}
impl SourcePrintfValue for traceError {
    fn source_type(&self) -> &str { "*workerqueue.traceError" }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { Some(self) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if recursion.depth == 0 && verb != 'p' {
            append(output, b"&")?;
            return write_source_struct(&TraceFields(self), verb, state, profile, mode, recursion.child(true), output);
        }
        write_source_address(self, self as *const Self as usize as u64, verb, state, profile, mode, recursion, output)
    }
    fn into_source_error(self) -> Option<BoxError> {
        Some(crate::utility_error_alias::SourceSharedError::from_original(Box::new(self)).into_error())
    }
}

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
struct TraceFields<'a>(&'a traceError);
impl SourcePrintfValue for TraceFields<'_> {
    fn source_type(&self) -> &str { "workerqueue.traceError" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl SourcePrintfStructFields for TraceFields<'_> {
    fn source_field_count(&self) -> usize { 1 }
    fn source_field_name(&self, _: usize) -> &str { "err" }
    fn source_field_interfaceable(&self, _: usize) -> bool { false }
    fn source_write_field(&self, _: usize, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        match self.0.source_inner_error() {
            Some(error) => write_source_error_value(error, verb, state, profile, mode, recursion, output),
            None => append(output, if state.flags.sharp_v { b"error(nil)" } else { b"<nil>" }),
        }
    }
}
