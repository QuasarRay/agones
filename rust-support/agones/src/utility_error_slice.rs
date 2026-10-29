//! Retained error interfaces and mutable source Unwrap slice backing.
// Adapted from Go fmt/errors.go, errors/wrap.go and runtime slice contracts.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
// These typed error cells are the canonical backing, not a second error tree.
use std::{error::Error, fmt, sync::{Arc, RwLock}};
use derive_deftly::Deftly;
use crate::{
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces,
    errors::BoxError, utility_format_copy::SourceFmtValueCopy,
    utility_panic::{SourceBoundsError, SourceBoundsCode, unwind_source_runtime_error},
    utility_http::{HeaderAllocationProfile, SourceSliceAllocationError, reserve_source_array,
        NativeSourceCollectionContext, complete_source_collection},
};

/// Interface copies retain the original Box, including opaque native errors.
/// Formatting/type binding is still supplied by the original error's adapter.
pub struct SourceRetainedError { original: Arc<BoxError> }
impl SourceRetainedError {
    pub fn from_original(error: BoxError) -> Self {
        match error.downcast::<Self>() { Ok(error) => *error,
            Err(error) => Self { original: Arc::new(error) } }
    }
    pub fn original(&self) -> &(dyn Error + Send + Sync + 'static) { self.original.as_ref().as_ref() }
    pub fn into_error(self) -> BoxError { Box::new(self) }
    /// Native convenience identity for retained-field descriptors. Source
    /// error equality uses its declared type/comparability operation instead.
    pub(crate) fn native_same_owner(&self, other: &Self) -> bool { Arc::ptr_eq(&self.original, &other.original) }
}
impl SourceFmtValueCopy for SourceRetainedError {
    fn source_fmt_copy(&self) -> Self { Self { original: Arc::clone(&self.original) } }
}
impl fmt::Debug for SourceRetainedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self.original(), out) }
}
impl fmt::Display for SourceRetainedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self.original(), out) }
}
impl Error for SourceRetainedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { self.original().source() }
}

/// Bounds expressions keep the original signedness and all 64 value bits.
#[derive(Clone, Copy)]
pub struct SourceErrorSliceIndex { pub value: i64, pub signed: bool }
impl SourceErrorSliceIndex {
    pub fn signed(value: i64) -> Self { Self { value, signed: true } }
    pub fn unsigned(value: u64) -> Self { Self { value: value as i64, signed: false } }
    fn unsigned_bits(self) -> u64 { self.value as u64 }
}
struct ErrorSliceBacking { cells: Box<[RwLock<Option<SourceRetainedError>>]> }

