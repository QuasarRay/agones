//! Values carried by the source `TypedRateLimitingInterface[any]`.
//! Hash buckets use Go-style equality explicitly, allowing non-reflexive
//! comparable values (for example NaN) without violating Rust's Eq contract.
use std::{any::Any, collections::HashMap, fmt, hash::{Hash, Hasher}, sync::Arc};
use serde::{Serialize, Serializer};
use serde_json::Value;
use crate::{errors::{BoxError, Errors}, logfields::LogEntry, runtime::SourceLogType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct QueueKey(pub Vec<u8>);
impl From<&str> for QueueKey { fn from(value: &str) -> Self { Self(value.as_bytes().to_vec()) } }
impl From<String> for QueueKey { fn from(value: String) -> Self { Self(value.into_bytes()) } }
impl From<Vec<u8>> for QueueKey { fn from(value: Vec<u8>) -> Self { Self(value) } }
impl fmt::Debug for QueueKey { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(out) } }
impl fmt::Display for QueueKey { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { String::from_utf8_lossy(&self.0).fmt(out) } }
impl Serialize for QueueKey { fn serialize<S: Serializer>(&self, output: S) -> Result<S::Ok, S::Error> { output.serialize_str(&String::from_utf8_lossy(&self.0)) } }
impl SourceLogType for QueueKey { const SOURCE_LOG_TYPE: &'static str = "string"; }

trait QueueValue: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn hash_value(&self) -> u64;
    fn equals(&self, other: &dyn QueueValue) -> bool;
    fn source_type(&self) -> &'static str;
    fn log_value(&self) -> Result<Value, serde_json::Error>;
}
struct Comparable<T>(T);
impl<T> QueueValue for Comparable<T>
where T: Any + Send + Sync + PartialEq + Hash + Serialize + SourceLogType,
{
    fn as_any(&self) -> &dyn Any { self }
    fn hash_value(&self) -> u64 {
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        std::any::TypeId::of::<T>().hash(&mut hash); self.0.hash(&mut hash); hash.finish()
    }
    fn equals(&self, other: &dyn QueueValue) -> bool { other.as_any().downcast_ref::<Self>().is_some_and(|other| self.0 == other.0) }
    fn source_type(&self) -> &'static str { T::SOURCE_LOG_TYPE }
    fn log_value(&self) -> Result<Value, serde_json::Error> { serde_json::to_value(&self.0) }
}
struct Unhashable<T>(T);
impl<T> QueueValue for Unhashable<T> where T: Any + Send + Sync + Serialize + SourceLogType {
    fn as_any(&self) -> &dyn Any { self }
    fn hash_value(&self) -> u64 { panic!("hash of unhashable type {}", T::SOURCE_LOG_TYPE) }
    fn equals(&self, _other: &dyn QueueValue) -> bool { panic!("comparing uncomparable type {}", T::SOURCE_LOG_TYPE) }
    fn source_type(&self) -> &'static str { T::SOURCE_LOG_TYPE }
    fn log_value(&self) -> Result<Value, serde_json::Error> { serde_json::to_value(&self.0) }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize)]
