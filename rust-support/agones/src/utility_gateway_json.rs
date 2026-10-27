//! The fixed grpc-gateway JSONPb profile selected by runtime.NewServerMux.
//! Descriptor pools describe protobuf data; they are not effect/provider registries.
//! Source: vendored grpc-gateway/runtime/marshal_jsonpb.go and protobuf/protojson.
use std::{collections::{BTreeMap, HashSet}, fmt};
use base64::Engine;
use prost_reflect::{Cardinality, DynamicMessage, EnumDescriptor, ExtensionDescriptor,
    FieldDescriptor, Kind, MapKey, MessageDescriptor, ReflectMessage, Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor}, ser::{Error as _, SerializeMap, SerializeSeq}};
use serde_json::value::RawValue;
use crate::{errors::{BoxError, Errors}, utility_http::HttpRequest};
use crate::utility_gateway_targets::GatewayDecodeTarget;
use crate::utility_gateway_framing::{first_value, ReadFailure};

const ERRORS: Errors = Errors::from_package("github.com/grpc-ecosystem/grpc-gateway/v2/runtime");
const MAX_MESSAGE_DEPTH: usize = 10_000;

/// A concrete generated HttpBody value has an outbound-only raw-body override.
/// A DynamicMessage with the same protobuf name follows ordinary ProtoJSON.
#[derive(Clone, Debug, Default)]
pub struct GatewayHttpBody {
    pub content_type: String,
    pub data: Vec<u8>,
    pub extensions: Vec<DynamicMessage>,
}

