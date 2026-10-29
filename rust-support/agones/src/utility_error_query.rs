//! Source errors.Is, As and AsType over retained, typed error data.
// Adapted from Go errors/wrap.go at go1.26.5. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. These are per-value method views, not
// runtime providers, a reflection registry, or an application lifecycle.
use std::{error::Error, fmt};
use crate::{
    utility_error_alias::{SourceSharedError, source_error_original},
    utility_error_slice::{SourceErrorSlice, SourceRetainedError},
    utility_format_copy::SourceFmtValueCopy,
    utility_format_methods::{SourcePrintfMethodViews, SourcePrintfPanic},
};

/// A source interface copy keeps its selected allocation alive even when a
/// callback replaces the slice cell through which it was reached.
pub enum SourceErrorCursor<'a> {
    Borrowed(&'a (dyn Error + 'static)),
    Retained(SourceRetainedError),
}
impl<'a> SourceErrorCursor<'a> {
    pub fn borrowed(error: &'a (dyn Error + 'static)) -> Self { Self::Borrowed(error) }
    pub fn retained(error: SourceRetainedError) -> Self { Self::Retained(error) }
    pub fn raw_error(&self) -> &(dyn Error + 'static) {
        match self { Self::Borrowed(error) => *error, Self::Retained(error) => error.original() }
    }
    pub fn original(&self) -> &(dyn Error + 'static) {
        let mut error = self.raw_error();
        loop {
            error = source_error_original(error);
            if let Some(runtime) = error.downcast_ref::<crate::utility_panic::SourceRuntimeErrorEnvelope>() {
                error = runtime.error();
            } else { return error; }
        }
    }
}
impl SourceFmtValueCopy for SourceErrorCursor<'_> {
    fn source_fmt_copy(&self) -> Self {
        match self { Self::Borrowed(error) => Self::Borrowed(*error),
            Self::Retained(error) => Self::Retained(error.source_fmt_copy()) }
    }
}

pub trait SourceErrorIs {
    fn source_error_is(&self, target: &SourceErrorIsTarget<'_>) -> bool;
}
pub trait SourceErrorAs {
    /// The library has validated a non-nil pointer target. Concrete source
    /// adapters use its typed target cell or assign a retained error through
    /// assign_owned; the capability is not a general Go any interpreter.
    fn source_error_as<'error>(&self, target: &mut dyn SourceErrorAsSlot<'error>) -> bool;
}
pub trait SourceErrorUnwrapOne {
    fn source_unwrap_one(&self) -> Option<SourceRetainedError>;
}
pub trait SourceErrorUnwrapMany {
    fn source_unwrap_many(&self) -> SourceErrorSlice;
}

#[derive(Default)]
pub struct SourceErrorMethods<'a> {
    /// False means that this native type's source method set is unbound.
    /// An explicitly declared leaf has true and four absent methods.
    pub declared: bool,
    pub is: Option<&'a dyn SourceErrorIs>,
    pub as_target: Option<&'a dyn SourceErrorAs>,
    pub unwrap_one: Option<&'a dyn SourceErrorUnwrapOne>,
    pub unwrap_many: Option<&'a dyn SourceErrorUnwrapMany>,
}
impl SourceErrorMethods<'_> {
    pub fn leaf() -> Self { Self { declared: true, ..Self::default() } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceErrorBindingError { pub source_type: Option<&'static str> }
impl fmt::Display for SourceErrorBindingError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "unbound source error method set: {:?}", self.source_type)
    }
}
impl Error for SourceErrorBindingError {}

