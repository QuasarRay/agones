//! Source fsnotify Windows effects behind the existing CGP watcher owner.
//! One overlapped read and one OS wait registration belong to each watcher.
//! No detached Rust thread or alternative filesystem manager is introduced.
// Source policy adapted from fsnotify. Copyright The Go Authors and fsnotify
// Authors. BSD-3-Clause; see ../FSNOTIFY-LICENSE.
use super::*;
use std::{cell::UnsafeCell, ffi::c_void, io, os::windows::{ffi::OsStrExt, io::{AsRawHandle, FromRawHandle, OwnedHandle}},
    path::Path, pin::Pin, task::Poll};
use futures::{future::poll_fn, task::AtomicWaker};
use windows_sys::Win32::{Foundation::{HANDLE, INVALID_HANDLE_VALUE, ERROR_ACCESS_DENIED, ERROR_IO_INCOMPLETE,
    ERROR_IO_PENDING, ERROR_OPERATION_ABORTED, ERROR_MORE_DATA}, Storage::FileSystem::{CreateFileW,
    GetFileAttributesW, GetFileInformationByHandle, ReadDirectoryChangesW, BY_HANDLE_FILE_INFORMATION,
    FILE_ATTRIBUTE_DIRECTORY, INVALID_FILE_ATTRIBUTES, FILE_LIST_DIRECTORY, FILE_SHARE_READ, FILE_SHARE_WRITE,
    FILE_SHARE_DELETE, OPEN_EXISTING, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OVERLAPPED,
    FILE_NOTIFY_CHANGE_LAST_WRITE, FILE_NOTIFY_CHANGE_FILE_NAME, FILE_NOTIFY_CHANGE_DIR_NAME,
    FILE_ACTION_MODIFIED, FILE_ACTION_REMOVED, FILE_ACTION_RENAMED_OLD_NAME, FILE_ACTION_RENAMED_NEW_NAME},
    System::{IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED}, Threading::{CreateEventW,
        RegisterWaitForSingleObject, UnregisterWaitEx, WT_EXECUTEDEFAULT, INFINITE}}};