/// Explicit source value categories preserve typed nil, nil slice and enum
/// behavior without Rust Any downcasts or a global reflection/service locator.
#[derive(Clone, Debug)]
pub enum GatewayValue {
    Proto(DynamicMessage),
    NilProto(MessageDescriptor),
    HttpBody(Option<GatewayHttpBody>),
    /// The linked google.rpc.Status wire shape used by gateway errors.
    Status { code: i32, message: String, details: Vec<DynamicMessage> },
    Json(serde_json::Value),
    Enum(EnumDescriptor, i32),
    Bytes(Option<Vec<u8>>),
    List(Option<Vec<GatewayValue>>),
    Map(Option<BTreeMap<String, GatewayValue>>),
    Null,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GatewayJsonPb;
impl GatewayJsonPb {
    pub fn content_type<'a>(&self, value: &'a GatewayValue) -> &'a str {
        match value {
            GatewayValue::HttpBody(Some(body)) => &body.content_type,
            GatewayValue::HttpBody(None) => "",
            _ => "application/json",
        }
    }
    /// Marshal has no delimiter. JSONPb.NewEncoder adds one separately.
    pub fn marshal(&self, value: &GatewayValue) -> Result<Vec<u8>, BoxError> {
        if let GatewayValue::HttpBody(body) = value {
            return Ok(body.as_ref().map_or_else(Vec::new, |body| body.data.clone()));
        }
        let bytes = serialize_value(value)?;
        check_output_required(value)?;
        Ok(bytes)
    }
    pub fn marshal_message(&self, message: &DynamicMessage) -> Result<Vec<u8>, BoxError> {
        let bytes = serde_json::to_vec(&ProtoJson(message))?;
        check_required(message)?;
        Ok(bytes)
    }
    pub fn encode(&self, value: &GatewayValue) -> Result<Vec<u8>, BoxError> {
        // HTTPBodyMarshaler inherits JSONPb.NewEncoder: its override of
        // Marshal does not turn an encoded HttpBody into raw body data.
        let mut bytes = serialize_value(value)?;
        check_output_required(value)?;
        bytes.push(b'\n');
        Ok(bytes)
    }
    /// Like JSONPb.Unmarshal, Decode one RawMessage before resetting the proto.
    /// A syntax/EOF error in the outer JSON decoder leaves the target alone.
    pub fn unmarshal(&self, bytes: &[u8], target: &mut DynamicMessage) -> Result<(), BoxError> {
        self.unmarshal_into(bytes, target)
    }
    pub fn unmarshal_into<T: GatewayDecodeTarget + ?Sized>(&self, bytes: &[u8], target: &mut T) -> Result<(), BoxError> {
        target.prepare_gateway_decode()?;
        let (raw, _) = first_value(bytes, true)?.ok_or_else(|| ERRORS.New("EOF"))?;
        target.decode_gateway_raw(&raw)
    }
    pub fn unmarshal_raw(&self, raw: &RawValue, target: &mut DynamicMessage) -> Result<(), BoxError> {
        *target = DynamicMessage::new(target.descriptor());
        decode_message(target, raw, 0)?;
        check_required(target)
    }
}
fn serialize_value(value: &GatewayValue) -> Result<Vec<u8>, BoxError> {
    let bytes = serde_json::to_vec(&NonProtoJson(value))?;
    let proto = matches!(value, GatewayValue::Proto(_) | GatewayValue::NilProto(_) | GatewayValue::HttpBody(_) | GatewayValue::Status { .. });
    let direct_sequence = matches!(value, GatewayValue::List(Some(values)) if values.iter().all(|value|
        matches!(value, GatewayValue::Proto(_) | GatewayValue::NilProto(_) | GatewayValue::HttpBody(_) | GatewayValue::Status { .. } | GatewayValue::Enum(..))));
    if proto || direct_sequence { return Ok(bytes); }
    // encoding/json's map/primitive path escapes HTML even inside RawMessage.
    Ok(String::from_utf8(bytes)?.replace('&', "\\u0026").replace('<', "\\u003c").replace('>', "\\u003e")
        .replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029").into_bytes())
}

/// Own the decoder's read-ahead, including a second value in one HTTP frame.
/// Returning None is io.EOF, which generated unary handlers may ignore.
#[derive(Default)]
pub struct GatewayDecoder { buffered: Vec<u8>, eof: bool, read_failure: Option<ReadFailure> }
impl GatewayDecoder {
    pub async fn decode<T: GatewayDecodeTarget + ?Sized>(&mut self, request: &mut HttpRequest, target: &mut T) -> Result<bool, BoxError> {
        target.prepare_gateway_decode()?;
        if let Some(error) = &self.read_failure { return Err(Box::new(error.clone())); }
        loop {
            match first_value(&self.buffered, self.eof) {
                Ok(Some((raw, offset))) => {
                    self.buffered.drain(..offset);
                    target.decode_gateway_raw(&raw)?; // Semantic errors do not poison the stream.
                    return Ok(true);
                }
                Ok(None) if self.eof => return Ok(false),
                Ok(None) => {},
                Err(error) => return Err(self.remember_failure(error)),
            }
            match request.read_body_chunk().await {
                Ok(Some(bytes)) => self.buffered.extend_from_slice(&bytes),
                Ok(None) => self.eof = true,
                Err(error) => return Err(self.remember_failure(error)),
            }
        }
    }
    fn remember_failure(&mut self, error: BoxError) -> BoxError {
        let error = ReadFailure::new(error); self.read_failure = Some(error.clone()); Box::new(error)
    }
}

struct NonProtoJson<'a>(&'a GatewayValue);
impl Serialize for NonProtoJson<'_> {
    fn serialize<S: Serializer>(&self, out: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            GatewayValue::Proto(message) => ProtoJson(message).serialize(out),
            GatewayValue::NilProto(desc) => ProtoJson(&DynamicMessage::new(desc.clone())).serialize(out),
            GatewayValue::HttpBody(body) => {
                // Nested values are handled by JSONPb itself, not its outer
                // HTTPBodyMarshaler. Preserve the protobuf field encoding here.
                let empty = GatewayHttpBody::default();
                let body = body.as_ref().unwrap_or(&empty);
                let mut map = out.serialize_map(Some(3))?;
                map.serialize_entry("content_type", &body.content_type)?;
                map.serialize_entry("data", &base64::engine::general_purpose::STANDARD.encode(&body.data))?;
                map.serialize_entry("extensions", &ProtoSequence(&body.extensions))?;
                map.end()
            }
            GatewayValue::Status { code, message, details } => {
                let mut map = out.serialize_map(Some(3))?;
                map.serialize_entry("code", code)?;
                map.serialize_entry("message", message)?;
                map.serialize_entry("details", &ProtoSequence(details))?;
                map.end()
            }
            GatewayValue::Json(value) => value.serialize(out),
            GatewayValue::Enum(desc, number) => out.serialize_str(&desc.get_value(*number)
                .map_or_else(|| number.to_string(), |value| value.name().to_owned())),
            GatewayValue::Bytes(bytes) => match bytes {
                Some(bytes) => out.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes)),
                None => out.serialize_seq(Some(0))?.end(), // nil slice + EmitUnpopulated
            },
            GatewayValue::List(values) => {
                let mut seq = out.serialize_seq(Some(values.as_ref().map_or(0, Vec::len)))?;
                if let Some(values) = values { for value in values { seq.serialize_element(&NonProtoJson(value))?; } }
                seq.end()
            }
            GatewayValue::Map(values) => {
                let mut map = out.serialize_map(Some(values.as_ref().map_or(0, BTreeMap::len)))?;
                if let Some(values) = values { for (key, value) in values { map.serialize_entry(key, &NonProtoJson(value))?; } }
                map.end()
            }
            GatewayValue::Null => out.serialize_none(),
        }
    }
}
struct ProtoSequence<'a>(&'a [DynamicMessage]);
impl Serialize for ProtoSequence<'_> {
    fn serialize<S: Serializer>(&self, out: S) -> Result<S::Ok, S::Error> {
        let mut seq = out.serialize_seq(Some(self.0.len()))?;
        for value in self.0 { seq.serialize_element(&ProtoJson(value))?; }
        seq.end()
    }
}