pub(crate) fn canonical_query_methods(error: &(dyn Error + 'static)) -> Option<SourceErrorMethods<'_>> {
    use crate::utility_error_format::{MessageError, WrappedError, FormattedError};
    let error = source_error_original(error);
    if let Some(error) = error.downcast_ref::<WrappedError>() {
        return Some(SourceErrorMethods { unwrap_one: Some(error), ..SourceErrorMethods::leaf() });
    }
    if let Some(error) = error.downcast_ref::<FormattedError>() {
        return Some(SourceErrorMethods { unwrap_many: Some(error), ..SourceErrorMethods::leaf() });
    }
    error.is::<MessageError>().then(SourceErrorMethods::leaf)
}
impl SourceErrorUnwrapOne for crate::utility_error_format::WrappedError {
    fn source_unwrap_one(&self) -> Option<SourceRetainedError> { self.unwrapped() }
}
impl SourceErrorUnwrapMany for crate::utility_error_format::FormattedError {
    fn source_unwrap_many(&self) -> SourceErrorSlice { self.wrapped() }
}

fn query_methods(mut error: &(dyn Error + 'static)) -> Result<SourceErrorMethods<'_>, SourceErrorBindingError> {
    loop {
        if let Some(retained) = error.downcast_ref::<SourceRetainedError>() { error = retained.original(); continue; }
        if let Some(shared) = error.downcast_ref::<SourceSharedError>() {
            let methods = shared.source_methods().error_methods;
            return if methods.declared { Ok(methods) }
                else { Err(SourceErrorBindingError { source_type: Some(shared.source_type_name()) }) };
        }
        if let Some(runtime) = error.downcast_ref::<crate::utility_panic::SourceRuntimeErrorEnvelope>() {
            error = runtime.error(); continue;
        }
        if let Some(methods) = canonical_query_methods(error) { return Ok(methods); }
        if let Some(status) = error.downcast_ref::<crate::utility_error_view::ApiStatusEnvelope>() {
            let methods = status.query_methods();
            return if methods.declared { Ok(methods) }
                else { Err(SourceErrorBindingError { source_type: Some(status.source_type()) }) };
        }
        // These are explicit declarations for existing concrete source types;
        // no unknown Rust Error::source edge is treated as a Go Unwrap method.
        if error.is::<crate::utility_panic::SourceRuntimeErrorString>()
            || error.is::<crate::utility_panic::SourceNilMapAssignment>()
            || error.is::<crate::utility_panic::SourceBoundsError>()
            || error.is::<crate::utility_panic::SourceAbortHandler>()
            || error.is::<crate::source::workerqueue::traceError>()
            || error.is::<crate::source::workerqueue::ApiStatusError>()
            || error.is::<crate::utility_http::ServerClosed>()
        { return Ok(SourceErrorMethods::leaf()); }
        return Err(SourceErrorBindingError { source_type: crate::utility_error_view::source_error_type(error) });
    }
}

pub type SourceErrorEquality = fn(&(dyn Error + 'static), &(dyn Error + 'static)) -> bool;
pub struct SourceErrorIsTarget<'a> {
    error: Option<SourceErrorCursor<'a>>,
    equality: Option<SourceErrorEquality>,
}
impl<'a> SourceErrorIsTarget<'a> {
    pub fn nil() -> Self { Self { error: None, equality: None } }
    /// None selects a source non-comparable type. The declaration supplies
    /// equality for a comparable type; Rust trait-object/vtable addresses and
    /// Display/type-name strings never stand in for source equality.
    pub fn declared(error: SourceErrorCursor<'a>, equality: Option<SourceErrorEquality>) -> Self {
        Self { error: Some(error), equality }
    }
    pub fn pointer<Value: Error + 'static>(error: &'a Value) -> Self {
        Self::declared(SourceErrorCursor::borrowed(error), Some(pointer_equal::<Value>))
    }
    pub fn value<Value: Error + PartialEq + 'static>(error: &'a Value) -> Self {
        Self::declared(SourceErrorCursor::borrowed(error), Some(value_equal::<Value>))
    }
    pub fn error(&self) -> Option<&(dyn Error + 'static)> { self.error.as_ref().map(SourceErrorCursor::original) }
    pub fn comparable(&self) -> bool { self.equality.is_some() }
}
pub fn pointer_equal<Value: Error + 'static>(left: &(dyn Error + 'static), right: &(dyn Error + 'static)) -> bool {
    match (left.downcast_ref::<Value>(), right.downcast_ref::<Value>()) {
        (Some(left), Some(right)) => std::ptr::eq(left, right), _ => false,
    }
}
pub fn value_equal<Value: Error + PartialEq + 'static>(left: &(dyn Error + 'static), right: &(dyn Error + 'static)) -> bool {
    match (left.downcast_ref::<Value>(), right.downcast_ref::<Value>()) {
        (Some(left), Some(right)) => left == right, _ => false,
    }
}

