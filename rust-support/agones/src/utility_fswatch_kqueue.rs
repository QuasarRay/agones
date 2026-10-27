//! gopkg.in/fsnotify.v1 1.4.7 kqueue policy in the existing CGP watcher phase.
// Algorithms adapted from fsnotify v1 and Go filepath.walkSymlinks.
// Copyright The Go Authors. BSD-3-Clause; see ../FSNOTIFY-V1-LICENSE
// and ../GO-LICENSE. This is the pinned utility dependency, not fsnotify 1.9.
use super::*;
use std::{collections::{HashMap, HashSet}, ffi::{CString, OsStr, OsString}, fs::{self, Metadata}, io,
    os::{fd::{AsRawFd, FromRawFd, OwnedFd}, unix::{ffi::OsStrExt, fs::FileTypeExt}}, path::PathBuf};
use tokio::io::unix::AsyncFd;

const ALL: u32 = libc::NOTE_DELETE | libc::NOTE_WRITE | libc::NOTE_ATTRIB | libc::NOTE_RENAME;
struct Entry { file: Option<OwnedFd>, descriptor: i32, name: PathBuf, directory: bool, directory_flags: u32 }
struct Reader {
    queue: Option<AsyncFd<OwnedFd>>, entries: HashMap<i32, Entry>, paths: HashMap<OsString, i32>,
    seen: HashSet<OsString>, user: HashSet<OsString>, watch: Arc<NativeWatch>,
}
impl Reader {
    fn descriptor(&self) -> i32 { self.queue.as_ref().expect("owned kqueue").get_ref().as_raw_fd() }
    fn register(&self, fd: i32, flags: u32, notes: u32) -> io::Result<()> {
        let mut event: libc::kevent = unsafe { std::mem::zeroed() };
        event.ident = fd as _; event.filter = libc::EVFILT_VNODE as _; event.flags = flags as _; event.fflags = notes;
        let count = unsafe { libc::kevent(self.descriptor(), &event, 1, std::ptr::null_mut(), 0, std::ptr::null()) };
        if count < 0 { Err(io::Error::last_os_error()) } else { Ok(()) }
    }
    fn by_path(&self, path: &Path) -> Option<&Entry> { self.paths.get(path.as_os_str()).and_then(|fd| self.entries.get(fd)) }
    fn add(&mut self, requested: &Path, flags: u32) -> Result<PathBuf, BoxError> {
        let mut name = clean_unix_path(requested);
        if self.watch.cancel.is_cancelled() { return Err(Errors::default().New("kevent instance already closed")); }
        let mut existing = self.by_path(&name).map(|entry| entry.descriptor);
        let mut fresh = None;
        let descriptor = if let Some(fd) = existing { fd } else {
            let mut metadata = fs::symlink_metadata(&name)?;
            if metadata.file_type().is_socket() || metadata.file_type().is_fifo() { return Ok(PathBuf::new()); }
            // v1 follows internal and explicit symlinks, resolves the entire
            // chain, and suppresses every EvalSymlinks or following Lstat error.
            if metadata.file_type().is_symlink() {
                name = match eval_symlinks(&name) { Ok(name) => name, Err(_) => return Ok(PathBuf::new()) };
                existing = self.by_path(&name).map(|entry| entry.descriptor);
                if existing.is_some() { return Ok(name); }
                metadata = match fs::symlink_metadata(&name) { Ok(metadata) => metadata, Err(_) => return Ok(PathBuf::new()) };
            }
            let path = CString::new(name.as_os_str().as_bytes()).map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            let mode = libc::O_EVTONLY;
            #[cfg(not(any(target_os = "macos", target_os = "ios")))]
            let mode = libc::O_NONBLOCK | libc::O_RDONLY;
            let raw = unsafe { libc::open(path.as_ptr(), mode, 0o700) };
            if raw < 0 { return Err(Box::new(io::Error::last_os_error())); }
            let file = unsafe { OwnedFd::from_raw_fd(raw) };
            fresh = Some(Entry { file: Some(file), descriptor: raw, name: name.clone(), directory: metadata.is_dir(), directory_flags: 0 });
            raw
        };
        if let Err(error) = self.register(descriptor, (libc::EV_ADD | libc::EV_CLEAR | libc::EV_ENABLE) as u32, flags) {
            // Source closes even an already-watched fd and leaves stale maps.
            // Keep that closed state explicit without giving a stale number a
            // second OwnedFd owner. Reused-number failure policy remains open.
            if let Some(entry) = self.entries.get_mut(&descriptor) { drop(entry.file.take()); }
            return Err(Box::new(error));
        }
        if let Some(entry) = fresh { self.paths.insert(name.as_os_str().to_owned(), descriptor); self.entries.insert(descriptor, entry); }
        let entry = self.entries.get_mut(&descriptor).expect("registered source watch");
        let list = entry.directory && flags & libc::NOTE_WRITE != 0 && (existing.is_none() || entry.directory_flags & libc::NOTE_WRITE == 0);
        if entry.directory { entry.directory_flags = flags; }
        if list {
            for (path, metadata) in directory_infos(&name)? {
                let path = self.internal_watch(&path, &metadata)?; self.seen.insert(path.into_os_string());
            }
        }
        Ok(name)
    }
    fn remove(&mut self, requested: &Path) -> Result<(), BoxError> {
        let name = clean_unix_path(requested);
        let Some(fd) = self.paths.get(name.as_os_str()).copied() else {
            return Err(Errors::default().New(format!("can't remove non-existent kevent watch for: {}", name.display())));
        };
        self.register(fd, libc::EV_DELETE as u32, 0)?;
        let entry = self.entries.remove(&fd).expect("source watch entry");
        let directory = entry.directory; drop(entry); self.paths.remove(name.as_os_str());
        // v1 Remove keeps externalWatches and fileExists; readEvents deletes
        // the removed event's fileExists entry separately.
        if directory {
            let children: Vec<_> = self.entries.values().filter(|entry|
                clean_unix_path(entry.name.parent().unwrap_or(Path::new("."))) == name
                && !self.user.contains(entry.name.as_os_str())).map(|entry| entry.name.clone()).collect();
            for child in children { let _ = self.remove(&child); }
        }
        Ok(())
    }
    fn internal_watch(&mut self, path: &Path, metadata: &Metadata) -> Result<PathBuf, BoxError> {
        let flags = if metadata.is_dir() { self.by_path(path).map_or(0, |entry| entry.directory_flags) | libc::NOTE_DELETE | libc::NOTE_RENAME } else { ALL };
        self.add(path, flags)
    }
    async fn send_event(&self) -> bool { tokio::select! { _ = self.watch.cancel.cancelled() => false, sent = self.watch.events.send(()) => sent } }
    async fn send_error(&self, error: BoxError) -> bool { tokio::select! { _ = self.watch.cancel.cancelled() => false, sent = self.watch.errors.send(error) => sent } }
    async fn create_if_new(&mut self, path: &Path, metadata: &Metadata) -> Result<(), BoxError> {
        if !self.seen.contains(path.as_os_str()) && !self.send_event().await { return Ok(()); }
        let name = self.internal_watch(path, metadata)?; self.seen.insert(name.into_os_string()); Ok(())
    }
    async fn directory_change(&mut self, directory: &Path) {
        let files = match directory_infos(directory) {
            Ok(files) => files, Err(error) => { self.send_error(error).await; return; }
        };
        for (path, metadata) in files { if self.create_if_new(&path, &metadata).await.is_err() { return; } }
    }
    async fn run(mut self) -> Result<(), BoxError> {
        loop {
            let events = tokio::select! {
                _ = self.watch.cancel.cancelled() => return Ok(()),
                result = read_events(self.queue.as_ref().expect("owned kqueue")) => match result {
                    Ok(events) => events, Err(error) if error.raw_os_error() == Some(libc::EINTR) => continue,
                    Err(error) => { if !self.send_error(Box::new(error)).await { return Ok(()); } continue; }
                }
            };
            for (descriptor, flags) in events {
                let (name, directory) = self.entries.get(&descriptor).map_or_else(|| (PathBuf::new(), false), |entry| (entry.name.clone(), entry.directory));
                let mut remove = flags & libc::NOTE_DELETE != 0;
                if directory && !remove && fs::symlink_metadata(&name).err().is_some_and(|error| error.kind() == io::ErrorKind::NotFound) { remove = true; }
                if remove || flags & libc::NOTE_RENAME != 0 { let _ = self.remove(&name); self.seen.remove(name.as_os_str()); }
                if directory && flags & libc::NOTE_WRITE != 0 && !remove { self.directory_change(&name).await; }
                else if !self.send_event().await { return Ok(()); } // v1 also sends an Op==0 event
                if remove {
                    let name = clean_unix_path(&name);
                    if directory {
                        if self.by_path(&name).is_some() && fs::symlink_metadata(&name).is_ok() { self.directory_change(&name).await; }
                    } else if let Ok(metadata) = fs::symlink_metadata(&name) { let _ = self.create_if_new(&name, &metadata).await; }
                }
            }
        }
    }
}
impl Drop for Reader {
    fn drop(&mut self) {
        self.entries.clear(); self.paths.clear(); drop(self.queue.take());
        self.watch.events.close(); self.watch.errors.close(); self.watch.completion.finish();
    }
}
fn directory_infos(directory: &Path) -> Result<Vec<(PathBuf, Metadata)>, BoxError> {
    // ioutil.ReadDir/Readdir obtains all FileInfo values before sorting and
    // before fsnotify changes any watches. Disappeared entries are skipped.
    let mut files = Vec::new();
    for entry in fs::read_dir(directory)? {
        let path = clean_unix_path(&directory.join(entry?.file_name()));
        match fs::symlink_metadata(&path) {
            Ok(metadata) => files.push((path, metadata)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => (),
            Err(error) => return Err(Box::new(error)),
        }
    }
    files.sort_by(|left, right| left.0.file_name().cmp(&right.0.file_name())); Ok(files)
}
fn eval_symlinks(requested: &Path) -> Result<PathBuf, BoxError> {
    let mut path = requested.as_os_str().as_bytes().to_vec();
    let mut volume = usize::from(path.starts_with(b"/"));
    let mut dest = path[..volume].to_vec(); let mut start = volume; let mut links = 0;
    while start < path.len() {
        while start < path.len() && path[start] == b'/' { start += 1; }
        let end = path[start..].iter().position(|byte| *byte == b'/').map_or(path.len(), |index| start + index);
        if start == end { break; }
        let component = &path[start..end]; start = end;
        if component == b"." { continue; }
        if component == b".." {
            let slash = dest[volume..].iter().rposition(|byte| *byte == b'/').map(|index| volume + index);
            if let Some(slash) = slash.filter(|slash| &dest[slash+1..] != b"..") { dest.truncate(slash); }
            else { if dest.len() > volume { dest.push(b'/'); } dest.extend_from_slice(b".."); }
            continue;
        }
        if !dest.is_empty() && !dest.ends_with(b"/") { dest.push(b'/'); }
        dest.extend_from_slice(component);
        let metadata = fs::symlink_metadata(Path::new(OsStr::from_bytes(&dest)))?;
        if !metadata.file_type().is_symlink() {
            if !metadata.is_dir() && end < path.len() { return Err(Box::new(io::Error::from_raw_os_error(libc::ENOTDIR))); }
            continue;
        }
        links += 1; if links > 255 { return Err(Errors::default().New("EvalSymlinks: too many links")); }
        let link = fs::read_link(Path::new(OsStr::from_bytes(&dest)))?;
        let mut pending = link.as_os_str().as_bytes().to_vec(); pending.extend_from_slice(&path[end..]);
        if link.is_absolute() { dest = vec![b'/']; volume = 1; start = 1; }
        else {
            let slash = dest[volume..].iter().rposition(|byte| *byte == b'/').map(|index| volume + index);
            dest.truncate(slash.unwrap_or(volume)); start = 0;
        }
        path = pending;
    }
    Ok(clean_unix_path(Path::new(OsStr::from_bytes(&dest))))
}
async fn read_events(queue: &AsyncFd<OwnedFd>) -> io::Result<Vec<(i32, u32)>> {
    loop {
        let mut ready = queue.readable().await?;
        let result = ready.try_io(|queue| {
            let mut events: [libc::kevent; 10] = unsafe { std::mem::zeroed() };
            let timeout = libc::timespec { tv_sec: 0, tv_nsec: 0 };
            let count = unsafe { libc::kevent(queue.get_ref().as_raw_fd(), std::ptr::null(), 0, events.as_mut_ptr(), events.len() as _, &timeout) };
            if count < 0 { return Err(io::Error::last_os_error()); }
            if count == 0 { return Err(io::Error::from(io::ErrorKind::WouldBlock)); }
            Ok(events[..count as usize].iter().map(|event| (event.ident as i32, event.fflags)).collect())
        });
        match result { Ok(result) => return result, Err(_) => continue }
    }
}
#[cgp_provider(FileWatcherOpenerComponent)]
impl FileWatcherOpener<UtilityRuntimeContext> for OpenNativeFileWatcher {
    fn open_file_watcher(context: &UtilityRuntimeContext, path: &Path) -> Result<Arc<NativeWatch>, BoxError> {
        let raw = unsafe { libc::kqueue() };
        if raw < 0 { return Err(Box::new(io::Error::last_os_error())); }
        let queue = unsafe { OwnedFd::from_raw_fd(raw) };
        // SAFETY: this private OwnedFd remains open and unchanged until its
        // registration is dropped. No borrowed descriptor is closed here.
        let queue = unsafe { AsyncFd::register_with_interest(queue, tokio::io::Interest::READABLE) }.map_err(io::Error::from)?;
        let watch = Arc::new(NativeWatch { events: Arc::default(), errors: Arc::default(), cancel: context.utility_child_token()?,
            completion: Arc::default(), explicitly_cancelled: AtomicBool::new(false) });
        let mut reader = Reader { queue: Some(queue), entries: HashMap::new(), paths: HashMap::new(), seen: HashSet::new(), user: HashSet::new(), watch: Arc::clone(&watch) };
        reader.user.insert(path.as_os_str().to_owned()); reader.add(path, ALL)?;
        context.spawn_utility_task("kqueue filesystem reader", Box::pin(reader.run()))?; Ok(watch)
    }
}
