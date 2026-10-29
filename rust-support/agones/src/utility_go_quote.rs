//! Go quoted strings and selected canonical diagnostic value syntax.
// Adapted from Go strconv/quote.go, unicode/graphic.go and fmt/print.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use std::error::Error;
use crate::{utility_http::{SourceUnicodeProfile, SourceSliceAllocationError, reserve_source_array},
    utility_panic::{SourceRuntimeErrorString, SourceNilMapAssignment, SourceBoundsError, SourceBoundsCode}};

pub(crate) fn append(output: &mut Vec<u8>, value: &[u8]) -> Result<(), SourceSliceAllocationError> {
    let length = output.len().checked_add(value.len()).ok_or(SourceSliceAllocationError::NativeAddressSpace)?;
    if length > isize::MAX as usize { return Err(SourceSliceAllocationError::NativeAddressSpace); }
    output.try_reserve(value.len()).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
    output.extend_from_slice(value); Ok(())
}
const HEX: &[u8; 16] = b"0123456789abcdef";
pub fn try_source_go_quote(value: &[u8], profile: SourceUnicodeProfile) -> Result<String, SourceSliceAllocationError> {
    try_source_go_quote_with_ascii(value, profile, false)
}

/// fmt's plus-q path uses QuoteToASCII; ordinary q retains printable Unicode.
pub fn try_source_go_quote_with_ascii(value: &[u8], profile: SourceUnicodeProfile, ascii_only: bool)
    -> Result<String, SourceSliceAllocationError>
{
    quote_delimited(value, profile, ascii_only, b'"')
}

pub fn try_source_go_quote_rune(rune: char, profile: SourceUnicodeProfile, ascii_only: bool)
    -> Result<String, SourceSliceAllocationError>
{
    let mut bytes = [0; 4];
    quote_delimited(rune.encode_utf8(&mut bytes).as_bytes(), profile, ascii_only, b'\'')
}

fn quote_delimited(value: &[u8], profile: SourceUnicodeProfile, ascii_only: bool, quote: u8)
    -> Result<String, SourceSliceAllocationError>
{
    let length = value.len().checked_add(2).ok_or(SourceSliceAllocationError::NativeAddressSpace)?;
    let mut output = reserve_source_array::<u8>(length as u64)?;
    append(&mut output, &[quote])?;
    let mut offset = 0;
    while offset < value.len() {
        let first = value[offset];
        let width = match first { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => 0 };
        let decoded = (width != 0 && offset + width <= value.len()).then(|| &value[offset..offset + width])
            .and_then(|bytes| std::str::from_utf8(bytes).ok()).and_then(|text| text.chars().next());
        let Some(rune) = decoded.map(|rune| rune as u32) else {
            append(&mut output, &[b'\\', b'x', HEX[(first >> 4) as usize], HEX[(first & 15) as usize]])?;
            offset += 1; continue;
        };
        if rune == quote as u32 || rune == b'\\' as u32 { append(&mut output, &[b'\\', rune as u8])?; }
        else if profile.is_print(rune) && (!ascii_only || rune < 0x80) { append(&mut output, &value[offset..offset + width])?; }
        else {
            let short = match rune { 7 => Some(b'a'), 8 => Some(b'b'), 12 => Some(b'f'), 10 => Some(b'n'),
                13 => Some(b'r'), 9 => Some(b't'), 11 => Some(b'v'), _ => None };
            if let Some(escape) = short { append(&mut output, &[b'\\', escape])?; }
            else if rune < 0x20 || rune == 0x7f {
                append(&mut output, &[b'\\', b'x', HEX[(rune >> 4) as usize], HEX[(rune & 15) as usize]])?;
            } else {
                let digits = if rune < 0x10000 { 4 } else { 8 };
                append(&mut output, if digits == 4 { b"\\u" } else { b"\\U" })?;
                for index in (0..digits).rev() { append(&mut output, &[HEX[((rune >> (index * 4)) & 15) as usize]])?; }
            }
        }
        offset += width;
    }
    append(&mut output, &[quote])?;
    Ok(String::from_utf8(output).expect("Go quoting emits UTF8 or ASCII escapes"))
}

pub fn source_can_backquote(mut value: &[u8]) -> bool {
    while !value.is_empty() {
        let (rune, width) = crate::utility_format_scan::decode_source_rune(value);
        value = &value[width..];
        if width > 1 {
            if rune == '\u{feff}' { return false; }
            continue;
        }
        if rune == '\u{fffd}' || (rune < ' ' && rune != '\t') || rune == '`' || rune == '\u{7f}' { return false; }
    }
    true
}

/// Selected source dynamic types whose complete value data is owned here.
/// Other errors still require their GoStringer/reflection/pointer profile.
pub fn try_source_error_go_syntax(error: &(dyn Error + 'static), profile: SourceUnicodeProfile)
    -> Result<Option<String>, SourceSliceAllocationError>
{
    let error = crate::utility_error_alias::source_error_original(error);
    if let Some(error) = error.downcast_ref::<SourceRuntimeErrorString>() {
        return try_source_go_quote(error.message.as_bytes(), profile).map(Some);
    }
    if error.is::<SourceNilMapAssignment>() {
        return try_source_go_quote(b"assignment to entry in nil map", profile).map(Some);
    }
    if let Some(error) = error.downcast_ref::<SourceBoundsError>() {
        let code = match error.code { SourceBoundsCode::Index => 0, SourceBoundsCode::SliceLength => 1,
            SourceBoundsCode::SliceCapacity => 2, SourceBoundsCode::SliceLow => 3, SourceBoundsCode::Slice3Length => 4,
            SourceBoundsCode::Slice3Capacity => 5, SourceBoundsCode::Slice3High => 6, SourceBoundsCode::Slice3Low => 7,
            SourceBoundsCode::SliceConvert => 8 };
        return Ok(Some(format!("runtime.boundsError{{x:{}, y:{}, signed:{}, code:0x{code:x}}}", error.x, error.y, error.signed)));
    }
    if let Some(error) = error.downcast_ref::<crate::errors::MessageError>() {
        return Ok(Some(format!("&errors.errorString{{s:{}}}", try_source_go_quote(error.0.as_bytes(), profile)?)));
    }
    Ok(None)
}
