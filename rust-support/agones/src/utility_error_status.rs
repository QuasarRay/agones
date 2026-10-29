//! APIStatus interface assignment without calling Status during errors.As.
// Source: pinned vendor/k8s.io/apimachinery/pkg/api/errors/errors.go.
// Copyright 2014 The Kubernetes Authors. Apache-2.0; see LICENSE.MD.
use crate::{utility_admission::Status,
    utility_error_query::{SourceErrorCursor, SourceAsElement, SourceErrorAsReadSlot,
        SourceErrorAsTarget, try_source_error_as, SourceErrorBindingError},
    utility_error_target::SourceErrorAsValueSlot, utility_format_copy::SourceFmtValueCopy};

pub struct SourceApiStatusView<'error> {
    error: SourceErrorCursor<'error>,
    status: fn(&(dyn std::error::Error + 'static)) -> Status,
}
impl SourceFmtValueCopy for SourceApiStatusView<'_> {
    fn source_fmt_copy(&self) -> Self { Self { error: self.error.source_fmt_copy(), status: self.status } }
}
impl SourceApiStatusView<'_> {
    pub fn status(&self) -> Status { (self.status)(self.error.original()) }
}
fn project<'error>(error: SourceErrorCursor<'error>) -> Option<Option<SourceApiStatusView<'error>>> {
    use crate::{utility_error_view::ApiStatusEnvelope, source::workerqueue::ApiStatusError};
    let status: fn(&(dyn std::error::Error + 'static)) -> Status = if error.original().is::<ApiStatusEnvelope>() {
        |error| error.downcast_ref::<ApiStatusEnvelope>().expect("assigned APIStatus envelope").status()
    } else if error.original().is::<ApiStatusError>() {
        |error| crate::utility_error_view::SourceApiStatus::source_api_status(
            error.downcast_ref::<ApiStatusError>().expect("assigned StatusError"))
    } else { return None; };
    Some(Some(SourceApiStatusView { error, status }))
}

pub fn try_api_status(error: &(dyn std::error::Error + 'static))
    -> Result<(Option<SourceApiStatusView<'_>>, bool), SourceErrorBindingError>
{
    // The apimachinery caller asserts APIStatus before invoking errors.As.
    // Do not allocate a target or call a method on that successful branch.
    if let Some(status) = project(SourceErrorCursor::borrowed(error)) { return Ok((status, true)); }
    let mut target = SourceErrorAsValueSlot::new(None, SourceAsElement::Interface, project);
    if try_source_error_as(Some(SourceErrorCursor::borrowed(error)), SourceErrorAsTarget::Pointer(&mut target))? {
        Ok((target.read(), true))
    } else { Ok((None, false)) }
}

