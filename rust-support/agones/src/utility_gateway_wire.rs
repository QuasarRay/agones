//! Deterministic protobuf payload emission for JSON-decoded google.protobuf.Any.
//! prost-reflect's map storage is a HashMap; its ordinary Message encoder does
//! not provide the source's Deterministic map-key ordering option.
use prost_reflect::{DynamicMessage, Kind, MapKey, Value};
use crate::errors::{BoxError, Errors};

const ERRORS: Errors = Errors::from_package("google.golang.org/protobuf/proto");

pub(crate) fn encode_any_payload(message: &DynamicMessage) -> Result<Vec<u8>, BoxError> {
    let mut bytes = Vec::new();
    // Go emits extensions before the message's ordinary fields. JSON decoding
    // has already removed unknown fields, while nested Any wire bytes are data.
    let mut extensions: Vec<_> = message.extensions().collect();
    extensions.sort_by_key(|(field, _)| field.number());
    for (field, value) in extensions { encode_field(&mut bytes, field.number(), &field.kind(), field.is_packed(), field.is_group(), value)?; }
    let mut fields: Vec<_> = message.fields().collect();
    fields.sort_by_key(|(field, _)| field.number());
    for (field, value) in fields { encode_field(&mut bytes, field.number(), &field.kind(), field.is_packed(), field.is_group(), value)?; }
    for field in message.unknown_fields() { field.encode(&mut bytes); }
    Ok(bytes)
}
fn encode_field(bytes: &mut Vec<u8>, number: u32, kind: &Kind, packed: bool, group: bool, value: &Value) -> Result<(), BoxError> {
    match value {
        Value::List(values) => {
            if packed && !values.is_empty() {
                let mut payload = Vec::new();
                for value in values { encode_scalar(&mut payload, kind, value)?; }
                length_delimited(bytes, number, &payload);
            } else { for value in values { encode_field(bytes, number, kind, false, group, value)?; } }
        }
        Value::Map(values) => {
            let Kind::Message(entry) = kind else { return Err(ERRORS.New("map entry descriptor missing")); };
            let key_field = entry.map_entry_key_field(); let value_field = entry.map_entry_value_field();
            let mut entries: Vec<_> = values.iter().collect();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (key, value) in entries {
                let mut payload = Vec::new();
                let key = match key {
                    MapKey::Bool(value) => Value::Bool(*value), MapKey::I32(value) => Value::I32(*value),
                    MapKey::I64(value) => Value::I64(*value), MapKey::U32(value) => Value::U32(*value),
                    MapKey::U64(value) => Value::U64(*value), MapKey::String(value) => Value::String(value.clone()),
                };
                encode_field(&mut payload, key_field.number(), &key_field.kind(), false, false, &key)?;
                encode_field(&mut payload, value_field.number(), &value_field.kind(), false, value_field.is_group(), value)?;
                length_delimited(bytes, number, &payload);
            }
        }
        Value::Message(message) => {
            let payload = encode_any_payload(message)?;
            if group { key(bytes, number, 3); bytes.extend_from_slice(&payload); key(bytes, number, 4); }
            else { length_delimited(bytes, number, &payload); }
        }
        Value::String(value) => length_delimited(bytes, number, value.as_bytes()),
        Value::Bytes(value) => length_delimited(bytes, number, value),
        _ => {
            let wire = match kind { Kind::Double | Kind::Fixed64 | Kind::Sfixed64 => 1, Kind::Float | Kind::Fixed32 | Kind::Sfixed32 => 5, _ => 0 };
            key(bytes, number, wire); encode_scalar(bytes, kind, value)?;
        }
    }
    Ok(())
}
fn encode_scalar(bytes: &mut Vec<u8>, kind: &Kind, value: &Value) -> Result<(), BoxError> {
    match (kind, value) {
        (Kind::Bool, Value::Bool(value)) => varint(bytes, u64::from(*value)),
        (Kind::Int32, Value::I32(value)) | (Kind::Enum(_), Value::EnumNumber(value)) => varint(bytes, *value as i64 as u64),
        (Kind::Int64, Value::I64(value)) => varint(bytes, *value as u64),
        (Kind::Uint32, Value::U32(value)) => varint(bytes, u64::from(*value)),
        (Kind::Uint64, Value::U64(value)) => varint(bytes, *value),
        (Kind::Sint32, Value::I32(value)) => varint(bytes, u64::from((value.wrapping_shl(1) ^ (value >> 31)) as u32)),
        (Kind::Sint64, Value::I64(value)) => varint(bytes, (value.wrapping_shl(1) ^ (value >> 63)) as u64),
        (Kind::Fixed32, Value::U32(value)) => bytes.extend_from_slice(&value.to_le_bytes()),
        (Kind::Sfixed32, Value::I32(value)) => bytes.extend_from_slice(&value.to_le_bytes()),
        (Kind::Fixed64, Value::U64(value)) => bytes.extend_from_slice(&value.to_le_bytes()),
        (Kind::Sfixed64, Value::I64(value)) => bytes.extend_from_slice(&value.to_le_bytes()),
        (Kind::Float, Value::F32(value)) => bytes.extend_from_slice(&value.to_bits().to_le_bytes()),
        (Kind::Double, Value::F64(value)) => bytes.extend_from_slice(&value.to_bits().to_le_bytes()),
        _ => return Err(ERRORS.New("invalid protobuf scalar/descriptor pair")),
    }
    Ok(())
}
fn key(bytes: &mut Vec<u8>, number: u32, wire: u64) { varint(bytes, (u64::from(number) << 3) | wire); }
fn length_delimited(bytes: &mut Vec<u8>, number: u32, value: &[u8]) {
    key(bytes, number, 2); varint(bytes, value.len() as u64); bytes.extend_from_slice(value);
}
fn varint(bytes: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 { bytes.push(value as u8 | 0x80); value >>= 7; }
    bytes.push(value as u8);
}
