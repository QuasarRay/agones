//! One process-signal resource phase, statically selected by CGP.
//! Subscribers are the source os/signal channels, not effect-provider entries.
use std::{collections::BTreeMap, sync::{Arc, Mutex, Weak, atomic::{AtomicBool, Ordering}}};
use cgp::prelude::*;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, utility_scope::CanSpawnUtilityTask};

#[derive(Default)]
struct Hub {
    next_id: u64,
    subscribers: BTreeMap<u64, mpsc::Sender<()>>,
    #[cfg(unix)]
    session: Option<unix::Session>,
}
#[derive(Default)]
pub struct SourceSignalState { closing: AtomicBool, hub: Mutex<Hub> }

/// Dropping a Go stop function does not call it. This registration likewise
/// remains installed until explicit stop or the enclosing resource phase ends.
#[derive(Clone)]
pub struct SignalRegistration { state: Weak<SourceSignalState>, id: u64 }
impl SignalRegistration {
    pub fn stop(&self) -> Result<(), BoxError> {
        if let Some(state) = self.state.upgrade() { state.remove(self.id)?; }
        Ok(())
    }
}
pub struct SignalSubscription {
    pub receiver: mpsc::Receiver<()>,
    pub registration: SignalRegistration,
    pub owner_stop: CancellationToken,
}
impl SourceSignalState {
    fn remove(&self, id: u64) -> Result<(), BoxError> {
        let mut hub = self.hub.lock().unwrap_or_else(|error| error.into_inner());
        hub.subscribers.remove(&id);
        #[cfg(unix)]
        if hub.subscribers.is_empty() {
            if let Some(session) = &hub.session { session.lease.restore()?; session.stop.cancel(); }
            hub.session = None;
        }
        Ok(())
    }
    pub fn close(&self) -> Result<(), BoxError> {
        self.closing.store(true, Ordering::Release);
        let mut hub = self.hub.lock().unwrap_or_else(|error| error.into_inner());
        hub.subscribers.clear();
        #[cfg(unix)]
        {
            if let Some(session) = &hub.session { session.lease.restore()?; session.stop.cancel(); }
            hub.session = None;
        }
        Ok(())
    }
}

#[cgp_getter]
pub trait HasSourceSignalState { fn source_signal_state(&self) -> &Arc<SourceSignalState>; }
#[cgp_component(SourceSignalSubscriber)]
pub trait CanSubscribeSourceSignals { fn subscribe_source_signals(&self) -> Result<SignalSubscription, BoxError>; }
pub struct SubscribeNativeSourceSignals;

#[cfg(unix)]
mod unix {
    use super::*;
    use std::{io, os::fd::{AsRawFd, FromRawFd, OwnedFd}, sync::atomic::{AtomicI32, AtomicUsize}};
    use tokio::io::unix::AsyncFd;

    // Only the process owner writes these async-signal-safe cells. In-flight
    // handlers quiesce before the fd can close or a new owner can reuse it.
    static OWNED: AtomicBool = AtomicBool::new(false);
    static WRITE_FD: AtomicI32 = AtomicI32::new(-1);
    static ACTIVE: AtomicUsize = AtomicUsize::new(0);
    static PENDING: AtomicBool = AtomicBool::new(false);
    extern "C" fn notify_signal(_signal: libc::c_int) {
        let saved_errno = errno::errno();
        ACTIVE.fetch_add(1, Ordering::SeqCst);
        let fd = WRITE_FD.load(Ordering::SeqCst);
        if fd >= 0 && !PENDING.swap(true, Ordering::SeqCst) {
            let byte = 1u8;
            // No allocation, lock, callback, Tokio API or error formatter is
            // called in a signal handler. Channel notification may coalesce.
            if unsafe { libc::write(fd, (&byte as *const u8).cast(), 1) } < 0 { PENDING.store(false, Ordering::SeqCst); }
        }
        ACTIVE.fetch_sub(1, Ordering::SeqCst);
        errno::set_errno(saved_errno);
    }

