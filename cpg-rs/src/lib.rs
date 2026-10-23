//! Native CPG schema, graph and generation machinery.
//! Upstream source and per-body port mappings are retained in verification/agones.

pub mod schema;
pub mod codegen;
pub mod construction;
pub mod protobuf_schema;
pub mod utilities;
pub mod execution;
pub mod graph;
pub mod passes;
#[cfg(feature="native-schema")]
pub mod schema_tools;
#[path = "../generated/domain.rs"]
#[cfg(feature = "generated-domain")]
pub mod domain;
#[path = "../generated/schema_constructors.rs"]
#[cfg(feature = "native-schema")]
pub mod cpg_schema;

#[derive(Debug)]
pub enum Error {
    Invalid(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => formatter.write_str(message),
            Self::Io(error) => error.fmt(formatter),
            Self::Json(error) => error.fmt(formatter),
        }
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self { Self::Io(value) }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self { Self::Json(value) }
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn content_digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    const HEX: &[u8; 16] = b"0123456789abcdef";
    Sha256::digest(bytes).iter().flat_map(|&byte| [HEX[(byte >> 4) as usize], HEX[(byte & 15) as usize]])
        .map(char::from).collect()
}
