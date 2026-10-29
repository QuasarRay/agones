//! Source integer arithmetic and decimal digit storage for float conversion.
// Adapted from Go internal/strconv/math.go, itoa.go and ftoadbox.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use crate::utility_float_powers::{POWER10, POWER10_MIN, POWER10_MAX};

#[derive(Clone, Copy, Default)]
pub(crate) struct SourceUint128 { pub high: u64, pub low: u64 }

/// A borrowed projection of the owner's digit scratch; no numeric state copy.
pub(crate) struct SourceFloatDigits<'a> {
    pub digits: &'a [u8],
    pub count: i32,
    pub point: i32,
}

// Go shifts at or beyond the width produce zero. Internal callers provide
// nonnegative counts from the binary32/binary64 exponent domain.
pub(crate) fn shr64(value: u64, shift: u32) -> u64 { value.checked_shr(shift).unwrap_or(0) }
pub(crate) fn shl64(value: u64, shift: u32) -> u64 { value.checked_shl(shift).unwrap_or(0) }
pub(crate) fn shl32(value: u32, shift: u32) -> u32 { value.checked_shl(shift).unwrap_or(0) }

pub(crate) fn multiply128(x: u64, y: u64) -> SourceUint128 {
    let product = u128::from(x) * u128::from(y);
    SourceUint128 { high: (product >> 64) as u64, low: product as u64 }
}
pub(crate) fn multiply192(x: u64, y: SourceUint128) -> (u64, u64, u64) {
    let low = multiply128(x, y.low); let high = multiply128(x, y.high);
    let (middle, carry) = low.high.overflowing_add(high.low);
    (high.high.wrapping_add(u64::from(carry)), middle, low.low)
}
fn add128(mut value: SourceUint128, addend: u64) -> SourceUint128 {
    let sum = value.low.wrapping_add(addend);
    if sum < value.low { value.high = value.high.wrapping_add(1); }
    value.low = sum; value
}
pub(crate) fn multiply96_upper64(x: u32, y: u64) -> u64 {
    let high = u64::from(x) * u64::from((y >> 32) as u32);
    let low = u64::from(x) * u64::from(y as u32);
    high.wrapping_add(low >> 32)
}
pub(crate) fn multiply96_lower64(x: u32, y: u64) -> u64 { u64::from(x).wrapping_mul(y) }
fn multiply128_upper64(x: u64, y: u64) -> u64 {
    let (a, b, c, d) = (x >> 32, u64::from(x as u32), y >> 32, u64::from(y as u32));
    let (ac, bc, ad, bd) = (a * c, b * c, a * d, b * d);
    let middle = (bd >> 32).wrapping_add(u64::from(ad as u32)).wrapping_add(u64::from(bc as u32));
    ac.wrapping_add(middle >> 32).wrapping_add(ad >> 32).wrapping_add(bc >> 32)
}
pub(crate) fn multiply192_upper128(x: u64, y: SourceUint128) -> SourceUint128 {
    add128(multiply128(x, y.high), multiply128_upper64(x, y.low))
}
pub(crate) fn multiply192_lower128(x: u64, y: SourceUint128) -> SourceUint128 {
    let lower = multiply128(x, y.low);
    SourceUint128 { high: x.wrapping_mul(y.high).wrapping_add(lower.high), low: lower.low }
}

pub(crate) fn power10(exponent: i32) -> Option<(SourceUint128, i32)> {
    if exponent < POWER10_MIN || exponent > POWER10_MAX { return None; }
    Some((POWER10[(exponent - POWER10_MIN) as usize], 1 + ((exponent * 108853) >> 15)))
}
pub(crate) fn mul_log10_2(value: i32) -> i32 { (value * 78913) >> 18 }
pub(crate) fn mul_log10_2_minus_log10_4_over_3(value: i32) -> i32 { (value * 631305 - 261663) >> 21 }

pub(crate) fn divisible_power5(value: u64, exponent: i32) -> bool {
    if !(1..=22).contains(&exponent) { return false; }
    let (inverse, maximum) = DIVIDE5[(exponent - 1) as usize];
    value.wrapping_mul(inverse) <= maximum
}
const DIVIDE5: [(u64, u64); 22] = [
    (0xcccccccccccccccd, u64::MAX / 5),
    (0x8f5c28f5c28f5c29, u64::MAX / 5 / 5),
    (0x1cac083126e978d5, u64::MAX / 5 / 5 / 5),
    (0xd288ce703afb7e91, u64::MAX / 5 / 5 / 5 / 5),
    (0x5d4e8fb00bcbe61d, u64::MAX / 5 / 5 / 5 / 5 / 5),
    (0x790fb65668c26139, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5),
    (0xe5032477ae8d46a5, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0xc767074b22e90e21, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x8e47ce423a2e9c6d, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x4fa7f60d3ed61f49, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x0fee64690c913975, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x3662e0e1cf503eb1, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0xa47a2cf9f6433fbd, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x54186f653140a659, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x7738164770402145, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0xe4a4d1417cd9a041, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0xc75429d9e5c5200d, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0xc1773b91fac10669, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x26b172506559ce15, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0xd489e3a9addec2d1, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x90e860bb892c8d5d, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
    (0x502e79bf1b6f4f79, u64::MAX / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5),
];

pub(crate) fn trim_zeros(mut value: u64) -> (u64, i32) {
    // Source precondition: nonzero mantissa, already handled by genericFtoa.
    let mut zeros = 0;
    loop {
        let divided = value.wrapping_mul(0xc767074b22e90e21).rotate_right(8);
        if divided > u64::MAX / 100000000 { break; }
        value = divided; zeros += 8;
    }
    for (inverse, maximum, shift) in [(0xd288ce703afb7e91, u64::MAX / 10000, 4),
        (0x8f5c28f5c28f5c29, u64::MAX / 100, 2), (0xcccccccccccccccd, u64::MAX / 10, 1)]
    {
        let divided = value.wrapping_mul(inverse).rotate_right(shift);
        if divided <= maximum { value = divided; zeros += shift as i32; }
    }
    (value, zeros)
}

const SMALLS: &[u8] = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";
pub(crate) fn format_base10(buffer: &mut [u8], mut value: u64) -> usize {
    let mut index = buffer.len();
    // Both source host-word predicates have the same value. This native data
    // path preserves uint32 chunks; source/native target codegen stays open.
    while value >> 29 != 0 {
        let mut low = (value % 1000000000) as u32; value /= 1000000000;
        for _ in 0..4 {
            let pair = (low % 100 * 2) as usize; low /= 100; index -= 2;
            buffer[index..index + 2].copy_from_slice(&SMALLS[pair..pair + 2]);
        }
        index -= 1; buffer[index] = SMALLS[(low * 2 + 1) as usize];
        if value == 0 { return index; }
    }
    let mut low = value as u32;
    while low >= 100 {
        let pair = (low % 100 * 2) as usize; low /= 100; index -= 2;
        buffer[index..index + 2].copy_from_slice(&SMALLS[pair..pair + 2]);
    }
    index -= 1; let pair = (low * 2) as usize; buffer[index] = SMALLS[pair + 1];
    if low >= 10 { index -= 1; buffer[index] = SMALLS[pair]; }
    index
}