    struct Claim(bool);
    impl Drop for Claim { fn drop(&mut self) { if self.0 { OWNED.store(false, Ordering::SeqCst); } } }
    pub(super) struct SignalLease {
        _writer: OwnedFd,
        old_actions: Vec<(libc::c_int, libc::sigaction)>,
        restored: Mutex<bool>,
    }
    impl SignalLease {
        pub(super) fn restore(&self) -> io::Result<()> {
            let mut restored = self.restored.lock().unwrap_or_else(|error| error.into_inner());
            if *restored { return Ok(()); }
            // Valid saved sigactions are restored before disabling the pipe.
            // If the OS rejects restoration, keep the live lease for a retry.
            for (signal, action) in &self.old_actions {
                if unsafe { libc::sigaction(*signal, action, std::ptr::null_mut()) } < 0 { return Err(io::Error::last_os_error()); }
            }
            WRITE_FD.store(-1, Ordering::SeqCst);
            while ACTIVE.load(Ordering::SeqCst) != 0 { std::hint::spin_loop(); }
            PENDING.store(false, Ordering::SeqCst);
            *restored = true;
            OWNED.store(false, Ordering::SeqCst);
            Ok(())
        }
    }
    impl Drop for SignalLease {
        fn drop(&mut self) {
            if let Err(error) = self.restore() {
                // Continuing after a failed final restore would leave a
                // handler writing into a closed/reused descriptor.
                eprintln!("fatal signal-handler restoration failure: {error}");
                std::process::abort();
            }
        }
    }
    pub(super) struct Session { pub lease: Arc<SignalLease>, pub stop: CancellationToken }
    fn open_pipe() -> io::Result<(OwnedFd, OwnedFd)> {
        let mut fds = [-1; 2];
        #[cfg(any(target_os = "linux", target_os = "android"))]
        let result = unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_NONBLOCK | libc::O_CLOEXEC) };
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        let result = unsafe { libc::pipe(fds.as_mut_ptr()) };
        if result < 0 { return Err(io::Error::last_os_error()); }
        let (reader, writer) = unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) };
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        for fd in [&reader, &writer] {
            let flags = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_GETFL) };
            if flags < 0 || unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0
                || unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC) } < 0 { return Err(io::Error::last_os_error()); }
        }
        Ok((reader, writer))
    }
    fn install() -> io::Result<(AsyncFd<OwnedFd>, Arc<SignalLease>)> {
        if OWNED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "process signals already have a utility resource owner"));
        }
        let mut claim = Claim(true);
        let (reader, writer) = open_pipe()?;
        let reader = AsyncFd::new(reader)?;
        let mut lease = SignalLease { _writer: writer, old_actions: Vec::new(), restored: Mutex::new(false) };
        claim.0 = false; // the lease now owns every success/error cleanup path
        PENDING.store(false, Ordering::SeqCst);
        WRITE_FD.store(lease._writer.as_raw_fd(), Ordering::SeqCst);
        for signal in [libc::SIGINT, libc::SIGTERM] {
            let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
            action.sa_sigaction = notify_signal as *const () as libc::sighandler_t;
            action.sa_flags = libc::SA_RESTART | libc::SA_ONSTACK;
            unsafe { libc::sigemptyset(&mut action.sa_mask); libc::sigaddset(&mut action.sa_mask, libc::SIGINT); libc::sigaddset(&mut action.sa_mask, libc::SIGTERM); }
            let mut previous = std::mem::MaybeUninit::uninit();
            if unsafe { libc::sigaction(signal, &action, previous.as_mut_ptr()) } < 0 { return Err(io::Error::last_os_error()); }
            lease.old_actions.push((signal, unsafe { previous.assume_init() }));
        }
        Ok((reader, Arc::new(lease)))
    }
    struct Reader { fd: AsyncFd<OwnedFd>, lease: Arc<SignalLease>, state: Arc<SourceSignalState>, stop: CancellationToken, owner_stop: CancellationToken }
    impl Drop for Reader {
        fn drop(&mut self) {
            let mut hub = self.state.hub.lock().unwrap_or_else(|error| error.into_inner());
            if hub.session.as_ref().is_some_and(|session| Arc::ptr_eq(&session.lease, &self.lease)) {
                hub.subscribers.clear();
                self.lease.restore().unwrap_or_else(|error| { eprintln!("fatal signal-handler restoration failure: {error}"); std::process::abort() });
                hub.session = None;
            }
        }
    }
    impl Reader {
        async fn run(self) -> Result<(), BoxError> {
            let mut bytes = [0u8; 64];
            loop {
                let ready = tokio::select! { _ = self.stop.cancelled() => return Ok(()), _ = self.owner_stop.cancelled() => return Ok(()), ready = self.fd.readable() => ready? };
                let mut ready = ready;
                match ready.try_io(|fd| {
                    let count = unsafe { libc::read(fd.get_ref().as_raw_fd(), bytes.as_mut_ptr().cast(), bytes.len()) };
                    if count < 0 { Err(io::Error::last_os_error()) } else { Ok(count as usize) }
                }) {
                    Err(_would_block) => continue,
                    Ok(Err(error)) if error.kind() == io::ErrorKind::Interrupted => continue,
                    Ok(Err(error)) => return Err(Box::new(error)),
                    Ok(Ok(0)) => return Err(Errors::default().New("unexpected EOF on process signal pipe")),
                    Ok(Ok(_)) => {
                        let hub = self.state.hub.lock().unwrap_or_else(|error| error.into_inner());
                        if !hub.session.as_ref().is_some_and(|session| Arc::ptr_eq(&session.lease, &self.lease)) { return Ok(()); }
                        PENDING.store(false, Ordering::SeqCst);
                        for sender in hub.subscribers.values() { let _ = sender.try_send(()); }
                    }
                }
            }
        }
    }
    #[cgp_provider(SourceSignalSubscriberComponent)]
    impl<Context> SourceSignalSubscriber<Context> for SubscribeNativeSourceSignals
    where Context: HasSourceSignalState + CanSpawnUtilityTask,
    {
        fn subscribe_source_signals(context: &Context) -> Result<SignalSubscription, BoxError> {
            let state = Arc::clone(context.source_signal_state());
            let owner_stop = context.utility_child_token()?;
            let (sender, receiver) = mpsc::channel(1);
            let (id, reader) = {
                let mut hub = state.hub.lock().unwrap_or_else(|error| error.into_inner());
                if state.closing.load(Ordering::Acquire) { return Err(Errors::default().New("process signal owner is closing")); }
                let id = hub.next_id.checked_add(1).ok_or_else(|| Errors::default().New("signal subscription identity exhausted"))?;
                hub.next_id = id;
                let reader = if hub.session.is_none() {
                    let (fd, lease) = install()?; let stop = CancellationToken::new();
                    hub.session = Some(Session { lease: Arc::clone(&lease), stop: stop.clone() });
                    Some(Reader { fd, lease, state: Arc::clone(&state), stop, owner_stop: owner_stop.clone() })
                } else { None };
                hub.subscribers.insert(id, sender);
                (id, reader)
            };
            // Release the hub lock before spawning: a rejected future drops
            // its Reader and restores hooks, acquiring this same lock.
            if let Some(reader) = reader { context.spawn_utility_task("process signal pipe", Box::pin(reader.run()))?; }
            Ok(SignalSubscription { receiver, registration: SignalRegistration { state: Arc::downgrade(&state), id }, owner_stop })
        }
    }
}
