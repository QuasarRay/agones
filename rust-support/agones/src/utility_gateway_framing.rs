//! Go encoding/json.Decoder's first-value boundary, outside ProtoJSON.
//! A scalar requires one look-ahead byte or EOF; an object/array can finish at
//! its closing delimiter. The look-ahead is retained for the following Decode.
// Portions adapted from the Go standard library. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE.
use std::{error::Error, fmt, io, sync::Arc};
use serde_json::value::RawValue;
use crate::errors::BoxError;

#[derive(Clone, Debug)]
pub(crate) struct ReadFailure(Arc<dyn Error + Send + Sync>);
impl ReadFailure {
    pub fn new(error: BoxError) -> Self { Self(Arc::from(error)) }
}
impl fmt::Display for ReadFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self.0.as_ref(), out) }
}
impl Error for ReadFailure { fn source(&self) -> Option<&(dyn Error + 'static)> { Some(self.0.as_ref()) } }

pub(crate) fn first_value(bytes: &[u8], eof: bool) -> Result<Option<(Box<RawValue>, usize)>, BoxError> {
    let space = |byte: &u8| matches!(*byte, b' ' | b'\t' | b'\r' | b'\n');
    let start = bytes.iter().position(|byte| !space(byte));
    let Some(start) = start else { return Ok(None); };
    let first = bytes[start];
    let end = match first {
        b'{' | b'[' => container_end(bytes, start)?, b'"' => string_end(bytes, start),
        b't' => keyword_end(bytes, start, b"true"), b'f' => keyword_end(bytes, start, b"false"),
        b'n' => keyword_end(bytes, start, b"null"), b'-' | b'0'..=b'9' => number_end(bytes, start), _ => None,
    };
    if let Some(end) = end {
        // RawValue validates this value without imposing a separator rule on
        // the following value. Decoder accepts e.g. 01 as 0 followed by 1.
        let raw = serde_json::from_slice::<Box<RawValue>>(&bytes[start..end])?;
        if matches!(first, b'{' | b'[') || end < bytes.len() || eof { return Ok(Some((raw, end))); }
        return Ok(None);
    }
    match serde_json::from_slice::<Box<RawValue>>(&bytes[start..]) {
        Err(error) if !error.is_eof() => Err(Box::new(error)),
        _ if eof => Err(Box::new(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"))),
        _ => Ok(None),
    }
}
fn keyword_end(bytes: &[u8], start: usize, word: &[u8]) -> Option<usize> {
    (bytes.get(start..start + word.len()) == Some(word)).then_some(start + word.len())
}
fn string_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut escaped = false;
    for (position, byte) in bytes.iter().enumerate().skip(start + 1) {
        if escaped { escaped = false; }
        else if *byte == b'\\' { escaped = true; }
        else if *byte == b'"' { return Some(position + 1); }
    }
    None
}
fn container_end(bytes: &[u8], start: usize) -> Result<Option<usize>, BoxError> {
    let mut stack = Vec::new(); let mut quoted = false; let mut escaped = false;
    for (position, byte) in bytes.iter().enumerate().skip(start) {
        if quoted {
            if escaped { escaped = false; }
            else if *byte == b'\\' { escaped = true; }
            else if *byte == b'"' { quoted = false; }
            continue;
        }
        match byte {
            b'"' => quoted = true,
            b'{' | b'[' => {
                if stack.len() == 10_000 { return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData,
                    format!("invalid character {:?} exceeded max depth", char::from(*byte))))); }
                stack.push(if *byte == b'{' { b'}' } else { b']' });
            }
            b'}' | b']' => { if stack.pop() != Some(*byte) { return Ok(None); } if stack.is_empty() { return Ok(Some(position + 1)); } },
            _ => {},
        }
    }
    Ok(None)
}
fn number_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut position = start;
    if bytes.get(position) == Some(&b'-') { position += 1; }
    match bytes.get(position)? {
        b'0' => position += 1,
        b'1'..=b'9' => { position += 1; while bytes.get(position).is_some_and(u8::is_ascii_digit) { position += 1; } },
        _ => return None,
    }
    if bytes.get(position) == Some(&b'.') {
        position += 1; let first = position;
        while bytes.get(position).is_some_and(u8::is_ascii_digit) { position += 1; }
        if position == first { return None; }
    }
    if bytes.get(position).is_some_and(|byte| matches!(*byte, b'e' | b'E')) {
        position += 1;
        if bytes.get(position).is_some_and(|byte| matches!(*byte, b'+' | b'-')) { position += 1; }
        let first = position;
        while bytes.get(position).is_some_and(u8::is_ascii_digit) { position += 1; }
        if position == first { return None; }
    }
    Some(position)
}
