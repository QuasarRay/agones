//! Recoverable source panic data shared by the existing runtime/HTTP owners.
//! This is a leaf error/string bridge, not a type registry or a Go object model.
// Adapted from Go runtime/error.go and net/http's ErrAbortHandler identity.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use std::{any::Any, error::Error, fmt};
use derive_deftly::Deftly;
use crate::{errors::BoxError, runtime::SourceLogType,
    utility_declarations::{derive_deftly_template_SourceErrorOrigin,
        derive_deftly_template_SourcePrintfInterfaces, derive_deftly_template_SourceFmtValueCopy}};

/// Go runtime.Error is identified by its no-op RuntimeError method. Preserve
/// that interface separately from ordinary application Error implementations.
pub trait SourceRuntimeError: Error {
    fn runtime_error(&self) {}
    fn into_source_error(self) -> BoxError
    where Self: SourceLogType + Send + Sync + Sized + 'static,
    { with_runtime_error(self) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceFmtValueCopy)]
#[deftly(source_package = "runtime", source_struct = "errorString", source_log_type = "runtime.errorString", source_error_method)]
pub struct SourceRuntimeErrorString { pub(crate) message: &'static str }
impl SourceRuntimeErrorString {
    pub const fn new(message: &'static str) -> Self { Self { message } }
}
impl Error for SourceRuntimeErrorString {}
impl SourceRuntimeError for SourceRuntimeErrorString {}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceFmtValueCopy)]
#[deftly(source_package = "runtime", source_struct = "plainError", source_log_type = "runtime.plainError", source_error_method)]
pub struct SourceNilMapAssignment;
impl Error for SourceNilMapAssignment {}
impl SourceRuntimeError for SourceNilMapAssignment {}

/// internal/abi.BoundsErrorCode is a uint8, including values outside the nine
/// named codes. Preserve those values and their source table-index failures.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deftly)]
#[derive_deftly(SourcePrintfInterfaces, SourceFmtValueCopy)]
pub struct SourceBoundsCode(pub u8);
#[allow(non_upper_case_globals)]
impl SourceBoundsCode {
    pub const Index: Self = Self(0);
    pub const SliceLength: Self = Self(1);
    pub const SliceCapacity: Self = Self(2);
    pub const SliceLow: Self = Self(3);
    pub const Slice3Length: Self = Self(4);
    pub const Slice3Capacity: Self = Self(5);
    pub const Slice3High: Self = Self(6);
    pub const Slice3Low: Self = Self(7);
    pub const SliceConvert: Self = Self(8);
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceFmtValueCopy)]
#[deftly(source_package = "runtime", source_struct = "boundsError", source_log_type = "runtime.boundsError", source_error_method)]
pub struct SourceBoundsError { pub x: i64, pub y: i64, pub signed: bool, pub code: SourceBoundsCode }
impl Error for SourceBoundsError {}
impl SourceRuntimeError for SourceBoundsError {}

trait RuntimeErrorObject: Send + Sync {
    fn error(&self) -> &(dyn Error + Send + Sync + 'static);
    fn runtime_error_view(&self) -> &dyn SourceRuntimeError;
    fn source_type(&self) -> &'static str;
}
impl<T: SourceRuntimeError + SourceLogType + Send + Sync + 'static> RuntimeErrorObject for T {
    fn error(&self) -> &(dyn Error + Send + Sync + 'static) { self }
    fn runtime_error_view(&self) -> &dyn SourceRuntimeError { self }
    fn source_type(&self) -> &'static str { T::SOURCE_LOG_TYPE }
}

struct SharedRuntimeErrorOwner { error: crate::utility_error_alias::SourceSharedError }
impl RuntimeErrorObject for SharedRuntimeErrorOwner {
    fn error(&self) -> &(dyn Error + Send + Sync + 'static) { &self.error }
    fn runtime_error_view(&self) -> &dyn SourceRuntimeError {
        runtime_error_view(&self.error).expect("shared runtime owner retains its RuntimeError view")
    }
    fn source_type(&self) -> &'static str { self.error.source_type_name() }
}

/// Preserve the RuntimeError method set when a typed source error crosses a
/// native BoxError boundary. The original error is the source value; this
/// envelope supplies its erased interface view without another Unwrap layer.
pub struct SourceRuntimeErrorEnvelope { inner: Box<dyn RuntimeErrorObject> }
impl SourceRuntimeErrorEnvelope {
    pub fn error(&self) -> &(dyn Error + Send + Sync + 'static) { self.inner.error() }
    pub fn runtime_error(&self) -> &dyn SourceRuntimeError { self.inner.runtime_error_view() }
    pub fn source_type(&self) -> &'static str { self.inner.source_type() }
}
impl fmt::Debug for SourceRuntimeErrorEnvelope {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self.error(), out) }
}
impl fmt::Display for SourceRuntimeErrorEnvelope {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self.error(), out) }
}
impl Error for SourceRuntimeErrorEnvelope {
    fn source(&self) -> Option<&(dyn Error + 'static)> { self.error().source() }
}
pub fn with_runtime_error<T: SourceRuntimeError + SourceLogType + Send + Sync + 'static>(error: T) -> BoxError {
    Box::new(SourceRuntimeErrorEnvelope { inner: Box::new(error) })
}

