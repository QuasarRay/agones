//! Admission-v1 wire declarations used by the source webhook algorithm.
//! Source: pinned k8s.io/api/admission/v1/types.go and apimachinery meta/v1.
//! These are wire values, not another cluster integration.
use std::{collections::BTreeMap, sync::Arc};
use base64::{Engine, engine::general_purpose::STANDARD};
use cgp::prelude::*;
use futures::future::BoxFuture;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::value::RawValue;
use crate::errors::BoxError;

pub fn null_default<'de, D: Deserializer<'de>, T: Deserialize<'de> + Default>(input: D) -> Result<T, D::Error> {
    Ok(Option::<T>::deserialize(input)?.unwrap_or_default())
}
fn zero(value: &i32) -> bool { *value == 0 }
fn empty_list<T>(value: &Option<Vec<T>>) -> bool { value.as_ref().is_none_or(Vec::is_empty) }
fn empty_map<K, V>(value: &Option<BTreeMap<K, V>>) -> bool { value.as_ref().is_none_or(BTreeMap::is_empty) }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TypeMeta {
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub kind: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub api_version: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ListMeta {
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub self_link: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub resource_version: String,
    #[serde(rename = "continue", skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub continue_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: Option<i64>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupVersionKind {
    #[serde(deserialize_with = "null_default")] pub group: String,
    #[serde(deserialize_with = "null_default")] pub version: String,
    #[serde(deserialize_with = "null_default")] pub kind: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupVersionResource {
    #[serde(deserialize_with = "null_default")] pub group: String,
    #[serde(deserialize_with = "null_default")] pub version: String,
    #[serde(deserialize_with = "null_default")] pub resource: String,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupKind {
    #[serde(deserialize_with = "null_default")] pub group: String,
    #[serde(deserialize_with = "null_default")] pub kind: String,
}

/// RawExtension keeps JSON bytes (including numeric spellings and duplicate
/// object members) available to admission handlers, instead of a Value roundtrip.
#[derive(Clone, Debug, Default)]
pub struct RawExtension { pub raw: Option<Box<RawValue>> }
impl Serialize for RawExtension {
    fn serialize<S: Serializer>(&self, output: S) -> Result<S::Ok, S::Error> {
        match &self.raw { Some(raw) => raw.serialize(output), None => output.serialize_none() }
    }
}
impl<'de> Deserialize<'de> for RawExtension {
    fn deserialize<D: Deserializer<'de>>(input: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(input)?;
        Ok(Self { raw: if raw.get() == "null" { None } else { Some(raw) } })
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct UserInfo {
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub username: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub uid: String,
    #[serde(skip_serializing_if = "empty_list")] pub groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "empty_map")] pub extra: Option<BTreeMap<String, Option<Vec<String>>>>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AdmissionReview {
    #[serde(flatten)] pub type_meta: TypeMeta,
    #[serde(skip_serializing_if = "Option::is_none")] pub request: Option<AdmissionRequest>,
    #[serde(skip_serializing_if = "Option::is_none")] pub response: Option<AdmissionResponse>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AdmissionRequest {
    #[serde(deserialize_with = "null_default")] pub uid: String,
    #[serde(deserialize_with = "null_default")] pub kind: GroupVersionKind,
    #[serde(deserialize_with = "null_default")] pub resource: GroupVersionResource,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub sub_resource: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub request_kind: Option<GroupVersionKind>,
    #[serde(skip_serializing_if = "Option::is_none")] pub request_resource: Option<GroupVersionResource>,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub request_sub_resource: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub name: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub namespace: String,
    #[serde(deserialize_with = "null_default")] pub operation: String,
    #[serde(deserialize_with = "null_default")] pub user_info: UserInfo,
    pub object: RawExtension,
    pub old_object: RawExtension,
    #[serde(skip_serializing_if = "Option::is_none")] pub dry_run: Option<bool>,
    pub options: RawExtension,
}

mod patch_bytes {
    use super::*;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Input { Encoded(String), Bytes(Vec<u8>) }
    pub fn deserialize<'de, D: Deserializer<'de>>(input: D) -> Result<Option<Vec<u8>>, D::Error> {
        match Option::<Input>::deserialize(input)? {
            None => Ok(None),
            Some(Input::Bytes(bytes)) => Ok(Some(bytes)),
            Some(Input::Encoded(text)) => {
                let text: Vec<_> = text.bytes().filter(|byte| *byte != b'\r' && *byte != b'\n').collect();
                STANDARD.decode(text).map(Some).map_err(serde::de::Error::custom)
            }
        }
    }
    pub fn serialize<S: Serializer>(bytes: &Option<Vec<u8>>, output: S) -> Result<S::Ok, S::Error> {
        match bytes { Some(bytes) => output.serialize_str(&STANDARD.encode(bytes)), None => output.serialize_none() }
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AdmissionResponse {
    #[serde(deserialize_with = "null_default")] pub uid: String,
    #[serde(deserialize_with = "null_default")] pub allowed: bool,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")] pub result: Option<Status>,
    #[serde(with = "patch_bytes", skip_serializing_if = "empty_list")] pub patch: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub patch_type: Option<String>,
    #[serde(skip_serializing_if = "empty_map")] pub audit_annotations: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "empty_list")] pub warnings: Option<Vec<String>>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Status {
    #[serde(flatten)] pub type_meta: TypeMeta,
    #[serde(deserialize_with = "null_default")] pub metadata: ListMeta,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub status: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub message: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub details: Option<StatusDetails>,
    #[serde(skip_serializing_if = "zero", deserialize_with = "null_default")] pub code: i32,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct StatusDetails {
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub name: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub group: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub kind: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub uid: String,
    #[serde(skip_serializing_if = "empty_list")] pub causes: Option<Vec<StatusCause>>,
    #[serde(skip_serializing_if = "zero", deserialize_with = "null_default")] pub retry_after_seconds: i32,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StatusCause {
    #[serde(rename = "reason", skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub cause_type: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub message: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")] pub field: String,
}

/// The returned review is retained even on error, as in the Go two-result API.
pub type Handler = Arc<dyn Fn(AdmissionReview) -> BoxFuture<'static, (AdmissionReview, Option<BoxError>)> + Send + Sync>;
#[cgp_component(AdmissionHandlerCaller)]
pub trait CanCallAdmissionHandler: Sync {
    fn call_admission_handler<'a>(&'a self, handler: &'a Handler, review: AdmissionReview) -> BoxFuture<'a, (AdmissionReview, Option<BoxError>)>;
}
#[cgp_new_provider]
impl<Context: Sync> AdmissionHandlerCaller<Context> for CallSourceAdmissionHandler {
    fn call_admission_handler<'a>(_context: &'a Context, handler: &'a Handler, review: AdmissionReview) -> BoxFuture<'a, (AdmissionReview, Option<BoxError>)> { handler(review) }
}
