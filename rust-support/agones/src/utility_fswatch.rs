//! Owned filesystem notifications and source-style rendezvous channels.
//! The Linux/Android adapter uses the kernel fd directly so completion means
//! the reader has closed it; no unjoined library thread supplies this effect.
use std::{collections::VecDeque, path::Path, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}};
use cgp::prelude::*;
use tokio::sync::{Notify, oneshot};
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, utility_scope::CanSpawnUtilityTask, utility_runtime::UtilityRuntimeContext};

const EVENT_OVERFLOW: &str = "fsnotify: queue or buffer overflow";

struct Receipt { identity: Arc<()>, accepted: oneshot::Sender<()> }
struct Queued<T> { value: T, receipt: Option<Receipt> }
struct ChannelState<T> { values: VecDeque<Queued<T>>, closed: bool }
/// The source channel boundary. Default is unbuffered; Windows fsnotify uses
/// its source 50-event buffer while Errors remains unbuffered on every target.
pub struct Rendezvous<T> { capacity: usize, state: Mutex<ChannelState<T>>, readable: Notify, writable: Notify }
impl<T> Default for Rendezvous<T> {
    fn default() -> Self { Self::with_capacity(0) }
}
struct Retract<'a, T> { channel: &'a Rendezvous<T>, identity: Arc<()> }
impl<T> Drop for Retract<'_, T> {
    fn drop(&mut self) {
        let mut state = self.channel.state.lock().unwrap_or_else(|error| error.into_inner());
        if let Some(index) = state.values.iter().position(|entry| entry.receipt.as_ref().is_some_and(|receipt| Arc::ptr_eq(&receipt.identity, &self.identity))) {
            state.values.remove(index); self.channel.writable.notify_one();
        }
    }
}
impl<T> Rendezvous<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { capacity, state: Mutex::new(ChannelState { values: VecDeque::new(), closed: false }), readable: Notify::new(), writable: Notify::new() }
    }
    pub async fn send(&self, value: T) -> bool {
        let mut value = Some(value);
        loop {
            let wait = self.writable.notified(); tokio::pin!(wait); wait.as_mut().enable();
            let posted = {
                let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
                if state.closed { return false; }
                if state.values.len() < self.capacity.max(1) {
                    let (receipt, pending) = if self.capacity == 0 {
                        let identity = Arc::new(()); let (accepted, received) = oneshot::channel();
                        (Some(Receipt { identity: Arc::clone(&identity), accepted }), Some((identity, received)))
                    } else { (None, None) };
                    state.values.push_back(Queued { value: value.take().expect("one channel value"), receipt });
                    Some(pending)
                } else { None }
            };
            if let Some(pending) = posted {
                self.readable.notify_one();
                if let Some((identity, accepted)) = pending {
                    let _retract = Retract { channel: self, identity };
                    return accepted.await.is_ok();
                }
                return true;
            }
            wait.await;
        }
    }
    pub async fn recv(&self) -> Option<T> {
        loop {
            let wait = self.readable.notified(); tokio::pin!(wait); wait.as_mut().enable();
            {
                let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
                if let Some(entry) = state.values.pop_front() {
                    self.writable.notify_one();
                    if let Some(receipt) = entry.receipt { let _ = receipt.accepted.send(()); }
                    return Some(entry.value);
                }
                if state.closed { return None; }
            }
            wait.await;
        }
    }
    pub fn close(&self) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state.closed = true;
        if self.capacity == 0 { state.values.clear(); }
        drop(state);
        self.readable.notify_waiters(); self.writable.notify_waiters();
    }
}

