//! Literal utility filenames, registered outside the mirror.
#[path = "../../../agones-rs/pkg/util/doc.rs"]
mod documentation;
#[path = "../../../agones-rs/pkg/util/errors/errors.rs"]
pub mod errors;
#[path = "../../../agones-rs/pkg/util/logfields/logfields.rs"]
pub mod logfields;
#[path = "../../../agones-rs/pkg/util/runtime/features.rs"]
pub mod features;
#[path = "../../../agones-rs/pkg/util/crd/crd.rs"]
pub mod crd;

// These source names share one Go package. Runtime effects are added from
// runtime/runtime.rs when their concrete lifecycle/effect adoption is authored.
pub mod runtime { pub use super::features::*; }
