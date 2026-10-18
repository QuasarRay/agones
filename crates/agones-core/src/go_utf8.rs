//! Shared non-ASCII UTF-8 prefix decoding for Go string observation boundaries.
//!
//! Reused from the Go-compatible quote implementation. Go BSD notices are in
//! ../GO-STRCONV-LICENSE. UTF-8 width is at most four, so continuation handling
//! is explicit; symbolic checks need not unroll a generic slice iterator.
pub(crate) fn decode_nonascii_scalar(bytes: &[u8]) -> Option<(char, usize)> {
    let first = *bytes.first()?;
    let (width, mask) = match first {
        0xc2..=0xdf => (2, 0x1f),
        0xe0..=0xef => (3, 0x0f),
        0xf0..=0xf4 => (4, 0x07),
        _ => return None,
    };
    if bytes.len() < width {
        return None;
    }
    let second = bytes[1];
    if second & 0xc0 != 0x80
        || (width >= 3 && bytes[2] & 0xc0 != 0x80)
        || (width == 4 && bytes[3] & 0xc0 != 0x80)
        || (first == 0xe0 && second < 0xa0)
        || (first == 0xed && second >= 0xa0)
        || (first == 0xf0 && second < 0x90)
        || (first == 0xf4 && second >= 0x90)
    {
        return None;
    }
    let mut value = (u32::from(first & mask) << 6) | u32::from(second & 0x3f);
    if width >= 3 {
        value = (value << 6) | u32::from(bytes[2] & 0x3f);
    }
    if width == 4 {
        value = (value << 6) | u32::from(bytes[3] & 0x3f);
    }
    char::from_u32(value).map(|rune| (rune, width))
}