// One compile-time family list supplies owned and borrowed views for the
// canonical runtime classes already authored here. Other statically typed
// RuntimeError implementations retain their method set via the envelope.
macro_rules! runtime_error_views {
    ($($ty:ty),+ $(,)?) => {
        fn direct_runtime_panic_error(payload: &(dyn Any + Send)) -> Option<&(dyn Error + Send + Sync + 'static)> {
            $(if let Some(error) = payload.downcast_ref::<$ty>() { return Some(error); })+
            None
        }
        fn take_runtime_error(error: BoxError) -> Result<Box<dyn RuntimeErrorObject>, BoxError> {
            let error = match error.downcast::<crate::utility_error_alias::SourceSharedError>() {
                Ok(error) if runtime_error_view(error.as_ref()).is_some() =>
                    return Ok(Box::new(SharedRuntimeErrorOwner { error: *error })),
                Ok(error) => error as BoxError,
                Err(error) => error,
            };
            let error = match error.downcast::<SourceRuntimeErrorEnvelope>() {
                Ok(error) => return Ok((*error).inner), Err(error) => error,
            };
            $(let error = match error.downcast::<$ty>() { Ok(error) => return Ok(error), Err(error) => error };)+
            Err(error)
        }
        pub fn runtime_error_view(error: &(dyn Error + 'static)) -> Option<&dyn SourceRuntimeError> {
            let mut error = error;
            while let Some(shared) = error.downcast_ref::<crate::utility_error_alias::SourceSharedError>() {
                if let Some(runtime) = shared.runtime_error() { return Some(runtime); }
                error = shared.original();
            }
            if let Some(error) = error.downcast_ref::<SourceRuntimeErrorEnvelope>() { return Some(error.runtime_error()); }
            $(if let Some(error) = error.downcast_ref::<$ty>() { return Some(error); })+
            None
        }
        pub(crate) fn runtime_error_type(error: &(dyn Error + 'static)) -> Option<&'static str> {
            let mut error = error;
            while let Some(shared) = error.downcast_ref::<crate::utility_error_alias::SourceSharedError>() {
                if shared.runtime_error().is_some() { return Some(shared.source_type_name()); }
                error = shared.original();
            }
            if let Some(error) = error.downcast_ref::<SourceRuntimeErrorEnvelope>() { return Some(error.source_type()); }
            $(if error.is::<$ty>() { return Some(<$ty as SourceLogType>::SOURCE_LOG_TYPE); })+
            None
        }
    };
}
runtime_error_views!(SourceRuntimeErrorString, SourceNilMapAssignment, SourceBoundsError);

enum PanicErrorKind { Application(BoxError), Runtime(Box<dyn RuntimeErrorObject>) }
pub struct SourceErrorPanic { kind: PanicErrorKind }
impl SourceErrorPanic {
    pub fn application(error: BoxError) -> Self {
        Self { kind: match take_runtime_error(error) {
            Ok(error) => PanicErrorKind::Runtime(error), Err(error) => PanicErrorKind::Application(error),
        } }
    }
    pub fn runtime<T: SourceRuntimeError + SourceLogType + Send + Sync + 'static>(error: T) -> Self {
        Self { kind: PanicErrorKind::Runtime(Box::new(error)) }
    }
    /// Retain the original boxed error and its own source chain. The panic
    /// envelope is not an additional source Unwrap layer.
    pub fn error(&self) -> &(dyn Error + Send + Sync + 'static) {
        match &self.kind { PanicErrorKind::Application(error) => error.as_ref(), PanicErrorKind::Runtime(error) => error.error() }
    }
    pub fn runtime_error(&self) -> Option<&dyn SourceRuntimeError> {
        match &self.kind { PanicErrorKind::Runtime(error) => Some(error.runtime_error_view()), PanicErrorKind::Application(_) => None }
    }
    pub fn source_type(&self) -> Option<&'static str> {
        match &self.kind { PanicErrorKind::Runtime(error) => Some(error.source_type()),
            PanicErrorKind::Application(error) => crate::utility_error_view::source_error_type(error.as_ref()) }
    }
}
impl fmt::Debug for SourceErrorPanic {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.debug_struct("SourceErrorPanic").field("error", &self.error()).finish() }
}

struct AbortIdentity { _unique: u8 }
static ABORT_IDENTITY: AbortIdentity = AbortIdentity { _unique: 0 };
#[derive(Clone, Copy, Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "errors", source_struct = "errorString", source_log_type = "*errors.errorString")]
pub struct SourceAbortHandler { identity: &'static AbortIdentity }
impl fmt::Debug for AbortIdentity {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("http.ErrAbortHandler") }
}
impl PartialEq for SourceAbortHandler {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.identity, other.identity) }
}
impl Eq for SourceAbortHandler {}
impl fmt::Display for SourceAbortHandler {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("net/http: abort Handler") }
}
impl Error for SourceAbortHandler {}
pub const ERR_ABORT_HANDLER: SourceAbortHandler = SourceAbortHandler { identity: &ABORT_IDENTITY };

