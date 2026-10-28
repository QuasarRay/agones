//! The URL pointer retained by a source Request and by LogRequest fields.
// Adapted from Go net/url.URL, Userinfo and URL.EscapedPath.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{SourceByteString, mux};
use bytes::Bytes;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use std::sync::{Arc, RwLock};

/// Userinfo is immutable in the Go package. A copied URL retains this pointer;
/// its unexported fields serialize as {} rather than exposing its password.
#[derive(Clone, Debug)]
pub struct SourceUserinfo(Arc<Userinfo>);
#[derive(Debug)]
struct Userinfo { username: Bytes, password: Bytes, password_set: bool }
impl SourceUserinfo {
    pub fn user(username: impl AsRef<[u8]>) -> Self {
        Self(Arc::new(Userinfo { username: Bytes::copy_from_slice(username.as_ref()), password: Bytes::new(), password_set: false }))
    }
    pub fn user_password(username: impl AsRef<[u8]>, password: impl AsRef<[u8]>) -> Self {
        Self(Arc::new(Userinfo { username: Bytes::copy_from_slice(username.as_ref()), password: Bytes::copy_from_slice(password.as_ref()), password_set: true }))
    }
    pub fn username(&self) -> Bytes { self.0.username.clone() }
    pub fn password(&self) -> (Bytes, bool) { (self.0.password.clone(), self.0.password_set) }
}
impl PartialEq for SourceUserinfo { fn eq(&self, other: &Self) -> bool { Arc::ptr_eq(&self.0, &other.0) } }
impl Eq for SourceUserinfo {}
impl Serialize for SourceUserinfo {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serializer.serialize_struct("Userinfo", 0)?.end() }
}

/// One URL value, in the declaration/JSON field order of both selected
/// Go1.26.5 and Go1.27.1 sources. Every Go string retains its original bytes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceUrlFields {
    pub scheme: Bytes,
    pub opaque: Bytes,
    pub user: Option<SourceUserinfo>,
    pub host: Bytes,
    pub path: Bytes,
    pub fragment: Bytes,
    pub raw_query: Bytes,
    pub raw_path: Bytes,
    pub raw_fragment: Bytes,
    pub force_query: bool,
    pub omit_host: bool,
}
impl Serialize for SourceUrlFields {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut value = serializer.serialize_struct("URL", 11)?;
        macro_rules! string_field {
            ($name:literal, $field:ident) => { value.serialize_field($name, &SourceByteString(self.$field.clone()))?; };
        }
        string_field!("Scheme", scheme); string_field!("Opaque", opaque);
        value.serialize_field("User", &self.user)?;
        string_field!("Host", host); string_field!("Path", path); string_field!("Fragment", fragment);
        string_field!("RawQuery", raw_query); string_field!("RawPath", raw_path); string_field!("RawFragment", raw_fragment);
        value.serialize_field("ForceQuery", &self.force_query)?; value.serialize_field("OmitHost", &self.omit_host)?;
        value.end()
    }
}

/// Clone is Go pointer assignment. `fields()` copies the URL value and
/// `new(fields)` allocates a different pointer, retaining immutable strings
/// and the same Userinfo pointer. Request.URL replacement is a separate field
/// assignment and cannot retarget an already-created log entry.
#[derive(Clone, Debug)]
pub struct SourceUrl(Arc<RwLock<SourceUrlFields>>);
impl SourceUrl {
    pub fn new(fields: SourceUrlFields) -> Self { Self(Arc::new(RwLock::new(fields))) }
    pub fn fields(&self) -> SourceUrlFields { self.0.read().unwrap_or_else(|error| error.into_inner()).clone() }
    pub fn update(&self, edit: impl FnOnce(&mut SourceUrlFields)) {
        edit(&mut self.0.write().unwrap_or_else(|error| error.into_inner()));
    }
    pub fn path(&self) -> Bytes { self.0.read().unwrap_or_else(|error| error.into_inner()).path.clone() }
    pub fn host(&self) -> Bytes { self.0.read().unwrap_or_else(|error| error.into_inner()).host.clone() }
    pub fn raw_query(&self) -> Bytes { self.0.read().unwrap_or_else(|error| error.into_inner()).raw_query.clone() }
    pub fn escaped_path(&self) -> Vec<u8> {
        let fields = self.0.read().unwrap_or_else(|error| error.into_inner());
        if !fields.raw_path.is_empty() && mux::valid_encoded_path(&fields.raw_path)
            && mux::path_unescape(&fields.raw_path).as_deref() == Some(fields.path.as_ref()) {
            return fields.raw_path.to_vec();
        }
        if fields.path.as_ref() == b"*" { return vec![b'*']; }
        mux::escape_url_path(&fields.path)
    }
}
impl PartialEq for SourceUrl { fn eq(&self, other: &Self) -> bool { Arc::ptr_eq(&self.0, &other.0) } }
impl Eq for SourceUrl {}
impl Serialize for SourceUrl {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.read().unwrap_or_else(|error| error.into_inner()).serialize(serializer)
    }
}
