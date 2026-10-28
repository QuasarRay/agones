//! Source integer limits for grpc-gateway pattern string operations.
// Adapted from Go strings.Join, runtime.concatstrings and fmt's string buffer.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{PatternSourceProfile, PatternBoundsError, PatternMatchResult, PatternConstruction, PatternInputDomainError};
use bytes::Bytes;
use cgp::prelude::*;
use crate::{utility_tls::CanExitSourceProcess, utility_http::{HeaderValues, SourceSliceAllocationError,
    raise_source_allocation_failure, unwind_source_panic}};
use std::{error::Error, fmt, io::Write};

/// These are distinct source failure sites. Join panics with a string;
/// fmt's append overflow panics with a runtime error; concatstrings throws
/// without unwinding. This is not a complete Go allocator/runtime profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternStringFailure {
    JoinOverflow, FormatBufferOverflow, SliceGrowthOverflow, MakeSliceLengthOverflow, MakeSliceCapacityOverflow, ConcatenationTooLong, OutOfMemory,
    /// The selected Go allocation fits its source domain but cannot be
    /// represented by this Rust host. This is not source overflow credit.
    NativeAddressSpace,
}
impl PatternStringFailure {
    pub const fn message(self) -> &'static str {
        match self {
            Self::JoinOverflow => "strings: Join output length overflow",
            Self::FormatBufferOverflow => "runtime error: growslice: len out of range",
            Self::SliceGrowthOverflow => "runtime error: growslice: len out of range",
            Self::MakeSliceLengthOverflow => "runtime error: makeslice: len out of range",
            Self::MakeSliceCapacityOverflow => "runtime error: makeslice: cap out of range",
            Self::ConcatenationTooLong => "string concatenation too long",
            Self::OutOfMemory => "out of memory",
            Self::NativeAddressSpace => "source allocation exceeds native Rust address space",
        }
    }
}
impl fmt::Display for PatternStringFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(self.message()) }
}
impl Error for PatternStringFailure {}
impl From<SourceSliceAllocationError> for PatternStringFailure {
    fn from(error: SourceSliceAllocationError) -> Self {
        match error {
            SourceSliceAllocationError::MakeLength => Self::MakeSliceLengthOverflow,
            SourceSliceAllocationError::MakeCapacity => Self::MakeSliceCapacityOverflow,
            SourceSliceAllocationError::GrowLength => Self::SliceGrowthOverflow,
            SourceSliceAllocationError::OutOfMemory => Self::OutOfMemory,
            SourceSliceAllocationError::NativeAddressSpace => Self::NativeAddressSpace,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternOperationFailure { String(PatternStringFailure), Bounds(PatternBoundsError), Input(PatternInputDomainError) }
impl From<PatternStringFailure> for PatternOperationFailure { fn from(error: PatternStringFailure) -> Self { Self::String(error) } }
impl From<PatternBoundsError> for PatternOperationFailure { fn from(error: PatternBoundsError) -> Self { Self::Bounds(error) } }
impl From<PatternInputDomainError> for PatternOperationFailure { fn from(error: PatternInputDomainError) -> Self { Self::Input(error) } }
impl From<SourceSliceAllocationError> for PatternOperationFailure { fn from(error: SourceSliceAllocationError) -> Self { Self::String(error.into()) } }
impl fmt::Display for PatternOperationFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::String(error) => fmt::Display::fmt(error, out), Self::Bounds(error) => fmt::Display::fmt(error, out), Self::Input(error) => fmt::Display::fmt(error, out) }
    }
}
impl Error for PatternOperationFailure {}

