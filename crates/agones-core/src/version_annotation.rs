//! Go encoding/json's observable string value for a raw Go byte string.
//!
//! Each invalid UTF-8 byte becomes one U+FFFD. Rust's from_utf8_lossy can group
//! truncated prefixes, so preserve Go's one-byte invalid-rune consumption.
pub(crate) fn version_annotation_text(input: &[u8]) -> String {
    let mut output = String::with_capacity(input.len());
    let mut offset = 0;
    while offset < input.len() {
        if input[offset] < 0x80 {
            output.push(char::from(input[offset]));
            offset += 1;
        } else if let Some((rune, width)) = crate::go_utf8::decode_nonascii_scalar(&input[offset..])
        {
            output.push(rune);
            offset += width;
        } else {
            output.push('\u{fffd}');
            offset += 1;
        }
    }
    output
}
