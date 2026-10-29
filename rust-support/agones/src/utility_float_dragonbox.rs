//! Source Dragonbox shortest conversion for binary32 and binary64.
// Adapted from Go internal/strconv/ftoadbox.go at go1.26.5.
// Copyright 2025 The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use crate::{utility_float_reference::SourceFloatWidth, utility_http::HeaderAllocationProfile, utility_float_math::{
    SourceUint128, SourceFloatDigits, power10, mul_log10_2,
    mul_log10_2_minus_log10_4_over_3, trim_zeros, format_base10,
    multiply192_upper128, multiply192_lower128, multiply96_upper64,
    multiply96_lower64, shr64, shl64, shl32,
}};

pub(crate) fn source_dragonbox(buffer: &mut [u8; 32], mantissa: u64, exponent: i32,
    denormal: bool, width: SourceFloatWidth, profile: HeaderAllocationProfile) -> SourceFloatDigits<'_>
{
    let (significand, decimal_exponent) = match width {
        SourceFloatWidth::Binary32 => dragonbox32(mantissa as u32, exponent, denormal),
        SourceFloatWidth::Binary64 => dragonbox64(mantissa, exponent, denormal),
    };
    let start = format_base10(buffer, significand, profile);
    let count = (buffer.len() - start) as i32;
    SourceFloatDigits { digits: &buffer[start..], count, point: count + decimal_exponent }
}

// The source has two algorithm copies to retain native-width arithmetic.
// This declaration emits both concrete bodies; no runtime width erasure.
macro_rules! dragonbox {
    ($name:ident, $uint:ty, $mantissa_bits:literal, $kappa:literal, $p10:literal,
        $power:ident, $range:ident, $round_up:ident, $multiply:ident, $delta:ident,
        $parity:ident, $shift:ident) => {
        fn $name(mantissa: $uint, exponent: i32, denormal: bool) -> (u64, i32) {
            if mantissa == (1 as $uint) << $mantissa_bits && !denormal {
                let k0 = -mul_log10_2_minus_log10_4_over_3(exponent);
                let (phi, beta) = $power(k0, exponent);
                let (mut left, right) = $range(phi, beta);
                if exponent != 2 && exponent != 3 { left = left.wrapping_add(1); }
                let quotient = right / 10;
                if left <= quotient * 10 {
                    let (significand, zeros) = trim_zeros(quotient as u64);
                    return (significand, -k0 + 1 + zeros);
                }
                let mut rounded = $round_up(phi, beta);
                if exponent == -77 && rounded % 2 != 0 { rounded = rounded.wrapping_sub(1); }
                else if rounded < left { rounded = rounded.wrapping_add(1); }
                return (rounded as u64, -k0);
            }
            let k0 = -mul_log10_2(exponent);
            let (phi, beta) = $power($kappa + k0, exponent);
            let scaled = $shift(mantissa.wrapping_mul(2).wrapping_add(1), beta as u32);
            let (integer, exact) = $multiply(scaled, phi);
            let mut significand = integer / ($p10 * 10);
            let mut remainder = (integer % ($p10 * 10)) as u32;
            let delta = $delta(phi, beta);
            if remainder < delta {
                if remainder != 0 || !exact || mantissa % 2 == 0 {
                    let (significand, zeros) = trim_zeros(significand as u64);
                    return (significand, -k0 + 1 + zeros);
                }
                significand = significand.wrapping_sub(1); remainder = $p10 * 10;
            } else if remainder == delta {
                let (parity, exact) = $parity(mantissa.wrapping_mul(2).wrapping_sub(1), phi, beta);
                if parity || exact && mantissa % 2 == 0 {
                    let (significand, zeros) = trim_zeros(significand as u64);
                    return (significand, -k0 + 1 + zeros);
                }
            }
            let distance = remainder.wrapping_add($p10 / 2).wrapping_sub(delta / 2);
            let quotient = distance / $p10; let residual = distance % $p10;
            let mut rounded = significand.wrapping_mul(10).wrapping_add(quotient as $uint);
            if residual == 0 {
                let (parity, exact) = $parity(mantissa.wrapping_mul(2), phi, beta);
                if parity != (distance.wrapping_sub($p10 / 2) % 2 != 0) || exact && rounded % 2 != 0 {
                    rounded = rounded.wrapping_sub(1);
                }
            }
            (rounded as u64, -k0)
        }
    };
}
dragonbox!(dragonbox64, u64, 52, 2, 100, power64, range64, round_up64, multiply_power64, delta64, parity64, shl64);
dragonbox!(dragonbox32, u32, 23, 1, 10, power32, range32, round_up32, multiply_power32, delta32, parity32, shl32);

fn power64(k: i32, exponent: i32) -> (SourceUint128, i32) {
    let (mut phi, binary_exponent) = power10(k).unwrap_or_default();
    if k < 0 || k > 55 { phi.low = phi.low.wrapping_add(1); }
    (phi, exponent + binary_exponent - 1)
}
fn power32(k: i32, exponent: i32) -> (u64, i32) {
    let (mut phi, binary_exponent) = power10(k).unwrap_or_default();
    if k < 0 || k > 27 { phi.high = phi.high.wrapping_add(1); }
    (phi.high, exponent + binary_exponent - 1)
}
fn range64(phi: SourceUint128, beta: i32) -> (u64, u64) {
    let left = shr64(phi.high.wrapping_sub(phi.high >> 54), (64 - 52 - 1 - beta) as u32);
    let right = shr64(phi.high.wrapping_add(phi.high >> 53), (64 - 52 - 1 - beta) as u32);
    (left, right)
}
fn range32(phi: u64, beta: i32) -> (u32, u32) {
    let left = shr64(phi.wrapping_sub(phi >> 25), (64 - 23 - 1 - beta) as u32) as u32;
    let right = shr64(phi.wrapping_add(phi >> 24), (64 - 23 - 1 - beta) as u32) as u32;
    (left, right)
}
fn round_up64(phi: SourceUint128, beta: i32) -> u64 {
    shr64(phi.high, (64 - 52 - 2 - beta) as u32).wrapping_add(1) / 2
}
fn round_up32(phi: u64, beta: i32) -> u32 {
    (shr64(phi, (64 - 23 - 2 - beta) as u32).wrapping_add(1) as u32) / 2
}
fn multiply_power64(value: u64, phi: SourceUint128) -> (u64, bool) {
    let result = multiply192_upper128(value, phi); (result.high, result.low == 0)
}
fn multiply_power32(value: u32, phi: u64) -> (u32, bool) {
    let result = multiply96_upper64(value, phi); ((result >> 32) as u32, result as u32 == 0)
}
fn parity64(mantissa2: u64, phi: SourceUint128, beta: i32) -> (bool, bool) {
    let result = multiply192_lower128(mantissa2, phi);
    let parity = shr64(result.high, (64 - beta) as u32) & 1 != 0;
    let exact = (shl64(result.high, beta as u32) | shr64(result.low, (64 - beta) as u32)) == 0;
    (parity, exact)
}
fn parity32(mantissa2: u32, phi: u64, beta: i32) -> (bool, bool) {
    let result = multiply96_lower64(mantissa2, phi);
    (shr64(result, (64 - beta) as u32) & 1 != 0, shr64(result, (32 - beta) as u32) as u32 == 0)
}
fn delta64(phi: SourceUint128, beta: i32) -> u32 { shr64(phi.high, (64 - 1 - beta) as u32) as u32 }
fn delta32(phi: u64, beta: i32) -> u32 { shr64(phi, (64 - 1 - beta) as u32) as u32 }
