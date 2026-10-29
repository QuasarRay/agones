//! Shared typed backing for source callback and callback-argument slices.
// Go runtime/slice.go and apimachinery util/runtime, selected pinned sources.
// Copyright The Go Authors, BSD-3-Clause (../GO-LICENSE); Kubernetes Authors,
// Apache-2.0. This is passive slice data, not a callback/provider registry.
use std::sync::{Arc, RwLock};
use crate::{utility_format_copy::SourceFmtValueCopy, utility_error_slice::SourceErrorSliceIndex,
    utility_panic::{SourceBoundsError, SourceBoundsCode, unwind_source_runtime_error},
    utility_http::{HeaderAllocationProfile, SourceSliceAllocationError, reserve_source_array,
        NativeSourceCollectionContext, complete_source_collection}};

struct CallbackBacking<Value> { cells: Box<[RwLock<Value>]> }
pub(crate) struct SourceCallbackSlice<Value> {
    backing: Option<Arc<CallbackBacking<Value>>>, start: usize, length: usize, capacity: usize,
    profile: HeaderAllocationProfile, source_words: u8,
}
impl<Value: Default + SourceFmtValueCopy> SourceCallbackSlice<Value> {
    pub fn nil(profile: HeaderAllocationProfile, source_words: u8) -> Self {
        Self { backing: None, start: 0, length: 0, capacity: 0, profile, source_words }
    }
    pub fn make(length: i64, capacity: i64, profile: HeaderAllocationProfile, source_words: u8) -> Self {
        complete_source_collection(&NativeSourceCollectionContext, Self::try_make(length, capacity, profile, source_words))
    }
    fn try_make(length: i64, capacity: i64, profile: HeaderAllocationProfile, source_words: u8) -> Result<Self, SourceSliceAllocationError> {
        let size = u64::from(profile.word_bits() / 8) * u64::from(source_words);
        let layout = profile.make_slice_layout(length, capacity, size)?;
        let capacity = usize::try_from(layout.capacity).map_err(|_| SourceSliceAllocationError::NativeAddressSpace)?;
        Ok(Self { backing: Some(new_backing(capacity)?), start: 0, length: length as usize, capacity, profile, source_words })
    }
    pub fn is_nil(&self) -> bool { self.backing.is_none() }
    pub fn len(&self) -> usize { self.length }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn get(&self, index: SourceErrorSliceIndex) -> Value {
        let index = self.index(index);
        self.backing.as_ref().expect("nonempty callback slice has backing").cells[self.start + index]
            .read().unwrap_or_else(|poison| poison.into_inner()).source_fmt_copy()
    }
    pub fn set(&self, index: SourceErrorSliceIndex, value: Value) -> Value {
        let index = self.index(index);
        let previous = {
            let mut slot = self.backing.as_ref().expect("nonempty callback slice has backing").cells[self.start + index]
                .write().unwrap_or_else(|poison| poison.into_inner());
            std::mem::replace(&mut *slot, value)
        };
        previous
    }
    fn index(&self, index: SourceErrorSliceIndex) -> usize {
        if index.value as u64 >= self.length as u64 { bounds(index, self.length, SourceBoundsCode::Index); }
        index.value as u64 as usize
    }
    pub fn slice2(&self, low: SourceErrorSliceIndex, high: SourceErrorSliceIndex) -> Self {
        if high.value as u64 > self.capacity as u64 { bounds(high, self.capacity, SourceBoundsCode::SliceCapacity); }
        if low.value as u64 > high.value as u64 { bounds(low, high.value as u64 as usize, SourceBoundsCode::SliceLow); }
        self.window(low.value as u64 as usize, high.value as u64 as usize, self.capacity)
    }
    pub fn slice3(&self, low: SourceErrorSliceIndex, high: SourceErrorSliceIndex, maximum: SourceErrorSliceIndex) -> Self {
        if maximum.value as u64 > self.capacity as u64 { bounds(maximum, self.capacity, SourceBoundsCode::Slice3Capacity); }
        if high.value as u64 > maximum.value as u64 { bounds(high, maximum.value as u64 as usize, SourceBoundsCode::Slice3High); }
        if low.value as u64 > high.value as u64 { bounds(low, high.value as u64 as usize, SourceBoundsCode::Slice3Low); }
        self.window(low.value as u64 as usize, high.value as u64 as usize, maximum.value as u64 as usize)
    }
    fn window(&self, low: usize, high: usize, maximum: usize) -> Self {
        Self { backing: self.backing.as_ref().map(Arc::clone), start: self.start + low, length: high - low,
            capacity: maximum - low, profile: self.profile, source_words: self.source_words }
    }
    pub fn append(&mut self, value: Value) { complete_source_collection(&NativeSourceCollectionContext, self.try_append(value)); }
    pub fn try_append(&mut self, value: Value) -> Result<(), SourceSliceAllocationError> {
        let length = self.length.checked_add(1).ok_or(SourceSliceAllocationError::GrowLength)?;
        if length > self.capacity {
            let size = u64::from(self.profile.word_bits() / 8) * u64::from(self.source_words);
            let layout = self.profile.grow_slice_layout(self.capacity as u64, length as u64, size, false)?;
            let capacity = usize::try_from(layout.capacity).map_err(|_| SourceSliceAllocationError::NativeAddressSpace)?;
            let backing = new_backing(capacity)?;
            for index in 0..self.length {
                *backing.cells[index].write().unwrap_or_else(|poison| poison.into_inner()) =
                    self.get(SourceErrorSliceIndex::unsigned(index as u64));
            }
            self.backing = Some(backing); self.start = 0; self.capacity = capacity;
        }
        let previous = {
            let mut slot = self.backing.as_ref().expect("appended callback slice has backing").cells[self.start + self.length]
                .write().unwrap_or_else(|poison| poison.into_inner());
            std::mem::replace(&mut *slot, value)
        };
        self.length = length; drop(previous); Ok(())
    }
}
impl<Value> SourceFmtValueCopy for SourceCallbackSlice<Value> {
    fn source_fmt_copy(&self) -> Self { Self { backing: self.backing.as_ref().map(Arc::clone),
        start: self.start, length: self.length, capacity: self.capacity, profile: self.profile, source_words: self.source_words } }
}
fn new_backing<Value: Default>(capacity: usize) -> Result<Arc<CallbackBacking<Value>>, SourceSliceAllocationError> {
    let mut cells = reserve_source_array::<RwLock<Value>>(capacity as u64)?;
    cells.resize_with(capacity, || RwLock::new(Value::default()));
    Ok(Arc::new(CallbackBacking { cells: cells.into_boxed_slice() }))
}
fn bounds(index: SourceErrorSliceIndex, limit: usize, code: SourceBoundsCode) -> ! {
    unwind_source_runtime_error(SourceBoundsError { x: index.value, y: limit as i64, signed: index.signed, code })
}