#[cgp_component(SourcePatternStringCompletion)]
pub trait CanCompleteSourcePatternString {
    fn complete_source_pattern_string(&self, result: Result<Bytes, PatternStringFailure>) -> Bytes;
    fn complete_source_pattern_render(&self, result: Result<Bytes, PatternOperationFailure>) -> Bytes;
    fn complete_source_pattern_match(&self, result: Result<PatternMatchResult, PatternOperationFailure>) -> PatternMatchResult;
    fn complete_source_pattern_slice(&self, result: Result<HeaderValues, PatternStringFailure>) -> HeaderValues;
    fn complete_source_pattern_construction(&self, result: Result<PatternConstruction, PatternOperationFailure>) -> PatternConstruction;
    fn complete_source_pattern_input(&self, result: Result<(), PatternInputDomainError>);
}
#[cgp_new_provider]
impl<Context: CanExitSourceProcess> SourcePatternStringCompletion<Context> for CompleteSourcePatternString {
    fn complete_source_pattern_string(context: &Context, result: Result<Bytes, PatternStringFailure>) -> Bytes {
        complete_pattern(context, result.map_err(Into::into))
    }
    fn complete_source_pattern_render(context: &Context, result: Result<Bytes, PatternOperationFailure>) -> Bytes { complete_pattern(context, result) }
    fn complete_source_pattern_match(context: &Context, result: Result<PatternMatchResult, PatternOperationFailure>) -> PatternMatchResult {
        complete_pattern(context, result)
    }
    fn complete_source_pattern_slice(context: &Context, result: Result<HeaderValues, PatternStringFailure>) -> HeaderValues {
        complete_pattern(context, result.map_err(Into::into))
    }
    fn complete_source_pattern_construction(context: &Context, result: Result<PatternConstruction, PatternOperationFailure>) -> PatternConstruction { complete_pattern(context, result) }
    fn complete_source_pattern_input(context: &Context, result: Result<(), PatternInputDomainError>) { complete_pattern(context, result.map_err(Into::into)); }
}
fn complete_pattern<T>(context: &impl CanExitSourceProcess, result: Result<T, PatternOperationFailure>) -> T {
    match result {
        Ok(value) => value,
        Err(PatternOperationFailure::Bounds(error)) => crate::utility_panic::unwind_source_runtime_error(error),
        Err(PatternOperationFailure::Input(error)) => std::panic::panic_any(error),
        Err(PatternOperationFailure::String(error)) => match error {
            PatternStringFailure::JoinOverflow => unwind_source_panic(PatternStringFailure::JoinOverflow.message()),
            PatternStringFailure::FormatBufferOverflow | PatternStringFailure::SliceGrowthOverflow => raise_source_allocation_failure(context, SourceSliceAllocationError::GrowLength),
            PatternStringFailure::MakeSliceLengthOverflow => raise_source_allocation_failure(context, SourceSliceAllocationError::MakeLength),
            PatternStringFailure::MakeSliceCapacityOverflow => raise_source_allocation_failure(context, SourceSliceAllocationError::MakeCapacity),
            PatternStringFailure::NativeAddressSpace => raise_source_allocation_failure(context, SourceSliceAllocationError::NativeAddressSpace),
            PatternStringFailure::OutOfMemory => raise_source_allocation_failure(context, SourceSliceAllocationError::OutOfMemory),
            PatternStringFailure::ConcatenationTooLong => {
                // runtime.throw bypasses grpclog (including io.Discard) and
                // is unrecoverable. Full Go stack/crash/secure-mode output
                // and native write ordering remain required runtime work.
                let _ = std::io::stderr().write_all(b"fatal error: string concatenation too long\n");
                context.exit_source_process(2);
                // A returning effect provider must not turn a source throw
                // into a recoverable Rust panic or a successful result.
                std::process::abort();
            }
        },
    }
}

pub(super) fn join_components(profile: PatternSourceProfile, components: &HeaderValues) -> Result<Bytes, PatternStringFailure> {
    match components.len() {
        0 => return Ok(Bytes::new()),
        1 => return Ok(components.get(0).expect("one source component")),
        _ => {},
    }
    let maximum = profile.word().max_int() as u128;
    let separators = (components.len() - 1) as u128;
    // strings.Join uses >=, even at the exactly divisible boundary. Do
    // not replace it by a sum-only check or Rust's host integer maximum.
    if 1 >= maximum / separators { return Err(PatternStringFailure::JoinOverflow); }
    let mut length = separators;
    for component in components.iter() {
        if component.len() as u128 > maximum - length { return Err(PatternStringFailure::JoinOverflow); }
        length += component.len() as u128;
    }
    // Join constructs a fresh Builder. Grow(n) uses MakeNoZero(n), with
    // a makeslice panic for the requested length, before native allocation.
    let capacity = profile.allocation().make_no_zero_byte_capacity(length)
        .ok_or(PatternStringFailure::MakeSliceLengthOverflow)?;
    let mut joined = allocate_bytes(native_length(capacity as u128)?)?;
    joined.extend_from_slice(&components.get(0).expect("nonempty source components"));
    for component in components.iter().skip(1) { joined.push(b'/'); joined.extend_from_slice(&component); }
    Ok(Bytes::from(joined))
}

