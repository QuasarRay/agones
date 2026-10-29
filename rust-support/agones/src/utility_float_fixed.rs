//! Source fixed-digit float conversion with exactness and ties-to-even rounding.
// Adapted from Go internal/strconv/ftoafixed.go at go1.26.5.
// Copyright 2025 The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use crate::{utility_float_math::{SourceFloatDigits, power10, mul_log10_2,
    divisible_power5, multiply192, format_base10, shr64, shl64},
    utility_format_methods::SourcePrintfPanic, utility_http::HeaderAllocationProfile};

const UINT64_POWER10: [u64; 20] = [1, 10, 100, 1000, 10000, 100000, 1000000,
    10000000, 100000000, 1000000000, 10000000000, 100000000000, 1000000000000,
    10000000000000, 100000000000000, 1000000000000000, 10000000000000000,
    100000000000000000, 1000000000000000000, 10000000000000000000];

pub(crate) fn source_fixed(buffer: &mut [u8; 24], mut mantissa: u64, mut exponent: i32,
    mut digits: i32, precision: i32, verb: u8, profile: HeaderAllocationProfile) -> SourceFloatDigits<'_>
{
    if digits > 18 { SourcePrintfPanic::raise(String::from("fixedFtoa called with digits > 18")); }
    // genericFtoa supplies mantissa > 0 and 1 <= digits <= 18.
    let leading = mantissa.leading_zeros(); mantissa <<= leading; exponent -= leading as i32;
    let p = digits - 1 - mul_log10_2(63 + exponent);
    let (mut power, power_exponent) = power10(p).unwrap_or_else(||
        SourcePrintfPanic::raise(String::from("fixedFtoa: pow10 out of range")));
    if -22 <= p && p < 0 { power.low = power.low.wrapping_add(1); }
    let (mut decimal, low1, low0) = multiply192(mantissa, power);
    let decimal_exponent = exponent + power_exponent;
    let mut truncated = if 0 <= p && p <= 55 { u64::from(low1 | low0 != 0) }
        else if -22 <= p && p < 0 && divisible_power5(mantissa, -p) { 0 } else { 1 };
    let shift = (-decimal_exponent - 1) as u32;
    truncated |= u64::from(decimal & shl64(1, shift).wrapping_sub(1) != 0);
    decimal = shr64(decimal, shift);
    let mut point = digits - p;
    let mut maximum = UINT64_POWER10[digits as usize] << 1;
    if decimal >= maximum {
        let residual = decimal % 10; decimal /= 10;
        truncated |= u64::from(residual != 0); point += 1;
    }
    if verb == b'f' && digits != point + precision {
        while digits > point + precision {
            let residual = decimal % 10; decimal /= 10;
            truncated |= u64::from(residual != 0); digits -= 1;
        }
        if digits <= 0 { digits = 1; point += 1; }
        maximum = UINT64_POWER10[digits as usize] << 1;
    }
    // Preserve the selected source uint cast before its rounding expression.
    let word = if profile.word_bits() == 32 { u64::from(decimal as u32) } else { decimal };
    decimal = decimal.wrapping_add(word & (truncated | (word >> 1)) & 1);
    decimal >>= 1;
    if decimal == maximum >> 1 { decimal = UINT64_POWER10[(digits - 1) as usize]; point += 1; }
    let mut count = 0;
    if decimal != 0 {
        if format_base10(&mut buffer[..digits as usize], decimal, profile) != 0 {
            SourcePrintfPanic::raise(String::from("formatBase10"));
        }
        count = digits;
        while buffer[(count - 1) as usize] == b'0' { count -= 1; }
    }
    SourceFloatDigits { digits: &buffer[..digits as usize], count, point }
}
