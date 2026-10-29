//! Byte-preserving source HTTP maps and shared []string backing views.
//! Fallible entrypoints cover explicit buffer/table reservations. Shared
//! control allocations, serializer internals and source GC/runtime allocation
//! schedules still require their native boundary/profile implementation.
// Adapted from Go net/http/header.go and net/textproto's MIMEHeader operations.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use bytes::Bytes;
use serde::{Serialize, Serializer, ser::Error as _};
use std::{borrow::Borrow, collections::HashMap, fmt, sync::{Arc, RwLock}};
use super::allocation::{complete_native_source_collection, nil_source_map_assignment};
#[path = "utility_http_header_capacity.rs"]
mod capacity;
pub use capacity::{HeaderAllocationProfile, SourceSliceAllocationError, SourceSliceAllocationLayout};
pub(crate) use capacity::reserve_source_array;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderName(Bytes);
impl HeaderName {
    pub fn new(name: impl AsRef<[u8]>) -> Self { complete_native_source_collection(Self::try_new(name.as_ref())) }
    pub fn try_new(name: &[u8]) -> Result<Self, SourceSliceAllocationError> {
        let mut copied = reserve_source_array::<u8>(name.len() as u64)?;
        copied.extend_from_slice(name); Ok(Self(Bytes::from(copied)))
    }
    pub fn as_bytes(&self) -> &[u8] { &self.0 }
    pub fn starts_with(&self, prefix: impl AsRef<[u8]>) -> bool { self.0.starts_with(prefix.as_ref()) }
    pub fn strip_prefix(&self, prefix: impl AsRef<[u8]>) -> Option<Self> {
        self.starts_with(prefix.as_ref()).then(|| Self(self.0.slice(prefix.as_ref().len()..)))
    }
}
impl AsRef<[u8]> for HeaderName { fn as_ref(&self) -> &[u8] { &self.0 } }
impl Borrow<[u8]> for HeaderName { fn borrow(&self) -> &[u8] { &self.0 } }
impl fmt::Debug for HeaderName { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(&self.0, out) } }