/// Own the actual catch_unwind payload. Source error recovery exposes its
/// original error and RuntimeError interface; forwarding retains its identity.
pub struct SourceRecoveredPanic { payload: Box<dyn Any + Send> }
impl SourceRecoveredPanic {
    pub fn new(payload: Box<dyn Any + Send>) -> Self { Self { payload } }
    pub fn payload(&self) -> &(dyn Any + Send) { self.payload.as_ref() }
    pub fn into_payload(self) -> Box<dyn Any + Send> { self.payload }
    pub fn resume(self) -> ! { std::panic::resume_unwind(self.payload) }
    pub fn error(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
        if let Some(value) = self.payload.downcast_ref::<crate::utility_format_methods::SourcePrintfPanic>() { return value.error(); }
        if let Some(error) = self.payload.downcast_ref::<SourceErrorPanic>() { return Some(error.error()); }
        if let Some(error) = self.payload.downcast_ref::<BoxError>() {
            return Some(match error.downcast_ref::<SourceRuntimeErrorEnvelope>() {
                Some(error) => error.error(), None => error.as_ref(),
            });
        }
        if let Some(error) = self.payload.downcast_ref::<crate::utility_error_alias::SourceSharedError>() { return Some(error); }
        if let Some(error) = direct_runtime_panic_error(self.payload.as_ref()) { return Some(error); }
        self.payload.downcast_ref::<SourceAbortHandler>().map(|error| error as &(dyn Error + Send + Sync + 'static))
    }
    pub fn runtime_error(&self) -> Option<&dyn SourceRuntimeError> {
        if let Some(error) = self.payload.downcast_ref::<SourceErrorPanic>() { return error.runtime_error(); }
        if let Some(error) = self.payload.downcast_ref::<BoxError>() { return runtime_error_view(error.as_ref()); }
        runtime_error_view(self.error()?)
    }
    pub fn source_type(&self) -> Option<&str> {
        if let Some(value) = self.payload.downcast_ref::<crate::utility_format_methods::SourcePrintfPanic>() { return Some(value.source_type()); }
        if let Some(error) = self.payload.downcast_ref::<SourceErrorPanic>() { return error.source_type(); }
        if let Some(error) = self.payload.downcast_ref::<crate::utility_error_alias::SourceSharedError>() { return Some(error.source_type_name()); }
        if let Some(source_type) = direct_runtime_panic_error(self.payload.as_ref()).and_then(|error| runtime_error_type(error)) { return Some(source_type); }
        if let Some(source_type) = self.payload.downcast_ref::<BoxError>().and_then(|error| crate::utility_error_view::source_error_type(error.as_ref())) { return Some(source_type); }
        if self.payload.is::<String>() || self.payload.is::<&'static str>() { return Some("string"); }
        if self.is_abort_handler() { return Some(SourceAbortHandler::SOURCE_LOG_TYPE); }
        None
    }
    /// Suppress logging only for the direct sentinel value. An error that
    /// wraps the sentinel has different source interface equality.
    pub fn is_abort_handler(&self) -> bool {
        self.error().map(|error| crate::utility_error_alias::source_error_original(error))
            .and_then(|error| error.downcast_ref::<SourceAbortHandler>()).is_some_and(|error| *error == ERR_ABORT_HANDLER)
    }
}
impl fmt::Display for SourceRecoveredPanic {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.payload.downcast_ref::<crate::utility_format_methods::SourcePrintfPanic>() {
            // Native Display has no context argument. Source-aware consumers
            // provide their selected Unicode profile and keep the byte result.
            let bytes = value.render(crate::utility_http::SourceUnicodeProfile::default(), false).map_err(|_| fmt::Error)?;
            return out.write_str(&String::from_utf8_lossy(&bytes));
        }
        if let Some(error) = self.error() { return fmt::Display::fmt(error, out); }
        if let Some(value) = self.payload.downcast_ref::<String>() { return out.write_str(value); }
        if let Some(value) = self.payload.downcast_ref::<&'static str>() { return out.write_str(value); }
        // Native opaque payloads have no Go formatting contract. Full source
        // fmt/type/caller/panic(nil) profiles remain required implementation.
        out.write_str("Box<dyn Any>")
    }
}

/// Skip Rust's eager hook: source recovery decides whether output is needed.
pub fn unwind_source_panic<T: Any + Send>(payload: T) -> ! { std::panic::resume_unwind(Box::new(payload)) }
pub fn unwind_source_error(error: BoxError) -> ! { unwind_source_panic(SourceErrorPanic::application(error)) }
pub fn unwind_source_runtime_error<T: SourceRuntimeError + SourceLogType + Send + Sync + 'static>(error: T) -> ! {
    unwind_source_panic(SourceErrorPanic::runtime(error))
}
