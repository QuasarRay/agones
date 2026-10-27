//! Mutable fields of the single source Request shared with ResponseWriter.
//! Cloning retains the same request identity, including Body assignment.
use super::SourceBody;
use bytes::Bytes;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct RequestFields(Arc<RwLock<Fields>>);
struct Fields { method: Bytes, content_length: i64, body: SourceBody }
impl RequestFields {
    pub(super) fn new(method: Bytes, content_length: i64, body: SourceBody) -> Self {
        Self(Arc::new(RwLock::new(Fields { method, content_length, body })))
    }
    pub fn method(&self) -> Bytes { self.0.read().unwrap_or_else(|error| error.into_inner()).method.clone() }
    pub fn set_method(&self, method: impl AsRef<[u8]>) { self.0.write().unwrap_or_else(|error| error.into_inner()).method = Bytes::copy_from_slice(method.as_ref()); }
    pub fn content_length(&self) -> i64 { self.0.read().unwrap_or_else(|error| error.into_inner()).content_length }
    pub fn set_content_length(&self, length: i64) { self.0.write().unwrap_or_else(|error| error.into_inner()).content_length = length; }
    pub fn body(&self) -> SourceBody { self.0.read().unwrap_or_else(|error| error.into_inner()).body.clone() }
    pub fn set_body(&self, body: SourceBody) { self.0.write().unwrap_or_else(|error| error.into_inner()).body = body; }
}
