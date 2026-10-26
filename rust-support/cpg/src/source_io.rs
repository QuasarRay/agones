//! Filesystem effects and resources owned by one CGP build phase.
//! Scala Source readers remain owned until phase teardown; sbt Using readers
//! and writers close in finally. Preserve those distinct failure/close policies.
use std::{
    fs::{self, File, ReadDir}, io::{Read, Seek, Write}, path::{Path, PathBuf},
    sync::{Mutex, MutexGuard}, time::Duration,
};
use cgp::prelude::*;
use crate::{codec, Error, Result};

fn locked<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    // Java monitors are released after an exception and have no poison flag.
    mutex.lock().unwrap_or_else(|error| error.into_inner())
}

#[cgp_getter]
pub trait HasBuildIo { fn build_io(&self) -> &BuildIo; }

pub struct BuildIo {
    project_root: PathBuf,
    walks: Mutex<Vec<NativeWalk>>,
    readers: Mutex<Vec<OpenReader>>,
    directory_retry_limit: i32,
}
impl Default for BuildIo { fn default() -> Self { Self::at(Path::new(".")) } }

struct DirectoryFrame { source_path: PathBuf, entries: ReadDir }
struct NativeWalk { first: Option<PathBuf>, directories: Vec<DirectoryFrame>, root: PathBuf }

impl NativeWalk {
    fn new(root: &Path, source: &Path) -> Result<Self> {
        let path = resolve(root, source);
        let metadata = fs::symlink_metadata(&path).map_err(|error| Error::nio(source, error, false))?;
        let directories = if metadata.is_dir() {
            vec![DirectoryFrame { source_path: source.to_owned(),
                entries: fs::read_dir(&path).map_err(|error| Error::nio(source, error, true))? }]
        } else { Vec::new() };
        Ok(Self { first: Some(source.to_owned()), directories, root: root.to_owned() })
    }
    fn next_path(&mut self) -> Option<Result<PathBuf>> {
        if let Some(root) = self.first.take() { return Some(Ok(root)); }
        while let Some(directory) = self.directories.last_mut() {
            match directory.entries.next() {
                None => { self.directories.pop(); }
                Some(Err(error)) => {
                    let path = directory.source_path.clone();
                    self.directories.pop();
                    return Some(Err(Error::nio(&path, error, false)));
                }
                Some(Ok(entry)) => {
                    let source = directory.source_path.join(entry.file_name());
                    let path = resolve(&self.root, &source);
                    let metadata = match fs::symlink_metadata(&path) {
                        Ok(metadata) => metadata,
                        Err(error) => return Some(Err(Error::nio(&source, error, false))),
                    };
                    // No FOLLOW_LINKS. The source's separate isDirectory filter
                    // follows links, so a directory symlink is not traversed.
                    if metadata.is_dir() {
                        match fs::read_dir(&path) {
                            Ok(entries) => self.directories.push(DirectoryFrame { source_path: source.clone(), entries }),
                            Err(error) => return Some(Err(Error::nio(&source, error, true))),
                        }
                    }
                    return Some(Ok(source));
                }
            }
        }
        None
    }
}

pub struct SourceWalk<'context> { owner: &'context BuildIo, index: usize }
impl Iterator for SourceWalk<'_> {
    type Item = Result<PathBuf>;
    fn next(&mut self) -> Option<Self::Item> {
        locked(&self.owner.walks)[self.index].next_path()
            .map(|entry| entry.map_err(Error::unchecked_io))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReaderOrigin { NioChannel, FileStream }
struct OpenReader { file: Option<File>, origin: ReaderOrigin }
pub struct SourceReader<'context> { owner: &'context BuildIo, index: usize }

