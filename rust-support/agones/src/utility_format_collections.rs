//! Borrowed native slice/array views for source collection formatting.
// Adapted from Go fmt/print.go. Copyright The Go Authors. BSD-3-Clause;
// see ../GO-LICENSE. The caller remains the storage owner. These typed views
// do not add a writable graph, erased Go value model or runtime registry.
use derive_deftly::Deftly;
use crate::{
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces,
    utility_format_arguments::SourcePrintfValue,
    utility_format_methods::{SourcePrintfRecursion, SourcePrintfMethodViews, SourcePrintfInterfaces, write_source_value, bad_verb},
    utility_format_print::SourcePrintfMode,
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_pointer::write_source_address,
    utility_format_text::write_text,
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError, reserve_source_array},
};

/// None is a nil source slice; Some(empty) is a non-nil empty slice. Type
/// spelling comes from the pinned declaration, rather than a Rust type name.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfSlice<'a, Value> {
    pub values: Option<&'a [Value]>,
    pub source_type: &'a str,
}
impl<'a> SourcePrintfSlice<'a, u8> {
    pub fn bytes(values: Option<&'a [u8]>) -> Self { Self { values, source_type: "[]uint8" } }
}
impl<'a> SourcePrintfSlice<'a, i32> {
    pub fn runes(values: Option<&'a [i32]>) -> Self { Self { values, source_type: "[]int32" } }
}
impl<Value: SourcePrintfValue> SourcePrintfValue for SourcePrintfSlice<'_, Value> {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { self.source_type }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if recursion.depth == 0 && verb == 'p' {
            // Original native backing address only. Full source slice-header,
            // zero-base/heap/address/word/ABI identity remains required.
            let address = self.values.map_or(0, |values| values.as_ptr() as usize as u64);
            return write_source_address(self, address, verb, state, profile, mode, recursion, output);
        }
        write_sequence(self.values.unwrap_or(&[]), self.values.is_none(), false, true,
            self.source_type, verb, state, profile, mode, recursion, output)
    }
}

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfArray<'a, Value, const LENGTH: usize> {
    pub values: &'a [Value; LENGTH],
    pub source_type: &'a str,
    /// Go argument arrays are generally non-addressable. A source reflected
    /// addressable array may expose its bytes directly instead of copying.
    pub source_addressable: bool,
}

/// Source-derived declarations provide typed field access in original order.
/// The caller supplies source visibility; no runtime reflection registry or
/// independently writable struct-field map is constructed here.
pub trait SourcePrintfStructFields: SourcePrintfValue {
    fn source_field_count(&self) -> usize;
    fn source_field_name(&self, index: usize) -> &str;
    fn source_field_interfaceable(&self, index: usize) -> bool;
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>;
}

pub struct SourcePrintfStruct<Value>(pub Value);
impl<Value: SourcePrintfStructFields> SourcePrintfMethodViews for SourcePrintfStruct<Value> {
    fn source_methods(&self) -> SourcePrintfInterfaces<'_> { self.0.source_methods() }
}
impl<Value: SourcePrintfStructFields> SourcePrintfValue for SourcePrintfStruct<Value> {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { self.0.source_type() }
    fn source_error_view(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> { self.0.source_error_view() }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(&self.0, verb, state, profile, mode, recursion, output) }
    fn into_source_error(self) -> Option<crate::errors::BoxError> { self.0.into_source_error() }
}

pub fn write_source_struct<Value: SourcePrintfStructFields>(value: &Value, verb: char,
    state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile, mode: SourcePrintfMode,
    recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    if recursion.depth == 0 && verb == 'p' { return bad_verb(value, verb, state, profile, mode, recursion, output); }
    if state.flags.sharp_v { append(output, value.source_type().as_bytes())?; }
    append(output, b"{")?;
    for index in 0..value.source_field_count() {
        if index != 0 { append(output, if state.flags.sharp_v { b", " } else { b" " })?; }
        if state.flags.plus_v || state.flags.sharp_v {
            let name = value.source_field_name(index);
            if !name.is_empty() { append(output, name.as_bytes())?; append(output, b":")?; }
        }
        value.source_write_field(index, verb, state, profile, mode,
            recursion.child(value.source_field_interfaceable(index)), output)?;
    }
    append(output, b"}")
}
impl<Value: SourcePrintfValue, const LENGTH: usize> SourcePrintfValue for SourcePrintfArray<'_, Value, LENGTH> {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { self.source_type }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if recursion.depth == 0 && verb == 'p' { return bad_verb(self, verb, state, profile, mode, recursion, output); }
        write_sequence(self.values.as_slice(), false, true, self.source_addressable,
            self.source_type, verb, state, profile, mode, recursion, output)
    }
}

fn write_sequence<Value: SourcePrintfValue>(values: &[Value], nil: bool, array: bool, addressable: bool,
    source_type: &str, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
    mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    if Value::SOURCE_KIND_UINT8 && matches!(verb, 's' | 'q' | 'x' | 'X') {
        // Source byte conversion deliberately skips element methods. Native
        // uint8 slices and addressable arrays can lend their contiguous bytes.
        if !array || addressable {
            if let Some(bytes) = Value::source_contiguous_bytes(values) {
                return write_text(output, bytes, verb, state, profile);
            }
        }
        let mut bytes = reserve_source_array::<u8>(values.len() as u64)?;
        for value in values {
            bytes.push(value.source_uint8().expect("source uint8 kind supplies its byte value"));
        }
        return write_text(output, &bytes, verb, state, profile);
    }
    if state.flags.sharp_v {
        append(output, source_type.as_bytes())?;
        if nil { return append(output, b"(nil)"); }
    }
    append(output, if state.flags.sharp_v { b"{" } else { b"[" })?;
    for (index, value) in values.iter().enumerate() {
        if index != 0 { append(output, if state.flags.sharp_v { b", " } else { b" " })?; }
        // Precision and width apply to each element, never to the length of
        // this source slice/array. Panic/erroring state follows the recursion.
        write_source_value(value, verb, state, profile, mode, recursion.child(true), output)?;
    }
    append(output, if state.flags.sharp_v { b"}" } else { b"]" })
}