struct NilInterface;
impl SourceLogType for NilInterface { const SOURCE_LOG_TYPE: &'static str = "<nil>"; }

#[derive(PartialEq)]
struct Float64(f64);
impl Hash for Float64 { fn hash<H: Hasher>(&self, state: &mut H) { (if self.0 == 0.0 { 0 } else { self.0.to_bits() }).hash(state); } }
impl SourceLogType for Float64 { const SOURCE_LOG_TYPE: &'static str = "float64"; }
impl Serialize for Float64 {
    fn serialize<S: Serializer>(&self, output: S) -> Result<S::Ok, S::Error> {
        if self.0.is_finite() { output.serialize_f64(self.0) } else { Err(serde::ser::Error::custom("json: unsupported non-finite float64 value")) }
    }
}

#[derive(Clone)]
pub struct QueueItem(Arc<dyn QueueValue>);
impl QueueItem {
    pub fn string(value: impl Into<QueueKey>) -> Self { Self::comparable(value.into()) }
    pub fn nil() -> Self { Self::comparable(NilInterface) }
    pub fn float64(value: f64) -> Self { Self::comparable(Float64(value)) }
    pub fn comparable<T>(value: T) -> Self
    where T: Any + Send + Sync + PartialEq + Hash + Serialize + SourceLogType { Self(Arc::new(Comparable(value))) }
    pub fn unhashable<T>(value: T) -> Self
    where T: Any + Send + Sync + Serialize + SourceLogType { Self(Arc::new(Unhashable(value))) }
    pub fn string_key(&self) -> Option<&QueueKey> { self.0.as_any().downcast_ref::<Comparable<QueueKey>>().map(|value| &value.0) }
    pub fn source_type(&self) -> &'static str { self.0.source_type() }
    pub fn with_log_field(&self, logger: &LogEntry, field: &str) -> LogEntry {
        match self.0.log_value() {
            Ok(value) => logger.with_field(field, value),
            Err(_) => logger.with_unserializable_field(self.source_type()),
        }
    }
    fn hash(&self) -> u64 { self.0.hash_value() }
    fn equals(&self, other: &Self) -> bool { self.0.equals(other.0.as_ref()) }
}
impl fmt::Debug for QueueItem {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.debug_struct("QueueItem").field("source_type", &self.source_type()).finish_non_exhaustive() }
}

pub(crate) struct ItemMap<V> { buckets: HashMap<u64, Vec<(QueueItem, V)>>, length: usize }
impl<V> Default for ItemMap<V> { fn default() -> Self { Self { buckets: HashMap::new(), length: 0 } } }
impl<V> ItemMap<V> {
    pub fn len(&self) -> usize { self.length }
    pub fn contains(&self, item: &QueueItem) -> bool { self.get(item).is_some() }
    pub fn get(&self, item: &QueueItem) -> Option<&V> { self.buckets.get(&item.hash())?.iter().find(|(key, _)| key.equals(item)).map(|(_, value)| value) }
    pub fn insert(&mut self, item: QueueItem, value: V) -> Option<V> {
        let bucket = self.buckets.entry(item.hash()).or_default();
        if let Some((_, previous)) = bucket.iter_mut().find(|(key, _)| key.equals(&item)) { return Some(std::mem::replace(previous, value)); }
        bucket.push((item, value)); self.length += 1; None
    }
    pub fn remove(&mut self, item: &QueueItem) -> Option<V> {
        let hash = item.hash(); let bucket = self.buckets.get_mut(&hash)?;
        let index = bucket.iter().position(|(key, _)| key.equals(item))?;
        let (_, value) = bucket.swap_remove(index); self.length -= 1;
        if bucket.is_empty() { self.buckets.remove(&hash); } Some(value)
    }
}

pub trait QueueMetadata: Send + Sync {
    fn get_namespace(&self) -> QueueKey;
    fn get_name(&self) -> QueueKey;
}
pub trait QueueObject: Send + Sync {
    fn explicit_key(&self) -> Option<QueueKey> { None }
    fn metadata(&self) -> Option<Arc<dyn QueueMetadata>> { None }
    fn log_object(&self, logger: &LogEntry) -> LogEntry;
}
pub struct ExplicitKey(pub QueueKey);
impl QueueObject for ExplicitKey {
    fn explicit_key(&self) -> Option<QueueKey> { Some(self.0.clone()) }
    fn log_object(&self, logger: &LogEntry) -> LogEntry { logger.with_field("obj", String::from_utf8_lossy(&self.0.0).into_owned()) }
}
impl QueueObject for QueueItem { fn log_object(&self, logger: &LogEntry) -> LogEntry { self.with_log_field(logger, "obj") } }
pub struct MetadataObject { pub metadata: Arc<dyn QueueMetadata>, pub log_value: Value }
impl QueueObject for MetadataObject {
    fn metadata(&self) -> Option<Arc<dyn QueueMetadata>> { Some(Arc::clone(&self.metadata)) }
    fn log_object(&self, logger: &LogEntry) -> LogEntry { logger.with_field("obj", self.log_value.clone()) }
}
pub fn meta_namespace_key(object: &dyn QueueObject) -> Result<QueueKey, BoxError> {
    if let Some(key) = object.explicit_key() { return Ok(key); }
    let metadata = object.metadata().ok_or_else(|| Errors::default().New("object has no meta: object does not implement the Object interfaces"))?;
    // Match MetaObjectToName's getter order, including its second namespace
    // read when the first is nonempty. Metadata getters may have effects.
    if !metadata.get_namespace().0.is_empty() {
        let mut namespace = metadata.get_namespace().0; namespace.push(b'/'); namespace.extend(metadata.get_name().0); Ok(QueueKey(namespace))
    } else { Ok(metadata.get_name()) }
}
