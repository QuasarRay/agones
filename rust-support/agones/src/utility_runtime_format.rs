//! Source formatting and Error methods for existing runtime error values.
// Adapted from Go runtime/error.go, internal/abi/bounds.go and fmt/print.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
// Pure typed data paths participate in the existing CGP formatter/crash owner.
use std::{borrow::Cow, cmp::Ordering, fmt};
use crate::{
    errors::BoxError, runtime::SourceLogType,
    utility_panic::{SourceRuntimeErrorString, SourceNilMapAssignment, SourceBoundsError, SourceBoundsCode,
        unwind_source_runtime_error},
    utility_error_alias::SourceSharedError,
    utility_format_arguments::SourcePrintfValue,
    utility_format_methods::{SourceErrorMethod, SourcePrintfRecursion, bad_verb},
    utility_format_collections::{SourcePrintfStructFields, write_source_struct},
    utility_format_integer::write_integer, utility_format_bool::write_bool,
    utility_format_text::write_text, utility_format_print::SourcePrintfMode,
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts},
    utility_format_map::SourceFmtMapKey,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

const NIL_MAP_MESSAGE: &[u8] = b"assignment to entry in nil map";
const BOUNDS_FORMATS: [&[u8]; 9] = [
    b"index out of range [%x] with length %y",
    b"slice bounds out of range [:%x] with length %y",
    b"slice bounds out of range [:%x] with capacity %y",
    b"slice bounds out of range [%x:%y]",
    b"slice bounds out of range [::%x] with length %y",
    b"slice bounds out of range [::%x] with capacity %y",
    b"slice bounds out of range [:%x:%y]",
    b"slice bounds out of range [%x:%y:]",
    b"cannot convert slice with length %y to array or pointer to array with length %x",
];
const NEGATIVE_BOUNDS_FORMATS: [&[u8]; 8] = [
    b"index out of range [%x]", b"slice bounds out of range [:%x]",
    b"slice bounds out of range [:%x]", b"slice bounds out of range [%x:]",
    b"slice bounds out of range [::%x]", b"slice bounds out of range [::%x]",
    b"slice bounds out of range [:%x:]", b"slice bounds out of range [%x::]",
];

impl SourceErrorMethod for SourceRuntimeErrorString {
    fn source_error_message(&self) -> Cow<'_, [u8]> {
        let mut message = Vec::with_capacity(b"runtime error: ".len() + self.message.len());
        message.extend_from_slice(b"runtime error: "); message.extend_from_slice(self.message.as_bytes());
        Cow::Owned(message)
    }
}
impl SourceErrorMethod for SourceNilMapAssignment {
    fn source_error_message(&self) -> Cow<'_, [u8]> { Cow::Borrowed(NIL_MAP_MESSAGE) }
}
impl SourceErrorMethod for SourceBoundsError {
    fn source_error_message(&self) -> Cow<'_, [u8]> {
        let code = usize::from(self.code.0);
        // The positive table is indexed first in the source, even when a
        // negative override will be selected. Convert has no negative entry.
        let mut format = *BOUNDS_FORMATS.get(code).unwrap_or_else(|| bounds_table_failure(code, BOUNDS_FORMATS.len()));
        if self.signed && self.x < 0 {
            format = *NEGATIVE_BOUNDS_FORMATS.get(code).unwrap_or_else(|| bounds_table_failure(code, NEGATIVE_BOUNDS_FORMATS.len()));
        }
        let mut message = Vec::with_capacity(100);
        message.extend_from_slice(b"runtime error: ");
        let mut index = 0;
        while index < format.len() {
            let byte = format[index]; index += 1;
            if byte != b'%' { message.push(byte); continue; }
            match format[index] {
                b'x' => append_int(&mut message, self.x, self.signed),
                b'y' => append_int(&mut message, self.y, true),
                _ => unreachable!("source bounds format placeholder"),
            }
            index += 1;
        }
        Cow::Owned(message)
    }
}
fn bounds_table_failure(index: usize, length: usize) -> ! {
    unwind_source_runtime_error(SourceBoundsError { x: index as i64, y: length as i64,
        signed: false, code: SourceBoundsCode::Index })
}
fn append_int(output: &mut Vec<u8>, mut value: i64, signed: bool) {
    if signed && value < 0 { output.push(b'-'); value = value.wrapping_neg(); }
    let mut number = value as u64; let mut buffer = [0; 20]; let mut start = buffer.len();
    loop {
        start -= 1; buffer[start] = b'0' + (number % 10) as u8; number /= 10;
        if number == 0 { break; }
    }
    output.extend_from_slice(&buffer[start..]);
}

