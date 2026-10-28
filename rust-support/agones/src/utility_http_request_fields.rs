//! Mutable fields of the single source Request shared with ResponseWriter.
//! Cloning retains the same request identity, including Body/URL assignment.
//! Host and RequestURI are string values; a retained log captures their bytes.
use super::{SourceBody, SourceUrl};
use bytes::Bytes;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct RequestFields(Arc<RwLock<Fields>>);
struct Fields { method: Bytes, content_length: i64, body: SourceBody, url: Option<SourceUrl>, host: Bytes, request_uri: Bytes }
impl RequestFields {
    pub(super) fn new(method: Bytes, content_length: i64, body: SourceBody, url: Option<SourceUrl>, host: Bytes, request_uri: Bytes) -> Self {
        Self(Arc::new(RwLock::new(Fields { method, content_length, body, url, host, request_uri })))
    }
    pub fn method(&self) -> Bytes { self.0.read().unwrap_or_else(|error| error.into_inner()).method.clone() }
    pub fn set_method(&self, method: impl AsRef<[u8]>) { self.0.write().unwrap_or_else(|error| error.into_inner()).method = Bytes::copy_from_slice(method.as_ref()); }
    pub fn content_length(&self) -> i64 { self.0.read().unwrap_or_else(|error| error.into_inner()).content_length }
    pub fn set_content_length(&self, length: i64) { self.0.write().unwrap_or_else(|error| error.into_inner()).content_length = length; }
    pub fn body(&self) -> SourceBody { self.0.read().unwrap_or_else(|error| error.into_inner()).body.clone() }
    pub fn set_body(&self, body: SourceBody) { self.0.write().unwrap_or_else(|error| error.into_inner()).body = body; }
    pub fn url(&self) -> Option<SourceUrl> { self.0.read().unwrap_or_else(|error| error.into_inner()).url.clone() }
    pub fn set_url(&self, url: Option<SourceUrl>) { self.0.write().unwrap_or_else(|error| error.into_inner()).url = url; }
    pub fn host(&self) -> Bytes { self.0.read().unwrap_or_else(|error| error.into_inner()).host.clone() }
    pub fn set_host(&self, host: impl AsRef<[u8]>) { self.0.write().unwrap_or_else(|error| error.into_inner()).host = Bytes::copy_from_slice(host.as_ref()); }
    pub fn request_uri(&self) -> Bytes { self.0.read().unwrap_or_else(|error| error.into_inner()).request_uri.clone() }
    pub fn set_request_uri(&self, uri: impl AsRef<[u8]>) { self.0.write().unwrap_or_else(|error| error.into_inner()).request_uri = Bytes::copy_from_slice(uri.as_ref()); }
}
