//! Statically typed destinations for JSONPb's non-message field decoder.
//! These are gateway wire categories, not a general Go object/reflection layer.
//! The caller chooses a concrete type and pure element factory; no registry,
//! service lookup, effect implementation selection or application task is added.
use std::collections::HashMap;
use prost_reflect::{DynamicMessage, EnumDescriptor, MessageDescriptor};
use serde_json::value::RawValue;
use crate::{errors::{BoxError, Errors}, utility_gateway_json::{GatewayJsonPb, RawObject}, utility_gateway_scalar as scalar};

const ERRORS: Errors = Errors::from_package("github.com/grpc-ecosystem/grpc-gateway/v2/runtime");

pub trait GatewayDecodeTarget {
    /// Source pointer/map allocation occurs before the outer JSON Decode.
    fn prepare_gateway_decode(&mut self) -> Result<(), BoxError> { Ok(()) }
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError>;
    /// The encoding/json fallback sees the original pointer. Container, enum
    /// and protobuf branches instead operate on its already dereferenced value.
    fn json_null_clears_pointer(&self) -> bool { true }
}
impl GatewayDecodeTarget for DynamicMessage {
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> { GatewayJsonPb.unmarshal_raw(raw, self) }
    fn json_null_clears_pointer(&self) -> bool { false }
}
macro_rules! primitive_targets {
    ($($kind:ty),* $(,)?) => { $(
        impl GatewayDecodeTarget for $kind {
            fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
                if raw.get() != "null" { *self = serde_json::from_str(raw.get())?; }
                Ok(())
            }
        }
    )* };
}
primitive_targets!(bool, String, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
impl GatewayDecodeTarget for f32 {
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
        if raw.get() != "null" {
            let _: serde_json::Number = serde_json::from_str(raw.get())?;
            *self = scalar::parse_float(raw.get(), 32)? as f32;
        }
        Ok(())
    }
}
impl GatewayDecodeTarget for f64 {
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
        if raw.get() != "null" {
            let _: serde_json::Number = serde_json::from_str(raw.get())?;
            *self = scalar::parse_float(raw.get(), 64)?;
        }
        Ok(())
    }
}
impl GatewayDecodeTarget for serde_json::Value {
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
        let mut first_error = None;
        *self = decode_interface(raw, &mut first_error)?;
        if let Some(error) = first_error { Err(error) } else { Ok(()) }
    }
}
fn decode_interface(raw: &RawValue, first_error: &mut Option<BoxError>) -> Result<serde_json::Value, BoxError> {
    use serde_json::Value;
    Ok(match raw.get().as_bytes().first() {
        Some(b'{') => {
            let RawObject(fields) = serde_json::from_str(raw.get())?; let mut object = serde_json::Map::new();
            for (key, raw) in fields { object.insert(key, decode_interface(&raw, first_error)?); }
            Value::Object(object)
        }
        Some(b'[') => {
            let items: Vec<Box<RawValue>> = serde_json::from_str(raw.get())?; let mut array = Vec::with_capacity(items.len());
            for raw in items { array.push(decode_interface(&raw, first_error)?); }
            Value::Array(array)
        }
        Some(b'-' | b'0'..=b'9') => match scalar::parse_float(raw.get(), 64) {
            Ok(value) => Value::Number(serde_json::Number::from_f64(value).expect("JSON numbers are finite")),
            Err(error) => { if first_error.is_none() { *first_error = Some(Box::new(error)); } Value::Null },
        },
        _ => serde_json::from_str(raw.get())?,
    })
}

/// Pure data construction, analogous to a DeserializeSeed. Factories retain
/// immutable protobuf descriptors when a Rust Default cannot name the schema.
pub trait GatewayTargetFactory<T> { fn new_target(&self) -> T; }
#[derive(Clone, Copy, Default)]
pub struct DefaultTargetFactory;
impl<T: Default> GatewayTargetFactory<T> for DefaultTargetFactory { fn new_target(&self) -> T { T::default() } }
#[derive(Clone)]
pub struct MessageTargetFactory(pub MessageDescriptor);
impl GatewayTargetFactory<DynamicMessage> for MessageTargetFactory {
    fn new_target(&self) -> DynamicMessage { DynamicMessage::new(self.0.clone()) }
}