// Native Display uses the same source message body; source byte consumers
// keep the Cow bytes and do not pass through native lossy string rendering.
macro_rules! runtime_displays {
    ($($ty:ty),+ $(,)?) => { $(impl fmt::Display for $ty {
        fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
            out.write_str(&String::from_utf8_lossy(&self.source_error_message()))
        }
    })+ };
}
runtime_displays!(SourceRuntimeErrorString, SourceNilMapAssignment, SourceBoundsError);

macro_rules! runtime_strings {
    ($($ty:ty => $bytes:expr),+ $(,)?) => { $(impl SourcePrintfValue for $ty {
        fn source_type(&self) -> &str { <Self as SourceLogType>::SOURCE_LOG_TYPE }
        fn source_error_view(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> { Some(self) }
        fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
            mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
        {
            if matches!(verb, 'v' | 's' | 'q' | 'x' | 'X') { write_text(output, ($bytes)(self), verb, state, profile) }
            else { bad_verb(self, verb, state, profile, mode, recursion, output) }
        }
        fn into_source_error(self) -> Option<BoxError> {
            Some(SourceSharedError::from_runtime_original(Box::new(self)).into_error())
        }
    })+ };
}
runtime_strings!(SourceRuntimeErrorString => |value: &SourceRuntimeErrorString| value.message.as_bytes(),
    SourceNilMapAssignment => |_: &SourceNilMapAssignment| NIL_MAP_MESSAGE);

impl SourcePrintfValue for SourceBoundsCode {
    const SOURCE_KIND_UINT8: bool = true;
    fn source_type(&self) -> &str { "abi.BoundsErrorCode" }
    fn source_integer(&self) -> SourcePrintfInteger { SourcePrintfInteger::Unsigned(u64::from(self.0)) }
    fn source_uint8(&self) -> Option<u8> { Some(self.0) }
    fn source_contiguous_bytes(values: &[Self]) -> Option<&[u8]> {
        // SAFETY: repr(transparent) over u8 has identical size/alignment and
        // all 256 byte patterns are valid. The shared slice keeps its lifetime
        // and immutability; no second allocation or writable view is created.
        Some(unsafe { std::slice::from_raw_parts(values.as_ptr().cast::<u8>(), values.len()) })
    }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_integer(output, u64::from(self.0), false, self.source_type(), verb, state, profile) }
}
impl SourcePrintfValue for SourceBoundsError {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { <Self as SourceLogType>::SOURCE_LOG_TYPE }
    fn source_error_view(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> { Some(self) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
    fn into_source_error(self) -> Option<BoxError> { Some(SourceSharedError::from_runtime_original(Box::new(self)).into_error()) }
}
impl SourcePrintfStructFields for SourceBoundsError {
    fn source_field_count(&self) -> usize { 4 }
    fn source_field_name(&self, index: usize) -> &str { ["x", "y", "signed", "code"][index] }
    fn source_field_interfaceable(&self, _: usize) -> bool { false }
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, _: SourcePrintfMode, _: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        match index {
            0 => write_integer(output, self.x as u64, true, "int64", verb, state, profile),
            1 => write_integer(output, self.y as u64, true, "int", verb, state, profile),
            2 => write_bool(output, self.signed, verb, state),
            3 => write_integer(output, u64::from(self.code.0), false, "abi.BoundsErrorCode", verb, state, profile),
            _ => panic!("source bounds field index out of range"),
        }
    }
}
impl SourceFmtMapKey for SourceRuntimeErrorString {
    fn source_key_cmp(&self, other: &Self) -> Ordering { self.message.as_bytes().cmp(other.message.as_bytes()) }
}
impl SourceFmtMapKey for SourceNilMapAssignment {
    fn source_key_cmp(&self, _: &Self) -> Ordering { Ordering::Equal }
}
impl SourceFmtMapKey for SourceBoundsCode {
    fn source_key_cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
}
impl SourceFmtMapKey for SourceBoundsError {
    fn source_key_cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.signed.cmp(&other.signed)).then_with(|| self.code.source_key_cmp(&other.code))
    }
}
