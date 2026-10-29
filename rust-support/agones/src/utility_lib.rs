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
pub mod utility_error_alias;
pub mod utility_error_slice;
pub mod utility_error_walk;
pub mod utility_error_query;
pub mod utility_error_target;
pub mod utility_error_nil;
mod utility_callback_slice;
pub mod utility_error_handlers;
mod utility_klog_values;
pub mod utility_klog_dispatch;
pub mod utility_logr;
pub mod utility_logr_context;
pub mod utility_klog;
pub mod utility_klog_header;
pub mod utility_klog_output;
pub mod utility_klog_file;
pub mod utility_klog_file_env;
pub mod utility_klog_path;
pub mod utility_klog_daemon;
pub mod utility_slog_values;
pub mod utility_time_values;
mod utility_time_format;
pub mod utility_time_json_error;
mod utility_trace_error;
pub mod utility_error_status;
pub mod utility_error_format;
pub mod utility_format_scan;
pub mod utility_format_print;
pub mod utility_format_integer;
pub mod utility_format_bool;
pub mod utility_format_arguments;
pub mod utility_format_methods;
pub mod utility_format_adapters;
pub mod utility_format_collections;
pub mod utility_format_pointer;
pub mod utility_format_map;
pub mod utility_format_copy;
pub mod utility_format_snapshot;
mod utility_format_sort;
mod utility_float_decimal;
mod utility_float_math;
mod utility_float_powers;
mod utility_float_dragonbox;
mod utility_float_fixed;
pub mod utility_float_reference;
pub mod utility_format_float;
pub mod utility_format_text;
pub mod utility_panic;
mod utility_runtime_format;
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
