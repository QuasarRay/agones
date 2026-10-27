//! Source scalar wire rules shared by the gateway's typed decoders.
//! Base64 follows Go 1.26.5 encoding/base64.decodeQuantum, without its fast
//! path. Offsets refer to original input bytes, including ignored CR and LF.
// Portions adapted from the Go standard library. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE.
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug)]
pub struct Base64InputError(pub usize);
impl fmt::Display for Base64InputError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { write!(out, "illegal base64 data at input byte {}", self.0) }
}
impl Error for Base64InputError {}

#[derive(Clone, Debug)]
pub struct NumberInputError { pub operation: &'static str, pub input: String, pub reason: &'static str }
impl fmt::Display for NumberInputError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { write!(out, "strconv.{}: parsing {:?}: {}", self.operation, self.input, self.reason) }
}
impl Error for NumberInputError {}
fn number_error(operation: &'static str, input: &str, range: bool) -> NumberInputError {
    NumberInputError { operation, input: input.into(), reason: if range { "value out of range" } else { "invalid syntax" } }
}
pub fn parse_bool(input: &str) -> Result<bool, NumberInputError> {
    match input { "1" | "t" | "T" | "TRUE" | "true" | "True" => Ok(true),
        "0" | "f" | "F" | "FALSE" | "false" | "False" => Ok(false), _ => Err(number_error("ParseBool", input, false)) }
}
pub fn parse_signed(input: &str, base: u32, bits: u32) -> Result<i64, NumberInputError> {
    let (negative, text) = if let Some(text) = input.strip_prefix('-') { (true, text) }
        else { (false, input.strip_prefix('+').unwrap_or(input)) };
    let value = parse_unsigned(text, base, bits).map_err(|error| NumberInputError { operation: "ParseInt", input: input.into(), reason: error.reason })?;
    let cutoff = 1_u64 << (bits - 1);
    if !negative && value >= cutoff || negative && value > cutoff { return Err(number_error("ParseInt", input, true)); }
    Ok(if negative { (value as i64).wrapping_neg() } else { value as i64 })
}
pub fn parse_unsigned(input: &str, base: u32, bits: u32) -> Result<u64, NumberInputError> {
    let invalid = || number_error("ParseUint", input, false);
    if input.is_empty() || bits == 0 || bits > 64 || base != 0 && !(2..=36).contains(&base) { return Err(invalid()); }
    let mut text = input.as_bytes(); let inferred = base == 0; let mut base = base;
    if inferred {
        base = 10;
        if text[0] == b'0' {
            base = 8;
            if text.len() >= 3 {
                match text[1].to_ascii_lowercase() {
                    b'b' => { base = 2; text = &text[2..]; }, b'o' => { text = &text[2..]; },
                    b'x' => { base = 16; text = &text[2..]; }, _ => { text = &text[1..]; },
                }
            } else { text = &text[1..]; }
        }
    }
    let maximum = if bits == 64 { u64::MAX } else { (1_u64 << bits) - 1 };
    let mut value = 0_u64; let mut underscores = false;
    for byte in text {
        if *byte == b'_' && inferred { underscores = true; continue; }
        let digit = match byte.to_ascii_lowercase() { b'0'..=b'9' => u32::from(*byte - b'0'), b'a'..=b'z' => u32::from(byte.to_ascii_lowercase() - b'a' + 10), _ => return Err(invalid()) };
        if digit >= base { return Err(invalid()); }
        value = value.checked_mul(u64::from(base)).and_then(|value| value.checked_add(u64::from(digit)))
            .filter(|value| *value <= maximum).ok_or_else(|| number_error("ParseUint", input, true))?;
    }
    if underscores && !underscores_valid(input) { return Err(invalid()); }
    Ok(value)
}
fn underscores_valid(text: &str) -> bool {
    let mut input = text.as_bytes(); let mut previous = b'^'; let mut position = 0;
    if input.first().is_some_and(|byte| matches!(byte, b'+' | b'-')) { input = &input[1..]; }
    let mut hex = false;
    if input.len() >= 2 && input[0] == b'0' && matches!(input[1].to_ascii_lowercase(), b'b' | b'o' | b'x') {
        position = 2; previous = b'0'; hex = input[1].eq_ignore_ascii_case(&b'x');
    }
    for byte in &input[position..] {
        if byte.is_ascii_digit() || hex && (b'a'..=b'f').contains(&byte.to_ascii_lowercase()) { previous = b'0'; continue; }
        if *byte == b'_' { if previous != b'0' { return false; } previous = b'_'; continue; }
        if previous == b'_' { return false; }
        previous = b'!';
    }
    previous != b'_'
}

