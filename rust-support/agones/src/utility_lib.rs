//! Production registration for the existing util Cargo package.
//! Source helpers live in agones-rs; Rust-specific composition lives here.
pub mod utility_declarations;
#[path = "utility_modules.rs"]
pub mod source;
pub use source::{apiserver, crd, errors, features, fswatch, https, httpserver, logfields, runtime, signals, webhooks};
pub mod utility_lifecycle;
pub use utility_lifecycle::UtilityFeatureContext;
pub mod utility_crd;
pub use utility_crd::RusternetesCrdContext;
pub mod utility_logging;
pub mod utility_scope;
pub mod utility_runtime;
pub mod utility_runtime_panic;
pub mod utility_crash;
pub mod utility_go_quote;
pub use utility_runtime::{UtilityRuntimeContext, UtilityRuntimeOwner};
pub mod utility_http;
pub mod utility_fswatch;
pub mod utility_tls;
pub mod utility_admission;
pub mod utility_discovery;
pub mod utility_signals;
pub mod utility_queue;
pub mod utility_queue_values;
pub mod utility_queue_limiters;
pub mod utility_client_config;
pub mod utility_error_view;
pub mod utility_error_format;
pub mod utility_panic;
pub mod utility_gateway;
pub mod utility_gateway_json;
mod utility_gateway_wire;
mod utility_gateway_wkt;
pub mod utility_gateway_scalar;
pub mod utility_gateway_targets;
mod utility_gateway_framing;

/// Original module path retained as an alias of the literal source hierarchy.
pub mod pkg { pub use crate::source as util; }

// Preserve the old public queue API while callers are transferred explicitly.
// It is distinct from pkg::util::workerqueue's source implementation and has
// no new source or verification credit from the new queue's registration.
#[path = "../../../crates/util/src/workerqueue.rs"]
pub mod workerqueue;
