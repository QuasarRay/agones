//! The recorded JDK 21 default codec is UTF-8. Scala Codec uses REPORT;
//! sbt IO.read uses String's UTF-8 replacement decoding instead.
use crate::{Error, Result};

pub fn decode_scala_source(bytes: &[u8]) -> Result<String> {
    match std::str::from_utf8(bytes) {
        Ok(text) => Ok(text.to_owned()),
        Err(error) => Err(Error::malformed_input(java_malformed_length(&bytes[error.valid_up_to()..]))),
    }
}

fn java_malformed_length(bytes: &[u8]) -> usize {
    let first = bytes[0];
    let width = match first { 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => 1 };
    let mut consumed = 1;
    while consumed < width && consumed < bytes.len() {
        let next = bytes[consumed];
        if next & 0xc0 != 0x80 || consumed == 1 && (
            first == 0xe0 && next < 0xa0 || first == 0xf0 && next < 0x90 || first == 0xf4 && next > 0x8f
        ) { break; }
        consumed += 1;
    }
    consumed
}

pub fn decode_sbt_text(bytes: &[u8]) -> String {
    cpg_rs::utilities::decode_utf8_replacing(bytes)
}

/// Decode one input-stream buffer, retaining at most an unfinished encoding.
/// A complete surrogate encoding is malformed as one Java group of length 3.
pub fn decode_scala_buffer(pending: &mut Vec<u8>, bytes: &[u8], end: bool) -> Result<String> {
    pending.try_reserve(bytes.len()).map_err(|_| Error::out_of_memory())?;
    pending.extend_from_slice(bytes);
    match std::str::from_utf8(pending) {
        Ok(text) => { let text = text.to_owned(); pending.clear(); Ok(text) }
        Err(error) => {
            let valid = error.valid_up_to();
            let tail = &pending[valid..];
            let width = match tail[0] { 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => 1 };
            let consumed = java_malformed_length(tail);
            if !end && tail.len() < width && consumed == tail.len() {
                let prefix = std::str::from_utf8(&pending[..valid])
                    .expect("decoder identified a valid prefix").to_owned();
                pending.drain(..valid);
                Ok(prefix)
            } else { Err(Error::malformed_input(consumed)) }
        }
    }
}