struct ProtoJson<'a>(&'a DynamicMessage);
impl Serialize for ProtoJson<'_> {
    fn serialize<S: Serializer>(&self, out: S) -> Result<S::Ok, S::Error> {
        let desc = self.0.descriptor();
        match desc.full_name() {
            "google.protobuf.Any" => serialize_any(self.0, out),
            "google.protobuf.Timestamp" | "google.protobuf.Duration" | "google.protobuf.FieldMask" =>
                out.serialize_str(&crate::utility_gateway_wkt::marshal_text(self.0).map_err(S::Error::custom)?),
            "google.protobuf.Struct" => {
                let field = desc.get_field_by_name("fields").expect("Struct.fields");
                ProtoField(&self.0.get_field(&field), &field.kind()).serialize(out)
            }
            "google.protobuf.ListValue" => {
                let field = desc.get_field_by_name("values").expect("ListValue.values");
                ProtoField(&self.0.get_field(&field), &field.kind()).serialize(out)
            }
            "google.protobuf.Value" => {
                let (field, value) = self.0.fields().next().ok_or_else(|| S::Error::custom("google.protobuf.Value: none of the oneof fields is set"))?;
                if let Value::F64(value) = value {
                    if !value.is_finite() { return Err(S::Error::custom("google.protobuf.Value.number_value: invalid non-finite value")); }
                }
                ProtoField(value, &field.kind()).serialize(out)
            }
            name if is_wrapper(name) => {
                let field = desc.get_field_by_name("value").expect("wrapper.value");
                ProtoField(&self.0.get_field(&field), &field.kind()).serialize(out)
            }
            _ => {
                let mut map = out.serialize_map(None)?;
                serialize_fields(self.0, &mut map)?;
                map.end()
            }
        }
    }
}
fn serialize_fields<M: SerializeMap>(message: &DynamicMessage, map: &mut M) -> Result<(), M::Error> {
    let desc = message.descriptor();
    // Descriptor order is declaration index, not field number. UseProtoNames
    // uses group text names, and extension fields follow in full-name order.
    let mut fields: Vec<_> = desc.fields().collect();
    fields.sort_by(|left, right| left.path().cmp(right.path()));
    for field in fields {
        let present = message.has_field(&field);
        if !present && field.containing_oneof().is_some() { continue; }
        let name = text_name(&field);
        if !present && field.supports_presence() { map.serialize_entry(&name, &Option::<()>::None)?; }
        else { map.serialize_entry(&name, &ProtoField(&message.get_field(&field), &field.kind()))?; }
    }
    let mut extensions: Vec<_> = message.extensions().collect();
    extensions.sort_by(|(left, _), (right, _)| left.full_name().cmp(right.full_name()));
    for (field, value) in extensions { map.serialize_entry(&format!("[{}]", field.full_name()), &ProtoField(value, &field.kind()))?; }
    Ok(())
}
fn text_name(field: &FieldDescriptor) -> String {
    if field.is_group() {
        if let Kind::Message(message) = field.kind() { return message.name().to_owned(); }
    }
    field.name().to_owned()
}
fn is_wrapper(name: &str) -> bool {
    matches!(name, "google.protobuf.BoolValue" | "google.protobuf.Int32Value" | "google.protobuf.Int64Value"
        | "google.protobuf.UInt32Value" | "google.protobuf.UInt64Value" | "google.protobuf.FloatValue"
        | "google.protobuf.DoubleValue" | "google.protobuf.StringValue" | "google.protobuf.BytesValue")
}
fn has_custom_json(name: &str) -> bool {
    is_wrapper(name) || matches!(name, "google.protobuf.Any" | "google.protobuf.Timestamp" | "google.protobuf.Duration"
        | "google.protobuf.Struct" | "google.protobuf.ListValue" | "google.protobuf.Value" | "google.protobuf.FieldMask")
}
fn serialize_any<S: Serializer>(message: &DynamicMessage, out: S) -> Result<S::Ok, S::Error> {
    let desc = message.descriptor();
    let type_field = desc.get_field_by_name("type_url").expect("Any.type_url");
    let value_field = desc.get_field_by_name("value").expect("Any.value");
    if !message.has_field(&type_field) {
        if message.has_field(&value_field) { return Err(S::Error::custom("google.protobuf.Any: type_url is not set")); }
        return out.serialize_map(Some(0))?.end();
    }
    let type_value = message.get_field(&type_field);
    let Value::String(type_url) = type_value.as_ref() else { return Err(S::Error::custom("invalid Any type_url")); };
    let payload_value = message.get_field(&value_field);
    let Value::Bytes(bytes) = payload_value.as_ref() else { return Err(S::Error::custom("invalid Any value")); };
    let payload_desc = resolve_type(&desc, type_url).map_err(S::Error::custom)?;
    let payload = DynamicMessage::decode(payload_desc.clone(), bytes.as_ref()).map_err(S::Error::custom)?;
    let mut map = out.serialize_map(None)?;
    map.serialize_entry("@type", type_url)?;
    if has_custom_json(payload_desc.full_name()) { map.serialize_entry("value", &ProtoJson(&payload))?; }
    else { serialize_fields(&payload, &mut map)?; }
    map.end()
}
fn resolve_type(container: &MessageDescriptor, url: &str) -> Result<MessageDescriptor, BoxError> {
    let name = url.rsplit('/').next().unwrap_or(url);
    container.parent_pool().get_message_by_name(name)
        .ok_or_else(|| ERRORS.New(format!("unable to resolve {url:?}")))
}

