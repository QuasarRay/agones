//! Concrete typed nil error pointers used by source worker classification.
// Source: Agones workerqueue.go and vendored apimachinery api/errors/errors.go
// at 6eb4a02588d8e7b581de187f3bc849733a78dcb2. Apache-2.0 notices retained.
// Nil method formatting/order follows Go fmt/print.go and errors/wrap.go,
// go1.26.5; The Go Authors, BSD-3-Clause, see ../GO-LICENSE.
use derive_deftly::Deftly;
use crate::{utility_admission::Status, utility_error_view::SourceApiStatus,
    utility_declarations::{derive_deftly_template_SourceErrorOrigin,
        derive_deftly_template_SourcePrintfInterfaces, derive_deftly_template_SourceNilErrorPointer,
        derive_deftly_template_SourceFmtValueCopy}};

pub(crate) fn nil_error_receiver() -> ! {
    crate::utility_panic::unwind_source_runtime_error(
        crate::utility_panic::SourceRuntimeErrorString::new("invalid memory address or nil pointer dereference"))
}

/// A non-nil error/APIStatus interface containing (*errors.StatusError)(nil).
/// It has Error and Status methods; it has no custom Is/As/Unwrap method.
#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceNilErrorPointer, SourceFmtValueCopy)]
#[deftly(source_package = "k8s.io/apimachinery/pkg/api/errors", source_struct = "StatusError",
    source_log_type = "*errors.StatusError", source_error_method, source_api_status, source_nil_error_pointer,
    source_error_target = "crate::source::workerqueue::ApiStatusError")]
pub struct SourceNilStatusError;
impl SourceApiStatus for SourceNilStatusError {
    fn source_api_status(&self) -> Status { nil_error_receiver() }
    fn source_error_query_methods(&self) -> crate::utility_error_query::SourceErrorMethods<'_> {
        crate::utility_format_methods::SourcePrintfMethodViews::source_methods(self).error_methods
    }
}

/// A non-nil error interface containing (*workerqueue.traceError)(nil).
/// The source Error method dereferences l.err before it can check err==nil.
#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceNilErrorPointer, SourceFmtValueCopy)]
#[deftly(source_package = "agones.dev/agones/pkg/util/workerqueue", source_struct = "traceError",
    source_log_type = "*workerqueue.traceError", source_error_method, source_nil_error_pointer,
    source_error_target = "crate::source::workerqueue::traceError")]
pub struct SourceNilTraceError;
