//! Native resource keys and the Agones metadata-to-key boundary.
//! Queue storage accepts only owned resource keys, so invalid dynamic values
//! cannot enter the retry, dirty or processing sets.
use std::{fmt, sync::Arc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{errors::{BoxError, Errors}, logfields::LogEntry};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QueueKey(pub Arc<str>);
impl From<&str> for QueueKey { fn from(value: &str) -> Self { Self(Arc::from(value)) } }
impl From<String> for QueueKey { fn from(value: String) -> Self { Self(Arc::from(value)) } }
impl AsRef<str> for QueueKey { fn as_ref(&self) -> &str { &self.0 } }
impl fmt::Display for QueueKey {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result { output.write_str(&self.0) }
}
impl QueueKey {
    pub fn with_log_field(&self, logger: &LogEntry, field: &str) -> LogEntry { logger.with_field(field, self.as_ref()) }
}

pub trait QueueMetadata: Send + Sync {
    fn get_namespace(&self) -> &str;
    fn get_name(&self) -> &str;
}
pub trait QueueObject: Send + Sync {
    fn explicit_key(&self) -> Option<QueueKey> { None }
    fn metadata(&self) -> Option<Arc<dyn QueueMetadata>> { None }
    fn log_object(&self, logger: &LogEntry) -> LogEntry;
}
pub struct ExplicitKey(pub QueueKey);
impl QueueObject for ExplicitKey {
    fn explicit_key(&self) -> Option<QueueKey> { Some(self.0.clone()) }
    fn log_object(&self, logger: &LogEntry) -> LogEntry { self.0.with_log_field(logger, "obj") }
}
impl QueueObject for QueueKey {
    fn explicit_key(&self) -> Option<QueueKey> { Some(self.clone()) }
    fn log_object(&self, logger: &LogEntry) -> LogEntry { self.with_log_field(logger, "obj") }
}
pub struct MetadataObject { pub metadata: Arc<dyn QueueMetadata>, pub log_value: Value }
impl QueueObject for MetadataObject {
    fn metadata(&self) -> Option<Arc<dyn QueueMetadata>> { Some(Arc::clone(&self.metadata)) }
    fn log_object(&self, logger: &LogEntry) -> LogEntry { logger.with_field("obj", self.log_value.clone()) }
}
pub fn meta_namespace_key(object: &dyn QueueObject) -> Result<QueueKey, BoxError> {
    if let Some(key) = object.explicit_key() { return Ok(key); }
    let metadata = object.metadata().ok_or_else(|| Errors::default().New("object has no resource metadata or explicit key"))?;
    let namespace = metadata.get_namespace();
    let name = metadata.get_name();
    if namespace.is_empty() { Ok(name.into()) } else { Ok(format!("{namespace}/{name}").into()) }
}
