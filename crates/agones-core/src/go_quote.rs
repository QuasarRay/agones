//! Go-compatible byte-string quoting for the feature error boundary.
//!
//! Adapted from Go 1.27.1 strconv/quote.go (Copyright 2009 The Go Authors).
//! BSD terms and notices are retained in ../GO-STRCONV-LICENSE. Printable-rune
//! data is generated from that pinned Go toolchain, independent of Rust's
//! Unicode version. No lossy UTF-8 replacement is performed.

include!("generated/go_print_ranges.rs");

fn go_is_print(value: u32) -> bool {
    if value <= 0xff {
        return (value >= 0x20 && value <= 0x7e)
            || (value >= 0xa1 && value <= 0xff && value != 0xad);
    }
    let mut low = 0;
    let mut high = GO_PRINT_RANGES.len();
    while low < high {
        let middle = low + (high - low) / 2;
        let (start, end) = GO_PRINT_RANGES[middle];
        if value < start {
            high = middle;
        } else if value > end {
            low = middle + 1;
        } else {
            return true;
        }
    }
    false
}

fn decode_scalar(bytes: &[u8]) -> Option<(char, usize)> {
    let first = bytes[0];
    let (width, mask) = match first {
        0xc2..=0xdf => (2, 0x1f),
        0xe0..=0xef => (3, 0x0f),
        0xf0..=0xf4 => (4, 0x07),
        _ => return None,
    };
    if bytes.len() < width {
        return None;
    }
    for byte in &bytes[1..width] {
        if byte & 0xc0 != 0x80 {
            return None;
        }
    }
    let second = bytes[1];
    if (first == 0xe0 && second < 0xa0)
        || (first == 0xed && second >= 0xa0)
        || (first == 0xf0 && second < 0x90)
        || (first == 0xf4 && second >= 0x90)
    {
        return None;
    }
    let mut value = u32::from(first & mask);
    for byte in &bytes[1..width] {
        value = (value << 6) | u32::from(byte & 0x3f);
    }
    char::from_u32(value).map(|value| (value, width))
}

fn append_hex(output: &mut String, marker: char, value: u32, digits: usize) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    output.push('\\');
    output.push(marker);
    for digit in (0..digits).rev() {
        output.push(char::from(HEX[((value >> (digit * 4)) & 15) as usize]));
    }
}

/// Matches Go strconv.Quote, including one \xNN escape per invalid UTF-8 byte.
/// The result is a quoted, valid UTF-8 Go string literal; inputs remain raw bytes.
pub fn quote_feature_error_bytes(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() + 2);
    output.push('"');
    let mut offset = 0;
    while offset < bytes.len() {
        let first = bytes[offset];
        let (rune, width) = if first < 0x80 {
            (char::from(first), 1)
        } else if let Some(decoded) = decode_scalar(&bytes[offset..]) {
            decoded
        } else {
            append_hex(&mut output, 'x', u32::from(first), 2);
            offset += 1;
            continue;
        };
        if rune == '"' || rune == '\\' {
            output.push('\\');
            output.push(rune);
        } else if go_is_print(u32::from(rune)) {
            output.push(rune);
        } else {
            match rune {
                '\x07' => output.push_str("\\a"),
                '\x08' => output.push_str("\\b"),
                '\x0c' => output.push_str("\\f"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                '\x0b' => output.push_str("\\v"),
                value if value < ' ' || value == '\x7f' => {
                    append_hex(&mut output, 'x', u32::from(value), 2)
                }
                value if u32::from(value) < 0x10000 => {
                    append_hex(&mut output, 'u', u32::from(value), 4)
                }
                value => append_hex(&mut output, 'U', u32::from(value), 8),
            }
        }
        offset += width;
    }
    output.push('"');
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printable_table_matches_the_independent_category_bitmap_for_every_codepoint() {
        let expected = include_bytes!("../../../verification/agones/go-print-expected.bin");
        assert_eq!(expected.len(), 0x110000 / 8);
        for value in 0..0x110000_u32 {
            let bit = expected[value as usize / 8] & (1 << (value % 8)) != 0;
            assert_eq!(go_is_print(value), bit, "U+{value:04x}");
        }
        assert!(!go_is_print(u32::MAX));
    }

    #[test]
    fn exact_go_controls_raw_bytes_and_unicode_are_preserved() {
        for (input, expected) in [
            (&b""[..], r#""""#),
            (&b"a\"\\'z"[..], r#""a\"\\'z""#),
            (
                &b"\0\x07\x08\t\n\x0b\x0c\r\x1f\x7f"[..],
                r#""\x00\a\b\t\n\v\f\r\x1f\x7f""#,
            ),
            (
                &b"\xff\xc0\xaf\xed\xa0\x80"[..],
                r#""\xff\xc0\xaf\xed\xa0\x80""#,
            ),
            (&b"\xe2\x82"[..], r#""\xe2\x82""#),
            (&b"\xf4\x90\x80\x80"[..], r#""\xf4\x90\x80\x80""#),
            (
                "\u{85}\u{a0}\u{ad}\u{200b}\u{2028}\u{e000}\u{10ffff}".as_bytes(),
                r#""\u0085\u00a0\u00ad\u200b\u2028\ue000\U0010ffff""#,
            ),
            ("€😀�".as_bytes(), "\"€😀�\""),
        ] {
            assert_eq!(quote_feature_error_bytes(input), expected);
        }
    }

    #[test]
    fn utf8_decoder_preserves_every_scalar_and_rejects_invalid_two_byte_inputs() {
        let mut encoded = [0_u8; 4];
        for value in 0..0x110000_u32 {
            if let Some(rune) = char::from_u32(value) {
                let bytes = rune.encode_utf8(&mut encoded).as_bytes();
                if bytes.len() > 1 {
                    assert_eq!(decode_scalar(bytes), Some((rune, bytes.len())));
                }
            }
        }
        for first in 0x80..=0xff_u8 {
            for second in 0..=0xff_u8 {
                let bytes = [first, second];
                let reference = std::str::from_utf8(&bytes)
                    .ok()
                    .and_then(|text| text.chars().next().map(|rune| (rune, rune.len_utf8())));
                assert_eq!(decode_scalar(&bytes), reference, "{bytes:02x?}");
            }
        }
        for bytes in [
            &b"\xe0\x9f\xbf"[..],
            &b"\xed\xa0\x80"[..],
            &b"\xf0\x8f\xbf\xbf"[..],
            &b"\xf4\x90\x80\x80"[..],
            &b"\xf0\x90\x80"[..],
        ] {
            assert_eq!(decode_scalar(bytes), None);
        }
    }
}
