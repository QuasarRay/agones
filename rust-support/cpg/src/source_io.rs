//! Source filesystem effects owned by one CGP build context.
//!
//! Java walk/reader objects are not always explicitly closed by the source.
//! Retain those objects in the context, including after a failed operation.
//! Exhausted directory iterators and explicit reader closes release handles;
//! context teardown releases everything still owned by the build phase.
use std::{
    cell::RefCell,
    fs::{self, File, ReadDir},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};
use cgp::prelude::*;
use crate::{Error, Result};

#[cgp_getter]
pub trait HasBuildIo {
    fn build_io(&self) -> &BuildIo;
}

#[derive(Default)]
pub struct BuildIo {
    walks: RefCell<Vec<NativeWalk>>,
    readers: RefCell<Vec<Option<File>>>,
}

struct NativeWalk {
    first: Option<PathBuf>,
    directories: Vec<ReadDir>,
}

impl NativeWalk {
    fn new(root: &Path) -> std::io::Result<Self> {
        let metadata = fs::symlink_metadata(root)?;
        // Files.walk does not follow directory symlinks by default. Directory
        // filtering is a separate File.isDirectory operation and follows them.
        let directories = if metadata.is_dir() {
            vec![fs::read_dir(root)?]
        } else {
            Vec::new()
        };
        Ok(Self { first: Some(root.to_owned()), directories })
    }

    fn next_path(&mut self) -> Option<std::io::Result<PathBuf>> {
        if let Some(root) = self.first.take() {
            return Some(Ok(root));
        }
        while let Some(directory) = self.directories.last_mut() {
            match directory.next() {
                None => { self.directories.pop(); }
                Some(Err(error)) => return Some(Err(error)),
                Some(Ok(entry)) => {
                    let path = entry.path();
                    let metadata = match fs::symlink_metadata(&path) {
                        Ok(metadata) => metadata,
                        Err(error) => return Some(Err(error)),
                    };
                    if metadata.is_dir() {
                        match fs::read_dir(&path) {
                            Ok(children) => self.directories.push(children),
                            Err(error) => return Some(Err(error)),
                        }
                    }
                    return Some(Ok(path));
                }
            }
        }
        None
    }
}

pub struct SourceWalk<'context> {
    owner: &'context BuildIo,
    index: usize,
}

impl Iterator for SourceWalk<'_> {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        self.owner.walks.borrow_mut()[self.index].next_path().map(|item| item.map_err(Error::from))
    }
}

pub struct SourceReader<'context> {
    owner: &'context BuildIo,
    index: usize,
}

impl SourceReader<'_> {
    pub fn available(&self) -> Result<u64> {
        let mut readers = self.owner.readers.borrow_mut();
        let reader = readers[self.index].as_mut()
            .ok_or_else(|| Error::Io(std::io::Error::other("Stream Closed")))?;
        // Files.newInputStream uses the seekable-channel stream. Its available
        // value is the clamped size-minus-position, rather than an ioctl count.
        // Source: pinned OpenJDK 21.0.13 ChannelInputStream.available.
        let size = reader.metadata()?.len();
        let position = reader.stream_position()?;
        Ok(size.saturating_sub(position).min(i32::MAX as u64))
    }

    pub fn read_byte(&self) -> Result<Option<u8>> {
        let mut readers = self.owner.readers.borrow_mut();
        let reader = readers[self.index].as_mut()
            .ok_or_else(|| Error::Io(std::io::Error::other("Stream Closed")))?;
        let mut byte = [0];
        Ok(if reader.read(&mut byte)? == 0 { None } else { Some(byte[0]) })
    }

    pub fn close(self) -> Result<()> {
        let mut readers = self.owner.readers.borrow_mut();
        let Some(file) = readers[self.index].take() else { return Ok(()); };
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
                        readers[self.index] = Some(unsafe { File::from_raw_fd(descriptor) });
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

impl BuildIo {
    pub fn walk(&self, root: &Path) -> Result<SourceWalk<'_>> {
        let walk = NativeWalk::new(root)?;
        let mut walks = self.walks.borrow_mut();
        let index = walks.len();
        walks.push(walk);
        Ok(SourceWalk { owner: self, index })
    }

    pub fn open_reader(&self, path: &Path) -> Result<SourceReader<'_>> {
        let reader = File::open(path)?;
        let mut readers = self.readers.borrow_mut();
        let index = readers.len();
        readers.push(Some(reader));
        Ok(SourceReader { owner: self, index })
    }

    pub fn read_source_text(&self, path: &Path) -> Result<String> {
        let reader = self.open_reader(path)?;
        let mut bytes = Vec::new();
        self.readers.borrow_mut()[reader.index].as_mut()
            .ok_or_else(|| Error::Io(std::io::Error::other("Stream Closed")))?
            .read_to_end(&mut bytes)?;
        Ok(cpg_rs::utilities::decode_utf8_replacing(&bytes))
    }

    pub fn write_source_text(&self, path: &Path, value: &str) -> Result<()> {
        if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        let mut output = File::create(path)?;
        output.write_all(value.as_bytes())?;
        Ok(())
    }
}
