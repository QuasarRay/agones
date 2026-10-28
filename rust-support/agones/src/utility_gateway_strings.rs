//! Source integer limits for grpc-gateway pattern string operations.
// Adapted from Go strings.Join, runtime.concatstrings and fmt's string buffer.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::PatternWordProfile;
use bytes::Bytes;
use cgp::prelude::*;
use crate::utility_tls::CanExitSourceProcess;
use std::{error::Error, fmt, io::Write};

/// These are distinct source failure sites. Join panics with a string;
/// fmt's append overflow panics with a runtime error; concatstrings throws
/// without unwinding. This is not a complete Go allocator/runtime profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternStringFailure { JoinOverflow, FormatBufferOverflow, ConcatenationTooLong }
impl PatternStringFailure {
    pub const fn message(self) -> &'static str {
        match self {
            Self::JoinOverflow => "strings: Join output length overflow",
            Self::FormatBufferOverflow => "runtime error: growslice: len out of range",
            Self::ConcatenationTooLong => "string concatenation too long",
        }
    }
}
impl fmt::Display for PatternStringFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(self.message()) }
}
impl Error for PatternStringFailure {}

#[cgp_component(SourcePatternStringCompletion)]
pub trait CanCompleteSourcePatternString {
    fn complete_source_pattern_string(&self, result: Result<Bytes, PatternStringFailure>) -> Bytes;
}
#[cgp_new_provider]
impl<Context: CanExitSourceProcess> SourcePatternStringCompletion<Context> for CompleteSourcePatternString {
    fn complete_source_pattern_string(context: &Context, result: Result<Bytes, PatternStringFailure>) -> Bytes {
        match result {
            Ok(value) => value,
            Err(PatternStringFailure::JoinOverflow) => std::panic::panic_any(PatternStringFailure::JoinOverflow.message()),
            Err(PatternStringFailure::FormatBufferOverflow) => std::panic::panic_any(PatternStringFailure::FormatBufferOverflow),
            Err(PatternStringFailure::ConcatenationTooLong) => {
                // runtime.throw bypasses grpclog (including io.Discard) and
                // is unrecoverable. Full Go stack/crash/secure-mode output
                // and native write ordering remain required runtime work.
                let _ = std::io::stderr().write_all(b"fatal error: string concatenation too long\n");
                context.exit_source_process(2);
                // A returning effect provider must not turn a source throw
                // into a recoverable Rust panic or a successful result.
                std::process::abort();
            }
        }
    }
}

pub(super) fn join_components(profile: PatternWordProfile, components: &[Bytes]) -> Result<Bytes, PatternStringFailure> {
    match components {
        [] => return Ok(Bytes::new()),
        [only] => return Ok(only.clone()),
        _ => {},
    }
    let maximum = profile.max_int() as u128;
    let separators = (components.len() - 1) as u128;
    // strings.Join uses >=, even at the exactly divisible boundary. Do
    // not replace it by a sum-only check or Rust's host integer maximum.
    if 1 >= maximum / separators { return Err(PatternStringFailure::JoinOverflow); }
    let mut length = separators;
    for component in components {
        if component.len() as u128 > maximum - length { return Err(PatternStringFailure::JoinOverflow); }
        length += component.len() as u128;
    }
    let mut joined = Vec::with_capacity(native_length(length));
    joined.extend_from_slice(&components[0]);
    for component in &components[1..] { joined.push(b'/'); joined.extend_from_slice(component); }
    Ok(Bytes::from(joined))
}

pub(super) fn concat_strings(profile: PatternWordProfile, parts: &[Bytes]) -> Result<Bytes, PatternStringFailure> {
    let maximum = profile.max_int() as u128;
    let (mut length, mut nonempty, mut last) = (0_u128, 0_usize, None);
    for part in parts {
        if part.is_empty() { continue; }
        if part.len() as u128 > maximum - length { return Err(PatternStringFailure::ConcatenationTooLong); }
        length += part.len() as u128; nonempty += 1; last = Some(part);
    }
    if nonempty == 0 { return Ok(Bytes::new()); }
    // Bytes values own their immutable storage; none refers to a Go stack
    // temporary. This is concatstrings' sole nonempty heap-string fast path.
    if nonempty == 1 { return Ok(last.expect("one nonempty source string").clone()); }
    let mut result = Vec::with_capacity(native_length(length));
    for part in parts { result.extend_from_slice(part); }
    Ok(Bytes::from(result))
}

/// Only the two fixed string-only Sprintf forms in Pattern.String: no user
/// formatter, width, precision, Unicode conversion or Go format interpreter.
/// Append length overflow stays recoverable, unlike runtime concatenation.
/// Source heap limits, printer-pool capacity, allocation/failure ordering and
/// the final []byte-to-string copy still require their allocation profile.
pub(super) fn format_string_parts(profile: PatternWordProfile, parts: &[&[u8]]) -> Result<Bytes, PatternStringFailure> {
    let maximum = profile.max_int() as u128;
    let mut result = Vec::new();
    for part in parts {
        if part.len() as u128 > maximum - result.len() as u128 { return Err(PatternStringFailure::FormatBufferOverflow); }
        result.extend_from_slice(part);
    }
    Ok(Bytes::from(result))
}

fn native_length(length: u128) -> usize {
    // A Go64 result on a smaller Rust address space is a native hosting
    // limitation, not a source Join/concat overflow or a truncated length.
    usize::try_from(length).expect("source string exceeds native Rust address space")
}