struct ProtoField<'a>(&'a Value, &'a Kind);
impl Serialize for ProtoField<'_> {
    fn serialize<S: Serializer>(&self, out: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Value::Bool(value) => out.serialize_bool(*value),
            Value::I32(value) => out.serialize_i32(*value),
            Value::U32(value) => out.serialize_u32(*value),
            Value::I64(value) => out.collect_str(value),
            Value::U64(value) => out.collect_str(value),
            Value::F32(value) if value.is_finite() => out.serialize_f32(*value),
            Value::F64(value) if value.is_finite() => out.serialize_f64(*value),
            Value::F32(value) => out.serialize_str(if value.is_nan() { "NaN" } else if value.is_sign_positive() { "Infinity" } else { "-Infinity" }),
            Value::F64(value) => out.serialize_str(if value.is_nan() { "NaN" } else if value.is_sign_positive() { "Infinity" } else { "-Infinity" }),
            Value::String(value) => out.serialize_str(value),
            Value::Bytes(value) => out.serialize_str(&base64::engine::general_purpose::STANDARD.encode(value)),
            Value::EnumNumber(number) => {
                let Kind::Enum(desc) = self.1 else { return Err(S::Error::custom("enum descriptor missing")); };
                if desc.full_name() == "google.protobuf.NullValue" { out.serialize_none() }
                else if let Some(value) = desc.get_value(*number) { out.serialize_str(value.name()) }
                else { out.serialize_i32(*number) }
            }
            Value::Message(value) => ProtoJson(value).serialize(out),
            Value::List(values) => {
                let mut seq = out.serialize_seq(Some(values.len()))?;
                for value in values { seq.serialize_element(&ProtoField(value, self.1))?; }
                seq.end()
            }
            Value::Map(values) => {
                let Kind::Message(entry) = self.1 else { return Err(S::Error::custom("map entry descriptor missing")); };
                let kind = entry.map_entry_value_field().kind();
                let mut entries: Vec<_> = values.iter().collect();
                entries.sort_by(|(left, _), (right, _)| left.cmp(right));
                let mut map = out.serialize_map(Some(entries.len()))?;
                for (key, value) in entries { map.serialize_entry(&map_key_text(key), &ProtoField(value, &kind))?; }
                map.end()
            }
        }
    }
}
fn map_key_text(key: &MapKey) -> String {
    match key {
        MapKey::Bool(value) => value.to_string(), MapKey::I32(value) => value.to_string(),
        MapKey::I64(value) => value.to_string(), MapKey::U32(value) => value.to_string(),
        MapKey::U64(value) => value.to_string(), MapKey::String(value) => value.clone(),
    }
}
fn check_output_required(value: &GatewayValue) -> Result<(), BoxError> {
    match value {
        GatewayValue::Proto(value) => check_required(value),
        GatewayValue::NilProto(desc) => check_required(&DynamicMessage::new(desc.clone())),
        GatewayValue::List(Some(values)) => { for value in values { check_output_required(value)?; } Ok(()) },
        GatewayValue::Map(Some(values)) => { for value in values.values() { check_output_required(value)?; } Ok(()) },
        GatewayValue::HttpBody(Some(body)) => { for value in &body.extensions { check_required(value)?; } Ok(()) },
        GatewayValue::Status { details, .. } => { for value in details { check_required(value)?; } Ok(()) },
        _ => Ok(()),
    }
}
fn check_required(message: &DynamicMessage) -> Result<(), BoxError> {
    for field in message.descriptor().fields() {
        if field.cardinality() == Cardinality::Required && !message.has_field(&field) {
            return Err(ERRORS.New(format!("required field {} not set", field.full_name())));
        }
    }
    for (_, value) in message.fields() { check_required_value(value)?; }
    for (_, value) in message.extensions() { check_required_value(value)?; }
    Ok(())
}
fn check_required_value(value: &Value) -> Result<(), BoxError> {
    match value {
        Value::Message(value) => check_required(value),
        Value::List(values) => { for value in values { check_required_value(value)?; } Ok(()) },
        Value::Map(values) => { for value in values.values() { check_required_value(value)?; } Ok(()) },
        _ => Ok(()), // Do not inspect the wire bytes embedded in an Any.
    }
}

