//! Source bounds data and checked pattern indexes/slices.
// Adapted from Go runtime/error.go and the compiler's slice check ordering.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{PatternSourceProfile, PatternStringFailure};
use crate::utility_http::HeaderValues;
use std::ops::Range;
pub use crate::utility_panic::{SourceBoundsError as PatternBoundsError, SourceBoundsCode as PatternBoundsCode};

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