pub(super) fn concat_strings(profile: PatternSourceProfile, parts: &[Bytes]) -> Result<Bytes, PatternStringFailure> {
    let maximum = profile.word().max_int() as u128;
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
    let mut result = allocate_string_storage(profile, length)?;
    for part in parts { result.extend_from_slice(part); }
    Ok(Bytes::from(result))
}

/// Only the two fixed string-only Sprintf forms in Pattern.String: no user
/// formatter, width, precision, Unicode conversion or Go format interpreter.
/// Append length overflow stays recoverable, unlike runtime concatenation.
/// Fresh printer buffers use source growslice capacities; the final result
/// copies storage before releasing the scratch buffer. Actual fmt sync.Pool
/// reuse/GC/sanitizer/tiny allocator/native memory-pressure profiles remain.
pub(super) fn format_string_parts(profile: PatternSourceProfile, parts: &[&[u8]]) -> Result<Bytes, PatternStringFailure> {
    let maximum = profile.word().max_int() as u128;
    let mut result = Vec::new();
    let mut source_capacity = 0_u64;
    for part in parts {
        if part.len() as u128 > maximum - result.len() as u128 { return Err(PatternStringFailure::FormatBufferOverflow); }
        let length = result.len() as u128 + part.len() as u128;
        if length > source_capacity as u128 {
            let capacity = profile.allocation().source_grow_byte_capacity(source_capacity, length as u64)
                .ok_or(PatternStringFailure::FormatBufferOverflow)?;
            // A source grow allocates new backing even if Rust's allocator
            // happened to overallocate the prior block.
            let mut grown = allocate_bytes(native_length(capacity as u128)?)?;
            grown.extend_from_slice(&result); result = grown; source_capacity = capacity;
        }
        result.extend_from_slice(part);
    }
    match result.as_slice() {
        [] => Ok(Bytes::new()),
        [only] => {
            let index = *only as usize;
            Ok(Bytes::from_static(&SINGLE_BYTE_STRINGS[index..index + 1]))
        }
        _ => {
            let mut copied = allocate_string_storage(profile, result.len() as u128)?;
            copied.extend_from_slice(&result);
            Ok(Bytes::from(copied))
        }
    }
}

fn native_length(length: u128) -> Result<usize, PatternStringFailure> {
    // A Go64 result on a smaller Rust address space is a native hosting
    // limitation, not a source Join/concat overflow or a truncated length.
    if length > isize::MAX as u128 { return Err(PatternStringFailure::NativeAddressSpace); }
    Ok(length as usize)
}

fn allocate_bytes(capacity: usize) -> Result<Vec<u8>, PatternStringFailure> {
    let capacity = native_length(capacity as u128)?;
    let mut result = Vec::new();
    result.try_reserve_exact(capacity).map_err(|_| PatternStringFailure::OutOfMemory)?;
    Ok(result)
}

fn allocate_string_storage(profile: PatternSourceProfile, length: u128) -> Result<Vec<u8>, PatternStringFailure> {
    // rawstring/mallocgc does not raise the Builder's makeslice panic.
    // A request larger than the selected source heap can only fail to
    // allocate. Native pressure can also fail a smaller request.
    if length > profile.allocation().max_alloc() as u128 { return Err(PatternStringFailure::OutOfMemory); }
    allocate_bytes(native_length(length)?)
}

static SINGLE_BYTE_STRINGS: [u8; 256] = {
    let mut bytes = [0; 256];
    let mut index = 0;
    while index < bytes.len() { bytes[index] = index as u8; index += 1; }
    bytes
};
