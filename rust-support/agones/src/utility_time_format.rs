//! Source Time.String/GoString and strict RFC3339 JSON output.
// Go1.26.5 time/{time,format,format_rfc3339}.go, Copyright The Go Authors,
// BSD-3-Clause (../GO-LICENSE). Native word/allocator/timezone profiles and
// general Format/Parse/Date/binary/text APIs remain required separately.
use crate::{utility_time_values::{SourceGoTime, SourceTimeLocation, ABSOLUTE_YEARS},
    utility_error_slice::SourceRetainedError,
    utility_go_quote::append,
    utility_klog_dispatch::{FormatResult, SourceKlogJsonEncoder, SourceKlogEncodeError, SourceKlogFormatError},
    utility_http::{SourceSliceAllocationError, SourceUnicodeProfile}};
use std::sync::Arc;

// Source absDays.split/date and absSeconds.clock use unsigned arithmetic
// after translating to the March-based absolute epoch. Mul32 returns both
// halves; the low-half division is intentional and must not be simplified.
fn calendar(absolute: u64) -> (i64, i64, i64, i64, i64, i64) {
    let days = absolute / 86400;
    let d = days.wrapping_mul(4).wrapping_add(3);
    let century = d / 146097;
    let cd = (d % 146097) as u32 | 3;
    let product = 2939745_u64 * u64::from(cd);
    let cyear = (product >> 32) as i64;
    let ayday = (product as u32 / 2939745 / 4) as i64;
    let month_day = 2141_u32.wrapping_mul(ayday as u32).wrapping_add(197913);
    let jan_feb = i64::from(ayday >= 306);
    let year = century.wrapping_mul(100).wrapping_sub(ABSOLUTE_YEARS) as i64;
    let year = year.wrapping_add(cyear).wrapping_add(jan_feb);
    let month = i64::from(month_day >> 16) - jan_feb * 12;
    let day = 1 + i64::from((month_day & 0xffff) / 2141);
    let seconds = (absolute % 86400) as i64;
    (year, month, day, seconds / 3600, seconds % 3600 / 60, seconds % 60)
}
fn number(output: &mut Vec<u8>, value: i64, width: usize) -> Result<(), SourceSliceAllocationError> {
    let mut magnitude = value as u64;
    if value < 0 { append(output, b"-")?; magnitude = magnitude.wrapping_neg(); }
    let mut digits = [0_u8; 20]; let mut start = digits.len();
    loop { start -= 1; digits[start] = b'0' + (magnitude % 10) as u8; magnitude /= 10; if magnitude == 0 { break; } }
    for _ in digits.len() - start..width { append(output, b"0")?; }
    append(output, &digits[start..])
}
fn nano(output: &mut Vec<u8>, value: i32) -> Result<(), SourceSliceAllocationError> {
    if value == 0 { return Ok(()); }
    append(output, b".")?; number(output, i64::from(value), 9)?;
    while output.last() == Some(&b'0') { output.pop(); }
    if output.last() == Some(&b'.') { output.pop(); } Ok(())
}
fn date_time(output: &mut Vec<u8>, absolute: u64, separator: &[u8]) -> Result<(), SourceSliceAllocationError> {
    let (year, month, day, hour, minute, second) = calendar(absolute);
    number(output, year, 4)?; append(output, b"-")?; number(output, month, 2)?;
    append(output, b"-")?; number(output, day, 2)?; append(output, separator)?;
    number(output, hour, 2)?; append(output, b":")?; number(output, minute, 2)?;
    append(output, b":")?; number(output, second, 2)
}
fn zone(output: &mut Vec<u8>, offset: i64, colon: bool) -> Result<(), SourceSliceAllocationError> {
    let mut minutes = offset / 60;
    if minutes < 0 { append(output, b"-")?; minutes = -minutes; } else { append(output, b"+")?; }
    number(output, minutes / 60, 2)?; if colon { append(output, b":")?; } number(output, minutes % 60, 2)
}
impl SourceGoTime {
    pub fn try_string(&self) -> FormatResult<Vec<u8>> {
        let (offset, absolute) = self.location_abs()?;
        let mut output = Vec::new(); date_time(&mut output, absolute, b" ")?;
        nano(&mut output, self.nanosecond())?; append(&mut output, b" ")?;
        zone(&mut output, offset.offset, false)?; append(&mut output, b" ")?;
        if offset.name.0.is_empty() { zone(&mut output, offset.offset, false)?; }
        else { append(&mut output, &offset.name.0)?; }
        if self.has_monotonic() {
            let mut magnitude = self.ext as u64;
            append(&mut output, b" m=")?;
            if self.ext < 0 { append(&mut output, b"-")?; magnitude = magnitude.wrapping_neg(); }
            else { append(&mut output, b"+")?; }
            let fractional = magnitude % 1_000_000_000;
            let seconds = magnitude / 1_000_000_000;
            let billions = seconds / 1_000_000_000;
            if billions != 0 { number(&mut output, billions as i64, 0)?; }
            number(&mut output, (seconds % 1_000_000_000) as i64, if billions == 0 { 0 } else { 9 })?;
            append(&mut output, b".")?; number(&mut output, fractional as i64, 9)?;
        }
        Ok(output)
    }
    pub fn try_go_string(&self, _: SourceUnicodeProfile) -> FormatResult<Vec<u8>> {
        let (_, absolute) = self.location_abs()?;
        let (year, month, day, hour, minute, second) = calendar(absolute);
        const MONTHS: [&[u8]; 12] = [b"January", b"February", b"March", b"April", b"May", b"June", b"July", b"August", b"September", b"October", b"November", b"December"];
        let mut output = b"time.Date(".to_vec(); number(&mut output, year, 0)?;
        if (1..=12).contains(&month) { append(&mut output, b", time.")?; append(&mut output, MONTHS[(month - 1) as usize])?; }
        else { number(&mut output, month, 0)?; } // source fallback has no comma
        for value in [day, hour, minute, second, i64::from(self.nanosecond())] {
            append(&mut output, b", ")?; number(&mut output, value, 0)?;
        }
        append(&mut output, b", ")?;
        let location = self.location();
        if Arc::ptr_eq(&location, &SourceTimeLocation::utc()) { append(&mut output, b"time.UTC")?; }
        else if location.is_current_local() { append(&mut output, b"time.Local")?; }
        else {
            append(&mut output, b"time.Location(")?; quote_location(&mut output, &location.name()?.0)?; append(&mut output, b")")?;
        }
        append(&mut output, b")")?; Ok(output)
    }
    fn append_strict_rfc3339(&self, output: &mut Vec<u8>) -> FormatResult<Option<&'static [u8]>> {
        let start = output.len(); let (offset, absolute) = self.location_abs()?;
        date_time(output, absolute, b"T")?; nano(output, self.nanosecond())?;
        if offset.offset == 0 { append(output, b"Z")?; } else { zone(output, offset.offset, true)?; }
        if output[start + 4] != b'-' { return Ok(Some(b"year outside of range [0,9999]")); }
        if output.last() != Some(&b'Z') {
            let len = output.len(); let sign = output[len - 6];
            let hour = 10_u8.wrapping_mul(output[len - 5].wrapping_sub(b'0')).wrapping_add(output[len - 4].wrapping_sub(b'0'));
            if sign.is_ascii_digit() || hour >= 24 { return Ok(Some(b"timezone hour outside of range [0,23]")); }
        }
        Ok(None)
    }
    /// Outer failure identifies a native binding/allocation gap; the inner
    /// error is the actual returned *errors.errorString from MarshalJSON.
    pub fn try_marshal_json(&self) -> FormatResult<Result<Vec<u8>, SourceRetainedError>> {
        let mut output = Vec::new(); append(&mut output, b"\"")?;
        let error = self.append_strict_rfc3339(&mut output)?; append(&mut output, b"\"")?;
        if let Some(error) = error {
            let mut message = b"Time.MarshalJSON: ".to_vec(); append(&mut message, error)?;
            return Ok(Err(SourceRetainedError::from_original(crate::errors::Errors::default().new_bytes(&message)?)));
        }
        Ok(Ok(output))
    }
}
// time.quote deliberately hex-escapes every non-ASCII/control original byte,
// including valid UTF8, and leaves DEL literal. It is not strconv.Quote.
fn quote_location(output: &mut Vec<u8>, name: &[u8]) -> Result<(), SourceSliceAllocationError> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    append(output, b"\"")?;
    for byte in name {
        if *byte >= 128 || *byte < b' ' { append(output, &[b'\\', b'x', HEX[(byte >> 4) as usize], HEX[(byte & 15) as usize]])?; }
        else { if matches!(*byte, b'"' | b'\\') { append(output, b"\\")?; } append(output, &[*byte])?; }
    }
    append(output, b"\"")
}
impl SourceKlogJsonEncoder for SourceGoTime {
    fn source_encode_json(&self, output: &mut Vec<u8>, _: SourceUnicodeProfile) -> Result<(), SourceKlogEncodeError> {
        match self.try_marshal_json() {
            Ok(Ok(json)) => { append(output, &json)?; append(output, b"\n")?; Ok(()) }
            Ok(Err(error)) => Err(SourceKlogEncodeError::Source(crate::utility_time_json_error::time_marshaler_error(error))),
            Err(SourceKlogFormatError::Allocation(error)) => Err(SourceKlogEncodeError::Allocation(error)),
            Err(SourceKlogFormatError::Unbound { .. }) => Err(SourceKlogEncodeError::Unbound),
        }
    }
}
