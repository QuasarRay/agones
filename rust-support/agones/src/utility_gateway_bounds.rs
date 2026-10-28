//! Source bounds data and checked pattern indexes/slices.
// Adapted from Go runtime/error.go and the compiler's slice check ordering.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{PatternSourceProfile, PatternStringFailure};
use crate::utility_http::HeaderValues;
use derive_deftly::Deftly;
use crate::utility_declarations::derive_deftly_template_SourceErrorOrigin;
use std::{error::Error, fmt, ops::Range};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternBoundsCode { Index, SliceLength, SliceCapacity, SliceLow, Slice3Length,
    Slice3Capacity, Slice3High, Slice3Low, SliceConvert }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "runtime", source_struct = "boundsError")]
pub struct PatternBoundsError { pub x: i64, pub y: i64, pub signed: bool, pub code: PatternBoundsCode }
impl PatternBoundsError {
    fn format(self) -> &'static str {
        use PatternBoundsCode::*;
        match (self.code, self.signed && self.x < 0) {
            (Index, false) => "index out of range [%x] with length %y",
            (Index, true) => "index out of range [%x]",
            (SliceLength, false) => "slice bounds out of range [:%x] with length %y",
            (SliceLength, true) => "slice bounds out of range [:%x]",
            (SliceCapacity, false) => "slice bounds out of range [:%x] with capacity %y",
            (SliceCapacity, true) => "slice bounds out of range [:%x]",
            (SliceLow, false) => "slice bounds out of range [%x:%y]",
            (SliceLow, true) => "slice bounds out of range [%x:]",
            (Slice3Length, false) => "slice bounds out of range [::%x] with length %y",
            (Slice3Length, true) => "slice bounds out of range [::%x]",
            (Slice3Capacity, false) => "slice bounds out of range [::%x] with capacity %y",
            (Slice3Capacity, true) => "slice bounds out of range [::%x]",
            (Slice3High, false) => "slice bounds out of range [:%x:%y]",
            (Slice3High, true) => "slice bounds out of range [:%x:]",
            (Slice3Low, false) => "slice bounds out of range [%x:%y:]",
            (Slice3Low, true) => "slice bounds out of range [%x::]",
            (SliceConvert, _) => "cannot convert slice with length %y to array or pointer to array with length %x",
        }
    }
}
impl fmt::Display for PatternBoundsError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str("runtime error: ")?;
        let mut bytes = self.format().bytes();
        while let Some(byte) = bytes.next() {
            if byte != b'%' { out.write_str(std::str::from_utf8(&[byte]).expect("ASCII bounds format"))?; continue; }
            match bytes.next() {
                Some(b'x') if self.signed => write!(out, "{}", self.x)?,
                Some(b'x') => write!(out, "{}", self.x as u64)?,
                Some(b'y') => write!(out, "{}", self.y)?,
                _ => unreachable!("source bounds format placeholder"),
            }
        }
        Ok(())
    }
}
impl Error for PatternBoundsError {}

pub(super) fn index(index: i64, length: usize) -> Result<usize, PatternBoundsError> {
    if index < 0 || index as u128 >= length as u128 {
        return Err(PatternBoundsError { x: index, y: length as i64, signed: true, code: PatternBoundsCode::Index });
    }
    Ok(index as usize)
}
/// A two-index slice checks its upper bound against capacity, then its
/// lower bound against that already nonnegative upper bound. An omitted
/// upper bound is supplied by the caller as the source slice length.
pub(super) fn slice(low: i64, high: i64, capacity: usize) -> Result<Range<usize>, PatternBoundsError> {
    if high < 0 || high as u128 > capacity as u128 {
        return Err(PatternBoundsError { x: high, y: capacity as i64, signed: true, code: PatternBoundsCode::SliceCapacity });
    }
    if low < 0 || low > high {
        return Err(PatternBoundsError { x: low, y: high, signed: true, code: PatternBoundsCode::SliceLow });
    }
    Ok(low as usize..high as usize)
}

pub(super) fn make_strings(profile: PatternSourceProfile, length: i64, capacity: i64) -> Result<HeaderValues, PatternStringFailure> {
    HeaderValues::try_make(length, capacity, profile.allocation()).map_err(Into::into)
}
