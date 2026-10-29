//! Native pointer/handle views and the source fmtPointer control branches.
// Adapted from Go fmt/print.go. Copyright The Go Authors. BSD-3-Clause;
// see ../GO-LICENSE. Addresses are native profile data, not Go heap evidence.
use derive_deftly::Deftly;
use crate::{
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces,
    utility_format_arguments::SourcePrintfValue,
    utility_format_methods::{SourcePrintfRecursion, write_source_value, bad_verb},
    utility_format_print::{SourcePrintfMode, pad},
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_integer::{write_number, write_integer},
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceAddressKind { Pointer, UnsafePointer, Channel, Function, Map, Slice }

/// A data-only native handle. Actual source method sets attach through the
/// existing per-value adapters; no address registry or resource owner exists.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfAddress<'a> {
    pub address: u64,
    pub source_type: &'a str,
    pub kind: SourceAddressKind,
}
impl SourcePrintfValue for SourcePrintfAddress<'_> {
    fn source_type(&self) -> &str { self.source_type }
    fn source_is_nil_pointer(&self) -> bool { self.kind == SourceAddressKind::Pointer && self.address == 0 }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_address(self, self.address, verb, state, profile, mode, recursion, output) }
}

/// Borrow a native target while formatting. This supplies pointer/base data;
/// actual source method sets, copied error interfaces and ownership transfer
/// require their concrete native declaration/owned-error adapter.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfPointer<'a, Value> {
    pub target: Option<&'a Value>,
    pub source_type: &'a str,
}
impl<Value: SourcePrintfValue> SourcePrintfValue for SourcePrintfPointer<'_, Value> {
    fn source_type(&self) -> &str { self.source_type }
    fn source_is_nil_pointer(&self) -> bool { self.target.is_none() }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if verb != 'p' && recursion.depth == 0 && Value::SOURCE_DEREFERENCE_AT_ROOT {
            if let Some(value) = self.target {
                append(output, b"&")?;
                return write_source_value(value, verb, state, profile, mode, recursion.child(true), output);
            }
        }
        let address = self.target.map_or(0, |value| value as *const Value as usize as u64);
        write_source_address(self, address, verb, state, profile, mode, recursion, output)
    }
}

pub fn write_source_address<Value: SourcePrintfValue>(value: &Value, address: u64, verb: char,
    state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile, mode: SourcePrintfMode,
    recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    match verb {
        'v' if state.flags.sharp_v => {
            append(output, b"(")?; append(output, value.source_type().as_bytes())?; append(output, b")(")?;
            if address == 0 { append(output, b"nil")?; }
            else {
                let mut number = state.value_state(); number.flags.sharp = true;
                write_number(output, address, false, 16, 'v', false, &number)?;
            }
            append(output, b")")
        }
        'v' if address == 0 => pad(output, b"<nil>", state),
        'v' | 'p' => {
            let mut number = state.value_state(); number.flags.sharp = !state.flags.sharp;
            write_number(output, address, false, 16, 'v', false, &number)
        }
        'b' | 'o' | 'd' | 'x' | 'X' => write_integer(output, address, false, value.source_type(), verb, state, profile),
        _ => bad_verb(value, verb, state, profile, mode, recursion, output),
    }
}
