//! Depth-first error views over retained, mutable Unwrap data.
// Source contracts: Go errors/wrap.go and fmt/errors.go. Copyright The Go
// Authors. BSD-3-Clause; see ../GO-LICENSE. Native error views remain explicit.
use std::{error::Error, fmt, ops::Deref, ptr::NonNull};
use crate::{utility_error_alias::source_error_original,
    utility_error_slice::SourceRetainedError, utility_format_copy::SourceFmtValueCopy,
    utility_error_format::{source_unwrapped_error, source_wrapped_errors}};

enum MatchAnchor<'a> {
    Borrowed { _root: &'a (dyn Error + 'static) },
    Retained { _owner: SourceRetainedError },
}

/// Keep a selected error alive if a later slice assignment removes its slot.
/// Native compatibility callers borrow through Deref; they cannot mutate or
/// detach the pointer from its original allocation owner.
pub struct SourceErrorMatch<'a, Value: Error + 'static> {
    pointer: NonNull<Value>,
    _anchor: MatchAnchor<'a>,
}
impl<Value: Error + 'static> Deref for SourceErrorMatch<'_, Value> {
    type Target = Value;
    fn deref(&self) -> &Value {
        // SAFETY: construction is private to find_match. The pointer is a
        // projection of an immutable error borrow owned by the retained Box
        // anchor or by the caller's root borrow. Error cells are unlocked
        // before projection; replacing a cell cannot release this anchor.
        unsafe { self.pointer.as_ref() }
    }
}
impl<Value: Error + 'static> AsRef<Value> for SourceErrorMatch<'_, Value> {
    fn as_ref(&self) -> &Value { self }
}
impl<Value: Error + 'static> fmt::Debug for SourceErrorMatch<'_, Value> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(&**self, out) }
}

fn typed_view<Value: Error + 'static>(error: &(dyn Error + 'static)) -> Option<&Value> {
    error.downcast_ref::<Value>().or_else(|| error
        .downcast_ref::<crate::utility_error_view::ApiStatusEnvelope>()?.original_any().downcast_ref::<Value>())
}
pub fn find_error_match<Value: Error + 'static>(error: &(dyn Error + 'static)) -> Option<SourceErrorMatch<'_, Value>> {
    find_match(error, None, error)
}
fn find_match<'a, Value: Error + 'static>(error: &(dyn Error + 'static),
    anchor: Option<&SourceRetainedError>, root: &'a (dyn Error + 'static)) -> Option<SourceErrorMatch<'a, Value>>
{
    let error = source_error_original(error);
    if let Some(value) = typed_view::<Value>(error) {
        let anchor = match anchor { Some(owner) => MatchAnchor::Retained { _owner: owner.source_fmt_copy() },
            None => MatchAnchor::Borrowed { _root: root } };
        return Some(SourceErrorMatch { pointer: NonNull::from(value), _anchor: anchor });
    }
    if let Some(error) = error.downcast_ref::<crate::utility_panic::SourceRuntimeErrorEnvelope>() {
        return find_match(error.error(), anchor, root);
    }
    if let Some(child) = source_unwrapped_error(error) {
        return child.and_then(|child| find_match(&child, Some(&child), root));
    }
    if let Some(children) = source_wrapped_errors(error) {
        // The returned header freezes len/window, not the later elements.
        // No cell lock remains held during a predicate or child traversal.
        for index in 0..children.len() {
            if let Some(child) = children.get(index) {
                if let Some(value) = find_match(&child, Some(&child), root) { return Some(value); }
            }
        }
        return None;
    }
    error.source().and_then(|child| find_match(child, anchor, root))
}

/// Views return owned data. A borrowed result from a mutable cell would be
/// invalid after its interface slot is replaced; find_error_match retains it.
pub fn first_error_view<Value>(error: &(dyn Error + 'static),
    view: &impl Fn(&(dyn Error + 'static)) -> Option<Value>) -> Option<Value>
{
    let error = source_error_original(error);
    if let Some(value) = view(error) { return Some(value); }
    if let Some(error) = error.downcast_ref::<crate::utility_panic::SourceRuntimeErrorEnvelope>() {
        return first_error_view(error.error(), view);
    }
    if let Some(child) = source_unwrapped_error(error) {
        return child.and_then(|child| first_error_view(&child, view));
    }
    if let Some(children) = source_wrapped_errors(error) {
        for index in 0..children.len() {
            if let Some(child) = children.get(index) {
                if let Some(value) = first_error_view(&child, view) { return Some(value); }
            }
        }
        return None;
    }
    error.source().and_then(|child| first_error_view(child, view))
}