pub fn parse_float(text: &str, bits: u32) -> Result<f64, NumberInputError> {
    let invalid = || number_error("ParseFloat", text, false);
    if bits != 32 && bits != 64 { return Err(invalid()); }
    if text.eq_ignore_ascii_case("nan") {
        return Ok(if bits == 32 { f64::from(f32::from_bits(0x7fc0_0000)) } else { f64::from_bits(0x7ff8_0000_0000_0001) });
    }
    let (negative, unsigned) = if let Some(text) = text.strip_prefix('-') { (true, text) }
        else { (false, text.strip_prefix('+').unwrap_or(text)) };
    if unsigned.eq_ignore_ascii_case("inf") || unsigned.eq_ignore_ascii_case("infinity") {
        return Ok(if negative { f64::NEG_INFINITY } else { f64::INFINITY });
    }
    let input = text.as_bytes(); let mut position = usize::from(input.first().is_some_and(|byte| matches!(byte, b'+' | b'-')));
    let hex = input.get(position..position + 2).is_some_and(|prefix| prefix[0] == b'0' && prefix[1].eq_ignore_ascii_case(&b'x'));
    if hex { position += 2; }
    let base = if hex { 16_u64 } else { 10 };
    let maximum_digits = if hex { 16 } else { 19 };
    let mut mantissa = 0_u64; let mut significant = 0_i64; let mut stored = 0_i64;
    let mut decimal_position = 0_i64; let mut dot = false; let mut saw_digit = false; let mut truncated = false;
    while let Some(byte) = input.get(position) {
        if *byte == b'_' { position += 1; continue; }
        if *byte == b'.' { if dot { break; } dot = true; decimal_position = significant; position += 1; continue; }
        let digit = match byte.to_ascii_lowercase() {
            b'0'..=b'9' => u64::from(*byte - b'0'), b'a'..=b'f' if hex => u64::from(byte.to_ascii_lowercase() - b'a' + 10), _ => break,
        };
        saw_digit = true; position += 1;
        if digit == 0 && significant == 0 { decimal_position -= 1; continue; }
        significant += 1;
        if stored < maximum_digits { mantissa = mantissa * base + digit; stored += 1; }
        else if digit != 0 { truncated = true; }
    }
    if !saw_digit { return Err(invalid()); }
    if !dot { decimal_position = significant; }
    if hex { decimal_position *= 4; stored *= 4; }
    let exponent_marker = if hex { b'p' } else { b'e' };
    if input.get(position).is_some_and(|byte| byte.to_ascii_lowercase() == exponent_marker) {
        position += 1; let mut direction = 1;
        if let Some(byte) = input.get(position).filter(|byte| matches!(**byte, b'+' | b'-')) { if *byte == b'-' { direction = -1; } position += 1; }
        if !input.get(position).is_some_and(u8::is_ascii_digit) { return Err(invalid()); }
        let mut exponent = 0_i64;
        while let Some(byte) = input.get(position).filter(|byte| byte.is_ascii_digit() || **byte == b'_') {
            if *byte != b'_' && exponent < 10_000 { exponent = exponent * 10 + i64::from(*byte - b'0'); }
            position += 1;
        }
        decimal_position += exponent * direction;
    } else if hex { return Err(invalid()); }
    if position != input.len() || text.contains('_') && !underscores_valid(text) { return Err(invalid()); }
    if hex {
        return hex_float(text, mantissa, if mantissa == 0 { 0 } else { decimal_position - stored }, negative, truncated, bits);
    }
    let normalized = text.replace('_', "");
    let value = if bits == 32 { f64::from(normalized.parse::<f32>().map_err(|_| invalid())?) }
        else { normalized.parse::<f64>().map_err(|_| invalid())? };
    if value.is_finite() { Ok(value) } else { Err(number_error("ParseFloat", text, true)) }
}
fn hex_float(text: &str, mut mantissa: u64, mut exponent: i64, negative: bool, truncated: bool, bits: u32) -> Result<f64, NumberInputError> {
    // Source atofHex's two guard/sticky bits and round-to-even policy.
    let (mantissa_bits, exponent_bits, bias) = if bits == 32 { (23, 8, -127_i64) } else { (52, 11, -1023) };
    let max_exponent = (1_i64 << exponent_bits) + bias - 2; let min_exponent = bias + 1;
    exponent += mantissa_bits;
    while mantissa != 0 && mantissa >> (mantissa_bits + 2) == 0 { mantissa <<= 1; exponent -= 1; }
    if truncated { mantissa |= 1; }
    while mantissa >> (1 + mantissa_bits + 2) != 0 { mantissa = mantissa >> 1 | mantissa & 1; exponent += 1; }
    while mantissa > 1 && exponent < min_exponent - 2 { mantissa = mantissa >> 1 | mantissa & 1; exponent += 1; }
    let mut round = mantissa & 3; mantissa >>= 2; round |= mantissa & 1; exponent += 2;
    if round == 3 { mantissa += 1; if mantissa == 1 << (1 + mantissa_bits) { mantissa >>= 1; exponent += 1; } }
    if mantissa >> mantissa_bits == 0 { exponent = bias; }
    if exponent > max_exponent { return Err(number_error("ParseFloat", text, true)); }
    let mut encoding = mantissa & ((1_u64 << mantissa_bits) - 1);
    encoding |= ((exponent - bias) as u64 & ((1_u64 << exponent_bits) - 1)) << mantissa_bits;
    if negative { encoding |= 1_u64 << (mantissa_bits + exponent_bits); }
    Ok(if bits == 32 { f64::from(f32::from_bits(encoding as u32)) } else { f64::from_bits(encoding) })
}