fn closed_reader(origin: ReaderOrigin) -> Error {
    match origin {
        ReaderOrigin::NioChannel => Error::exception("java.nio.channels.ClosedChannelException", None),
        ReaderOrigin::FileStream => Error::exception("java.io.IOException", Some("Stream Closed".into())),
    }
}
impl SourceReader<'_> {
    pub fn available(&self) -> Result<u64> {
        let mut readers = locked(&self.owner.readers);
        let row = &mut readers[self.index];
        let origin = row.origin;
        let reader = row.file.as_mut().ok_or_else(|| closed_reader(origin))?;
        // Files.newInputStream uses ChannelInputStream: size minus position,
        // clamped to the positive Java int range, not a terminal ioctl count.
        let size = reader.metadata().map_err(Error::stream_io)?.len();
        let position = reader.stream_position().map_err(Error::stream_io)?;
        Ok(size.saturating_sub(position).min(i32::MAX as u64))
    }
    pub fn read_byte(&self) -> Result<Option<u8>> {
        let mut byte = [0];
        Ok(if self.read_chunk(&mut byte)? == 0 { None } else { Some(byte[0]) })
    }
    fn read_chunk(&self, bytes: &mut [u8]) -> Result<usize> {
        let mut readers = locked(&self.owner.readers);
        let row = &mut readers[self.index];
        let origin = row.origin;
        let file = row.file.as_mut().ok_or_else(|| closed_reader(origin))?;
        loop {
            match file.read(bytes) {
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                result => return result.map_err(Error::stream_io),
            }
        }
    }
    pub fn close(self) -> Result<()> {
        let mut readers = locked(&self.owner.readers);
        let Some(file) = readers[self.index].file.take() else { return Ok(()); };
        #[cfg(unix)]
        {
            use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
            let descriptor = file.into_raw_fd();
            if (0..=2).contains(&descriptor) {
                // Preserve the source FileDescriptor close policy for a file
                // opened after one of the standard descriptors was closed.
                let null = match File::options().write(true).open("/dev/null") {
                    Ok(null) => null,
                    Err(error) => {
                        // SAFETY: into_raw_fd transferred this still-open fd;
                        // restoring its unique File owner prevents a leak.
                        readers[self.index].file = Some(unsafe { File::from_raw_fd(descriptor) });
                        return Err(error.into());
                    }
                };
                // SAFETY: both descriptors are live and owned here. The source
                // discards dup2's return value and keeps descriptors 0..=2 open.
                unsafe { libc::dup2(null.as_raw_fd(), descriptor); }
                return Ok(());
            }
            loop {
                // SAFETY: ownership was removed from File; this is its sole
                // logical close, with no later Rust destructor double-close.
                if unsafe { libc::close(descriptor) } == 0 { return Ok(()); }
                let error = std::io::Error::last_os_error();
                if error.kind() == std::io::ErrorKind::Interrupted {
                    if cfg!(target_os = "aix") { continue; }
                    return Ok(());
                }
                return Err(error.into());
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::io::IntoRawHandle;
            #[link(name = "kernel32")]
            unsafe extern "system" { fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; }
            let handle = file.into_raw_handle();
            // SAFETY: into_raw_handle transferred the sole File ownership.
            if unsafe { CloseHandle(handle) } == 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            return Ok(());
        }
        #[cfg(not(any(unix, windows)))]
        {
            drop(file);
            Ok(())
        }
    }
}
fn resolve(root: &Path, source: &Path) -> PathBuf {
    if source.is_absolute() { source.to_owned() } else { root.join(source) }
}