pub fn try_source_error_is(error: Option<SourceErrorCursor<'_>>, target: &SourceErrorIsTarget<'_>)
    -> Result<bool, SourceErrorBindingError>
{
    if error.is_none() || target.error.is_none() { return Ok(error.is_none() && target.error.is_none()); }
    is_error(error.expect("non-nil source error"), target)
}
fn is_error(mut error: SourceErrorCursor<'_>, target: &SourceErrorIsTarget<'_>) -> Result<bool, SourceErrorBindingError> {
    loop {
        if let Some(equal) = target.equality {
            if equal(error.original(), target.error().expect("non-nil source target")) { return Ok(true); }
        }
        let methods = query_methods(error.raw_error())?;
        if methods.is.is_some_and(|method| method.source_error_is(target)) { return Ok(true); }
        if let Some(method) = methods.unwrap_one {
            let Some(child) = method.source_unwrap_one() else { return Ok(false); };
            error = SourceErrorCursor::retained(child);
        } else if let Some(method) = methods.unwrap_many {
            let children = method.source_unwrap_many();
            for index in 0..children.len() {
                if let Some(child) = children.get(index) {
                    if is_error(SourceErrorCursor::retained(child), target)? { return Ok(true); }
                }
            }
            return Ok(false);
        } else { return Ok(false); }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceAsElement { Interface, ImplementsError, Other }
pub trait SourceErrorAsSlot<'error> {
    fn element(&self) -> SourceAsElement;
    /// A concrete source binding performs assignability and a value/header
    /// copy here. It must not call user As or Status methods during projection.
    fn try_assign(&mut self, error: SourceErrorCursor<'error>) -> bool;
    fn assign_owned(&mut self, error: SourceRetainedError) -> bool { self.try_assign(SourceErrorCursor::retained(error)) }
    fn target_cell(&self) -> Option<&dyn std::any::Any> { None }
}
pub trait SourceErrorAsReadSlot<'error>: SourceErrorAsSlot<'error> {
    type Value: SourceFmtValueCopy;
    fn read(&self) -> Self::Value;
}
pub enum SourceErrorAsTarget<'slot, 'error> {
    Nil, NonPointer, NilPointer,
    Pointer(&'slot mut dyn SourceErrorAsSlot<'error>),
}
pub fn try_source_error_as<'error>(error: Option<SourceErrorCursor<'error>>, target: SourceErrorAsTarget<'_, 'error>)
    -> Result<bool, SourceErrorBindingError>
{
    let Some(error) = error else { return Ok(false); };
    let target = match target {
        SourceErrorAsTarget::Nil => SourcePrintfPanic::raise("errors: target cannot be nil".to_owned()),
        SourceErrorAsTarget::NonPointer | SourceErrorAsTarget::NilPointer =>
            SourcePrintfPanic::raise("errors: target must be a non-nil pointer".to_owned()),
        SourceErrorAsTarget::Pointer(target) => target,
    };
    if target.element() == SourceAsElement::Other {
        SourcePrintfPanic::raise("errors: *target must be interface or implement error".to_owned());
    }
    as_error(error, target)
}
fn as_error<'error>(mut error: SourceErrorCursor<'error>, target: &mut dyn SourceErrorAsSlot<'error>)
    -> Result<bool, SourceErrorBindingError>
{
    loop {
        if target.try_assign(error.source_fmt_copy()) { return Ok(true); }
        let methods = query_methods(error.raw_error())?;
        if methods.as_target.is_some_and(|method| method.source_error_as(target)) { return Ok(true); }
        if let Some(method) = methods.unwrap_one {
            let Some(child) = method.source_unwrap_one() else { return Ok(false); };
            error = SourceErrorCursor::retained(child);
        } else if let Some(method) = methods.unwrap_many {
            let children = method.source_unwrap_many();
            for index in 0..children.len() {
                if let Some(child) = children.get(index) {
                    if as_error(SourceErrorCursor::retained(child), target)? { return Ok(true); }
                }
            }
            return Ok(false);
        } else { return Ok(false); }
    }
}

/// A declaration implementing this trait binds one source E:error. Native
/// TypeId, names and Rust Error alone do not establish Go assignability.
pub trait SourceAsTypeBinding<'error> {
    type Value: SourceFmtValueCopy;
    type Slot: SourceErrorAsReadSlot<'error, Value = Self::Value>;
    fn zero() -> Self::Value;
    fn project(error: SourceErrorCursor<'error>) -> Option<Self::Value>;
    fn new_slot() -> Self::Slot;
}
pub fn try_source_error_as_type<'error, Binding: SourceAsTypeBinding<'error>>(error: Option<SourceErrorCursor<'error>>)
    -> Result<(Binding::Value, bool), SourceErrorBindingError>
{
    let mut slot = None;
    if let Some(error) = error {
        if let Some(value) = as_type::<Binding>(error, &mut slot)? { return Ok((value, true)); }
    }
    // A failed custom As may have mutated its retained target; source AsType
    // nevertheless returns zero,false when no method/assertion succeeded.
    Ok((Binding::zero(), false))
}
fn as_type<'error, Binding: SourceAsTypeBinding<'error>>(mut error: SourceErrorCursor<'error>, slot: &mut Option<Binding::Slot>)
    -> Result<Option<Binding::Value>, SourceErrorBindingError>
{
    loop {
        if let Some(value) = Binding::project(error.source_fmt_copy()) { return Ok(Some(value)); }
        let methods = query_methods(error.raw_error())?;
        if let Some(method) = methods.as_target {
            let target = slot.get_or_insert_with(Binding::new_slot);
            if method.source_error_as(target) { return Ok(Some(target.read())); }
        }
        if let Some(method) = methods.unwrap_one {
            let Some(child) = method.source_unwrap_one() else { return Ok(None); };
            error = SourceErrorCursor::retained(child);
        } else if let Some(method) = methods.unwrap_many {
            let children = method.source_unwrap_many();
            for index in 0..children.len() {
                if let Some(child) = children.get(index) {
                    if let Some(value) = as_type::<Binding>(SourceErrorCursor::retained(child), slot)? { return Ok(Some(value)); }
                }
            }
            return Ok(None);
        } else { return Ok(None); }
    }
}

/// errors.Unwrap only consults Unwrap() error, including on a multi wrapper.
pub fn try_source_error_unwrap(error: Option<SourceErrorCursor<'_>>) -> Result<Option<SourceRetainedError>, SourceErrorBindingError> {
    let Some(error) = error else { return Ok(None); };
    Ok(query_methods(error.raw_error())?.unwrap_one.and_then(|method| method.source_unwrap_one()))
}
