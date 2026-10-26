//! The two source files retain one Go https package and exact filename stems.
#[path = "../../../agones-rs/pkg/util/https/https.rs"]
mod helpers;
pub use helpers::*;
#[path = "../../../agones-rs/pkg/util/https/server.rs"]
mod server;
pub use server::*;
