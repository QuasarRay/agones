//! Kubernetes-independent Agones domain model.
//!
//! The core intentionally contains no `k8s-*` dependency. Rusternetes adapters
//! can map their API/runtime types into this model without making Kubernetes
//! compatibility the architectural center of the Rust reimplementation.

pub mod fleet;
pub mod gameserver;
pub mod gameserverset;
pub mod multicluster;

pub use fleet::*;
pub use gameserver::*;
pub use gameserverset::*;
pub use multicluster::*;
