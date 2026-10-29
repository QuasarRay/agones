//! Concrete typed target cells for source As callbacks and lazy AsType.
// Cells belong to each call/escaped source pointer, with no registry/manager.
// Source target order: Go errors/wrap.go, go1.26.5; The Go Authors,
// BSD-3-Clause, see ../GO-LICENSE. Arc/RwLock/Any are native projections.
use std::{error::Error, marker::PhantomData, ops::Deref, sync::{Arc, RwLock}};
use crate::{utility_error_query::{SourceErrorCursor, SourceErrorAsSlot,
    SourceErrorAsReadSlot, SourceAsElement, SourceAsTypeBinding}, utility_format_copy::SourceFmtValueCopy};

pub struct SourceAsCell<Value> { value: RwLock<Value> }
impl<Value> SourceAsCell<Value> {
    pub fn new(value: Value) -> Self { Self { value: RwLock::new(value) } }
    pub fn replace(&self, value: Value) -> Value {
        let mut slot = self.value.write().unwrap_or_else(|poison| poison.into_inner());
        let previous = std::mem::replace(&mut *slot, value);
        drop(slot);
        // Any native Drop behavior runs after the cell lock is released.
        previous
    }
}
impl<Value: SourceFmtValueCopy> SourceAsCell<Value> {
    pub fn read(&self) -> Value {
        self.value.read().unwrap_or_else(|poison| poison.into_inner()).source_fmt_copy()
    }
}

/// The projector is emitted/selected for a concrete source target type. The
/// value can itself be an ownership-anchored interface view with a root borrow.
/// Borrowed target cells cannot expose a static Any or escape the root's
/// lifetime; those complete source escape/assignment bindings remain separate
/// from the static owned-cell path below.
pub struct SourceErrorAsValueSlot<'error, Value> {
    cell: Arc<SourceAsCell<Value>>,
    element: SourceAsElement,
    project: fn(SourceErrorCursor<'error>) -> Option<Value>,
}
impl<'error, Value> SourceErrorAsValueSlot<'error, Value> {
    pub fn new(value: Value, element: SourceAsElement, project: fn(SourceErrorCursor<'error>) -> Option<Value>) -> Self {
        Self { cell: Arc::new(SourceAsCell::new(value)), element, project }
    }
    pub fn cell(&self) -> Arc<SourceAsCell<Value>> { Arc::clone(&self.cell) }
}
impl<'error, Value: SourceFmtValueCopy> SourceErrorAsSlot<'error> for SourceErrorAsValueSlot<'error, Value> {
    fn element(&self) -> SourceAsElement { self.element }
    fn try_assign(&mut self, error: SourceErrorCursor<'error>) -> bool {
        let Some(value) = (self.project)(error) else { return false; };
        self.cell.replace(value);
        true
    }
}
impl<'error, Value: SourceFmtValueCopy> SourceErrorAsReadSlot<'error> for SourceErrorAsValueSlot<'error, Value> {
    type Value = Value;
    fn read(&self) -> Value { self.cell.read() }
}

/// Static source values also expose their exact typed Arc cell to callbacks.
/// A callback can clone that Arc and mutate it after As has returned; AsType
/// reads a source value copy at the successful callback's return boundary.
pub struct SourceErrorAsOwnedSlot<'error, Value: 'static> { inner: SourceErrorAsValueSlot<'error, Value> }
impl<'error, Value: 'static> SourceErrorAsOwnedSlot<'error, Value> {
    pub fn new(value: Value, element: SourceAsElement, project: fn(SourceErrorCursor<'error>) -> Option<Value>) -> Self {
        Self { inner: SourceErrorAsValueSlot::new(value, element, project) }
    }
    pub fn cell(&self) -> Arc<SourceAsCell<Value>> { self.inner.cell() }
}
impl<'error, Value: SourceFmtValueCopy + 'static> SourceErrorAsSlot<'error> for SourceErrorAsOwnedSlot<'error, Value> {
    fn element(&self) -> SourceAsElement { self.inner.element() }
    fn try_assign(&mut self, error: SourceErrorCursor<'error>) -> bool { self.inner.try_assign(error) }
    fn target_cell(&self) -> Option<&dyn std::any::Any> { Some(&self.inner.cell) }
}
impl<'error, Value: SourceFmtValueCopy + 'static> SourceErrorAsReadSlot<'error> for SourceErrorAsOwnedSlot<'error, Value> {
    type Value = Value;
    fn read(&self) -> Value { self.inner.read() }
}

/// A native projection of a declared source *T error. Copying it retains the
/// interface owner. Deref borrows through that owner, without a raw pointer.
pub struct SourceErrorPointer<'error, Value: Error + 'static> {
    error: SourceErrorCursor<'error>,
    value: PhantomData<fn() -> Value>,
}
impl<'error, Value: Error + 'static> SourceErrorPointer<'error, Value> {
    pub fn project(error: SourceErrorCursor<'error>) -> Option<Self> {
        error.original().is::<Value>().then(|| Self { error, value: PhantomData })
    }
}
impl<Value: Error + 'static> SourceFmtValueCopy for SourceErrorPointer<'_, Value> {
    fn source_fmt_copy(&self) -> Self { Self { error: self.error.source_fmt_copy(), value: PhantomData } }
}
impl<Value: Error + 'static> Deref for SourceErrorPointer<'_, Value> {
    type Target = Value;
    fn deref(&self) -> &Value { self.error.original().downcast_ref().expect("declared error pointer projection") }
}
fn project_pointer<'error, Value: Error + 'static>(error: SourceErrorCursor<'error>)
    -> Option<Option<SourceErrorPointer<'error, Value>>>
{ SourceErrorPointer::project(error).map(Some) }
pub fn source_error_pointer_slot<'error, Value: Error + 'static>()
    -> SourceErrorAsValueSlot<'error, Option<SourceErrorPointer<'error, Value>>>
{ SourceErrorAsValueSlot::new(None, SourceAsElement::ImplementsError, project_pointer::<Value>) }

/// Selected only where the source declaration binds E to a *T error type.
/// Rust Error by itself supplies neither source type nor interface metadata.
pub struct SourceErrorPointerType<Value>(PhantomData<fn() -> Value>);
impl<'error, Value: Error + 'static> SourceAsTypeBinding<'error> for SourceErrorPointerType<Value> {
    type Value = Option<SourceErrorPointer<'error, Value>>;
    type Slot = SourceErrorAsValueSlot<'error, Self::Value>;
    fn zero() -> Self::Value { None }
    fn project(error: SourceErrorCursor<'error>) -> Option<Self::Value> { project_pointer::<Value>(error) }
    fn new_slot() -> Self::Slot { source_error_pointer_slot::<Value>() }
}
