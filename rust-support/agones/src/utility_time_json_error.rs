//! Retained json.MarshalerError produced by the concrete time.Time encoder.
// Go1.26.5 encoding/json/encode.go; Copyright The Go Authors, BSD-3-Clause
// (../GO-LICENSE). Full mutable exported fields and reflect.Type/rtype native
// receiver/layout bindings remain required; an absent type view stays unbound.
use std::{borrow::Cow, error::Error, fmt};
use derive_deftly::Deftly;
use crate::{utility_error_slice::SourceRetainedError, utility_error_alias::SourceSharedError,
    utility_error_query::SourceErrorUnwrapOne, utility_error_handlers::SourceErrorArgument,
    utility_format_copy::SourceFmtValueCopy,
    utility_format_arguments::{SourcePrintfValue, write_source_error_value},
    utility_format_methods::{SourceErrorMethod, SourcePrintfRecursion, write_source_value},
    utility_format_collections::{SourcePrintfStructFields, write_source_struct},
    utility_format_pointer::write_source_address,
    utility_format_scan::SourcePrintfResolvedCounts, utility_format_print::SourcePrintfMode,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError, NativeSourceCollectionContext, complete_source_collection},
    utility_go_quote::append, utility_klog_dispatch::SourceKlogFormatError,
    utility_declarations::{derive_deftly_template_SourceErrorOrigin, derive_deftly_template_SourcePrintfInterfaces}};

#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces)]
#[deftly(source_package = "encoding/json", source_struct = "MarshalerError", source_log_type = "*json.MarshalerError",
    source_error_method, source_error_unwrap_one)]
pub struct SourceTimeMarshalerError {
    error: SourceRetainedError,
    // Host metadata distinguishes an unbound non-nil reflect.Type from a
    // source nil interface. Error's concrete type name is known at creation.
    native_type: Option<SourceErrorArgument>,
}
pub(crate) fn time_marshaler_error(error: SourceRetainedError) -> SourceRetainedError {
    let value = SourceTimeMarshalerError { error, native_type: None };
    SourceRetainedError::from_original(SourceSharedError::from_original(Box::new(value)).into_error())
}
impl SourceErrorMethod for SourceTimeMarshalerError {
    fn source_error_message(&self) -> Cow<'_, [u8]> {
        let mut message = b"json: error calling MarshalJSON for type time.Time: ".to_vec();
        let inner = crate::utility_error_format::source_error_message(self.error.original())
            .expect("time.MarshalJSON returns a bound errors.errorString");
        complete_source_collection(&NativeSourceCollectionContext, append(&mut message, &inner)); Cow::Owned(message)
    }
}
impl SourceErrorUnwrapOne for SourceTimeMarshalerError {
    fn source_unwrap_one(&self) -> Option<SourceRetainedError> { Some(self.error.source_fmt_copy()) }
}
impl fmt::Display for SourceTimeMarshalerError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str(&String::from_utf8_lossy(&self.source_error_message()))
    }
}
impl fmt::Debug for SourceTimeMarshalerError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self, output) }
}
impl Error for SourceTimeMarshalerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { Some(self.error.original()) }
}
impl SourcePrintfValue for SourceTimeMarshalerError {
    fn source_type(&self) -> &str { "*json.MarshalerError" }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { Some(self) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if recursion.depth == 0 && verb != 'p' {
            append(output, b"&")?;
            return write_source_struct(&MarshalerFields(self), verb, state, profile, mode, recursion.child(true), output);
        }
        write_source_address(self, self as *const Self as usize as u64, verb, state, profile, mode, recursion, output)
    }
}
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
struct MarshalerFields<'a>(&'a SourceTimeMarshalerError);
impl SourcePrintfValue for MarshalerFields<'_> {
    fn source_type(&self) -> &str { "json.MarshalerError" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl SourcePrintfStructFields for MarshalerFields<'_> {
    fn source_field_count(&self) -> usize { 3 }
    fn source_field_name(&self, index: usize) -> &str { match index { 0 => "Type", 1 => "Err", _ => "sourceFunc" } }
    fn source_field_interfaceable(&self, index: usize) -> bool { index < 2 }
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        match index {
            0 => {
                let binding = self.0.native_type.as_ref().unwrap_or_else(|| std::panic::panic_any(SourceKlogFormatError::Unbound {
                    source_type: Some("reflect.Type".to_owned()), operation: "time JSON MarshalerError.Type native receiver/layout" }));
                binding.try_write_recursive(verb, state, profile, mode, recursion, output).unwrap_or_else(||
                    std::panic::panic_any(SourceKlogFormatError::Unbound { source_type: Some("reflect.Type".to_owned()),
                        operation: "time JSON MarshalerError.Type formatting" }))
            }
            1 => write_source_error_value(self.0.error.original(), verb, state, profile, mode, recursion, output),
            _ => write_source_value(&"MarshalJSON", verb, state, profile, mode, recursion, output),
        }
    }
}