pub fn decode_proto_bytes(text: &str) -> Result<Vec<u8>, Base64InputError> {
    // protojson chooses alphabet and padding from the original string before
    // the decoder skips newlines. Do not normalize the input first.
    decode_base64(text.as_bytes(), text.contains(['-', '_']), text.len() % 4 == 0)
}
pub fn decode_json_bytes(text: &str) -> Result<Vec<u8>, Base64InputError> {
    decode_base64(text.as_bytes(), false, true)
}
pub fn decode_base64(input: &[u8], url: bool, padded: bool) -> Result<Vec<u8>, Base64InputError> {
    let mut result = Vec::with_capacity(input.len() / 4 * 3); let mut position = 0;
    while position < input.len() {
        let mut quantum = [0_u8; 4]; let mut size = 0;
        let mut trailing = None;
        while size < 4 {
            if position == input.len() {
                if size == 0 { return Ok(result); }
                if size == 1 || padded { return Err(Base64InputError(position - size)); }
                break;
            }
            let byte = input[position]; position += 1;
            let value = match byte {
                b'A'..=b'Z' => Some(byte - b'A'), b'a'..=b'z' => Some(byte - b'a' + 26),
                b'0'..=b'9' => Some(byte - b'0' + 52), b'+' if !url => Some(62), b'/' if !url => Some(63),
                b'-' if url => Some(62), b'_' if url => Some(63), _ => None,
            };
            if let Some(value) = value { quantum[size] = value; size += 1; continue; }
            if matches!(byte, b'\n' | b'\r') { continue; }
            if byte != b'=' || !padded || size < 2 { return Err(Base64InputError(position - 1)); }
            if size == 2 {
                while input.get(position).is_some_and(|byte| matches!(byte, b'\n' | b'\r')) { position += 1; }
                if position == input.len() { return Err(Base64InputError(input.len())); }
                if input[position] != b'=' { return Err(Base64InputError(position - 1)); }
                position += 1;
            }
            while input.get(position).is_some_and(|byte| matches!(byte, b'\n' | b'\r')) { position += 1; }
            if position < input.len() { trailing = Some(Base64InputError(position)); }
            break;
        }
        // Non-strict Go encodings deliberately accept nonzero unused bits.
        let packed = u32::from(quantum[0]) << 18 | u32::from(quantum[1]) << 12
            | u32::from(quantum[2]) << 6 | u32::from(quantum[3]);
        if size >= 2 { result.push((packed >> 16) as u8); }
        if size >= 3 { result.push((packed >> 8) as u8); }
        if size == 4 { result.push(packed as u8); }
        if let Some(error) = trailing { return Err(error); }
    }
    Ok(result)
}
