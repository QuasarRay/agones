//! Registration for the literal Agones source mirror.
//!
//! This crate owns the shared build identity. The existing agones-core API
//! re-exports that same identity, so callers use one implementation.
pub mod declarations;
pub mod pkg;

#[path = "../../../agones-rs/doc.rs"]
mod documentation;

#[cfg(feature = "source-tools")]
#[path = "../../../agones-rs/tools.rs"]
pub mod tools;

// Reuse the byte decoder already owned by the existing implementation.
// Its source is shared with the build script; neither copy owns mutable state.
#[path = "../../../crates/agones-core/src/go_utf8.rs"]
pub(crate) mod go_utf8;
