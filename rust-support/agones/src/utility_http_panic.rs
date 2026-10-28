//! HTTP-owned recovery reporting through the existing static diagnostic effect.
// Adapted from Go net/http conn.serve and bundled http2 runHandler.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use std::{backtrace::Backtrace, fmt::{self, Write}};
use cgp::prelude::*;
use super::{CanEmitSourceHttpDiagnostic, CanRaiseSourceCollectionFailure,
    complete_source_collection, reserve_source_array};
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
        let mut stack = RecoveryStack { bytes: complete_source_collection(context, reserve_source_array::<u8>(64 << 10)), truncated: false };
        // This native adapter captures Rust recovery frames, not Go goroutine
        // frames. The source traceback/caller/native profile remains required;
        // the source log prefix, byte bound and suppression branch are authored.
        let _ = write!(&mut stack, "{}", Backtrace::force_capture());
        let stack = String::from_utf8(stack.bytes).expect("formatted recovery stack is UTF8");
        let protocol = if http2 { "http2" } else { "http" };
        context.emit_source_http_diagnostic(&format!("{protocol}: panic serving {remote_addr}: {panic}\n{stack}"));
    }
}
struct RecoveryStack { bytes: Vec<u8>, truncated: bool }
impl fmt::Write for RecoveryStack {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        if self.truncated { return Ok(()); }
        let available = (64 << 10) - self.bytes.len();
        let mut length = available.min(value.len());
        while !value.is_char_boundary(length) { length -= 1; }
        self.bytes.extend_from_slice(&value.as_bytes()[..length]);
        self.truncated = length < value.len();
        Ok(())
    }
}