#[derive(Clone, Default)]
enum StringBacking {
    #[default] Nil,
    Empty,
    Slots(Arc<RwLock<Vec<Bytes>>>),
}
impl StringBacking {
    fn from_slots(slots: Vec<Bytes>) -> Self {
        if slots.is_empty() { Self::Empty } else { Self::Slots(Arc::new(RwLock::new(slots))) }
    }
    fn is_none(&self) -> bool { matches!(self, Self::Nil) }
    fn as_ref(&self) -> Option<&Arc<RwLock<Vec<Bytes>>>> { match self { Self::Slots(slots) => Some(slots), _ => None } }
}
#[derive(Clone, Default)]
pub struct HeaderValues { backing: StringBacking, offset: usize, length: usize, capacity: usize }
pub struct HeaderValueIter { values: HeaderValues, index: usize }
impl Iterator for HeaderValueIter {
    type Item = Bytes;
    fn next(&mut self) -> Option<Bytes> { let value = self.values.get(self.index)?; self.index += 1; Some(value) }
    fn size_hint(&self) -> (usize, Option<usize>) { let left = self.values.length - self.index; (left, Some(left)) }
}
impl ExactSizeIterator for HeaderValueIter {}
impl HeaderValues {
    pub fn nil() -> Self { Self::default() }
    pub fn is_nil(&self) -> bool { self.backing.is_none() }
    pub fn len(&self) -> usize { self.length }
    pub fn is_empty(&self) -> bool { self.length == 0 }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn make(length: usize, capacity: usize, profile: HeaderAllocationProfile) -> Self {
        let length = complete_native_source_collection(i64::try_from(length).map_err(|_| SourceSliceAllocationError::MakeLength));
        let capacity = complete_native_source_collection(i64::try_from(capacity).map_err(|_| SourceSliceAllocationError::MakeCapacity));
        complete_native_source_collection(Self::try_make(length, capacity, profile))
    }
    pub fn try_make(length: i64, capacity: i64, profile: HeaderAllocationProfile) -> Result<Self, SourceSliceAllocationError> {
        let layout = profile.make_slice_layout(length, capacity, (profile.word_bits() / 4) as u64)?;
        let mut slots = reserve_source_array::<Bytes>(layout.capacity)?;
        slots.resize_with(layout.capacity as usize, Bytes::new);
        Ok(Self { backing: StringBacking::from_slots(slots), offset: 0, length: length as usize, capacity: layout.capacity as usize })
    }
    fn try_extend_backing(&mut self, count: usize, profile: HeaderAllocationProfile) -> Result<usize, SourceSliceAllocationError> {
        let length = self.length.checked_add(count).ok_or(SourceSliceAllocationError::GrowLength)?;
        if length as u128 > (1u128 << (profile.word_bits() - 1)) - 1 { return Err(SourceSliceAllocationError::GrowLength); }
        if length > self.capacity {
            let layout = profile.grow_slice_layout(self.capacity as u64, length as u64, (profile.word_bits() / 4) as u64, false)?;
            let mut slots = reserve_source_array::<Bytes>(layout.capacity)?;
            slots.resize_with(layout.capacity as usize, Bytes::new);
            if let Some(backing) = self.backing.as_ref() {
                let old = backing.read().unwrap_or_else(|error| error.into_inner());
                slots[..self.length].clone_from_slice(&old[self.offset..self.offset + self.length]);
            }
            self.backing = StringBacking::from_slots(slots); self.offset = 0; self.capacity = layout.capacity as usize;
        }
        Ok(length)
    }
    /// A lowered append argument already consists of string values. Avoid
    /// an extra temporary Vec for the common source append(slice, value).
    pub fn try_append_values(&mut self, values: &[Bytes], profile: HeaderAllocationProfile) -> Result<(), SourceSliceAllocationError> {
        let length = self.try_extend_backing(values.len(), profile)?;
        if !values.is_empty() {
            let mut slots = self.backing.as_ref().expect("nonempty source slice backing").write().unwrap_or_else(|error| error.into_inner());
            slots[self.offset + self.length..self.offset + length].clone_from_slice(values);
        }
        self.length = length; Ok(())
    }
    /// Source append(dst, src...) retains overlapping-copy semantics. When
    /// descriptors share backing, take one lock and choose memmove order;
    /// holding a destination write lock while iterating src would deadlock.
    pub fn try_append_slice(&mut self, values: &Self, profile: HeaderAllocationProfile) -> Result<(), SourceSliceAllocationError> {
        let length = self.try_extend_backing(values.length, profile)?;
        if values.length == 0 { self.length = length; return Ok(()); }
        let destination = self.backing.as_ref().expect("nonempty source slice backing");
        let source = values.backing.as_ref().expect("nonempty source append argument");
        if Arc::ptr_eq(destination, source) {
            let mut slots = destination.write().unwrap_or_else(|error| error.into_inner());
            let start = self.offset + self.length;
            if start > values.offset && start < values.offset + values.length {
                for index in (0..values.length).rev() { slots[start + index] = slots[values.offset + index].clone(); }
            } else {
                for index in 0..values.length { slots[start + index] = slots[values.offset + index].clone(); }
            }
        } else {
            // Snapshot-free copy without holding two backing locks at once.
            // External concurrent mutation remains a source race profile.
            for index in 0..values.length {
                let value = values.get(index).expect("source append argument index");
                destination.write().unwrap_or_else(|error| error.into_inner())[self.offset + self.length + index] = value;
            }
        }
        self.length = length; Ok(())
    }
    pub fn from_values(values: impl IntoIterator<Item = Bytes>) -> Self {
        let values = complete_native_source_collection(collect_native_values(values)); let length = values.len();
        Self { backing: StringBacking::from_slots(values), offset: 0, length, capacity: length }
    }
    pub fn parsed(values: impl IntoIterator<Item = Bytes>, profile: HeaderAllocationProfile) -> Self {
        let mut result = Self::nil();
        for value in values { complete_native_source_collection(result.try_append_values(&[value], profile)); }
        result
    }
    pub fn get(&self, index: usize) -> Option<Bytes> {
        if index >= self.length { return None; }
        self.backing.as_ref().map(|backing| backing.read().unwrap_or_else(|error| error.into_inner())[self.offset + index].clone())
    }
    pub fn first(&self) -> Option<Bytes> { self.get(0) }
    pub fn set(&self, index: usize, value: impl Into<Bytes>) {
        if index >= self.length {
            crate::utility_panic::unwind_source_runtime_error(crate::utility_panic::SourceBoundsError {
                x: index as i64, y: self.length as i64, signed: false, code: crate::utility_panic::SourceBoundsCode::Index });
        }
        self.backing.as_ref().expect("nonempty source header backing").write().unwrap_or_else(|error| error.into_inner())[self.offset + index] = value.into();
    }
    pub fn reslice(&self, low: usize, high: usize, maximum: usize) -> Self {
        use crate::utility_panic::{SourceBoundsError, SourceBoundsCode, unwind_source_runtime_error};
        if maximum > self.capacity { unwind_source_runtime_error(SourceBoundsError {
            x: maximum as i64, y: self.capacity as i64, signed: false, code: SourceBoundsCode::Slice3Capacity }); }
        if high > maximum { unwind_source_runtime_error(SourceBoundsError {
            x: high as i64, y: maximum as i64, signed: false, code: SourceBoundsCode::Slice3High }); }
        if low > high { unwind_source_runtime_error(SourceBoundsError {
            x: low as i64, y: high as i64, signed: false, code: SourceBoundsCode::Slice3Low }); }
        Self { backing: self.backing.clone(), offset: self.offset + low, length: high - low, capacity: maximum - low }
    }
    pub fn append(&mut self, values: impl IntoIterator<Item = Bytes>, profile: HeaderAllocationProfile) {
        let values = complete_native_source_collection(collect_native_values(values));
        complete_native_source_collection(self.try_append_values(&values, profile));
    }
    pub fn iter(&self) -> HeaderValueIter { HeaderValueIter { values: self.clone(), index: 0 } }
    pub fn shares_backing_with(&self, other: &Self) -> bool {
        match (&self.backing, &other.backing) {
            (StringBacking::Slots(left), StringBacking::Slots(right)) => Arc::ptr_eq(left, right),
            (StringBacking::Nil, StringBacking::Nil) | (StringBacking::Empty, StringBacking::Empty) => true,
            _ => false,
        }
    }
}
/// Rust iterator bridge only. Source single-value and slice appends use
/// try_append_values/try_append_slice directly, without staging this Vec.
fn collect_native_values(values: impl IntoIterator<Item = Bytes>) -> Result<Vec<Bytes>, SourceSliceAllocationError> {
    let mut result = Vec::new();
    for value in values {
        if result.len() == result.capacity() {
            if result.len() as u128 >= isize::MAX as u128 / std::mem::size_of::<Bytes>() as u128 { return Err(SourceSliceAllocationError::NativeAddressSpace); }
            result.try_reserve(1).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
        }
        result.push(value);
    }
    Ok(result)
}
impl fmt::Debug for HeaderValues {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { if self.is_nil() { out.write_str("nil") } else { out.debug_list().entries(self.iter()).finish() } }
}
impl PartialEq for HeaderValues { fn eq(&self, other: &Self) -> bool { self.is_nil() == other.is_nil() && self.iter().eq(other.iter()) } }
impl Serialize for HeaderValues {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_json(self.json_bytes(), serializer)
    }
}
impl HeaderValues {
    fn json_bytes(&self) -> Vec<u8> { complete_native_source_collection(self.try_json_bytes()) }
    pub fn try_json_bytes(&self) -> Result<Vec<u8>, SourceSliceAllocationError> {
        let mut json = Vec::new();
        if self.is_nil() { append_json_bytes(&mut json, b"null")?; return Ok(json); }
        append_json_bytes(&mut json, b"[")?;
        for (index, value) in self.iter().enumerate() {
            if index != 0 { append_json_bytes(&mut json, b",")?; }
            append_json_string(&mut json, &value)?;
        }
        append_json_bytes(&mut json, b"]")?; Ok(json)
    }
}

