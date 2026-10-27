//! The owned accept-loop delay shared by the plain and TLS HTTP providers.
// Adapted from Go net/http/server.go, net/net.go and syscall's Errno policy.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{CanEmitSourceHttpDiagnostic, CanWaitSourceHttpDelay};
use crate::utility_runtime::UtilityRuntimeContext;
use std::{io, time::Duration};

#[derive(Default)]
pub(crate) struct AcceptBackoff { delay: Duration }
impl AcceptBackoff {
    pub(crate) fn accepted(&mut self) { self.delay = Duration::ZERO; }
    pub(crate) async fn retry(&mut self, runtime: &UtilityRuntimeContext, error: &io::Error) -> bool {
        if !temporary_syscall_error(error) { return false; }
        self.delay = if self.delay.is_zero() { Duration::from_millis(5) }
            else { (self.delay * 2).min(Duration::from_secs(1)) };
        let duration = if self.delay == Duration::from_secs(1) { "1s".to_owned() }
            else { format!("{}ms", self.delay.as_millis()) };
        runtime.emit_source_http_diagnostic(&format!("http: Accept error: {error}; retrying in {duration}"));
        // Source time.Sleep is independent of Shutdown's cancellation. The
        // outer loop observes the closed server after this delay completes.
        runtime.wait_source_http_delay(self.delay).await;
        true
    }
}

fn temporary_syscall_error(error: &io::Error) -> bool {
    let Some(errno) = error.raw_os_error() else { return false; };
    // TCPListener.Accept wraps an FD syscall error in os.SyscallError, then
    // net.OpError. OpError.Temporary unwraps that wrapper once and calls this
    // Errno rule. Its separate direct-Errno connection-reset exception must
    // not be applied through the syscall wrapper.
    #[cfg(unix)]
    { [libc::EINTR, libc::EMFILE, libc::ENFILE, libc::EAGAIN, libc::EWOULDBLOCK, libc::ETIMEDOUT].contains(&errno) }
    #[cfg(windows)]
    {
        // These Go Errno values are APPLICATION_ERROR+iota, not similarly
        // named Winsock errors. See pinned syscall/zerrors_windows.go.
        [0x20000026, 0x2000003b, 0x20000006, 0x2000007f, 0x20000079].contains(&errno)
    }
    #[cfg(not(any(unix, windows)))]
    { let _ = errno; false }
}