impl BuildIo {
    pub fn at(project_root: &Path) -> Self {
        Self { project_root: project_root.to_owned(), walks: Mutex::new(Vec::new()),
            readers: Mutex::new(Vec::new()), directory_retry_limit: 10 }
    }
    pub fn project_root(&self) -> &PathBuf { &self.project_root }
    pub fn with_directory_retry_limit(mut self, limit: i32) -> Self { self.directory_retry_limit = limit; self }
    pub fn fresh_generation(&self) -> Self {
        Self::at(&self.project_root).with_directory_retry_limit(self.directory_retry_limit)
    }
    pub fn file_exists(&self, source: &Path) -> bool { resolve(&self.project_root, source).exists() }
    pub fn file_is_directory(&self, source: &Path) -> bool { resolve(&self.project_root, source).is_dir() }
    pub fn file_delete(&self, source: &Path) -> bool {
        let path = resolve(&self.project_root, source);
        #[cfg(unix)]
        {
            let Ok(path) = std::ffi::CString::new(path.as_os_str().as_encoded_bytes()) else { return false; };
            // SAFETY: CString owns a terminated path with no interior NUL.
            // File.delete reports remove's bool outcome without throwing I/O.
            return unsafe { libc::remove(path.as_ptr()) } == 0;
        }
        #[cfg(not(unix))]
        { fs::remove_file(&path).or_else(|_| fs::remove_dir(&path)).is_ok() }
    }
    pub fn walk(&self, source: &Path) -> Result<SourceWalk<'_>> {
        let walk = NativeWalk::new(&self.project_root, source)?;
        let mut walks = locked(&self.walks);
        let index = walks.len(); walks.push(walk);
        Ok(SourceWalk { owner: self, index })
    }
    fn open_reader_at(&self, base: &Path, source: &Path, origin: ReaderOrigin) -> Result<SourceReader<'_>> {
        let path = resolve(base, source);
        let file = File::open(&path).map_err(|error| match origin {
            ReaderOrigin::NioChannel => Error::nio(source, error, false),
            ReaderOrigin::FileStream => Error::file_stream_open(source, error),
        })?;
        if origin == ReaderOrigin::FileStream {
            let metadata = file.metadata().map_err(|error| Error::file_stream_open(source, error))?;
            if metadata.is_dir() {
                #[cfg(unix)]
                let error = std::io::Error::from_raw_os_error(libc::EISDIR);
                #[cfg(not(unix))]
                let error = std::io::Error::from(std::io::ErrorKind::IsADirectory);
                return Err(Error::file_stream_open(source, error));
            }
        }
        let mut readers = locked(&self.readers);
        let index = readers.len(); readers.push(OpenReader { file: Some(file), origin });
        Ok(SourceReader { owner: self, index })
    }
    pub fn open_reader(&self, source: &Path) -> Result<SourceReader<'_>> {
        self.open_reader_at(&self.project_root, source, ReaderOrigin::NioChannel)
    }
    pub fn for_each_source_line_at(&self, base: &Path, source: &Path, mut visit: impl FnMut(&str)) -> Result<()> {
        let reader = self.open_reader_at(base, source, ReaderOrigin::FileStream)?;
        let mut pending = Vec::new();
        let mut line = String::new();
        let mut skip_lf = false;
        let mut bytes = [0; 8192];
        loop {
            let count = reader.read_chunk(&mut bytes)?;
            let text = codec::decode_scala_buffer(&mut pending, &bytes[..count], count == 0)?;
            for value in text.chars() {
                if skip_lf { skip_lf = false; if value == '\n' { continue; } }
                if value == '\r' || value == '\n' {
                    visit(&line); line.clear(); skip_lf = value == '\r';
                } else { line.push(value); }
            }
            if count == 0 { break; }
        }
        if !line.is_empty() { visit(&line); }
        // Source.fromFile(...).getLines has no explicit close in Versions.
        Ok(())
    }
    pub fn read_source_text_at(&self, base: &Path, source: &Path) -> Result<String> {
        let reader = self.open_reader_at(base, source, ReaderOrigin::FileStream)?;
        let bytes = self.read_all(&reader, Vec::new())?;
        codec::decode_scala_source(&bytes)
    }
    pub fn read_source_text(&self, source: &Path) -> Result<String> {
        self.read_source_text_at(&self.project_root, source)
    }
    fn read_all(&self, reader: &SourceReader<'_>, mut bytes: Vec<u8>) -> Result<Vec<u8>> {
        let mut buffer = [0; 8192];
        loop {
            let count = reader.read_chunk(&mut buffer)?;
            if count == 0 { return Ok(bytes); }
            bytes.try_reserve(count).map_err(|_| Error::out_of_memory())?;
            bytes.extend_from_slice(&buffer[..count]);
        }
    }
    fn create_directory_at(&self, base: &Path, source: &Path) -> Result<()> {
        let limit = self.directory_retry_limit;
        if limit < 1 {
            return Err(Error::IllegalArgument(format!("requirement failed: limit must be 1 or higher: was: {limit}")));
        }
        let path = resolve(base, source);
        let mut first_error = None;
        for _ in 0..limit {
            match fs::create_dir_all(&path) {
                Ok(()) => return Ok(()),
                Err(_) if path.is_dir() => return Ok(()),
                Err(error) => {
                    let error = Error::nio(source, error, false);
                    if error.class == "java.nio.file.FileAlreadyExistsException" {
                        return Err(Error::exception("java.io.IOException", Some(format!("Could not create directory {}: {}", source.display(), error))).with_cause(error));
                    }
                    if first_error.is_none() { first_error = Some(error); }
                    // sbt Retry sleeps after its final failed attempt as well,
                    // and propagates the first exception rather than the last.
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
        let first = first_error.expect("positive retry count and no successful attempt");
        Err(Error::exception("java.io.IOException", Some(format!("Could not create directory {}: {}", source.display(), first))).with_cause(first))
    }
    fn prepare_sbt_parent(&self, base: &Path, source: &Path) -> Result<()> {
        if let Some(parent) = source.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            match self.create_directory_at(base, parent) {
                Err(error) if error.is_io_exception() => (),
                result => result?,
            }
        }
        Ok(())
    }
    pub fn read_sbt_text_at(&self, base: &Path, source: &Path) -> Result<String> {
        // IO.read creates ByteArrayOutputStream(file.length.toInt) before it
        // invokes Using.fileInputStream (which attempts to create parents).
        let capacity = fs::metadata(resolve(base, source)).map(|metadata| metadata.len()).unwrap_or(0) as i32;
        if capacity < 0 { return Err(Error::IllegalArgument(format!("Negative initial size: {capacity}"))); }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(capacity as usize).map_err(|_| Error::out_of_memory())?;
        self.prepare_sbt_parent(base, source)?;
        let reader = self.open_reader_at(base, source, ReaderOrigin::FileStream)?;
        let read = self.read_all(&reader, bytes);
        // Using uses a finally block: close failure replaces a read failure.
        reader.close()?;
        Ok(codec::decode_sbt_text(&read?))
    }
    pub fn write_sbt_text_at(&self, base: &Path, source: &Path, value: &str) -> Result<()> {
        self.write_sbt_utf16_at(base, source, &value.encode_utf16().collect::<Vec<_>>())
    }
    pub(crate) fn close_owned_file(&self, file: File) -> Result<()> {
        let index = { let mut readers = locked(&self.readers); let index = readers.len();
            readers.push(OpenReader { file: Some(file), origin: ReaderOrigin::FileStream }); index };
        SourceReader { owner: self, index }.close()
    }
    pub fn write_sbt_utf16_at(&self, base: &Path, source: &Path, units: &[u16]) -> Result<()> {
        let text = String::from_utf16(units).map_err(|_| Error::Runtime("String cannot be encoded by charset UTF-8".into()))?;
        self.prepare_sbt_parent(base, source)?;
        let mut file = File::create(resolve(base, source)).map_err(|error| Error::file_stream_open(source, error))?;
        let write = file.write_all(text.as_bytes()).map_err(Error::stream_io);
        let close = self.close_owned_file(file);
        match (write, close) {
            (Err(mut first), Err(last)) if units.len() < 8192 => { first.suppressed.push(last); Err(first) }
            (_, Err(last)) => Err(last),
            (first, Ok(())) => first,
        }
    }
}