/// Clone copies a Go map descriptor; shallow_copy and deep_clone allocate maps.
#[derive(Clone)]
pub struct Headers { map: Option<Arc<RwLock<HashMap<HeaderName, HeaderValues>>>>, profile: HeaderAllocationProfile }
pub struct HeaderIter { map: Option<Arc<RwLock<HashMap<HeaderName, HeaderValues>>>>, keys: std::vec::IntoIter<HeaderName> }
impl Iterator for HeaderIter {
    type Item = (HeaderName, HeaderValues);
    fn next(&mut self) -> Option<Self::Item> {
        let map = self.map.as_ref()?;
        for key in self.keys.by_ref() {
            if let Some(value) = map.read().unwrap_or_else(|error| error.into_inner()).get(&key).cloned() { return Some((key, value)); }
        }
        None
    }
}
impl Default for Headers { fn default() -> Self { Self::nil() } }
impl Headers {
    // The current authored protocol profile is explicit Go1.26/64/48, not a
    // Rust-host capacity fallback. Production tool/ABI selection is M0027.
    pub fn new() -> Self { Self::with_profile(HeaderAllocationProfile::GO126_64) }
    pub fn with_profile(profile: HeaderAllocationProfile) -> Self { Self { map: Some(Arc::new(RwLock::new(HashMap::new()))), profile } }
    pub fn nil() -> Self { Self::nil_with_profile(HeaderAllocationProfile::GO126_64) }
    pub fn nil_with_profile(profile: HeaderAllocationProfile) -> Self { Self { map: None, profile } }
    pub fn allocation_profile(&self) -> HeaderAllocationProfile { self.profile }
    pub fn is_nil(&self) -> bool { self.map.is_none() }
    pub fn len(&self) -> usize { self.map.as_ref().map_or(0, |map| map.read().unwrap_or_else(|error| error.into_inner()).len()) }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn get(&self, name: impl AsRef<[u8]>) -> Option<HeaderValues> {
        self.map.as_ref()?.read().unwrap_or_else(|error| error.into_inner()).get(name.as_ref()).cloned()
    }
    pub fn contains_key(&self, name: impl AsRef<[u8]>) -> bool { self.get(name).is_some() }
    pub fn insert(&self, name: impl AsRef<[u8]>, values: HeaderValues) -> Option<HeaderValues> {
        complete_native_source_collection(self.try_insert(name, values))
    }
    pub fn try_insert(&self, name: impl AsRef<[u8]>, values: HeaderValues) -> Result<Option<HeaderValues>, SourceSliceAllocationError> {
        self.try_insert_name(HeaderName::try_new(name.as_ref())?, values)
    }
    fn insert_name(&self, name: HeaderName, values: HeaderValues) -> Option<HeaderValues> {
        complete_native_source_collection(self.try_insert_name(name, values))
    }
    fn try_insert_name(&self, name: HeaderName, values: HeaderValues) -> Result<Option<HeaderValues>, SourceSliceAllocationError> {
        let mut map = self.map.as_ref().unwrap_or_else(|| nil_source_map_assignment()).write().unwrap_or_else(|error| error.into_inner());
        if !map.contains_key(&name) { map.try_reserve(1).map_err(|_| SourceSliceAllocationError::OutOfMemory)?; }
        Ok(map.insert(name, values))
    }
    pub fn remove(&self, name: impl AsRef<[u8]>) -> Option<HeaderValues> {
        self.map.as_ref()?.write().unwrap_or_else(|error| error.into_inner()).remove(name.as_ref())
    }
    pub fn add(&self, name: impl AsRef<[u8]>, value: impl Into<Bytes>) {
        let name = canonical_header_key(name);
        self.append_exact_values(name, &[value.into()]);
    }
    /// Shared map/slice machinery also serves url.Values, whose keys must
    /// retain spelling and case rather than MIME-header canonicalization.
    pub(crate) fn append_exact(&self, name: impl AsRef<[u8]>, values: impl IntoIterator<Item = Bytes>) {
        let values = complete_native_source_collection(collect_native_values(values));
        self.append_exact_values(name, &values);
    }
    pub(crate) fn append_exact_values(&self, name: impl AsRef<[u8]>, values: &[Bytes]) {
        let name = HeaderName::new(name);
        let mut appended = self.get(name.as_bytes()).unwrap_or_else(HeaderValues::nil);
        // Source RHS append precedes map assignment, including on a nil map.
        // A failed append must not leave a newly inserted nil-valued key.
        complete_native_source_collection(appended.try_append_values(values, self.profile));
        self.insert_name(name, appended);
    }
    pub(crate) fn append_exact_slice(&self, name: impl AsRef<[u8]>, values: &HeaderValues) {
        let name = HeaderName::new(name);
        let mut appended = self.get(name.as_bytes()).unwrap_or_else(HeaderValues::nil);
        complete_native_source_collection(appended.try_append_slice(values, self.profile));
        self.insert_name(name, appended);
    }
    pub fn retain(&self, mut keep: impl FnMut(&HeaderName, &mut HeaderValues) -> bool) {
        if let Some(map) = &self.map { map.write().unwrap_or_else(|error| error.into_inner()).retain(|name, value| keep(name, value)); }
    }
    pub fn iter(&self) -> HeaderIter {
        complete_native_source_collection(self.try_iter())
    }
    pub fn try_iter(&self) -> Result<HeaderIter, SourceSliceAllocationError> {
        let keys = if let Some(map) = &self.map {
            let map = map.read().unwrap_or_else(|error| error.into_inner());
            let mut keys = reserve_source_array::<HeaderName>(map.len() as u64)?;
            keys.extend(map.keys().cloned()); keys
        } else { Vec::new() };
        Ok(HeaderIter { map: self.map.clone(), keys: keys.into_iter() })
    }
    pub fn keys(&self) -> impl Iterator<Item = HeaderName> { self.iter().map(|(name, _)| name) }
    fn try_fields(&self) -> Result<Vec<(HeaderName, HeaderValues)>, SourceSliceAllocationError> {
        let iter = self.try_iter()?;
        let mut fields = reserve_source_array::<(HeaderName, HeaderValues)>(iter.keys.len() as u64)?;
        fields.extend(iter); Ok(fields)
    }
    pub fn sorted(&self) -> Vec<(HeaderName, HeaderValues)> { complete_native_source_collection(self.try_sorted()) }
    pub fn try_sorted(&self) -> Result<Vec<(HeaderName, HeaderValues)>, SourceSliceAllocationError> {
        let mut fields = self.try_fields()?;
        // Map keys are unique; stable and unstable sorting have the same
        // byte order. The latter needs no additional merge scratch buffer.
        fields.sort_unstable_by(|left, right| left.0.cmp(&right.0)); Ok(fields)
    }
    pub fn shallow_copy(&self) -> Self {
        complete_native_source_collection(self.try_shallow_copy())
    }
    pub fn try_shallow_copy(&self) -> Result<Self, SourceSliceAllocationError> {
        let copy = Self::with_profile(self.profile);
        for (name, value) in self.try_iter()? { copy.try_insert_name(name, value)?; }
        Ok(copy)
    }
    pub fn deep_clone(&self) -> Self {
        complete_native_source_collection(self.try_deep_clone())
    }
    pub fn try_deep_clone(&self) -> Result<Self, SourceSliceAllocationError> {
        if self.is_nil() { return Ok(self.clone()); }
        let fields = self.try_fields()?;
        let total = self.profile.try_clone_length(fields.iter().map(|(_, values)| values.len()))?;
        let all = HeaderValues::try_make(total as i64, total as i64, self.profile)?;
        let copy = Self::with_profile(self.profile); let mut offset = 0;
        for (name, values) in fields {
            // Header.Clone uses source int accumulation, then copy into the
            // remaining combined buffer, including its overflow/truncation.
            let length = values.len().min(total - offset);
            for (index, value) in values.iter().take(length).enumerate() { all.set(offset + index, value); }
            copy.try_insert_name(name, if values.is_nil() { HeaderValues::nil() } else { all.reslice(offset, offset + length, offset + length) })?;
            offset += length;
        }
        Ok(copy)
    }
    pub fn h2_clone(&self) -> Self {
        complete_native_source_collection(self.try_h2_clone())
    }
    pub fn try_h2_clone(&self) -> Result<Self, SourceSliceAllocationError> {
        let copy = Self::with_profile(self.profile);
        // Bundled http2cloneHeader makes each value slice separately and
        // turns nil value slices into non-nil empty slices.
        for (name, values) in self.try_iter()? {
            let copied = HeaderValues::try_make(values.len() as i64, values.len() as i64, self.profile)?;
            for (index, value) in values.iter().enumerate() { copied.set(index, value); }
            copy.try_insert_name(name, copied)?;
        }
        Ok(copy)
    }
    pub fn try_json_bytes(&self) -> Result<Vec<u8>, SourceSliceAllocationError> {
        let mut json = Vec::new();
        if self.is_nil() { append_json_bytes(&mut json, b"null")?; return Ok(json); }
        append_json_bytes(&mut json, b"{")?;
        for (index, (name, values)) in self.try_sorted()?.into_iter().enumerate() {
            if index != 0 { append_json_bytes(&mut json, b",")?; }
            append_json_string(&mut json, name.as_bytes())?; append_json_bytes(&mut json, b":")?;
            append_json_bytes(&mut json, &values.try_json_bytes()?)?;
        }
        append_json_bytes(&mut json, b"}")?; Ok(json)
    }
}
impl IntoIterator for &Headers {
    type Item = (HeaderName, HeaderValues); type IntoIter = HeaderIter;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl fmt::Debug for Headers {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { if self.is_nil() { out.write_str("nil") } else { out.debug_map().entries(self.sorted()).finish() } }
}
impl PartialEq for Headers {
    fn eq(&self, other: &Self) -> bool { self.is_nil() == other.is_nil() && self.sorted() == other.sorted() }
}
impl Serialize for Headers {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.is_nil() { return serializer.serialize_none(); }
        serialize_json(complete_native_source_collection(self.try_json_bytes()), serializer)
    }
}
pub fn canonical_header_key(name: impl AsRef<[u8]>) -> HeaderName { complete_native_source_collection(try_canonical_header_key(name.as_ref())) }
pub fn try_canonical_header_key(name: &[u8]) -> Result<HeaderName, SourceSliceAllocationError> {
    let mut copied = reserve_source_array::<u8>(name.len() as u64)?;
    copied.extend_from_slice(name);
    if super::trailers::field_name(name) {
        let mut upper = true;
        for byte in &mut copied {
            let next = if upper { byte.to_ascii_uppercase() } else { byte.to_ascii_lowercase() };
            upper = *byte == b'-'; *byte = next;
        }
    }
    Ok(HeaderName(Bytes::from(copied)))
}
pub fn set_header(headers: &Headers, name: impl AsRef<[u8]>, value: impl AsRef<[u8]>) {
    complete_native_source_collection(try_set_header(headers, name.as_ref(), value.as_ref()));
}
pub fn try_set_header(headers: &Headers, name: &[u8], value: &[u8]) -> Result<(), SourceSliceAllocationError> {
    let name = try_canonical_header_key(name)?;
    let mut copied = reserve_source_array::<u8>(value.len() as u64)?;
    copied.extend_from_slice(value);
    let values = HeaderValues::try_make(1, 1, headers.profile)?;
    values.set(0, Bytes::from(copied));
    headers.try_insert_name(name, values)?; Ok(())
}
pub fn header(headers: &Headers, name: impl AsRef<[u8]>) -> Option<Bytes> { headers.get(canonical_header_key(name)).and_then(|values| values.first()) }
pub fn headers_json(headers: &Headers) -> serde_json::Value { serde_json::to_value(headers).expect("source header string values serialize") }
/// A Go string retains arbitrary original bytes until encoding/json escapes
/// each invalid UTF8 byte. Rust's lossy String conversion erases that data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceByteString(pub Bytes);
impl Serialize for SourceByteString {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut json = Vec::new(); complete_native_source_collection(append_json_string(&mut json, &self.0)); serialize_json(json, serializer)
    }
}
fn serialize_json<S: Serializer>(json: Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
    let json = String::from_utf8(json).map_err(S::Error::custom)?;
    let raw = serde_json::value::RawValue::from_string(json).map_err(S::Error::custom)?;
    raw.serialize(serializer)
}
fn append_json_bytes(result: &mut Vec<u8>, bytes: &[u8]) -> Result<(), SourceSliceAllocationError> {
    let length = result.len().checked_add(bytes.len()).ok_or(SourceSliceAllocationError::NativeAddressSpace)?;
    if length > isize::MAX as usize { return Err(SourceSliceAllocationError::NativeAddressSpace); }
    result.try_reserve(bytes.len()).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
    result.extend_from_slice(bytes); Ok(())
}
pub(crate) fn append_json_string(result: &mut Vec<u8>, bytes: &[u8]) -> Result<(), SourceSliceAllocationError> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    append_json_bytes(result, b"\"")?; let mut offset = 0;
    while offset < bytes.len() {
        let first = bytes[offset];
        if first < 0x80 {
            match first {
                b'"' | b'\\' => append_json_bytes(result, &[b'\\', first])?,
                b'\n' => append_json_bytes(result, b"\\n")?, b'\r' => append_json_bytes(result, b"\\r")?, b'\t' => append_json_bytes(result, b"\\t")?,
                8 => append_json_bytes(result, b"\\b")?, 12 => append_json_bytes(result, b"\\f")?,
                0..=31 | b'<' | b'>' | b'&' => append_json_bytes(result, &[b'\\', b'u', b'0', b'0', HEX[(first >> 4) as usize], HEX[(first & 15) as usize]])?,
                _ => append_json_bytes(result, &[first])?,
            }
            offset += 1; continue;
        }
        let width = match first { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => 0 };
        if width > 0 && offset + width <= bytes.len() {
            if let Ok(value) = std::str::from_utf8(&bytes[offset..offset + width]) {
                match value { "\u{2028}" => append_json_bytes(result, b"\\u2028")?, "\u{2029}" => append_json_bytes(result, b"\\u2029")?, _ => append_json_bytes(result, value.as_bytes())? }
                offset += width; continue;
            }
        }
        // The source emits an escape for each invalid original byte, while
        // an already valid U+FFFD remains literal UTF8. Keep that distinction.
        append_json_bytes(result, b"\\ufffd")?; offset += 1;
    }
    append_json_bytes(result, b"\"")?; Ok(())
}
