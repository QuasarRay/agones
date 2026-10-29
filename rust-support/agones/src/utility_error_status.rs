//! APIStatus interface assignment without calling Status during errors.As.
// Source: pinned vendor/k8s.io/apimachinery/pkg/api/errors/errors.go.
// Copyright 2014 The Kubernetes Authors. Apache-2.0; see LICENSE.MD.
use std::{error::Error, sync::Arc};
use crate::{utility_admission::Status, utility_error_slice::SourceRetainedError,
    utility_error_view::SourceApiStatus, utility_format_methods::SourcePrintfMethodViews,
    utility_error_query::{SourceErrorCursor, SourceAsElement, SourceErrorAsReadSlot,
        SourceErrorAsSlot, SourceErrorAsTarget, try_source_error_as, SourceErrorBindingError},
    utility_error_target::{SourceErrorAsValueSlot, SourceErrorAsOwnedSlot, SourceAsCell},
    utility_format_copy::SourceFmtValueCopy};

/// One concrete source APIStatus interface. Values assigned by custom As are
/// allowed to implement Status without implementing Error. Interface copies
/// retain the same immutable value data/reference owner; Status methods keep
/// responsibility for their declared receiver/value semantics.
enum ApiStatusData {
    Error(SourceRetainedError),
    Value(Arc<dyn SourceApiStatus + Send + Sync>),
}
pub struct SourceApiStatusValue { data: ApiStatusData }
impl SourceApiStatusValue {
    pub fn from_value<Value: SourceApiStatus + Send + Sync + 'static>(value: Value) -> Self {
        Self::from_shared(Arc::new(value))
    }
    pub fn from_shared<Value: SourceApiStatus + Send + Sync + 'static>(value: Arc<Value>) -> Self {
        Self { data: ApiStatusData::Value(value) }
    }
    pub fn status(&self) -> Status {
        match &self.data {
            ApiStatusData::Error(error) => api_status_method(error.original())
                .expect("assigned source APIStatus method").source_api_status(),
            ApiStatusData::Value(value) => value.source_api_status(),
        }
    }
    fn project(error: SourceErrorCursor<'_>) -> Option<Self> {
        api_status_method(error.raw_error())?;
        Some(Self { data: ApiStatusData::Error(error.retained_copy()?) })
    }
}
impl SourceFmtValueCopy for SourceApiStatusValue {
    fn source_fmt_copy(&self) -> Self {
        Self { data: match &self.data { ApiStatusData::Error(error) => ApiStatusData::Error(error.source_fmt_copy()),
            ApiStatusData::Value(value) => ApiStatusData::Value(Arc::clone(value)) } }
    }
}