/// A copied header keeps its own len/cap/window and shares mutable elements.
/// None is a nil slice; a non-nil empty slice retains a distinct backing owner.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourceErrorSlice {
    backing: Option<Arc<ErrorSliceBacking>>,
    start: usize,
    length: usize,
    capacity: usize,
    profile: HeaderAllocationProfile,
}
impl SourceErrorSlice {
    pub fn nil() -> Self {
        Self::nil_with_profile(if usize::BITS == 32 { HeaderAllocationProfile::GO126_32 }
            else { HeaderAllocationProfile::GO126_64 })
    }
    pub fn nil_with_profile(profile: HeaderAllocationProfile) -> Self {
        Self { backing: None, start: 0, length: 0, capacity: 0, profile }
    }
    pub fn try_make(length: i64, capacity: i64, profile: HeaderAllocationProfile)
        -> Result<Self, SourceSliceAllocationError>
    {
        let layout = profile.make_slice_layout(length, capacity, u64::from(profile.word_bits() / 4))?;
        let capacity = usize::try_from(layout.capacity).map_err(|_| SourceSliceAllocationError::NativeAddressSpace)?;
        Ok(Self { backing: Some(new_backing(capacity)?), start: 0, length: length as usize, capacity, profile })
    }
    pub fn is_nil(&self) -> bool { self.backing.is_none() }
    pub fn len(&self) -> usize { self.length }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn native_address(&self) -> u64 {
        self.backing.as_ref().map_or(0, |backing| backing.cells.as_ptr().wrapping_add(self.start) as usize as u64)
    }
    pub fn get(&self, index: usize) -> Option<SourceRetainedError> {
        self.get_source(SourceErrorSliceIndex::unsigned(index as u64))
    }
    pub fn get_source(&self, index: SourceErrorSliceIndex) -> Option<SourceRetainedError> {
        let index = self.checked_index(index);
        let value = self.backing.as_ref().expect("nonempty error slice has backing").cells[self.start + index]
            .read().unwrap_or_else(|poison| poison.into_inner());
        value.as_ref().map(SourceFmtValueCopy::source_fmt_copy)
    }
    pub fn set(&self, index: usize, value: Option<SourceRetainedError>) -> Option<SourceRetainedError> {
        self.set_source(SourceErrorSliceIndex::unsigned(index as u64), value)
    }
    pub fn set_source(&self, index: SourceErrorSliceIndex, value: Option<SourceRetainedError>) -> Option<SourceRetainedError> {
        let index = self.checked_index(index);
        // Release the cell lock before the caller drops an old native error.
        let previous = {
            let mut cell = self.backing.as_ref().expect("nonempty error slice has backing").cells[self.start + index]
                .write().unwrap_or_else(|poison| poison.into_inner());
            std::mem::replace(&mut *cell, value)
        };
        previous
    }
    fn checked_index(&self, index: SourceErrorSliceIndex) -> usize {
        if index.unsigned_bits() >= self.length as u64 { bounds_failure(index, self.length, SourceBoundsCode::Index); }
        index.unsigned_bits() as usize
    }
    pub fn slice2(&self, low: SourceErrorSliceIndex, high: SourceErrorSliceIndex) -> Self {
        if high.unsigned_bits() > self.capacity as u64 { bounds_failure(high, self.capacity, SourceBoundsCode::SliceCapacity); }
        if low.unsigned_bits() > high.unsigned_bits() { bounds_failure(low, high.unsigned_bits() as usize, SourceBoundsCode::SliceLow); }
        self.window(low.unsigned_bits() as usize, high.unsigned_bits() as usize, self.capacity)
    }
    pub fn slice3(&self, low: SourceErrorSliceIndex, high: SourceErrorSliceIndex, maximum: SourceErrorSliceIndex) -> Self {
        if maximum.unsigned_bits() > self.capacity as u64 { bounds_failure(maximum, self.capacity, SourceBoundsCode::Slice3Capacity); }
        if high.unsigned_bits() > maximum.unsigned_bits() { bounds_failure(high, maximum.unsigned_bits() as usize, SourceBoundsCode::Slice3High); }
        if low.unsigned_bits() > high.unsigned_bits() { bounds_failure(low, high.unsigned_bits() as usize, SourceBoundsCode::Slice3Low); }
        self.window(low.unsigned_bits() as usize, high.unsigned_bits() as usize, maximum.unsigned_bits() as usize)
    }
    fn window(&self, low: usize, high: usize, maximum: usize) -> Self {
        Self { backing: self.backing.as_ref().map(Arc::clone), start: self.start + low,
            length: high - low, capacity: maximum - low, profile: self.profile }
    }
    pub fn try_append(&mut self, value: Option<SourceRetainedError>) -> Result<(), SourceSliceAllocationError> {
        let length = self.length.checked_add(1).ok_or(SourceSliceAllocationError::GrowLength)?;
        if length > self.capacity {
            let layout = self.profile.grow_slice_layout(self.capacity as u64, length as u64,
                u64::from(self.profile.word_bits() / 4), false)?;
            let capacity = usize::try_from(layout.capacity).map_err(|_| SourceSliceAllocationError::NativeAddressSpace)?;
            let backing = new_backing(capacity)?;
            for index in 0..self.length {
                let copied = self.get(index);
                *backing.cells[index].write().unwrap_or_else(|poison| poison.into_inner()) = copied;
            }
            self.backing = Some(backing); self.start = 0; self.capacity = capacity;
        }
        let previous = {
            let mut cell = self.backing.as_ref().expect("append has non-nil error backing").cells[self.start + self.length]
                .write().unwrap_or_else(|poison| poison.into_inner());
            std::mem::replace(&mut *cell, value)
        };
        self.length = length; drop(previous); Ok(())
    }
    pub fn append(&mut self, value: Option<SourceRetainedError>) {
        complete_source_collection(&NativeSourceCollectionContext, self.try_append(value));
    }
}
impl SourceFmtValueCopy for SourceErrorSlice {
    fn source_fmt_copy(&self) -> Self { Self { backing: self.backing.as_ref().map(Arc::clone),
        start: self.start, length: self.length, capacity: self.capacity, profile: self.profile } }
}
impl fmt::Debug for SourceErrorSlice {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = out.debug_list();
        for index in 0..self.length { list.entry(&self.get(index)); }
        list.finish()
    }
}
impl crate::utility_format_arguments::SourcePrintfValue for SourceErrorSlice {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { "[]error" }
    fn write_source(&self, verb: char, state: &crate::utility_format_scan::SourcePrintfResolvedCounts,
        profile: crate::utility_http::SourceUnicodeProfile, mode: crate::utility_format_print::SourcePrintfMode,
        recursion: crate::utility_format_methods::SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if recursion.depth == 0 && verb == 'p' {
            return crate::utility_format_pointer::write_source_address(self, self.native_address(), verb, state, profile, mode, recursion, output);
        }
        if state.flags.sharp_v {
            crate::utility_go_quote::append(output, b"[]error")?;
            if self.is_nil() { return crate::utility_go_quote::append(output, b"(nil)"); }
        }
        crate::utility_go_quote::append(output, if state.flags.sharp_v { b"{" } else { b"[" })?;
        for index in 0..self.length {
            if index != 0 { crate::utility_go_quote::append(output, if state.flags.sharp_v { b", " } else { b" " })?; }
            match self.get(index) {
                Some(error) => crate::utility_format_arguments::write_source_error_value(
                    error.original(), verb, state, profile, mode, recursion.child(true), output)?,
                None => crate::utility_go_quote::append(output, if state.flags.sharp_v { b"error(nil)" } else { b"<nil>" })?,
            }
        }
        crate::utility_go_quote::append(output, if state.flags.sharp_v { b"}" } else { b"]" })
    }
}
fn new_backing(capacity: usize) -> Result<Arc<ErrorSliceBacking>, SourceSliceAllocationError> {
    let mut cells = reserve_source_array::<RwLock<Option<SourceRetainedError>>>(capacity as u64)?;
    cells.resize_with(capacity, || RwLock::new(None));
    Ok(Arc::new(ErrorSliceBacking { cells: cells.into_boxed_slice() }))
}
fn bounds_failure(index: SourceErrorSliceIndex, length: usize, code: SourceBoundsCode) -> ! {
    unwind_source_runtime_error(SourceBoundsError { x: index.value, y: length as i64, signed: index.signed, code })
}
