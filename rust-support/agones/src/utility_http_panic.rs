//! HTTP-owned recovery reporting through the existing static diagnostic effect.
// Adapted from Go net/http conn.serve and bundled http2 runHandler.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use cgp::prelude::*;
use super::{CanEmitSourceHttpDiagnostic, CanRaiseSourceCollectionFailure};
use crate::utility_panic::SourceRecoveredPanic;

#[cgp_component(SourceHttpPanicReporter)]
pub trait CanReportSourceHttpPanic {
    fn report_source_http_panic(&self, http2: bool, remote_addr: &str, panic: &SourceRecoveredPanic);
}
#[cgp_new_provider]
impl<Context> SourceHttpPanicReporter<Context> for ReportNativeSourceHttpPanic
where Context: CanEmitSourceHttpDiagnostic + CanRaiseSourceCollectionFailure,
{
    fn report_source_http_panic(context: &Context, http2: bool, remote_addr: &str, panic: &SourceRecoveredPanic) {
        if panic.is_abort_handler() { return; }
        // This native adapter captures Rust recovery frames, not Go goroutine
        // frames. The source traceback/caller/native profile remains required;
        // the source log prefix, byte bound and suppression branch are authored.
        let stack = crate::utility_runtime_panic::native_recovery_stack(context);
        let protocol = if http2 { "http2" } else { "http" };
        context.emit_source_http_diagnostic(&format!("{protocol}: panic serving {remote_addr}: {panic}\n{stack}"));
    }
}
