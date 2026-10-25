//! Native CPG schema, graph and generation machinery.
//! Upstream source and per-body port mappings are retained in verification/agones.

pub mod schema;
pub mod schema_base;
pub mod codegen;
pub mod construction;
pub mod protobuf_schema;
pub mod utilities;
pub mod execution;
pub mod graph;
pub mod passes;
pub mod archive;
pub mod cpg_loading;
#[cfg(feature="native-schema")]
pub mod protobuf_bindings;
#[cfg(feature="protobuf-bindings")]
#[path="../generated/cpg.rs"]
pub mod proto;
#[cfg(feature="protobuf-bindings")]
pub mod proto_loading;
#[cfg(feature="native-schema")]
pub mod schema_tools;
#[path = "../generated/domain.rs"]
#[cfg(feature = "generated-domain")]
pub mod domain;
#[path = "../generated/schema_constructors.rs"]
#[cfg(feature = "native-schema")]
pub mod cpg_schema;
#[cfg(feature = "native-schema")]
pub mod cpg_schema_definition;

#[derive(Debug)]
pub enum Error {
    Invalid(String),
    SourceMatchFailure { operation: &'static str },
    Conversion(String),
    IllegalArgument(String),
    NumberFormat(String),
    Runtime(String),
    NoSuchElement(&'static str),
    ClosedFileSystem,
    FileSystemAlreadyExists,
    GraphClosed,
    Deserialization(String),
    Database(grafeo::Error),
    LegacyStorage(String),
    BackwardsCompatibility(String),
    ClassCast(String),
    SourceNotImplemented(&'static str),
    MessageTypeCast(String),
    MessageIntegerOverflow(String),
    SourceAssertion(String),
    SchemaViolation(String),
    Io(std::io::Error),
    Json(serde_json::Error),
    Protobuf(protobuf::Error),
    Zip(zip::result::ZipError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => formatter.write_str(message),
            Self::SourceMatchFailure { operation } => write!(formatter, "source MatchError: {operation}"),
            Self::Conversion(message) | Self::IllegalArgument(message) | Self::NumberFormat(message) | Self::Runtime(message) => formatter.write_str(message),
            Self::NoSuchElement(message) => formatter.write_str(message),
            Self::ClosedFileSystem => formatter.write_str("closed ZIP filesystem"),
            Self::FileSystemAlreadyExists => formatter.write_str("serialized ZIP filesystem already open"),
            Self::GraphClosed => formatter.write_str("graph cannot be modified any longer since it is closed"),
            Self::Deserialization(message) => formatter.write_str(message),
            Self::Database(error) => error.fmt(formatter),
            Self::LegacyStorage(message) | Self::BackwardsCompatibility(message) | Self::ClassCast(message) => formatter.write_str(message),
            Self::SourceNotImplemented(operation) => write!(formatter,"source NotImplementedError: {operation}"),
            Self::MessageTypeCast(message) | Self::MessageIntegerOverflow(message) => formatter.write_str(message),
            Self::SourceAssertion(message) => formatter.write_str(message),
            Self::SchemaViolation(message) => formatter.write_str(message),
            Self::Io(error) => error.fmt(formatter),
            Self::Json(error) => error.fmt(formatter),
            Self::Protobuf(error) => error.fmt(formatter),
            Self::Zip(error) => error.fmt(formatter),
        }
    }
}
impl std::error::Error for Error {}
impl Error {
    pub fn source_exception_class(&self) -> Option<&'static str> {
        match self {
            Self::SourceMatchFailure { .. } => Some("scala.MatchError"),
            Self::Conversion(_) => Some("flatgraph.misc.ConversionException"),
            Self::IllegalArgument(_) => Some("java.lang.IllegalArgumentException"),
            Self::NumberFormat(_) => Some("java.lang.NumberFormatException"),
            Self::Runtime(_) => Some("java.lang.RuntimeException"),
            Self::NoSuchElement(_) => Some("java.util.NoSuchElementException"),
            Self::ClosedFileSystem => Some("java.nio.file.ClosedFileSystemException"),
            Self::FileSystemAlreadyExists => Some("java.nio.file.FileSystemAlreadyExistsException"),
            Self::GraphClosed => Some("flatgraph.GraphClosedException"),
            Self::Deserialization(_) => Some("flatgraph.storage.Deserialization$DeserializationException"),
            Self::LegacyStorage(_) => Some("java.lang.IllegalStateException"),
            Self::BackwardsCompatibility(_) => Some("overflowdb.storage.BackwardsCompatibilityError"),
            Self::ClassCast(_) => Some("java.lang.ClassCastException"),
            Self::SourceNotImplemented(_) => Some("scala.NotImplementedError"),
            Self::MessageTypeCast(_) => Some("org.msgpack.core.MessageTypeCastException"),
            Self::MessageIntegerOverflow(_) => Some("org.msgpack.core.MessageIntegerOverflowException"),
            Self::SourceAssertion(_) => Some("java.lang.AssertionError"),
            Self::SchemaViolation(_) => Some("flatgraph.SchemaViolationException"),
            _ => None,
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self { Self::Io(value) }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self { Self::Json(value) }
}
impl From<protobuf::Error> for Error { fn from(value: protobuf::Error) -> Self { Self::Protobuf(value) } }
impl From<zip::result::ZipError> for Error { fn from(value: zip::result::ZipError) -> Self { Self::Zip(value) } }
impl From<grafeo::Error> for Error { fn from(value: grafeo::Error) -> Self { Self::Database(value) } }
pub type Result<T> = std::result::Result<T, Error>;

pub fn content_digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    const HEX: &[u8; 16] = b"0123456789abcdef";
    Sha256::digest(bytes).iter().flat_map(|&byte| [HEX[(byte >> 4) as usize], HEX[(byte & 15) as usize]])
        .map(char::from).collect()
}
