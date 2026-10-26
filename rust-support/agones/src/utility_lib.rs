//! Production registration for the existing util Cargo package.
//! Source helpers live in agones-rs; Rust-specific composition lives here.
pub mod utility_declarations;
#[path = "utility_modules.rs"]
pub mod source;
pub use source::{apiserver, crd, errors, features, fswatch, https, httpserver, logfields, runtime, webhooks};
pub mod utility_lifecycle;
pub use utility_lifecycle::UtilityFeatureContext;
pub mod utility_crd;
pub use utility_crd::RusternetesCrdContext;
pub mod utility_logging;
pub mod utility_scope;
pub mod utility_runtime;
pub use utility_runtime::{UtilityRuntimeContext, UtilityRuntimeOwner};
pub mod utility_http;
pub mod utility_fswatch;
pub mod utility_tls;
pub mod utility_admission;
pub mod utility_discovery;

/// Original module path retained as an alias of the literal source hierarchy.
pub mod pkg { pub use crate::source as util; }

// Retain the existing queue implementation until the M0003 queue source/adoption
// map transfers its bodies and consumers. It has no new verification credit.
#[path = "../../../crates/util/src/workerqueue.rs"]
pub mod workerqueue;