#[derive(Default)]
struct CompletionBridge { ready: AtomicBool, waker: AtomicWaker }
impl CompletionBridge {
    async fn wait(&self) {
        poll_fn(|context| {
            self.waker.register(context.waker());
            if self.ready.swap(false, Ordering::AcqRel) { Poll::Ready(()) } else { Poll::Pending }
        }).await
    }
}
unsafe extern "system" fn completed(context: *mut c_void, timed_out: bool) {
    // The registration owner keeps this Arc allocation alive and synchronously
    // unregisters/waits for callbacks before releasing it.
    let bridge = unsafe { &*(context.cast::<CompletionBridge>()) };
    if !timed_out { bridge.ready.store(true, Ordering::Release); bridge.waker.wake(); }
}
struct WaitRegistration(HANDLE);
// A wait registration may be unregistered on any thread; it is never copied.
unsafe impl Send for WaitRegistration {}
impl Drop for WaitRegistration {
    fn drop(&mut self) {
        if unsafe { UnregisterWaitEx(self.0, INVALID_HANDLE_VALUE) } == 0 {
            // Releasing the callback context after a failed unregister could
            // leave the OS dereferencing freed memory. Keep failure explicit.
            eprintln!("fatal filesystem completion wait cleanup failure: {}", io::Error::last_os_error());
            std::process::abort();
        }
    }
}
struct KernelRead { overlapped: UnsafeCell<OVERLAPPED>, bytes: UnsafeCell<[u8; 65536]> }
// The pinned allocation can move between executor threads, but the kernel is
// its only writer while pending. Rust reads bytes only after I/O completion.
unsafe impl Send for KernelRead {}
struct Reader {
    file: Option<OwnedHandle>,
    event: Option<OwnedHandle>,
    registration: Option<WaitRegistration>,
    operation: Pin<Box<KernelRead>>,
    bridge: Arc<CompletionBridge>,
    watch: Arc<NativeWatch>,
    directory: bool,
    name: Option<String>,
    renamed_from: String,
    pending: bool,
}
impl Reader {
    fn handle(&self) -> HANDLE { self.file.as_ref().expect("owned directory handle").as_raw_handle().cast() }
    fn overlap(&self) -> *mut OVERLAPPED { self.operation.as_ref().get_ref().overlapped.get() }
    fn start_read(&mut self) -> io::Result<()> {
        self.bridge.ready.store(false, Ordering::Release);
        let flags = FILE_NOTIFY_CHANGE_LAST_WRITE | FILE_NOTIFY_CHANGE_FILE_NAME | FILE_NOTIFY_CHANGE_DIR_NAME;
        // UnsafeCell grants the kernel exclusive write access until completion.
        let bytes = self.operation.as_ref().get_ref().bytes.get().cast::<c_void>();
        let started = unsafe { ReadDirectoryChangesW(self.handle(), bytes, 65536, 0, flags,
            std::ptr::null_mut(), self.overlap(), None) };
        if started == 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() != Some(ERROR_IO_PENDING as i32) { return Err(error); }
        }
        self.pending = true;
        Ok(())
    }
    fn result(&mut self, wait: bool) -> io::Result<usize> {
        let mut count = 0;
        let result = unsafe { GetOverlappedResult(self.handle(), self.overlap(), &mut count, i32::from(wait)) };
        if result == 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() != Some(ERROR_IO_INCOMPLETE as i32) { self.pending = false; }
            return Err(error);
        }
        self.pending = false;
        Ok(count as usize)
    }
    async fn cancel_read(&mut self) {
        if !self.pending { return; }
        unsafe { CancelIoEx(self.handle(), self.overlap()); }
        loop {
            match self.result(false) {
                Err(error) if error.raw_os_error() == Some(ERROR_IO_INCOMPLETE as i32) => self.bridge.wait().await,
                _ => return,
            }
        }
    }
    async fn send_event(&mut self) -> bool {
        tokio::select! { _ = self.watch.cancel.cancelled() => false, result = self.watch.events.send(()) => result }
    }
    async fn send_error(&mut self, error: BoxError) -> bool {
        tokio::select! { _ = self.watch.cancel.cancelled() => false, result = self.watch.errors.send(error) => result }
    }
    async fn removed(&mut self) {
        if self.directory { self.send_event().await; }
        self.release_handles();
        // A removed watch stops delivering, but the fsnotify watcher channels
        // remain open until Close. Do not synthesize repeated errors/events.
        self.watch.cancel.cancelled().await;
    }
    async fn run(mut self) -> Result<(), BoxError> {
        loop {
            tokio::select! {
                _ = self.watch.cancel.cancelled() => { self.cancel_read().await; return Ok(()); },
                _ = self.bridge.wait() => {},
            }
            let count = match self.result(false) {
                Ok(count) => count,
                Err(error) if error.raw_os_error() == Some(ERROR_IO_INCOMPLETE as i32) => continue,
                Err(error) if error.raw_os_error() == Some(ERROR_OPERATION_ABORTED as i32) => {
                    if self.watch.cancel.is_cancelled() { return Ok(()); }
                    self.start_read()?; continue;
                }
                Err(error) if error.raw_os_error() == Some(ERROR_ACCESS_DENIED as i32) => { self.removed().await; return Ok(()); },
                Err(error) if error.raw_os_error() == Some(ERROR_MORE_DATA as i32) => {
                    // The pinned source overwrites n with sizeof(slice), not
                    // the buffer length. Preserve that source branch explicitly.
                    3 * std::mem::size_of::<usize>()
                }
                Err(error) => {
                    if !self.send_error(Box::new(error)).await { return Ok(()); }
                    // Go's reader waits for another completion after this
                    // terminal operation error; it does not invent a retry.
                    self.watch.cancel.cancelled().await; return Ok(());
                }
            };
            if count == 0 {
                if !self.send_error(Errors::default().New(EVENT_OVERFLOW)).await { return Ok(()); }
            } else if !self.deliver(count).await? { return Ok(()); }
            if !self.directory && self.name.is_none() {
                self.release_handles();
                self.watch.cancel.cancelled().await; return Ok(());
            }
            if let Err(error) = self.start_read() {
                if error.raw_os_error() == Some(ERROR_ACCESS_DENIED as i32) { self.removed().await; return Ok(()); }
                self.release_handles();
                self.send_error(Box::new(error)).await;
                self.watch.cancel.cancelled().await; return Ok(());
            }
        }
    }
    async fn deliver(&mut self, count: usize) -> Result<bool, BoxError> {
        let mut offset = 0;
        loop {
            // Kernel writes have completed; no read is reissued during sends.
            let bytes = unsafe { &*self.operation.as_ref().get_ref().bytes.get() };
            if offset + 12 > bytes.len() { return Err(Errors::default().New("Windows filesystem notification header exceeds owned buffer")); }
            let field = |start| u32::from_ne_bytes(bytes[start..start + 4].try_into().expect("four notification bytes"));
            let next = field(offset) as usize; let action = field(offset + 4); let size = field(offset + 8) as usize;
            if size % 2 != 0 || offset + 12 + size > bytes.len() { return Err(Errors::default().New("Windows filesystem notification name exceeds owned buffer")); }
            let name: Vec<_> = bytes[offset + 12..offset + 12 + size].chunks_exact(2).map(|pair| u16::from_ne_bytes([pair[0], pair[1]])).collect();
            let length = name.iter().position(|word| *word == 0).unwrap_or(name.len());
            let name = String::from_utf16_lossy(&name[..length]);
            let mut file_event = false;
            match action {
                FILE_ACTION_MODIFIED => file_event = self.name.as_ref() == Some(&name),
                FILE_ACTION_REMOVED => if self.name.as_ref() == Some(&name) { file_event = true; self.name = None; },
                FILE_ACTION_RENAMED_OLD_NAME => self.renamed_from = name.clone(),
                FILE_ACTION_RENAMED_NEW_NAME => if self.name.as_ref() == Some(&self.renamed_from) { self.name = Some(name.clone()); file_event = true; },
                _ => {},
            }
            let directory_event = self.directory && (1..=5).contains(&action);
            if (file_event || directory_event) && !self.send_event().await { return Ok(false); }
            if next == 0 { return Ok(true); }
            offset += next;
            if offset >= count {
                return Ok(self.send_error(Errors::default().New("Windows system assumed buffer larger than it is, events have likely been missed")).await);
            }
        }
    }
    fn release_handles(&mut self) {
        assert!(!self.pending, "drain filesystem I/O before releasing its handles");
        drop(self.registration.take()); drop(self.bridge.waker.take());
        drop(self.file.take()); drop(self.event.take());
    }
}
impl Drop for Reader {
    fn drop(&mut self) {
        // Emergency Drop cannot await. Cancel and synchronously drain the sole
        // outstanding kernel operation before releasing its pinned memory.
        if self.pending {
            unsafe { CancelIoEx(self.handle(), self.overlap()); }
            // Stop the callback wait before using the event for a synchronous
            // drain. Two waiters must not compete for its auto-reset signal.
            drop(self.registration.take());
            let _ = self.result(true);
            if self.pending { eprintln!("fatal pending filesystem I/O could not drain"); std::process::abort(); }
        }
        self.release_handles();
        self.watch.events.close(); self.watch.errors.close(); self.watch.completion.finish();
    }
}
fn wide(path: &Path) -> io::Result<Vec<u16>> {
    let mut result: Vec<_> = path.as_os_str().encode_wide().collect();
    if result.contains(&0) { return Err(io::Error::new(io::ErrorKind::InvalidInput, "path contains NUL")); }
    result.push(0); Ok(result)
}
#[cgp_provider(FileWatcherOpenerComponent)]
impl FileWatcherOpener<UtilityRuntimeContext> for OpenNativeFileWatcher {
    fn open_file_watcher(context: &UtilityRuntimeContext, path: &Path) -> Result<Arc<NativeWatch>, BoxError> {
        let path_wide = wide(path)?;
        let attributes = unsafe { GetFileAttributesW(path_wide.as_ptr()) };
        if attributes == INVALID_FILE_ATTRIBUTES { return Err(Box::new(io::Error::last_os_error())); }
        let directory = attributes & FILE_ATTRIBUTE_DIRECTORY != 0;
        let parent = if directory { path.to_path_buf() } else { path.parent().filter(|parent| !parent.as_os_str().is_empty()).unwrap_or(Path::new(".")).to_path_buf() };
        let parent_wide = wide(&parent)?;
        let file = unsafe { CreateFileW(parent_wide.as_ptr(), FILE_LIST_DIRECTORY,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE, std::ptr::null(), OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OVERLAPPED, std::ptr::null_mut()) };
        if file == INVALID_HANDLE_VALUE { return Err(Box::new(io::Error::last_os_error())); }
        let file = unsafe { OwnedHandle::from_raw_handle(file.cast()) };
        let mut information = BY_HANDLE_FILE_INFORMATION::default();
        if unsafe { GetFileInformationByHandle(file.as_raw_handle().cast(), &mut information) } == 0 { return Err(Box::new(io::Error::last_os_error())); }
        let event = unsafe { CreateEventW(std::ptr::null(), 0, 0, std::ptr::null()) };
        if event.is_null() { return Err(Box::new(io::Error::last_os_error())); }
        let event = unsafe { OwnedHandle::from_raw_handle(event.cast()) };
        let bridge = Arc::new(CompletionBridge::default());
        let mut wait = std::ptr::null_mut();
        if unsafe { RegisterWaitForSingleObject(&mut wait, event.as_raw_handle().cast(), Some(completed), Arc::as_ptr(&bridge).cast(), INFINITE, WT_EXECUTEDEFAULT) } == 0 {
            return Err(Box::new(io::Error::last_os_error()));
        }
        let registration = WaitRegistration(wait);
        let watch = Arc::new(NativeWatch { events: Arc::new(Rendezvous::with_capacity(50)), errors: Arc::default(),
            cancel: context.utility_child_token()?, completion: Arc::default(), explicitly_cancelled: AtomicBool::new(false) });
        let operation = Box::pin(KernelRead { overlapped: UnsafeCell::new(OVERLAPPED { hEvent: event.as_raw_handle().cast(), ..OVERLAPPED::default() }), bytes: UnsafeCell::new([0; 65536]) });
        let mut reader = Reader { file: Some(file), event: Some(event), registration: Some(registration), operation, bridge, watch: Arc::clone(&watch), directory,
            name: if directory { None } else { path.file_name().map(|name| name.to_string_lossy().into_owned()) }, renamed_from: String::new(), pending: false };
        reader.start_read()?;
        context.spawn_utility_task("Windows filesystem completion reader", Box::pin(reader.run()))?;
        Ok(watch)
    }
}