#[derive(Default)]
struct CloseCompletion { closed: AtomicBool, notify: Notify }
impl CloseCompletion {
    fn finish(&self) { self.closed.store(true, Ordering::Release); self.notify.notify_waiters(); }
    async fn wait(&self) {
        loop { let wait = self.notify.notified(); if self.closed.load(Ordering::Acquire) { return; } wait.await; }
    }
}
pub struct NativeWatch {
    pub events: Arc<Rendezvous<()>>,
    pub errors: Arc<Rendezvous<BoxError>>,
    pub cancel: CancellationToken,
    completion: Arc<CloseCompletion>,
    explicitly_cancelled: AtomicBool,
}
impl NativeWatch {
    pub async fn cancel(&self) {
        if self.explicitly_cancelled.swap(true, Ordering::AcqRel) { panic!("close of closed channel"); }
        self.cancel.cancel();
        self.completion.wait().await;
    }
    pub fn cancel_on_owner_drop(&self) { self.cancel.cancel(); }
}

#[cgp_component(FileWatcherOpener)]
pub trait CanOpenFileWatcher { fn open_file_watcher(&self, path: &Path) -> Result<Arc<NativeWatch>, BoxError>; }
pub struct OpenNativeFileWatcher;

#[cfg(windows)]
#[path = "utility_fswatch_windows.rs"]
mod windows;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux {
    use super::*;
    use std::{ffi::{CString, OsStr}, io, os::{fd::{AsRawFd, FromRawFd, OwnedFd}, unix::ffi::OsStrExt}, path::PathBuf};
    use tokio::io::unix::AsyncFd;

    struct OwnedRead {
        fd: Option<AsyncFd<OwnedFd>>,
        watch: Arc<NativeWatch>,
        path: PathBuf,
        descriptor: i32,
        known_descriptor: bool,
    }
    impl Drop for OwnedRead {
        fn drop(&mut self) {
            // Complete only after deregistration and fd close, including
            // startup failures, cancellation and a panicking callback owner.
            drop(self.fd.take());
            self.watch.events.close(); self.watch.errors.close(); self.watch.completion.finish();
        }
    }
    impl OwnedRead {
        async fn send_error(&self, error: BoxError) -> bool {
            tokio::select! { _ = self.watch.cancel.cancelled() => false, sent = self.watch.errors.send(error) => sent }
        }
        async fn run(mut self) -> Result<(), BoxError> {
            let mut bytes = vec![0u8; 16 * 4096];
            loop {
                let count = tokio::select! {
                    _ = self.watch.cancel.cancelled() => return Ok(()),
                    result = read_fd(self.fd.as_ref().expect("owned inotify fd"), &mut bytes) => match result {
                        Ok(count) => count,
                        Err(error) if error.raw_os_error() == Some(libc::EINTR) => continue,
                        Err(error) if error.raw_os_error() == Some(libc::EBADF) => return Ok(()),
                        Err(error) => { if !self.send_error(Box::new(error)).await { return Ok(()); } continue; }
                    },
                };
                if count == 0 { return Ok(()); }
                if count < 16 {
                    self.send_error(Errors::default().New("unexpected EOF")).await;
                    return Ok(());
                }
                let mut offset = 0;
                while offset + 16 <= count {
                    // Kernel records are native-endian. Decode bytes without
                    // casting an unaligned userspace buffer to inotify_event.
                    let descriptor = i32::from_ne_bytes(bytes[offset..offset+4].try_into().expect("four bytes"));
                    let mask = u32::from_ne_bytes(bytes[offset+4..offset+8].try_into().expect("four bytes"));
                    let length = u32::from_ne_bytes(bytes[offset+12..offset+16].try_into().expect("four bytes")) as usize;
                    if offset + 16 + length > count {
                        self.send_error(Errors::default().New("unexpected EOF")).await; return Ok(());
                    }
                    if mask & libc::IN_Q_OVERFLOW != 0 {
                        if !self.send_error(Errors::default().New(EVENT_OVERFLOW)).await { return Ok(()); }
                    }
                    let mut path = if descriptor == self.descriptor && self.known_descriptor { self.path.clone() } else { PathBuf::new() };
                    if descriptor == self.descriptor && mask & libc::IN_DELETE_SELF != 0 { self.known_descriptor = false; }
                    if length > 0 {
                        let name = &bytes[offset+16..offset+16+length];
                        let end = name.iter().rposition(|value| *value != 0).map_or(0, |index| index+1);
                        if path.as_os_str().is_empty() { path.push("/"); }
                        path.push(OsStr::from_bytes(&name[..end]));
                    }
                    let remove_or_rename = mask & (libc::IN_DELETE | libc::IN_DELETE_SELF | libc::IN_MOVED_FROM | libc::IN_MOVE_SELF) != 0;
                    let missing = mask & libc::IN_IGNORED == 0 && !remove_or_rename
                        && std::fs::symlink_metadata(&path).err().is_some_and(|error| error.kind() == io::ErrorKind::NotFound);
                    if mask & libc::IN_IGNORED == 0 && (remove_or_rename || !missing) {
                        let delivered = tokio::select! { _ = self.watch.cancel.cancelled() => false, sent = self.watch.events.send(()) => sent };
                        if !delivered { return Ok(()); }
                    }
                    offset += 16 + length;
                }
            }
        }
    }
    async fn read_fd(fd: &AsyncFd<OwnedFd>, bytes: &mut [u8]) -> io::Result<usize> {
        loop {
            let mut ready = fd.readable().await?;
            match ready.try_io(|inner| {
                // SAFETY: the AsyncFd owns a live descriptor, and bytes is a
                // valid writable buffer for exactly its advertised length.
                let count = unsafe { libc::read(inner.get_ref().as_raw_fd(), bytes.as_mut_ptr().cast(), bytes.len()) };
                if count < 0 { Err(io::Error::last_os_error()) } else { Ok(count as usize) }
            }) {
                Ok(result) => return result,
                Err(_) => continue,
            }
        }
    }