/// Inspect interface metadata only. Neither type assertion nor As assignment
/// calls Status. Repeated later Status calls use the original current receiver.
fn api_status_method(mut error: &(dyn Error + 'static)) -> Option<&dyn SourceApiStatus> {
    use crate::{utility_error_view::ApiStatusEnvelope, source::workerqueue::ApiStatusError};
    loop {
        if let Some(retained) = error.downcast_ref::<SourceRetainedError>() { error = retained.original(); continue; }
        if let Some(shared) = error.downcast_ref::<crate::utility_error_alias::SourceSharedError>() {
            return shared.source_methods().error_methods.api_status;
        }
        if let Some(runtime) = error.downcast_ref::<crate::utility_panic::SourceRuntimeErrorEnvelope>() {
            error = runtime.error(); continue;
        }
        if let Some(error) = error.downcast_ref::<ApiStatusEnvelope>() { return Some(error); }
        if let Some(error) = error.downcast_ref::<ApiStatusError>() { return error.source_methods().error_methods.api_status; }
        if let Some(error) = error.downcast_ref::<crate::utility_error_nil::SourceNilStatusError>() {
            return error.source_methods().error_methods.api_status;
        }
        return None;
    }
}
pub type SourceApiStatusTargetCell = Arc<SourceAsCell<Option<SourceApiStatusValue>>>;
enum ApiStatusSelectionData { Value(Option<SourceApiStatusValue>), Target(SourceApiStatusTargetCell) }
/// A projection of the caller's status variable, not a second mutable owner.
/// Each source status.Status() expression loads that variable again. Keeping
/// only the first target.read() result would hide a callback's later mutation.
pub struct SourceApiStatusSelection { data: ApiStatusSelectionData }
impl SourceApiStatusSelection {
    pub fn value_copy(&self) -> Option<SourceApiStatusValue> {
        match &self.data { ApiStatusSelectionData::Value(value) => value.source_fmt_copy(),
            ApiStatusSelectionData::Target(cell) => cell.read() }
    }
    pub fn status(&self) -> Status {
        // The interface load finishes before the method is invoked. A Status
        // callback may re-enter As or replace the same target cell here.
        let value = self.value_copy().unwrap_or_else(|| crate::utility_panic::unwind_source_runtime_error(
            crate::utility_panic::SourceRuntimeErrorString::new("invalid memory address or nil pointer dereference")));
        value.status()
    }
    fn snapshot(&self) -> Self { Self { data: ApiStatusSelectionData::Value(self.value_copy()) } }
}
fn project_retained(error: SourceErrorCursor<'_>) -> Option<Option<SourceApiStatusValue>> {
    SourceApiStatusValue::project(error).map(Some)
}
pub fn api_status_slot<'error>() -> SourceErrorAsOwnedSlot<'error, Option<SourceApiStatusValue>> {
    SourceErrorAsOwnedSlot::new(None, SourceAsElement::Interface, project_retained)
}
pub fn api_status_slot_with_cell<'error>(cell: SourceApiStatusTargetCell)
    -> SourceErrorAsOwnedSlot<'error, Option<SourceApiStatusValue>>
{ SourceErrorAsOwnedSlot::with_cell(cell, SourceAsElement::Interface, project_retained) }
/// A custom As adapter can retain this *APIStatus cell, set a non-error value,
/// or re-enter As with the same target. No callback executes under a cell lock.
pub fn api_status_target(target: &dyn SourceErrorAsSlot<'_>) -> Option<SourceApiStatusTargetCell> {
    target.target_cell()?.downcast_ref::<SourceApiStatusTargetCell>().map(Arc::clone)
}
pub fn try_retained_api_status(error: &SourceRetainedError)
    -> Result<(Option<SourceApiStatusSelection>, bool), SourceErrorBindingError>
{
    let cursor = SourceErrorCursor::retained(error.source_fmt_copy());
    if let Some(status) = SourceApiStatusValue::project(cursor.source_fmt_copy()) {
        return Ok((Some(SourceApiStatusSelection { data: ApiStatusSelectionData::Value(Some(status)) }), true));
    }
    let mut target = api_status_slot();
    let matched = try_source_error_as(Some(cursor), SourceErrorAsTarget::Pointer(&mut target))?;
    if matched { Ok((Some(SourceApiStatusSelection { data: ApiStatusSelectionData::Target(target.cell()) }), true)) }
    else { Ok((None, false)) }
}

enum ApiStatusViewData<'error> { Borrowed(SourceErrorCursor<'error>), Owned(SourceApiStatusSelection) }
/// Compatibility borrow for callers that have not transferred their error to
/// a retained owner. Production queue callers use the owned target path above.
pub struct SourceApiStatusView<'error> { data: ApiStatusViewData<'error> }
impl SourceFmtValueCopy for SourceApiStatusView<'_> {
    fn source_fmt_copy(&self) -> Self {
        Self { data: match &self.data {
            ApiStatusViewData::Borrowed(error) => ApiStatusViewData::Borrowed(error.source_fmt_copy()),
            ApiStatusViewData::Owned(value) => ApiStatusViewData::Owned(value.snapshot()),
        } }
    }
}
impl SourceApiStatusView<'_> {
    pub fn status(&self) -> Status {
        match &self.data {
            ApiStatusViewData::Borrowed(error) => api_status_method(error.raw_error())
                .expect("assigned borrowed APIStatus method").source_api_status(),
            ApiStatusViewData::Owned(value) => value.status(),
        }
    }
}
fn project<'error>(error: SourceErrorCursor<'error>) -> Option<Option<SourceApiStatusView<'error>>> {
    api_status_method(error.raw_error())?;
    Some(Some(SourceApiStatusView { data: ApiStatusViewData::Borrowed(error) }))
}

pub fn try_api_status(error: &(dyn std::error::Error + 'static))
    -> Result<(Option<SourceApiStatusView<'_>>, bool), SourceErrorBindingError>
{
    // Retained/source-shared callers use a static target cell that custom As
    // can retain. Never fake ownership for an arbitrary borrowed native value.
    if let Some(error) = SourceErrorCursor::borrowed(error).retained_copy() {
        let (status, matched) = try_retained_api_status(&error)?;
        return Ok((status.map(|value| SourceApiStatusView { data: ApiStatusViewData::Owned(value) }), matched));
    }
    // The apimachinery caller asserts APIStatus before invoking errors.As.
    // Do not allocate a target or call a method on that successful branch.
    if let Some(status) = project(SourceErrorCursor::borrowed(error)) { return Ok((status, true)); }
    let mut target = SourceErrorAsValueSlot::new(None, SourceAsElement::Interface, project);
    if try_source_error_as(Some(SourceErrorCursor::borrowed(error)), SourceErrorAsTarget::Pointer(&mut target))? {
        Ok((target.read(), true))
    } else { Ok((None, false)) }
}

