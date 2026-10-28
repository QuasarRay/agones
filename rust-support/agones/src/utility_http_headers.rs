//! Byte-preserving source HTTP maps and shared []string backing views.
// Adapted from Go net/http/header.go and net/textproto's MIMEHeader operations.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use bytes::Bytes;
use serde::{Serialize, Serializer, ser::Error as _};
use std::{borrow::Borrow, collections::HashMap, fmt, sync::{Arc, RwLock}};
#[path = "utility_http_header_capacity.rs"]
mod capacity;
pub use capacity::HeaderAllocationProfile;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderName(Bytes);
impl HeaderName {
    pub fn new(name: impl AsRef<[u8]>) -> Self { Self(Bytes::copy_from_slice(name.as_ref())) }
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
pub struct HeaderValues { backing: Option<Arc<RwLock<Vec<Bytes>>>>, offset: usize, length: usize, capacity: usize }
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
        if length > capacity { panic!("makeslice: cap out of range"); }
        profile.validate(capacity);
        Self { backing: Some(Arc::new(RwLock::new(vec![Bytes::new(); capacity]))), offset: 0, length, capacity }
    }
    pub fn from_values(values: impl IntoIterator<Item = Bytes>) -> Self {
        let values: Vec<_> = values.into_iter().collect(); let length = values.len();
        Self { backing: Some(Arc::new(RwLock::new(values))), offset: 0, length, capacity: length }
    }
    pub fn parsed(values: impl IntoIterator<Item = Bytes>, profile: HeaderAllocationProfile) -> Self {
        let mut result = Self::nil();
        for value in values { result.append([value], profile); }
        result
    }
    pub fn get(&self, index: usize) -> Option<Bytes> {
        if index >= self.length { return None; }
        self.backing.as_ref().map(|backing| backing.read().unwrap_or_else(|error| error.into_inner())[self.offset + index].clone())
    }
    pub fn first(&self) -> Option<Bytes> { self.get(0) }
    pub fn set(&self, index: usize, value: impl Into<Bytes>) {
        if index >= self.length { panic!("runtime error: index out of range [{index}] with length {}", self.length); }
        self.backing.as_ref().expect("nonempty source header backing").write().unwrap_or_else(|error| error.into_inner())[self.offset + index] = value.into();
    }
    pub fn reslice(&self, low: usize, high: usize, maximum: usize) -> Self {
        if low > high || high > maximum || maximum > self.capacity { panic!("runtime error: slice bounds out of range"); }
        Self { backing: self.backing.clone(), offset: self.offset + low, length: high - low, capacity: maximum - low }
    }
    pub fn append(&mut self, values: impl IntoIterator<Item = Bytes>, profile: HeaderAllocationProfile) {
        let values: Vec<_> = values.into_iter().collect();
        let length = self.length.checked_add(values.len()).unwrap_or_else(|| panic!("growslice: len out of range"));
        profile.validate(length);
        if length > self.capacity {
            let mut replacement = Self::make(length, profile.grow(self.capacity, length), profile);
            if let Some(backing) = &replacement.backing {
                let mut slots = backing.write().unwrap_or_else(|error| error.into_inner());
                for (index, value) in self.iter().chain(values).enumerate() { slots[index] = value; }
            }
            replacement.length = length; *self = replacement;
        } else {
            if let Some(backing) = &self.backing {
                let mut slots = backing.write().unwrap_or_else(|error| error.into_inner());
                for (index, value) in values.into_iter().enumerate() { slots[self.offset + self.length + index] = value; }
            }
            self.length = length;
        }
    }
    pub fn iter(&self) -> HeaderValueIter { HeaderValueIter { values: self.clone(), index: 0 } }
    pub fn shares_backing_with(&self, other: &Self) -> bool {
        match (&self.backing, &other.backing) { (Some(left), Some(right)) => Arc::ptr_eq(left, right), (None, None) => true, _ => false }
    }
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
    fn json_bytes(&self) -> Vec<u8> {
        if self.is_nil() { return b"null".to_vec(); }
        let mut json = vec![b'['];
        for (index, value) in self.iter().enumerate() { if index != 0 { json.push(b','); } append_json_string(&mut json, &value); }
        json.push(b']'); json
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
        self.map.as_ref().unwrap_or_else(|| panic!("assignment to entry in nil map")).write().unwrap_or_else(|error| error.into_inner()).insert(HeaderName::new(name), values)
    }
    pub fn remove(&self, name: impl AsRef<[u8]>) -> Option<HeaderValues> {
        self.map.as_ref()?.write().unwrap_or_else(|error| error.into_inner()).remove(name.as_ref())
    }
    pub fn add(&self, name: impl AsRef<[u8]>, value: impl Into<Bytes>) {
        let name = canonical_header_key(name);
        self.append_exact(name, [value.into()]);
    }
    /// Shared map/slice machinery also serves url.Values, whose keys must
    /// retain spelling and case rather than MIME-header canonicalization.
    pub(crate) fn append_exact(&self, name: impl AsRef<[u8]>, values: impl IntoIterator<Item = Bytes>) {
        let name = HeaderName::new(name);
        let mut map = self.map.as_ref().unwrap_or_else(|| panic!("assignment to entry in nil map")).write().unwrap_or_else(|error| error.into_inner());
        map.entry(name).or_default().append(values, self.profile);
    }
    pub fn retain(&self, mut keep: impl FnMut(&HeaderName, &mut HeaderValues) -> bool) {
        if let Some(map) = &self.map { map.write().unwrap_or_else(|error| error.into_inner()).retain(|name, value| keep(name, value)); }
    }
    pub fn iter(&self) -> HeaderIter {
        let keys = self.map.as_ref().map(|map| map.read().unwrap_or_else(|error| error.into_inner()).keys().cloned().collect::<Vec<_>>()).unwrap_or_default();
        HeaderIter { map: self.map.clone(), keys: keys.into_iter() }
    }
    pub fn keys(&self) -> impl Iterator<Item = HeaderName> { self.iter().map(|(name, _)| name) }
    pub fn sorted(&self) -> Vec<(HeaderName, HeaderValues)> { let mut fields: Vec<_> = self.iter().collect(); fields.sort_by(|left, right| left.0.cmp(&right.0)); fields }
    pub fn shallow_copy(&self) -> Self {
        let copy = Self::with_profile(self.profile);
        for (name, value) in self { copy.insert(name, value); }
        copy
    }
    pub fn deep_clone(&self) -> Self {
        if self.is_nil() { return self.clone(); }
        let fields: Vec<_> = self.iter().collect();
        let total = self.profile.clone_length(fields.iter().map(|(_, values)| values.len()));
        let all = HeaderValues::make(total, total, self.profile);
        let copy = Self::with_profile(self.profile); let mut offset = 0;
        for (name, values) in fields {
            // Header.Clone uses source int accumulation, then copy into the
            // remaining combined buffer, including its overflow/truncation.
            let length = values.len().min(total - offset);
            for (index, value) in values.iter().take(length).enumerate() { all.set(offset + index, value); }
            copy.insert(name, if values.is_nil() { HeaderValues::nil() } else { all.reslice(offset, offset + length, offset + length) });
            offset += length;
        }
        copy
    }
    pub fn h2_clone(&self) -> Self {
        let copy = Self::with_profile(self.profile);
        // Bundled http2cloneHeader makes each value slice separately and
        // turns nil value slices into non-nil empty slices.
        for (name, values) in self { copy.insert(name, HeaderValues::from_values(values.iter())); }
        copy
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
        let mut json = vec![b'{'];
        for (index, (name, values)) in self.sorted().into_iter().enumerate() {
            if index != 0 { json.push(b','); }
            append_json_string(&mut json, name.as_bytes()); json.push(b':'); json.extend(values.json_bytes());
        }
        json.push(b'}'); serialize_json(json, serializer)
    }
}
pub fn canonical_header_key(name: impl AsRef<[u8]>) -> HeaderName { HeaderName::new(super::trailers::canonical_name(name.as_ref())) }
pub fn set_header(headers: &Headers, name: impl AsRef<[u8]>, value: impl AsRef<[u8]>) {
    headers.insert(canonical_header_key(name), HeaderValues::from_values([Bytes::copy_from_slice(value.as_ref())]));
}
pub fn header(headers: &Headers, name: impl AsRef<[u8]>) -> Option<Bytes> { headers.get(canonical_header_key(name)).and_then(|values| values.first()) }
pub fn headers_json(headers: &Headers) -> serde_json::Value { serde_json::to_value(headers).expect("source header string values serialize") }
/// A Go string retains arbitrary original bytes until encoding/json escapes
/// each invalid UTF8 byte. Rust's lossy String conversion erases that data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceByteString(pub Bytes);
impl Serialize for SourceByteString {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut json = Vec::new(); append_json_string(&mut json, &self.0); serialize_json(json, serializer)
    }
}
fn serialize_json<S: Serializer>(json: Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
    let json = String::from_utf8(json).map_err(S::Error::custom)?;
    let raw = serde_json::value::RawValue::from_string(json).map_err(S::Error::custom)?;
    raw.serialize(serializer)
}
fn append_json_string(result: &mut Vec<u8>, bytes: &[u8]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    result.push(b'"'); let mut offset = 0;
    while offset < bytes.len() {
        let first = bytes[offset];
        if first < 0x80 {
            match first {
                b'"' | b'\\' => { result.push(b'\\'); result.push(first); }
                b'\n' => result.extend_from_slice(b"\\n"), b'\r' => result.extend_from_slice(b"\\r"), b'\t' => result.extend_from_slice(b"\\t"),
                8 => result.extend_from_slice(b"\\b"), 12 => result.extend_from_slice(b"\\f"),
                0..=31 | b'<' | b'>' | b'&' => { result.extend_from_slice(b"\\u00"); result.push(HEX[(first >> 4) as usize]); result.push(HEX[(first & 15) as usize]); }
                _ => result.push(first),
            }
            offset += 1; continue;
        }
        let width = match first { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => 0 };
        if width > 0 && offset + width <= bytes.len() {
            if let Ok(value) = std::str::from_utf8(&bytes[offset..offset + width]) {
                match value { "\u{2028}" => result.extend_from_slice(b"\\u2028"), "\u{2029}" => result.extend_from_slice(b"\\u2029"), _ => result.extend_from_slice(value.as_bytes()) }
                offset += width; continue;
            }
        }
        // The source emits an escape for each invalid original byte, while
        // an already valid U+FFFD remains literal UTF8. Keep that distinction.
        result.extend_from_slice(b"\\ufffd"); offset += 1;
    }
    result.push(b'"');
}
