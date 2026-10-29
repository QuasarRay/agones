//! Capture native map values at the source fmtsort call point.
// Adapted from Go internal/fmtsort/sort.go, reflect/{map,value}.go and
// fmt/print.go. Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
// Native map/hash/range/concurrency/GC/ABI profiles remain explicit gaps.
use std::{cell::RefCell, collections::{HashMap, BTreeMap}};
use derive_deftly::Deftly;
use crate::{
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces,
    utility_format_arguments::SourcePrintfValue,
    utility_format_copy::SourceFmtValueCopy,
    utility_format_map::{SourceFmtMapKey, begin_source_map, write_source_map_pairs},
    utility_format_methods::SourcePrintfRecursion,
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_print::SourcePrintfMode,
    utility_format_sort::source_stable_sort,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError, reserve_source_array},
};

/// Borrow only while enumerating a concrete native owner. A completed visit
/// has copied both key and value before the next source entry is requested.
/// Range order, map identity and concurrent-read/write policy belong to that
/// owner, not to a formatter registry or a second mutable map implementation.
pub trait SourceFmtMapRange {
    type Key: SourcePrintfValue + SourceFmtMapKey + SourceFmtValueCopy;
    type Value: SourcePrintfValue + SourceFmtValueCopy;
    fn source_map_len(&self) -> usize;
    fn source_map_range(&self, visit: impl FnMut(&Self::Key, &Self::Value)
        -> Result<(), SourceSliceAllocationError>) -> Result<(), SourceSliceAllocationError>;
}

impl<Key, Value, Hasher> SourceFmtMapRange for HashMap<Key, Value, Hasher>
where Key: SourcePrintfValue + SourceFmtMapKey + SourceFmtValueCopy,
      Value: SourcePrintfValue + SourceFmtValueCopy,
{
    type Key = Key;
    type Value = Value;
    fn source_map_len(&self) -> usize { self.len() }
    fn source_map_range(&self, mut visit: impl FnMut(&Key, &Value)
        -> Result<(), SourceSliceAllocationError>) -> Result<(), SourceSliceAllocationError>
    { for (key, value) in self.iter() { visit(key, value)?; } Ok(()) }
}
impl<Key, Value> SourceFmtMapRange for BTreeMap<Key, Value>
where Key: SourcePrintfValue + SourceFmtMapKey + SourceFmtValueCopy,
      Value: SourcePrintfValue + SourceFmtValueCopy,
{
    type Key = Key;
    type Value = Value;
    fn source_map_len(&self) -> usize { self.len() }
    fn source_map_range(&self, mut visit: impl FnMut(&Key, &Value)
        -> Result<(), SourceSliceAllocationError>) -> Result<(), SourceSliceAllocationError>
    { for (key, value) in self.iter() { visit(key, value)?; } Ok(()) }
}
impl<Map: SourceFmtMapRange> SourceFmtMapRange for RefCell<Map> {
    type Key = Map::Key;
    type Value = Map::Value;
    fn source_map_len(&self) -> usize { self.borrow().source_map_len() }
    fn source_map_range(&self, visit: impl FnMut(&Self::Key, &Self::Value)
        -> Result<(), SourceSliceAllocationError>) -> Result<(), SourceSliceAllocationError>
    {
        // The borrow ends before sorting/output/value methods. Those methods
        // may then change the original map while scalar snapshots stay fixed.
        // RefCell's own failure is not a Go concurrent-map/fatal profile.
        self.borrow().source_map_range(visit)
    }
}

/// Len -> allocation -> range -> Key copy -> Value copy -> append -> sort.
/// No key re-lookup occurs, so the range provider can retain distinct NaN
/// entries. Explicit copies also retain declared nested map/pointer headers.
pub fn capture_source_map<Map: SourceFmtMapRange>(map: &Map)
    -> Result<Vec<(Map::Key, Map::Value)>, SourceSliceAllocationError>
{
    let length = map.source_map_len();
    let mut entries = reserve_source_array::<(Map::Key, Map::Value)>(length as u64)?;
    map.source_map_range(|key, value| {
        let key = key.source_fmt_copy();
        let value = value.source_fmt_copy();
        // Go evaluates both entry copies before append may grow the slice.
        entries.try_reserve(1).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
        entries.push((key, value)); Ok(())
    })?;
    source_stable_sort(&mut entries, |left, right| left.0.source_key_cmp(&right.0));
    Ok(entries)
}

/// A borrowed live-map header. Snapshotting is deferred until reflection
/// formatting reaches this value; copying a nested map keeps the same owner.
/// Type spelling and address are explicit source/native projections.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfLiveMap<'a, Map> {
    pub owner: Option<&'a Map>,
    pub source_type: &'a str,
    pub native_address: u64,
}
impl<Map> SourceFmtValueCopy for SourcePrintfLiveMap<'_, Map> {
    fn source_fmt_copy(&self) -> Self { Self { owner: self.owner, source_type: self.source_type, native_address: self.native_address } }
}
impl<Map: SourceFmtMapRange> SourcePrintfValue for SourcePrintfLiveMap<'_, Map> {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { self.source_type }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if begin_source_map(self, self.owner.is_none(), self.native_address,
            verb, state, profile, mode, recursion, output)? { return Ok(()); }
        let entries = match self.owner { Some(map) => capture_source_map(map)?, None => Vec::new() };
        write_source_map_pairs(entries.iter().map(|(key, value)| (key, value)),
            verb, state, profile, mode, recursion, output)
    }
}
