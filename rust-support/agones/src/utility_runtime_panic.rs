//! Statically selected parent panic policy for existing utility child tasks.
// Adapted from Go runtime preprintpanics/printpanics/fatalpanic/error.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use std::{backtrace::Backtrace, fmt::{self, Write as _}, io::Write as _};
use cgp::prelude::*;
use crate::{utility_panic::SourceRecoveredPanic, utility_tls::CanExitSourceProcess,
    utility_http::{CanRaiseSourceCollectionFailure, complete_source_collection, reserve_source_array}};

#[cgp_component(SourceRuntimePanicReporter)]
pub trait CanReportSourceRuntimePanic {
    fn report_source_runtime_panic(&self, panic: &SourceRecoveredPanic);
}
#[cgp_new_provider]
impl<Context: CanRaiseSourceCollectionFailure> SourceRuntimePanicReporter<Context> for ReportNativeSourceRuntimePanic {
    fn report_source_runtime_panic(context: &Context, panic: &SourceRecoveredPanic) {
        // Error/String conversion precedes traceback collection. Indent every
        // embedded newline as runtime.printpanicval/printindented do. The
        // source panic chain, byte/format and traceback profiles remain open.
        let (prefix, value) = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| panic.to_string())) {
            Ok(value) => ("panic", value.replace('\n', "\n\t")),
            Err(payload) => {
                let nested = SourceRecoveredPanic::new(payload);
                let detail = if let Some(value) = nested.payload().downcast_ref::<String>() { value.clone() }
                    else if let Some(value) = nested.payload().downcast_ref::<&'static str>() { (*value).to_owned() }
                    else { format!("type {}", nested.source_type().unwrap_or("Box<dyn Any>")) };
                ("fatal error", format!("panic while printing panic value: {detail}"))
            }
        };
        let stack = native_recovery_stack(context);
        let _ = std::io::stderr().lock().write_all(format!("{prefix}: {value}\n\n{stack}").as_bytes());
    }
}

#[cgp_component(SourceUtilityPanicHandler)]
pub trait CanHandleSourceUtilityPanic {
    fn handle_source_utility_panic(&self, panic: SourceRecoveredPanic) -> !;
}
#[cgp_new_provider]
impl<Context> SourceUtilityPanicHandler<Context> for ExitAfterSourceUtilityPanic
where Context: CanReportSourceRuntimePanic + CanExitSourceProcess,
{
    fn handle_source_utility_panic(context: &Context, panic: SourceRecoveredPanic) -> ! {
        // An unhandled goroutine panic is process fatal. Unlike HTTP recovery,
        // the root does not suppress the ErrAbortHandler sentinel. This path
        // does not cancel/join unrelated children before the fatal decision.
        let report = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| context.report_source_runtime_panic(&panic)));
        if report.is_err() { let _ = std::io::stderr().lock().write_all(b"panic during panic\n"); }
        context.exit_source_process(2);
        std::process::abort(); // A process-exit effect must not return success.
    }
}

/// Native recovery frames for the existing source HTTP/root reporting effects.
/// They are bounded Rust frames, not Go goroutine/panic/traceback evidence.
pub(crate) fn native_recovery_stack(context: &impl CanRaiseSourceCollectionFailure) -> String {
    let mut stack = RecoveryStack { bytes: complete_source_collection(context, reserve_source_array::<u8>(64 << 10)), truncated: false };
    let _ = write!(&mut stack, "{}", Backtrace::force_capture());
    String::from_utf8(stack.bytes).expect("formatted recovery stack is UTF8")
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