pub struct GatewayPointer<T, Factory = DefaultTargetFactory> { pub value: Option<Box<T>>, pub factory: Factory }
impl<T, Factory: Default> Default for GatewayPointer<T, Factory> {
    fn default() -> Self { Self { value: None, factory: Factory::default() } }
}
impl<T: GatewayDecodeTarget, Factory: GatewayTargetFactory<T>> GatewayDecodeTarget for GatewayPointer<T, Factory> {
    fn prepare_gateway_decode(&mut self) -> Result<(), BoxError> {
        if self.value.is_none() { self.value = Some(Box::new(self.factory.new_target())); }
        self.value.as_mut().expect("prepared pointer").prepare_gateway_decode()
    }
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
        self.prepare_gateway_decode()?;
        if raw.get() == "null" && self.value.as_ref().expect("prepared pointer").json_null_clears_pointer() { self.value = None; return Ok(()); }
        self.value.as_mut().expect("prepared pointer").decode_gateway_raw(raw)
    }
    fn json_null_clears_pointer(&self) -> bool {
        self.value.as_ref().expect("pointer prepared before null selection").json_null_clears_pointer()
    }
}

pub struct GatewaySlice<T, Factory = DefaultTargetFactory> { pub values: Option<Vec<T>>, pub factory: Factory }
impl<T, Factory: Default> Default for GatewaySlice<T, Factory> {
    fn default() -> Self { Self { values: None, factory: Factory::default() } }
}
impl<T: GatewayDecodeTarget, Factory: GatewayTargetFactory<T>> GatewayDecodeTarget for GatewaySlice<T, Factory> {
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
        // Null does not clear an existing slice in decodeNonProtoField.
        if raw.get() == "null" { return Ok(()); }
        let values: Vec<Box<RawValue>> = serde_json::from_str(raw.get())?;
        self.values = Some(Vec::new());
        for raw in values {
            let mut value = self.factory.new_target(); value.prepare_gateway_decode()?; value.decode_gateway_raw(&raw)?;
            self.values.as_mut().expect("initialized slice").push(value);
        }
        Ok(())
    }
    fn json_null_clears_pointer(&self) -> bool { false }
}
#[derive(Clone, Debug, Default)]
pub struct GatewayBytes(pub Option<Vec<u8>>);
impl GatewayDecodeTarget for GatewayBytes {
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
        if raw.get() == "null" { return Ok(()); }
        let bytes = if raw.get().starts_with('"') {
            scalar::decode_json_bytes(&serde_json::from_str::<String>(raw.get())?)?
        } else {
            let values: Vec<Box<RawValue>> = serde_json::from_str(raw.get())?;
            let mut bytes = Vec::with_capacity(values.len());
            for raw in values { let mut value = 0_u8; value.decode_gateway_raw(&raw)?; bytes.push(value); }
            bytes
        };
        // The source decodes into a temporary []byte before replacing rv.
        self.0 = Some(bytes); Ok(())
    }
    fn json_null_clears_pointer(&self) -> bool { false }
}

#[derive(Clone, Debug)]
pub struct GatewayEnum { pub descriptor: EnumDescriptor, pub source_type: String, pub number: i32, pub cast_profile: EnumCastProfile }
#[derive(Clone)]
pub struct EnumTargetFactory { pub descriptor: EnumDescriptor, pub source_type: String, pub cast_profile: EnumCastProfile }
impl GatewayTargetFactory<GatewayEnum> for EnumTargetFactory {
    fn new_target(&self) -> GatewayEnum { GatewayEnum { descriptor: self.descriptor.clone(), source_type: self.source_type.clone(), number: 0, cast_profile: self.cast_profile } }
}
impl GatewayDecodeTarget for GatewayEnum {
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
        if raw.get().starts_with('"') {
            let name: String = serde_json::from_str(raw.get())?;
            return Err(ERRORS.New(format!("unmarshaling of symbolic enum {name:?} not supported: {}", self.source_type)));
        }
        // The source first decodes interface{} into float64, then int32(v).
        let mut number = 0_f64;
        if raw.get() == "null" || raw.get().starts_with(['[', '{', 't', 'f']) {
            return Err(ERRORS.New(format!("cannot assign {} into Go type {}", raw.get(), self.source_type)));
        }
        number.decode_gateway_raw(raw)?;
        self.number = self.cast_profile.cast(number);
        Ok(())
    }
    fn json_null_clears_pointer(&self) -> bool { false }
}
/// Immutable source numeric profile, selected explicitly with the source tool
/// identity when constructing an enum field. It is not chosen from Rust's host
/// architecture: Go 1.27 AMD64 compiler rules have two ConvertHash branches.
#[derive(Clone, Copy, Debug)]
pub enum EnumCastProfile { X86Cvtt, Amd64Go127Saturating, Arm64Fcvtzs }
impl EnumCastProfile {
    pub fn cast(self, value: f64) -> i32 {
        match self {
            Self::X86Cvtt => if !(-2_147_483_648.0..2_147_483_648.0).contains(&value) { i32::MIN } else { value as i32 },
            Self::Amd64Go127Saturating if value.is_nan() => if value.is_sign_negative() { i32::MIN } else { i32::MAX },
            Self::Amd64Go127Saturating | Self::Arm64Fcvtzs => value as i32,
        }
    }
}

