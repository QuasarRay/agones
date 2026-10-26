//! Source module registration stays outside agones-rs/pkg/.
#[path = "../../../agones-rs/pkg/doc.rs"]
mod documentation;

#[path = "../../../agones-rs/pkg/version.rs"]
pub mod version;

#[path = "api_groups.rs"]
pub mod apis;
