//! Closed, source-owned meta/v1 scheme and real discovery encoders.
//! protobuf fields follow the pinned generated.pb.go, including zero-valued
//! fields and the Kubernetes runtime.Unknown envelope. No build-time emitter.
use std::sync::Arc;
use cgp::prelude::*;
use futures::future::BoxFuture;
use serde::{Serialize, Deserialize};
use crate::{errors::BoxError, utility_admission::{null_default, TypeMeta, Status, ListMeta, StatusDetails, StatusCause}, utility_web::ResponseWriter};

fn empty<T>(value: &Option<Arc<Vec<T>>>) -> bool { value.as_ref().is_none_or(|values| values.is_empty()) }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct APIResource {
    #[serde(deserialize_with = "null_default")] pub name: String,
    #[serde(deserialize_with = "null_default")] pub singular_name: String,
    #[serde(deserialize_with = "null_default")] pub namespaced: bool,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub group: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub version: String,
    #[serde(deserialize_with = "null_default")] pub kind: String,
    pub verbs: Option<Arc<Vec<String>>>,
    #[serde(skip_serializing_if = "empty")] pub short_names: Option<Arc<Vec<String>>>,
    #[serde(skip_serializing_if = "empty")] pub categories: Option<Arc<Vec<String>>>,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub storage_version_hash: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct APIResourceList {
    #[serde(flatten)] pub type_meta: TypeMeta,
    #[serde(deserialize_with = "null_default")] pub group_version: String,
    pub resources: Option<Arc<Vec<APIResource>>>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum MetaObject { APIResourceList(APIResourceList), Status(Status) }
impl MetaObject {
    pub fn target_v1(&mut self) {
        let (meta, kind) = match self {
            Self::APIResourceList(list) => (&mut list.type_meta, "APIResourceList"),
            Self::Status(status) => (&mut status.type_meta, "Status"),
        };
        meta.kind = kind.into(); meta.api_version = "v1".into();
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SerializerInfo { Json, Yaml, Protobuf }
impl SerializerInfo {
    pub const fn media_type(self) -> &'static str { match self { Self::Json => "application/json", Self::Yaml => "application/yaml", Self::Protobuf => "application/vnd.kubernetes.protobuf" } }
}
pub struct SourceScheme { pub unversioned_version: &'static str, pub unversioned_kinds: &'static [&'static str] }
pub struct CodecFactory { pub media_types: Vec<SerializerInfo> }
impl Default for CodecFactory { fn default() -> Self { Self { media_types: vec![SerializerInfo::Json, SerializerInfo::Yaml, SerializerInfo::Protobuf] } } }

#[cgp_component(ApiObjectEncoder)]
pub trait CanEncodeApiObject: Sync {
    fn encode_api_object<'a>(&'a self, info: SerializerInfo, object: &'a MetaObject, writer: &'a mut ResponseWriter) -> BoxFuture<'a, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl<Context: Sync> ApiObjectEncoder<Context> for EncodeSourceMetaObject {
    fn encode_api_object<'a>(_context: &'a Context, info: SerializerInfo, object: &'a MetaObject, writer: &'a mut ResponseWriter) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            match info {
                SerializerInfo::Json => writer.encode_json(object).await,
                SerializerInfo::Yaml => {
                    // Keep the Agones data projection; serde_yaml supplies
                    // native Rust scalar spelling and presentation.
                    let value = serde_json::to_value(object)?;
                    let bytes = serde_yaml::to_string(&value)?;
                    writer.write(bytes.as_bytes()).await?; Ok(())
                }
                SerializerInfo::Protobuf => { writer.write(&encode_protobuf(object)).await?; Ok(()) }
            }
        })
    }
}

fn varint(out: &mut Vec<u8>, mut value: u64) {
    while value >= 128 { out.push((value as u8 & 0x7f) | 0x80); value >>= 7; }
    out.push(value as u8);
}
fn bytes(out: &mut Vec<u8>, field: u64, value: &[u8]) { varint(out, field << 3 | 2); varint(out, value.len() as u64); out.extend_from_slice(value); }
fn string(out: &mut Vec<u8>, field: u64, value: &str) { bytes(out, field, value.as_bytes()); }
fn integer(out: &mut Vec<u8>, field: u64, value: i64) { varint(out, field << 3); varint(out, value as u64); }
fn list_meta(meta: &ListMeta) -> Vec<u8> {
    let mut out = Vec::new(); string(&mut out, 1, &meta.self_link); string(&mut out, 2, &meta.resource_version); string(&mut out, 3, &meta.continue_token);
    if let Some(count) = meta.remaining_item_count { integer(&mut out, 4, count); } out
}
fn cause(cause: &StatusCause) -> Vec<u8> {
    let mut out = Vec::new(); string(&mut out, 1, &cause.cause_type); string(&mut out, 2, &cause.message); string(&mut out, 3, &cause.field); out
}
fn details(details: &StatusDetails) -> Vec<u8> {
    let mut out = Vec::new(); string(&mut out, 1, &details.name); string(&mut out, 2, &details.group); string(&mut out, 3, &details.kind);
    if let Some(causes) = &details.causes { for item in causes { bytes(&mut out, 4, &cause(item)); } }
    integer(&mut out, 5, details.retry_after_seconds as i64); string(&mut out, 6, &details.uid); out
}
fn resource(resource: &APIResource) -> Vec<u8> {
    let mut out = Vec::new(); string(&mut out, 1, &resource.name); integer(&mut out, 2, i64::from(resource.namespaced)); string(&mut out, 3, &resource.kind);
    if let Some(verbs) = &resource.verbs {
        let mut encoded = Vec::new(); for verb in verbs.iter() { string(&mut encoded, 1, verb); } bytes(&mut out, 4, &encoded);
    }
    if let Some(names) = &resource.short_names { for name in names.iter() { string(&mut out, 5, name); } }
    string(&mut out, 6, &resource.singular_name);
    if let Some(categories) = &resource.categories { for category in categories.iter() { string(&mut out, 7, category); } }
    string(&mut out, 8, &resource.group); string(&mut out, 9, &resource.version); string(&mut out, 10, &resource.storage_version_hash); out
}
pub fn encode_protobuf(object: &MetaObject) -> Vec<u8> {
    let mut raw = Vec::new();
    let meta = match object {
        MetaObject::APIResourceList(list) => {
            string(&mut raw, 1, &list.group_version);
            if let Some(resources) = &list.resources { for item in resources.iter() { bytes(&mut raw, 2, &resource(item)); } }
            &list.type_meta
        }
        MetaObject::Status(status) => {
            bytes(&mut raw, 1, &list_meta(&status.metadata)); string(&mut raw, 2, &status.status); string(&mut raw, 3, &status.message); string(&mut raw, 4, &status.reason);
            if let Some(value) = &status.details { bytes(&mut raw, 5, &details(value)); }
            integer(&mut raw, 6, status.code as i64); &status.type_meta
        }
    };
    let mut type_meta = Vec::new(); string(&mut type_meta, 1, &meta.api_version); string(&mut type_meta, 2, &meta.kind);
    let mut out = b"k8s\0".to_vec(); bytes(&mut out, 1, &type_meta); bytes(&mut out, 2, &raw); bytes(&mut out, 3, &[]); bytes(&mut out, 4, &[]); out
}