/// Keep duplicate names until schema-aware duplicate/oneof checks have run.
pub(crate) struct RawObject(pub(crate) Vec<(String, Box<RawValue>)>);
impl<'de> Deserialize<'de> for RawObject {
    fn deserialize<D: Deserializer<'de>>(input: D) -> Result<Self, D::Error> {
        struct ObjectVisitor;
        impl<'de> Visitor<'de> for ObjectVisitor {
            type Value = RawObject;
            fn expecting(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("a protobuf JSON object") }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<RawObject, M::Error> {
                let mut fields = Vec::new();
                while let Some(field) = map.next_entry()? { fields.push(field); }
                Ok(RawObject(fields))
            }
        }
        input.deserialize_map(ObjectVisitor)
    }
}
enum BoundField { Regular(FieldDescriptor), Extension(ExtensionDescriptor) }
impl BoundField {
    fn kind(&self) -> Kind { match self { Self::Regular(field) => field.kind(), Self::Extension(field) => field.kind() } }
    fn number(&self) -> u32 { match self { Self::Regular(field) => field.number(), Self::Extension(field) => field.number() } }
    fn is_list(&self) -> bool { match self { Self::Regular(field) => field.is_list(), Self::Extension(field) => field.is_list() } }
    fn is_map(&self) -> bool { match self { Self::Regular(field) => field.is_map(), Self::Extension(field) => field.is_map() } }
    fn set(&self, message: &mut DynamicMessage, value: Value) {
        match self { Self::Regular(field) => message.set_field(field, value), Self::Extension(field) => message.set_extension(field, value) }
    }
    fn get_mut<'a>(&self, message: &'a mut DynamicMessage) -> &'a mut Value {
        match self { Self::Regular(field) => message.get_field_mut(field), Self::Extension(field) => message.get_extension_mut(field) }
    }
}
fn decode_message(message: &mut DynamicMessage, raw: &RawValue, depth: usize) -> Result<(), BoxError> {
    if depth >= MAX_MESSAGE_DEPTH { return Err(ERRORS.New("exceeded max recursion depth")); }
    let desc = message.descriptor();
    match desc.full_name() {
        "google.protobuf.Any" => return decode_any(message, raw, depth + 1),
        "google.protobuf.Timestamp" | "google.protobuf.Duration" | "google.protobuf.FieldMask" => {
            let text: String = serde_json::from_str(raw.get())?;
            return crate::utility_gateway_wkt::unmarshal_text(message, &text);
        }
        "google.protobuf.Struct" => {
            let field = desc.get_field_by_name("fields").expect("Struct.fields");
            return decode_field_into(message, &BoundField::Regular(field), raw, depth + 1);
        }
        "google.protobuf.ListValue" => {
            let field = desc.get_field_by_name("values").expect("ListValue.values");
            return decode_field_into(message, &BoundField::Regular(field), raw, depth + 1);
        }
        "google.protobuf.Value" => return decode_known_value(message, raw, depth + 1),
        name if is_wrapper(name) => {
            let field = desc.get_field_by_name("value").expect("wrapper.value");
            if let Some(value) = decode_scalar(&field.kind(), raw, depth + 1)? { message.set_field(&field, value); }
            return Ok(());
        }
        _ => {},
    }
    let RawObject(fields) = serde_json::from_str(raw.get())?;
    let mut seen = HashSet::new();
    let mut oneofs = HashSet::new();
    for (name, raw) in fields {
        let field = if name.starts_with('[') && name.ends_with(']') {
            desc.get_extension_by_json_name(&name).map(BoundField::Extension)
        } else {
            desc.get_field_by_json_name(&name).or_else(|| desc.fields().find(|field| text_name(field) == name)).map(BoundField::Regular)
        };
        let Some(field) = field else {
            check_skipped_depth(&raw, MAX_MESSAGE_DEPTH - depth - 1)?;
            continue;
        }; // DiscardUnknown, including unknown enum names below.
        if !seen.insert(field.number()) { return Err(ERRORS.New(format!("duplicate field {name:?}"))); }
        let kind = field.kind();
        let accepts_null = matches!(&kind, Kind::Message(desc) if desc.full_name() == "google.protobuf.Value")
            || matches!(&kind, Kind::Enum(desc) if desc.full_name() == "google.protobuf.NullValue");
        if raw.get() == "null" && !accepts_null { continue; }
        if let BoundField::Regular(regular) = &field {
            if let Some(oneof) = regular.containing_oneof() {
                if !oneofs.insert(oneof.full_name().to_owned()) { return Err(ERRORS.New(format!("oneof {} is already set", oneof.full_name()))); }
            }
        }
        decode_field_into(message, &field, &raw, depth + 1)?;
    }
    Ok(())
}
fn decode_field_into(message: &mut DynamicMessage, field: &BoundField, raw: &RawValue, depth: usize) -> Result<(), BoxError> {
    let kind = field.kind();
    if field.is_list() {
        let Value::List(values) = field.get_mut(message) else { return Err(ERRORS.New("invalid list field storage")); };
        let raws: Vec<Box<RawValue>> = serde_json::from_str(raw.get())?;
        for raw in raws { if let Some(value) = decode_scalar(&kind, &raw, depth)? { values.push(value); } }
        return Ok(());
    }
    if field.is_map() {
        let Kind::Message(entry) = kind else { return Err(ERRORS.New("map entry descriptor missing")); };
        let key_kind = entry.map_entry_key_field().kind();
        let value_kind = entry.map_entry_value_field().kind();
        let Value::Map(values) = field.get_mut(message) else { return Err(ERRORS.New("invalid map field storage")); };
        let RawObject(raws) = serde_json::from_str(raw.get())?;
        for (key, raw) in raws {
            let key = decode_map_key(&key_kind, &key)?;
            if values.contains_key(&key) { return Err(ERRORS.New(format!("duplicate map key {:?}", map_key_text(&key)))); }
            if let Some(value) = decode_scalar(&value_kind, &raw, depth)? { values.insert(key, value); }
        }
        return Ok(());
    }
    if matches!(kind, Kind::Message(_)) {
        let Value::Message(value) = field.get_mut(message) else { return Err(ERRORS.New("invalid message field storage")); };
        return decode_message(value, raw, depth);
    }
    if let Some(value) = decode_scalar(&kind, raw, depth)? { field.set(message, value); }
    Ok(())
}
fn decode_scalar(kind: &Kind, raw: &RawValue, depth: usize) -> Result<Option<Value>, BoxError> {
    let text = raw.get();
    let value = match kind {
        Kind::Message(desc) => { let mut value = DynamicMessage::new(desc.clone()); decode_message(&mut value, raw, depth)?; Value::Message(value) },
        Kind::Bool => Value::Bool(serde_json::from_str(text)?),
        Kind::String => Value::String(serde_json::from_str(text)?),
        Kind::Bytes => {
            let text: String = serde_json::from_str(text)?;
            Value::Bytes(crate::utility_gateway_scalar::decode_proto_bytes(&text)?.into())
        }
        Kind::Int32 | Kind::Sint32 | Kind::Sfixed32 => Value::I32(integer_text(raw, true)?.parse()?),
        Kind::Int64 | Kind::Sint64 | Kind::Sfixed64 => Value::I64(integer_text(raw, true)?.parse()?),
        Kind::Uint32 | Kind::Fixed32 => Value::U32(integer_text(raw, false)?.parse()?),
        Kind::Uint64 | Kind::Fixed64 => Value::U64(integer_text(raw, false)?.parse()?),
        Kind::Float => Value::F32(parse_float32(raw)?),
        Kind::Double => Value::F64(parse_float64(raw)?),
        Kind::Enum(desc) => {
            if text == "null" && desc.full_name() == "google.protobuf.NullValue" { Value::EnumNumber(0) }
            else if text.starts_with('"') {
                let name: String = serde_json::from_str(text)?;
                let Some(value) = desc.get_value_by_name(&name) else { return Ok(None); };
                Value::EnumNumber(value.number())
            } else { Value::EnumNumber(integer_text(raw, true)?.parse()?) }
        }
    };
    Ok(Some(value))
}
fn numeric_text(raw: &RawValue) -> Result<String, BoxError> {
    let text = if raw.get().starts_with('"') { serde_json::from_str::<String>(raw.get())? } else { raw.get().to_owned() };
    if text.trim() != text { return Err(ERRORS.New("invalid whitespace in numeric string")); }
    let _: serde_json::Number = serde_json::from_str(&text)?;
    Ok(text)
}
fn integer_text(raw: &RawValue, signed: bool) -> Result<String, BoxError> {
    // Source json.normalizeToIntString, including its zero shortcut and signed
    // 32-bit exponent bound. No integer is rounded through a float.
    let text = numeric_text(raw)?;
    let negative = text.starts_with('-');
    let absolute = text.strip_prefix('-').unwrap_or(&text);
    let (mantissa, exponent) = absolute.split_once(['e', 'E']).unwrap_or((absolute, "0"));
    let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let integer = if integer == "0" { "" } else { integer };
    let fraction = fraction.trim_end_matches('0');
    if integer.is_empty() && fraction.is_empty() { return Ok("0".into()); }
    if negative && !signed { return Err(ERRORS.New("negative unsigned integer")); }
    let exponent: i32 = exponent.parse()?;
    let mut digits;
    if exponent >= 0 {
        let exponent = exponent as usize;
        if fraction.len() > exponent { return Err(ERRORS.New("non-integral protobuf integer")); }
        if integer.len().saturating_add(exponent) > 20 { return Err(ERRORS.New("protobuf integer out of range")); }
        digits = String::from(integer); digits.push_str(fraction);
        digits.extend(std::iter::repeat_n('0', exponent - fraction.len()));
    } else {
        if !fraction.is_empty() { return Err(ERRORS.New("non-integral protobuf integer")); }
        let keep = integer.len().checked_sub(exponent.unsigned_abs() as usize)
            .ok_or_else(|| ERRORS.New("non-integral protobuf integer"))?;
        if !integer[keep..].bytes().all(|byte| byte == b'0') { return Err(ERRORS.New("non-integral protobuf integer")); }
        digits = integer[..keep].to_owned();
    }
    Ok(format!("{}{digits}", if negative { "-" } else { "" }))
}
fn float_text(raw: &RawValue) -> Result<String, BoxError> {
    if raw.get().starts_with('"') {
        let value: String = serde_json::from_str(raw.get())?;
        if matches!(value.as_str(), "NaN" | "Infinity" | "-Infinity") { return Ok(value); }
    }
    numeric_text(raw)
}
fn parse_float32(raw: &RawValue) -> Result<f32, BoxError> {
    let text = float_text(raw)?;
    match text.as_str() { "NaN" => Ok(f32::from_bits(0x7fc0_0000)), "Infinity" => Ok(f32::INFINITY), "-Infinity" => Ok(f32::NEG_INFINITY), _ => {
        let value: f32 = text.parse()?;
        if value.is_finite() { Ok(value) } else { Err(ERRORS.New("protobuf float out of range")) }
    } }
}
fn parse_float64(raw: &RawValue) -> Result<f64, BoxError> {
    let text = float_text(raw)?;
    match text.as_str() { "NaN" => Ok(f64::from_bits(0x7ff8_0000_0000_0001)), "Infinity" => Ok(f64::INFINITY), "-Infinity" => Ok(f64::NEG_INFINITY), _ => {
        let value: f64 = text.parse()?;
        if value.is_finite() { Ok(value) } else { Err(ERRORS.New("protobuf double out of range")) }
    } }
}
fn decode_map_key(kind: &Kind, text: &str) -> Result<MapKey, BoxError> {
    Ok(match kind {
        Kind::String => MapKey::String(text.to_owned()),
        Kind::Bool => MapKey::Bool(match text { "true" => true, "false" => false, _ => return Err(ERRORS.New("invalid boolean map key")) }),
        Kind::Int32 | Kind::Sint32 | Kind::Sfixed32 => MapKey::I32(crate::utility_gateway_scalar::parse_signed(text, 10, 32)? as i32),
        Kind::Int64 | Kind::Sint64 | Kind::Sfixed64 => MapKey::I64(crate::utility_gateway_scalar::parse_signed(text, 10, 64)?),
        Kind::Uint32 | Kind::Fixed32 => MapKey::U32(crate::utility_gateway_scalar::parse_unsigned(text, 10, 32)? as u32),
        Kind::Uint64 | Kind::Fixed64 => MapKey::U64(crate::utility_gateway_scalar::parse_unsigned(text, 10, 64)?),
        _ => return Err(ERRORS.New("invalid protobuf map key kind")),
    })
}
fn decode_known_value(message: &mut DynamicMessage, raw: &RawValue, depth: usize) -> Result<(), BoxError> {
    let name = match raw.get().as_bytes().first() {
        Some(b'n') => "null_value", Some(b't' | b'f') => "bool_value", Some(b'"') => "string_value",
        Some(b'{') => "struct_value", Some(b'[') => "list_value", _ => "number_value",
    };
    let field = message.descriptor().get_field_by_name(name).expect("Value oneof field");
    if name == "number_value" && raw.get().starts_with('"') { return Err(ERRORS.New("invalid Value number")); }
    decode_field_into(message, &BoundField::Regular(field), raw, depth)
}
fn decode_any(message: &mut DynamicMessage, raw: &RawValue, depth: usize) -> Result<(), BoxError> {
    let RawObject(fields) = serde_json::from_str(raw.get())?;
    let mut type_url = None;
    for (name, value) in &fields {
        if name == "@type" {
            if type_url.is_some() { return Err(ERRORS.New("duplicate @type field")); }
            let value: String = serde_json::from_str(value.get())?;
            if value.is_empty() { return Err(ERRORS.New("@type field contains empty value")); }
            type_url = Some(value);
        }
    }
    let Some(type_url) = type_url else {
        check_skipped_depth(raw, MAX_MESSAGE_DEPTH.saturating_sub(depth))?;
        return Ok(());
    }; // All unknown fields are discarded.
    let desc = message.descriptor();
    let payload_desc = resolve_type(&desc, &type_url)?;
    let mut payload = DynamicMessage::new(payload_desc.clone());
    let empty_payload = payload_desc.full_name() == "google.protobuf.Empty";
    if has_custom_json(payload_desc.full_name()) || empty_payload {
        let mut value = None;
        for (name, raw) in &fields {
            if name == "value" {
                if value.is_some() { return Err(ERRORS.New("duplicate Any value field")); }
                value = Some(raw);
            }
        }
        if let Some(value) = value { decode_message(&mut payload, value, depth)?; }
        else if !empty_payload { return Err(ERRORS.New("missing Any value field")); }
    } else {
        let values: Vec<_> = fields.iter().filter(|(name, _)| name != "@type").map(|(name, value)| (name, value)).collect();
        // Serialize the entry sequence as an object while retaining duplicate
        // source keys for decode_message's schema-aware duplicate check.
        struct ObjectEntries<'a>(&'a [(&'a String, &'a Box<RawValue>)]);
        impl Serialize for ObjectEntries<'_> {
            fn serialize<S: Serializer>(&self, out: S) -> Result<S::Ok, S::Error> {
                let mut map = out.serialize_map(Some(self.0.len()))?;
                for (name, value) in self.0 { map.serialize_entry(name, value)?; }
                map.end()
            }
        }
        let json = serde_json::to_string(&ObjectEntries(&values))?;
        let raw = RawValue::from_string(json)?;
        decode_message(&mut payload, &raw, depth)?;
    }
    message.set_field_by_name("type_url", Value::String(type_url));
    message.set_field_by_name("value", Value::Bytes(crate::utility_gateway_wire::encode_any_payload(&payload)?.into()));
    Ok(())
}
fn check_skipped_depth(raw: &RawValue, limit: usize) -> Result<(), BoxError> {
    let mut open = 0_usize; let mut quoted = false; let mut escaped = false;
    for byte in raw.get().bytes() {
        if quoted {
            if escaped { escaped = false; }
            else if byte == b'\\' { escaped = true; }
            else if byte == b'"' { quoted = false; }
        } else {
            match byte {
                b'"' => quoted = true,
                b'{' | b'[' => { open += 1; if open > limit { return Err(ERRORS.New("exceeded max recursion depth")); } },
                b'}' | b']' => open -= 1,
                _ => {},
            }
        }
    }
    Ok(())
}