/// Go-compatible equality is explicit, so float keys need not lie about Rust Eq.
pub trait GatewayMapKey: Sized {
    fn validate_key_kind() -> Result<(), BoxError> { Ok(()) }
    fn parse_key(text: &str) -> Result<Self, BoxError>;
    fn key_equal(&self, other: &Self) -> bool;
}
impl GatewayMapKey for String {
    fn parse_key(text: &str) -> Result<Self, BoxError> { Ok(text.into()) }
    fn key_equal(&self, other: &Self) -> bool { self == other }
}
impl GatewayMapKey for bool {
    fn parse_key(text: &str) -> Result<Self, BoxError> { Ok(scalar::parse_bool(text)?) }
    fn key_equal(&self, other: &Self) -> bool { self == other }
}
macro_rules! integer_keys {
    ($parser:ident; $($kind:ty => $bits:literal),* $(,)?) => { $(
        impl GatewayMapKey for $kind {
            fn parse_key(text: &str) -> Result<Self, BoxError> { Ok(scalar::$parser(text, 0, $bits)? as Self) }
            fn key_equal(&self, other: &Self) -> bool { self == other }
        }
    )* };
}
integer_keys!(parse_signed; i32 => 32, i64 => 64);
integer_keys!(parse_unsigned; u32 => 32, u64 => 64);
impl GatewayMapKey for f32 {
    fn parse_key(text: &str) -> Result<Self, BoxError> { Ok(scalar::parse_float(text, 32)? as f32) }
    fn key_equal(&self, other: &Self) -> bool { self == other }
}
impl GatewayMapKey for f64 {
    fn parse_key(text: &str) -> Result<Self, BoxError> { Ok(scalar::parse_float(text, 64)?) }
    fn key_equal(&self, other: &Self) -> bool { self == other }
}
macro_rules! unsupported_keys {
    ($($kind:ty => $source:literal),* $(,)?) => { $(
        impl GatewayMapKey for $kind {
            fn validate_key_kind() -> Result<(), BoxError> { Err(ERRORS.New(concat!("unsupported type of map field key: ", $source))) }
            fn parse_key(_text: &str) -> Result<Self, BoxError> { Self::validate_key_kind()?; unreachable!("source rejects this map key kind") }
            fn key_equal(&self, other: &Self) -> bool { self == other }
        }
    )* };
}
unsupported_keys!(i8 => "int8", i16 => "int16", isize => "int", u8 => "uint8", u16 => "uint16", usize => "uint");

pub struct GatewayMap<Key, Value, Factory = DefaultTargetFactory> { pub entries: Option<Vec<(Key, Value)>>, pub factory: Factory }
impl<Key, Value, Factory: Default> Default for GatewayMap<Key, Value, Factory> {
    fn default() -> Self { Self { entries: None, factory: Factory::default() } }
}
impl<Key: GatewayMapKey, Value: GatewayDecodeTarget, Factory: GatewayTargetFactory<Value>> GatewayDecodeTarget for GatewayMap<Key, Value, Factory> {
    fn prepare_gateway_decode(&mut self) -> Result<(), BoxError> {
        if self.entries.is_none() { self.entries = Some(Vec::new()); }
        Key::validate_key_kind()
    }
    fn decode_gateway_raw(&mut self, raw: &RawValue) -> Result<(), BoxError> {
        self.prepare_gateway_decode()?;
        if raw.get() == "null" { return Ok(()); }
        let RawObject(fields) = serde_json::from_str(raw.get())?;
        // The source first decodes map[string]*RawMessage: duplicate names use
        // the last JSON value, then traversal has no insertion-order contract.
        let fields: HashMap<String, Box<RawValue>> = fields.into_iter().collect();
        for (text, raw) in fields {
            let key = Key::parse_key(&text)?;
            let mut value = self.factory.new_target(); value.prepare_gateway_decode()?; value.decode_gateway_raw(&raw)?;
            let entries = self.entries.as_mut().expect("prepared map");
            if let Some((_, old)) = entries.iter_mut().find(|(candidate, _)| candidate.key_equal(&key)) { *old = value; }
            else { entries.push((key, value)); }
        }
        Ok(())
    }
    fn json_null_clears_pointer(&self) -> bool { false }
}