    #[cgp_provider(FileWatcherOpenerComponent)]
    impl FileWatcherOpener<UtilityRuntimeContext> for OpenNativeFileWatcher {
        fn open_file_watcher(context: &UtilityRuntimeContext, path: &Path) -> Result<Arc<NativeWatch>, BoxError> {
            let path = clean_linux_path(path);
            let c_path = CString::new(path.as_os_str().as_bytes()).map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;
            // SAFETY: no pointers are passed; a successful new fd is given one
            // OwnedFd owner immediately, including every later failure path.
            let raw = unsafe { libc::inotify_init1(libc::IN_NONBLOCK | libc::IN_CLOEXEC) };
            if raw < 0 { return Err(Box::new(io::Error::last_os_error())); }
            let fd = unsafe { OwnedFd::from_raw_fd(raw) };
            let mask = libc::IN_MOVED_TO | libc::IN_MOVED_FROM | libc::IN_CREATE | libc::IN_ATTRIB
                | libc::IN_MODIFY | libc::IN_MOVE_SELF | libc::IN_DELETE | libc::IN_DELETE_SELF;
            // SAFETY: fd and CString are live, and the path is NUL terminated.
            let descriptor = unsafe { libc::inotify_add_watch(fd.as_raw_fd(), c_path.as_ptr(), mask) };
            if descriptor < 0 { return Err(Box::new(io::Error::last_os_error())); }
            let fd = AsyncFd::new(fd)?;
            let watch = Arc::new(NativeWatch { events: Arc::default(), errors: Arc::default(),
                cancel: context.utility_child_token()?, completion: Arc::default(), explicitly_cancelled: AtomicBool::new(false) });
            let reader = OwnedRead { fd: Some(fd), watch: Arc::clone(&watch), path, descriptor, known_descriptor: true };
            context.spawn_utility_task("fswatch kernel reader", Box::pin(reader.run()))?;
            Ok(watch)
        }
    }
    fn clean_linux_path(path: &Path) -> PathBuf {
        use std::path::Component;
        let mut result = PathBuf::new();
        for part in path.components() {
            match part {
                Component::CurDir => (),
                Component::ParentDir if result.file_name().is_some_and(|name| name != "..") => { result.pop(); }
                Component::ParentDir if result.has_root() => (),
                _ => result.push(part.as_os_str()),
            }
        }
        if result.as_os_str().is_empty() { PathBuf::from(".") } else { result }
    }
}
