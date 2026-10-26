// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/logfields/logfields.go. Copyright Contributors to Agones a Series
// of LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::{borrow::Cow, collections::BTreeMap, fmt};
use cgp::prelude::*;
use derive_deftly::Deftly;
use serde_json::Value;
use crate::utility_declarations::derive_deftly_template_SourceErrorOrigin;

pub const NilGameServer: &str = "NilGameServer";

#[derive(Clone, Debug, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "agones.dev/agones/pkg/util/logfields", source_struct = "ResourceType")]
pub struct ResourceType(pub Cow<'static, str>);
pub const GameServerKey: ResourceType = ResourceType(Cow::Borrowed("gsKey"));
pub const GameServerSetKey: ResourceType = ResourceType(Cow::Borrowed("gssKey"));
pub const GameServerAllocationKey: ResourceType = ResourceType(Cow::Borrowed("gsaKey"));
pub const FleetKey: ResourceType = ResourceType(Cow::Borrowed("fleetKey"));
pub const FleetAutoscalerKey: ResourceType = ResourceType(Cow::Borrowed("fasKey"));
impl From<String> for ResourceType { fn from(value: String) -> Self { Self(Cow::Owned(value)) } }
impl fmt::Display for ResourceType {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&self.0) }
}

/// Immutable field projection. The application's CGP logging capability owns
/// the shared formatter, level and output resource, not each derived entry.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LogEntry { pub fields: BTreeMap<String, Value> }
impl LogEntry {
    pub fn with_field(&self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        let mut entry = self.clone();
        entry.fields.insert(name.into(), value.into());
        entry
    }
}

pub fn AugmentLogEntry(base: &LogEntry, resourceType: &ResourceType, resourceID: &str) -> LogEntry {
    base.with_field(resourceType.0.clone(), resourceID)
}

#[cgp_component(ResourceLogAugmenter)]
pub trait CanAugmentResourceLog {
    fn augment_resource_log(&self, base: &LogEntry, resource_type: &ResourceType, resource_id: &str) -> LogEntry;
}
#[cgp_new_provider]
impl<Context> ResourceLogAugmenter<Context> for AugmentSourceResourceLog {
    fn augment_resource_log(_context: &Context, base: &LogEntry, resource_type: &ResourceType, resource_id: &str) -> LogEntry {
        AugmentLogEntry(base, resource_type, resource_id)
    }
}
