//! Source pattern construction effects, error identity and MustPattern exit.
// Adapted from grpc-gateway/runtime/pattern.go and grpc/grpclog's default
// logger. Copyright gRPC authors. Apache-2.0; see ../GRPC-LICENSE.
use super::GatewayPattern;
use crate::{errors::BoxError, utility_http::{HeaderValues, HeaderAllocationProfile}, utility_tls::CanExitSourceProcess};
use cgp::prelude::*;
use std::{collections::BTreeMap, error::Error, fmt, io::Write, sync::{Arc, Mutex, RwLock}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternWordProfile { Go32, Go64 }
impl PatternWordProfile {
    pub const fn max_int(self) -> u64 { match self { Self::Go32 => i32::MAX as u64, Self::Go64 => i64::MAX as u64 } }
    pub(super) fn integer(self, value: i64) -> i64 { match self { Self::Go32 => value as i32 as i64, Self::Go64 => value } }
    pub(super) fn add(self, left: i64, right: i64) -> i64 { self.integer(left.wrapping_add(right)) }
    pub(super) fn sub(self, left: i64, right: i64) -> i64 { self.integer(left.wrapping_sub(right)) }
}
/// One immutable source allocation profile owns both word width and heap
/// address width. Actual process tool/target selection remains M0027.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatternSourceProfile { allocation: HeaderAllocationProfile }
impl PatternSourceProfile {
    pub const fn go126(allocation: HeaderAllocationProfile) -> Self { Self { allocation } }
    pub const fn allocation(self) -> HeaderAllocationProfile { self.allocation }
    pub const fn word(self) -> PatternWordProfile {
        if self.allocation.word_bits() == 32 { PatternWordProfile::Go32 } else { PatternWordProfile::Go64 }
    }
}
impl From<PatternWordProfile> for PatternSourceProfile {
    fn from(word: PatternWordProfile) -> Self {
        Self::go126(match word { PatternWordProfile::Go32 => HeaderAllocationProfile::GO126_32,
            PatternWordProfile::Go64 => HeaderAllocationProfile::GO126_64 })
    }
}
#[derive(Debug)]
struct PatternErrorIdentity(u8);
static INVALID_PATTERN: PatternErrorIdentity = PatternErrorIdentity(0);
#[derive(Clone, Debug)]
pub struct InvalidPatternError { identity: &'static PatternErrorIdentity }
impl InvalidPatternError {
    pub fn same_identity(&self, other: &Self) -> bool { std::ptr::eq(self.identity, other.identity) }
}
impl fmt::Display for InvalidPatternError { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("invalid pattern") } }
impl Error for InvalidPatternError {}
/// Direct ErrInvalidPattern equality. Wrapping/unwrapping is a separate error
/// operation; this does not turn every containing error into direct equality.
pub fn is_invalid_pattern(error: &(dyn Error + 'static)) -> bool {
    error.downcast_ref::<InvalidPatternError>().is_some_and(|error| std::ptr::eq(error.identity, &INVALID_PATTERN))
}
pub struct PatternConstruction { pub pattern: GatewayPattern, pub error: Option<BoxError> }
impl PatternConstruction {
    pub fn into_result(self) -> Result<GatewayPattern, BoxError> { match self.error { Some(error) => Err(error), None => Ok(self.pattern) } }
}
pub enum PatternDiagnostic<'a> {
    UnsupportedVersion(i64), OddCodeCount(i64), DuplicatePushM,
    LiteralIndex(i64), ConcatSize(i64), StackUnderflow, CaptureIndex(i64), InvalidOpcode(i64),
    InitializationFailed(&'a (dyn Error + 'static)),
}
impl PatternDiagnostic<'_> {
    fn severity(&self) -> &'static str { if matches!(self, Self::InitializationFailed(_)) { "FATAL" } else { "ERROR" } }
}
impl fmt::Display for PatternDiagnostic<'_> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion(value) => write!(out, "unsupported version: {value}"),
            Self::OddCodeCount(value) => write!(out, "odd number of ops codes: {value}"),
            Self::DuplicatePushM => out.write_str("pushM appears twice"),
            Self::LiteralIndex(value) => write!(out, "negative literal index: {value}"),
            Self::ConcatSize(value) => write!(out, "negative concat size: {value}"),
            Self::StackUnderflow => out.write_str("stack underflow"),
            Self::CaptureIndex(value) => write!(out, "variable name index out of bound: {value}"),
            Self::InvalidOpcode(value) => write!(out, "invalid opcode: {value}"),
            Self::InitializationFailed(error) => write!(out, "Pattern initialization failed: {error}"),
        }
    }
}
/// Data selected by grpclog initialization. Default Error/Fatal output is
/// stderr text. Actual environment/custom-logger/ABI binding remains M0027.
#[derive(Clone, Copy, Debug)]
pub struct PatternLogProfile { pub enabled: bool, pub json: bool }
impl Default for PatternLogProfile { fn default() -> Self { Self { enabled: true, json: false } } }
pub struct SourcePatternLogger { output: Mutex<Box<dyn Write + Send>>, profile: RwLock<PatternLogProfile> }
impl SourcePatternLogger {
    pub fn new(output: impl Write + Send + 'static, profile: PatternLogProfile) -> Self {
        Self { output: Mutex::new(Box::new(output)), profile: RwLock::new(profile) }
    }
    pub fn stderr() -> Self { Self::new(std::io::stderr(), PatternLogProfile::default()) }
    pub fn set_profile(&self, profile: PatternLogProfile) { *self.profile.write().unwrap_or_else(|error| error.into_inner()) = profile; }
    fn write(&self, diagnostic: PatternDiagnostic<'_>) {
        let profile = *self.profile.read().unwrap_or_else(|error| error.into_inner());
        // The source io.Discard check precedes fmt, including user Error
        // methods. Fatal process exit is owned separately and still occurs.
        if !profile.enabled { return; }
        let severity = diagnostic.severity();
        let message = diagnostic.to_string();
        let mut line = if profile.json {
            serde_json::to_string(&BTreeMap::from([("message", message), ("severity", severity.to_owned())]))
                .expect("source diagnostic string map serializes").replace('&', "\\u0026").replace('<', "\\u003c").replace('>', "\\u003e")
                .replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029")
        } else { format!("{} {severity}: {message}", chrono::Local::now().format("%Y/%m/%d %H:%M:%S")) };
        if !line.ends_with('\n') { line.push('\n'); }
        // log.Logger.Output calls Write once; its caller ignores the result.
        let _ = self.output.lock().unwrap_or_else(|error| error.into_inner()).write(line.as_bytes());
    }
}
#[cgp_getter]
pub trait HasSourcePatternLogger { fn source_pattern_logger(&self) -> &Arc<SourcePatternLogger>; }
#[cgp_component(SourcePatternDiagnosticEmitter)]
pub trait CanEmitSourcePatternDiagnostic { fn emit_source_pattern_diagnostic(&self, diagnostic: PatternDiagnostic<'_>); }
#[cgp_new_provider]
impl<Context: HasSourcePatternLogger> SourcePatternDiagnosticEmitter<Context> for EmitSourcePatternDiagnostic {
    fn emit_source_pattern_diagnostic(context: &Context, diagnostic: PatternDiagnostic<'_>) { context.source_pattern_logger().write(diagnostic); }
}
pub(super) fn reject_pattern(context: &impl CanEmitSourcePatternDiagnostic, profile: PatternSourceProfile, diagnostic: PatternDiagnostic<'_>) -> PatternConstruction {
    context.emit_source_pattern_diagnostic(diagnostic);
    PatternConstruction { pattern: GatewayPattern::zero(profile), error: Some(Box::new(InvalidPatternError { identity: &INVALID_PATTERN })) }
}
#[cgp_component(SourceGatewayPatternConstructor)]
pub trait CanConstructSourceGatewayPattern {
    fn construct_source_gateway_pattern(&self, profile: PatternSourceProfile, version: i64, operations: &[i64], pool: HeaderValues, verb: &[u8]) -> PatternConstruction;
    fn require_source_gateway_pattern(&self, construction: PatternConstruction) -> GatewayPattern;
}
#[cgp_new_provider]
impl<Context: CanEmitSourcePatternDiagnostic + CanExitSourceProcess> SourceGatewayPatternConstructor<Context> for ConstructSourceGatewayPattern {
    fn construct_source_gateway_pattern(context: &Context, profile: PatternSourceProfile, version: i64, operations: &[i64], pool: HeaderValues, verb: &[u8]) -> PatternConstruction {
        GatewayPattern::new(context, profile, version, operations, pool, verb)
    }
    fn require_source_gateway_pattern(context: &Context, construction: PatternConstruction) -> GatewayPattern {
        if let Some(error) = construction.error {
            context.emit_source_pattern_diagnostic(PatternDiagnostic::InitializationFailed(error.as_ref()));
            context.exit_source_process(1);
            panic!("source process-exit provider returned after MustPattern failure");
        }
        construction.pattern
    }
}
