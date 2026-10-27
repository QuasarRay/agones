//! Windows console effects for the shared source signal resource phase.
//! Source: Go runtime/os_windows.go::ctrlHandler and os/signal.NotifyContext.
// Portions adapted from the Go standard library. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE.
use super::*;
use std::{io, sync::atomic::AtomicUsize, task::Poll};
use futures::{future::{poll_fn, BoxFuture}, task::AtomicWaker};
use windows_sys::Win32::System::{Console::{SetConsoleCtrlHandler, CTRL_C_EVENT, CTRL_BREAK_EVENT,
    CTRL_CLOSE_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT}, Threading::Sleep};

static OWNED: AtomicBool = AtomicBool::new(false);
static ENABLED: AtomicBool = AtomicBool::new(false);
static PENDING: AtomicBool = AtomicBool::new(false);
static ACTIVE: AtomicUsize = AtomicUsize::new(0);
static WAKER: AtomicWaker = AtomicWaker::new();

unsafe extern "system" fn console_signal(kind: u32) -> windows_sys::core::BOOL {
    let terminate = match kind {
        CTRL_C_EVENT | CTRL_BREAK_EVENT => false,
        CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT => true,
        _ => return 0,
    };
    ACTIVE.fetch_add(1, Ordering::SeqCst);
    if !ENABLED.load(Ordering::SeqCst) { ACTIVE.fetch_sub(1, Ordering::SeqCst); return 0; }
    PENDING.store(true, Ordering::SeqCst);
    WAKER.wake();
    ACTIVE.fetch_sub(1, Ordering::SeqCst);
    if terminate {
        // Returning even TRUE lets Windows terminate the process immediately.
        // Go parks this OS-created thread after SIGTERM delivery. It owns no
        // lease, pointer, waker or Rust task while parked; parent process exit
        // (or the OS control timeout) ends it after application cleanup.
        loop { unsafe { Sleep(u32::MAX); } }
    }
    1
}

pub(super) struct SignalLease { restored: Mutex<bool> }
impl SignalLease {
    pub(super) fn restore(&self) -> io::Result<()> {
        let mut restored = self.restored.lock().unwrap_or_else(|error| error.into_inner());
        if *restored { return Ok(()); }
        if unsafe { SetConsoleCtrlHandler(Some(console_signal), 0) } == 0 { return Err(io::Error::last_os_error()); }
        ENABLED.store(false, Ordering::SeqCst);
        while ACTIVE.load(Ordering::SeqCst) != 0 { std::hint::spin_loop(); }
        PENDING.store(false, Ordering::SeqCst);
        drop(WAKER.take()); // Do not retain a completed Tokio task in a static.
        *restored = true;
        OWNED.store(false, Ordering::SeqCst);
        Ok(())
    }
}
impl Drop for SignalLease {
    fn drop(&mut self) {
        if let Err(error) = self.restore() {
            eprintln!("fatal console signal-handler restoration failure: {error}");
            std::process::abort();
        }
    }
}
fn install() -> io::Result<Arc<SignalLease>> {
    if OWNED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "process signals already have a utility resource owner"));
    }
    PENDING.store(false, Ordering::SeqCst);
    ENABLED.store(true, Ordering::SeqCst);
    if unsafe { SetConsoleCtrlHandler(Some(console_signal), 1) } == 0 {
        let error = io::Error::last_os_error();
        ENABLED.store(false, Ordering::SeqCst);
        while ACTIVE.load(Ordering::SeqCst) != 0 { std::hint::spin_loop(); }
        drop(WAKER.take()); OWNED.store(false, Ordering::SeqCst);
        return Err(error);
    }
    Ok(Arc::new(SignalLease { restored: Mutex::new(false) }))
}
pub(super) struct Session { pub lease: Arc<SignalLease>, pub stop: CancellationToken }
impl Session {
    pub(super) fn open(state: Arc<SourceSignalState>, owner_stop: CancellationToken) -> Result<(Self, BoxFuture<'static, Result<(), BoxError>>), BoxError> {
        let lease = install()?; let stop = CancellationToken::new();
        let session = Self { lease: Arc::clone(&lease), stop: stop.clone() };
        let reader = Reader { lease, state, stop, owner_stop };
        Ok((session, Box::pin(reader.run())))
    }
}
struct Reader { lease: Arc<SignalLease>, state: Arc<SourceSignalState>, stop: CancellationToken, owner_stop: CancellationToken }
impl Drop for Reader {
    fn drop(&mut self) {
        let mut hub = self.state.hub.lock().unwrap_or_else(|error| error.into_inner());
        if hub.session.as_ref().is_some_and(|session| Arc::ptr_eq(&session.lease, &self.lease)) {
            hub.subscribers.clear();
            self.lease.restore().unwrap_or_else(|error| { eprintln!("fatal console signal-handler restoration failure: {error}"); std::process::abort() });
            hub.session = None;
        }
    }
}
impl Reader {
    async fn run(self) -> Result<(), BoxError> {
        loop {
            let pending = poll_fn(|context| {
                let hub = self.state.hub.lock().unwrap_or_else(|error| error.into_inner());
                if !hub.session.as_ref().is_some_and(|session| Arc::ptr_eq(&session.lease, &self.lease)) { return Poll::Ready(false); }
                WAKER.register(context.waker());
                if PENDING.swap(false, Ordering::SeqCst) { Poll::Ready(true) } else { Poll::Pending }
            });
            let ready = tokio::select! {
                _ = self.stop.cancelled() => return Ok(()), _ = self.owner_stop.cancelled() => return Ok(()), ready = pending => ready,
            };
            if !ready { return Ok(()); }
            let hub = self.state.hub.lock().unwrap_or_else(|error| error.into_inner());
            if !hub.session.as_ref().is_some_and(|session| Arc::ptr_eq(&session.lease, &self.lease)) { return Ok(()); }
            for sender in hub.subscribers.values() { let _ = sender.try_send(()); }
        }
    }
}
