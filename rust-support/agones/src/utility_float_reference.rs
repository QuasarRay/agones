//! Exact-decimal source fallback and binary/hex float conversion.
// Adapted from Go internal/strconv/ftoa.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. The selected multiprecision fallback is
// substantive conversion code; optimized Dragonbox/fixed paths remain required.
use crate::{utility_float_decimal::SourceDecimal, utility_go_quote::append,
    utility_http::SourceSliceAllocationError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceFloatWidth { Binary32, Binary64 }
#[derive(Clone, Copy)]
struct FloatInfo { mantissa: u32, exponent: u32, bias: i32 }
impl SourceFloatWidth {
    fn info(self) -> FloatInfo {
        match self { Self::Binary32 => FloatInfo { mantissa: 23, exponent: 8, bias: -127 },
            Self::Binary64 => FloatInfo { mantissa: 52, exponent: 11, bias: -1023 } }
    }
}

pub fn try_source_float_reference(value: f64, width: SourceFloatWidth, verb: u8, mut precision: i32)
    -> Result<Vec<u8>, SourceSliceAllocationError>
{
    let info = width.info();
    let bits = match width { SourceFloatWidth::Binary32 => u64::from((value as f32).to_bits()), SourceFloatWidth::Binary64 => value.to_bits() };
    let negative = bits >> (info.mantissa + info.exponent) != 0;
    let mut exponent = ((bits >> info.mantissa) & ((1u64 << info.exponent) - 1)) as i32;
    let mut mantissa = bits & ((1u64 << info.mantissa) - 1);
    let mut output = Vec::new();
    if exponent == (1i32 << info.exponent) - 1 {
        append(&mut output, if mantissa != 0 { b"NaN" } else if negative { b"-Inf" } else { b"+Inf" })?;
        return Ok(output);
    }
    if exponent == 0 { exponent += 1; } else { mantissa |= 1u64 << info.mantissa; }
    exponent += info.bias;
    if verb == b'b' {
        if negative { append(&mut output, b"-")?; }
        append_uint(&mut output, mantissa)?; append(&mut output, b"p")?;
        append_exponent(&mut output, exponent - info.mantissa as i32, 1)?;
        return Ok(output);
    }
    if verb == b'x' || verb == b'X' {
        write_hex(&mut output, mantissa, exponent, negative, precision, verb, info)?; return Ok(output);
    }
    let mut decimal = SourceDecimal::default(); decimal.assign(mantissa); decimal.shift(exponent - info.mantissa as i32);
    let shortest = precision < 0;
    if shortest {
        round_shortest(&mut decimal, mantissa, exponent, info);
        match verb { b'e' | b'E' => precision = decimal.count - 1,
            b'f' => precision = (decimal.count - decimal.point).max(0), b'g' | b'G' => precision = decimal.count, _ => {} }
    } else {
        match verb { b'e' | b'E' => decimal.round(precision + 1), b'f' => decimal.round(decimal.point + precision),
            b'g' | b'G' => { if precision == 0 { precision = 1; } decimal.round(precision); }, _ => {} }
    }
    match verb {
        b'e' | b'E' => write_e(&mut output, &decimal, negative, precision, verb)?,
        b'f' => write_f(&mut output, &decimal, negative, precision)?,
        b'g' | b'G' => {
            let mut exponent_precision = precision;
            if exponent_precision > decimal.count && decimal.count >= decimal.point { exponent_precision = decimal.count; }
            if shortest { exponent_precision = 6; }
            let exponent = decimal.point - 1;
            if exponent < -4 || exponent >= exponent_precision {
                precision = precision.min(decimal.count);
                write_e(&mut output, &decimal, negative, precision - 1, if verb == b'g' { b'e' } else { b'E' })?;
            } else {
                if precision > decimal.point { precision = decimal.count; }
                write_f(&mut output, &decimal, negative, (precision - decimal.point).max(0))?;
            }
        }
        _ => append(&mut output, &[b'%', verb])?,
    }
    Ok(output)
}

fn round_shortest(decimal: &mut SourceDecimal, mantissa: u64, exponent: i32, info: FloatInfo) {
    if mantissa == 0 { decimal.count = 0; return; }
    let minimum_exponent = info.bias + 1;
    if exponent > minimum_exponent && 332 * (decimal.point - decimal.count) >= 100 * (exponent - info.mantissa as i32) { return; }
    let mut upper = SourceDecimal::default(); upper.assign(mantissa * 2 + 1); upper.shift(exponent - info.mantissa as i32 - 1);
    let (lower_mantissa, lower_exponent) = if mantissa > 1u64 << info.mantissa || exponent == minimum_exponent {
        (mantissa - 1, exponent)
    } else { (mantissa * 2 - 1, exponent - 1) };
    let mut lower = SourceDecimal::default(); lower.assign(lower_mantissa * 2 + 1); lower.shift(lower_exponent - info.mantissa as i32 - 1);
    let inclusive = mantissa % 2 == 0;
    let mut upper_delta = 0u8; let mut index = 0;
    loop {
        let middle_index = index - upper.point + decimal.point;
        if middle_index >= decimal.count { break; }
        let lower_index = index - upper.point + lower.point;
        let low = if lower_index >= 0 && lower_index < lower.count { lower.digits[lower_index as usize] } else { b'0' };
        let middle = if middle_index >= 0 { decimal.digits[middle_index as usize] } else { b'0' };
        let high = if index < upper.count { upper.digits[index as usize] } else { b'0' };
        let down = low != middle || inclusive && lower_index + 1 == lower.count;
        if upper_delta == 0 && middle + 1 < high { upper_delta = 2; }
        else if upper_delta == 0 && middle != high { upper_delta = 1; }
        else if upper_delta == 1 && (middle != b'9' || high != b'0') { upper_delta = 2; }
        let up = upper_delta > 0 && (inclusive || upper_delta > 1 || index + 1 < upper.count);
        match (down, up) {
            (true, true) => { decimal.round(middle_index + 1); return; }
            (true, false) => { decimal.round_down(middle_index + 1); return; }
            (false, true) => { decimal.round_up(middle_index + 1); return; }
            _ => index += 1,
        }
    }
}

fn write_e(output: &mut Vec<u8>, decimal: &SourceDecimal, negative: bool, precision: i32, verb: u8)
    -> Result<(), SourceSliceAllocationError>
{
    if negative { append(output, b"-")?; }
    append(output, &[if decimal.count == 0 { b'0' } else { decimal.digits[0] }])?;
    if precision > 0 {
        append(output, b".")?;
        let count = (decimal.count - 1).max(0).min(precision) as usize;
        if count != 0 { append(output, &decimal.digits[1..1 + count])?; }
        repeat(output, b'0', precision as usize - count)?;
    }
    append(output, &[verb])?;
    append_exponent(output, if decimal.count == 0 { 0 } else { decimal.point - 1 }, 2)
}

fn write_f(output: &mut Vec<u8>, decimal: &SourceDecimal, negative: bool, precision: i32)
    -> Result<(), SourceSliceAllocationError>
{
    if negative { append(output, b"-")?; }
    if decimal.point > 0 {
        let count = decimal.count.min(decimal.point) as usize;
        append(output, &decimal.digits[..count])?; repeat(output, b'0', decimal.point as usize - count)?;
    } else { append(output, b"0")?; }
    if precision > 0 {
        append(output, b".")?;
        let leading = (-decimal.point).max(0).min(precision) as usize;
        repeat(output, b'0', leading)?;
        let start = decimal.point.max(0).min(decimal.count) as usize;
        let count = (precision as usize - leading).min(decimal.count as usize - start);
        append(output, &decimal.digits[start..start + count])?;
        repeat(output, b'0', precision as usize - leading - count)?;
    }
    Ok(())
}

fn write_hex(output: &mut Vec<u8>, mut mantissa: u64, mut exponent: i32, negative: bool,
    precision: i32, verb: u8, info: FloatInfo) -> Result<(), SourceSliceAllocationError>
{
    if mantissa == 0 { exponent = 0; }
    mantissa <<= 60 - info.mantissa;
    while mantissa != 0 && mantissa & (1u64 << 60) == 0 { mantissa <<= 1; exponent -= 1; }
    if (0..15).contains(&precision) {
        let shift = precision as u32 * 4;
        let extra = (mantissa << shift) & ((1u64 << 60) - 1);
        mantissa >>= 60 - shift;
        if (extra | (mantissa & 1)) > (1u64 << 59) { mantissa += 1; }
        mantissa <<= 60 - shift;
        if mantissa & (1u64 << 61) != 0 { mantissa >>= 1; exponent += 1; }
    }
    let alphabet = if verb == b'x' { b"0123456789abcdef" } else { b"0123456789ABCDEF" };
    if negative { append(output, b"-")?; }
    append(output, &[b'0', verb, b'0' + ((mantissa >> 60) & 1) as u8])?;
    mantissa <<= 4;
    if precision < 0 && mantissa != 0 {
        append(output, b".")?;
        while mantissa != 0 { append(output, &[alphabet[((mantissa >> 60) & 15) as usize]])?; mantissa <<= 4; }
    } else if precision > 0 {
        append(output, b".")?;
        for _ in 0..precision { append(output, &[alphabet[((mantissa >> 60) & 15) as usize]])?; mantissa <<= 4; }
    }
    append(output, if verb == b'x' { b"p" } else { b"P" })?;
    append_exponent(output, exponent, 2)
}

pub(crate) fn repeat(output: &mut Vec<u8>, byte: u8, count: usize) -> Result<(), SourceSliceAllocationError> {
    let length = output.len().checked_add(count).ok_or(SourceSliceAllocationError::NativeAddressSpace)?;
    if length > isize::MAX as usize { return Err(SourceSliceAllocationError::NativeAddressSpace); }
    output.try_reserve(count).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
    output.resize(length, byte); Ok(())
}
fn append_exponent(output: &mut Vec<u8>, exponent: i32, minimum_digits: usize) -> Result<(), SourceSliceAllocationError> {
    append(output, if exponent < 0 { b"-" } else { b"+" })?;
    let mut buffer = [0; 24]; let digits = unsigned_digits(u64::from(exponent.unsigned_abs()), &mut buffer);
    repeat(output, b'0', minimum_digits.saturating_sub(digits.len()))?; append(output, digits)
}
fn append_uint(output: &mut Vec<u8>, value: u64) -> Result<(), SourceSliceAllocationError> {
    let mut buffer = [0; 24]; append(output, unsigned_digits(value, &mut buffer))
}
fn unsigned_digits(mut value: u64, buffer: &mut [u8; 24]) -> &[u8] {
    let mut start = buffer.len();
    loop { start -= 1; buffer[start] = b'0' + (value % 10) as u8; value /= 10; if value == 0 { return &buffer[start..]; } }
}
