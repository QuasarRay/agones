//! Source klog syncBuffer, rotation and createFiles control.
// klog/v2/klog.go, Google Inc./Kubernetes Authors, Apache-2.0;
// Go1.26.5 bufio/bufio.go, Copyright The Go Authors, BSD-3-Clause
// (../GO-LICENSE). Native filesystem/path/error/clock/GC/ABI/race profiles
// remain required. Actual compiled file effects are data, not a registry.
use std::{fs::{File, OpenOptions}, io::Write, path::Path, sync::{Arc, Mutex, OnceLock}};
use crate::{utility_runtime::UtilityRuntimeContext,
    utility_klog::SourceKlogSeverity,
    utility_klog_output::{SourceKlogOutputWriter, SourceKlogSyncBuffer, SourceKlogSyncTarget,
        SourceKlogOutputFlags, SourceKlogWriter, SourceKlogFileFactory},
    utility_klog_dispatch::{FormatResult, SourceKlogFormatError},
    utility_klog_daemon::CanControlSourceKlogFlushDaemon,
    utility_error_slice::SourceRetainedError,
    utility_format_copy::{SourceFmtSharedString, SourceFmtValueCopy},
    utility_time_values::SourceGoTime, utility_go_quote::append,
    utility_http::reserve_source_array};

const BUFFER_SIZE: usize = 256 * 1024;
pub struct SourceKlogWrite { pub count: usize, pub error: Option<SourceRetainedError> }
/// File.Write retains both its count and its returned error. A Result alone
/// would lose partial-write counts, which affect bufio and syncBuffer.nbytes.
pub trait SourceKlogFile: SourceKlogSyncTarget {
    fn source_file_write(&self, bytes: &[u8]) -> SourceKlogWrite;
    fn source_file_size(&self) -> Result<i64, SourceRetainedError>;
    fn source_file_close(&self);
}
pub struct NativeKlogFile { file: Mutex<Option<File>> }
pub struct SourceKlogCreatedFile { pub file: Option<Arc<dyn SourceKlogFile>>, pub error: Option<SourceRetainedError> }
pub fn native_open_file(path: &Path, startup: bool) -> SourceKlogCreatedFile {
    let file = if startup { OpenOptions::new().read(true).write(true).create(true).append(true).open(path) }
        else { File::create(path) };
    match file {
        Ok(file) => SourceKlogCreatedFile { file: Some(Arc::new(NativeKlogFile { file: Mutex::new(Some(file)) })), error: None },
        Err(error) => SourceKlogCreatedFile { file: None, error: Some(native_error(error)) },
    }
}
fn native_error(error: std::io::Error) -> SourceRetainedError { SourceRetainedError::from_original(Box::new(error)) }
fn closed_file() -> SourceRetainedError {
    // Native error data is explicit; this does not declare a source *PathError
    // or erase its required Op/Path/Err receiver/type/error binding.
    native_error(std::io::Error::new(std::io::ErrorKind::NotConnected, "file already closed"))
}
impl SourceKlogSyncTarget for NativeKlogFile {
    fn source_sync(&self) -> Result<(), SourceRetainedError> {
        self.file.lock().unwrap_or_else(|e| e.into_inner()).as_ref().ok_or_else(closed_file)?
            .sync_all().map_err(native_error)
    }
}
impl SourceKlogFile for NativeKlogFile {
    fn source_file_write(&self, bytes: &[u8]) -> SourceKlogWrite {
        let mut file = self.file.lock().unwrap_or_else(|e| e.into_inner());
        let Some(file) = file.as_mut() else { return SourceKlogWrite { count: 0, error: Some(closed_file()) }; };
        match file.write(bytes) {
            Ok(count) => SourceKlogWrite { count, error: if count < bytes.len() { Some(short_write()) } else { None } },
            Err(error) => SourceKlogWrite { count: 0, error: Some(native_error(error)) },
        }
    }
    fn source_file_size(&self) -> Result<i64, SourceRetainedError> {
        self.file.lock().unwrap_or_else(|e| e.into_inner()).as_ref().ok_or_else(closed_file)?
            .metadata().map(|metadata| metadata.len() as i64).map_err(native_error)
    }
    fn source_file_close(&self) {
        // The rotation caller ignores Close's error. Native close-error and
        // descriptor/finalizer identities remain part of the filesystem gap.
        drop(self.file.lock().unwrap_or_else(|e| e.into_inner()).take());
    }
}
fn message_error(bytes: Vec<u8>) -> SourceRetainedError {
    SourceRetainedError::from_original(Box::new(crate::utility_error_format::MessageError(
        crate::utility_error_format::SourceErrorMessage::Bytes(bytes))))
}
fn short_write() -> SourceRetainedError {
    static ERROR: OnceLock<SourceRetainedError> = OnceLock::new();
    ERROR.get_or_init(|| message_error(b"short write".to_vec())).source_fmt_copy()
}
pub struct SourceKlogBinaryInfo {
    pub host: SourceFmtSharedString, pub compiler: SourceFmtSharedString, pub version: SourceFmtSharedString,
    pub operating_system: SourceFmtSharedString, pub architecture: SourceFmtSharedString,
}
pub type SourceKlogCreateFile = Arc<dyn Fn(&UtilityRuntimeContext, &SourceKlogOutputFlags,
    SourceKlogSeverity, &SourceGoTime, bool) -> FormatResult<SourceKlogCreatedFile> + Send + Sync>;
pub struct SourceKlogFileBindings {
    pub now: Arc<dyn Fn(&UtilityRuntimeContext) -> FormatResult<SourceGoTime> + Send + Sync>,
    pub create: SourceKlogCreateFile, pub binary: SourceKlogBinaryInfo,
}
pub fn calculate_max_size(flags: &SourceKlogOutputFlags) -> u64 {
    if flags.log_file.0.is_empty() { flags.max_size }
    else if flags.log_file_max_size_mb == 0 { u64::MAX }
    else { flags.log_file_max_size_mb.wrapping_mul(1024).wrapping_mul(1024) }
}
struct BufferedWriter {
    bytes: Vec<u8>, count: usize, error: Option<SourceRetainedError>, file: Arc<dyn SourceKlogFile>,
}
impl BufferedWriter {
    fn new(file: Arc<dyn SourceKlogFile>) -> FormatResult<Self> {
        let mut bytes = reserve_source_array::<u8>(BUFFER_SIZE as u64)?; bytes.resize(BUFFER_SIZE, 0);
        Ok(Self { bytes, count: 0, error: None, file })
    }
    fn flush(&mut self) -> FormatResult<Option<SourceRetainedError>> {
        if let Some(error) = &self.error { return Ok(Some(error.source_fmt_copy())); }
        if self.count == 0 { return Ok(None); }
        let mut result = self.file.source_file_write(&self.bytes[..self.count]);
        if result.count > self.count { return Err(missing("bufio Flush invalid file Write count")); }
        if result.count < self.count && result.error.is_none() { result.error = Some(short_write()); }
        if let Some(error) = result.error {
            if result.count > 0 && result.count < self.count { self.bytes.copy_within(result.count..self.count, 0); }
            self.count -= result.count; self.error = Some(error);
            return Ok(self.error.as_ref().map(SourceFmtValueCopy::source_fmt_copy));
        }
        self.count = 0; Ok(None)
    }
    fn write(&mut self, mut bytes: &[u8]) -> FormatResult<SourceKlogWrite> {
        let mut written = 0;
        while bytes.len() > self.bytes.len() - self.count && self.error.is_none() {
            let count;
            if self.count == 0 {
                let result = self.file.source_file_write(bytes);
                count = result.count; self.error = result.error;
            } else {
                count = (self.bytes.len() - self.count).min(bytes.len());
                self.bytes[self.count..self.count + count].copy_from_slice(&bytes[..count]);
                self.count += count; let _ = self.flush()?;
            }
            if count > bytes.len() { return Err(missing("bufio Write invalid file Write count")); }
            written += count; bytes = &bytes[count..];
        }
        if let Some(error) = &self.error { return Ok(SourceKlogWrite { count: written, error: Some(error.source_fmt_copy()) }); }
        self.bytes[self.count..self.count + bytes.len()].copy_from_slice(bytes);
        self.count += bytes.len(); written += bytes.len();
        Ok(SourceKlogWrite { count: written, error: None })
    }
}
struct SyncBufferData {
    file: Option<Arc<dyn SourceKlogFile>>, writer: Option<BufferedWriter>, bytes: u64, max_bytes: u64,
}
pub struct SourceKlogFileBuffer {
    data: Mutex<SyncBufferData>, severity: SourceKlogSeverity, bindings: Arc<SourceKlogFileBindings>,
}
impl SourceKlogFileBuffer {
    fn new(severity: SourceKlogSeverity, bindings: Arc<SourceKlogFileBindings>, max_bytes: u64) -> Self {
        Self { data: Mutex::new(SyncBufferData { file: None, writer: None, bytes: 0, max_bytes }), severity, bindings }
    }
    fn rotate(&self, runtime: &UtilityRuntimeContext, flags: &SourceKlogOutputFlags, now: &SourceGoTime,
        startup: bool) -> FormatResult<Option<SourceRetainedError>>
    {
        let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(file) = data.file.as_ref().cloned() {
            if let Some(writer) = data.writer.as_mut() { let _ = writer.flush()?; }
            file.source_file_close();
        }
        let created = (self.bindings.create)(runtime, flags, self.severity, now, startup)?;
        data.file = created.file;
        if let Some(error) = created.error { return Ok(Some(error)); }
        let file = data.file.as_ref().cloned().unwrap_or_else(|| crate::utility_error_nil::nil_error_receiver());
        if startup {
            match file.source_file_size() {
                Ok(size) => data.bytes = size as u64,
                Err(error) => {
                    let mut bytes = b"file stat could not get fileinfo: ".to_vec();
                    crate::utility_format_arguments::write_source_error_operand(&error, 'v', runtime.source_unicode_profile,
                        crate::utility_format_print::SourcePrintfMode::Sprintf, &mut bytes)?;
                    return Ok(Some(message_error(bytes)));
                }
            }
        } else { data.bytes = 0; }
        data.writer = Some(BufferedWriter::new(Arc::clone(&file))?);
        if flags.skip_log_headers { return Ok(None); }
        let mut header = b"Log file created at: ".to_vec(); append(&mut header, &now.klog_file_time()?)?;
        append(&mut header, b"\nRunning on machine: ")?; append(&mut header, &self.bindings.binary.host.0)?;
        append(&mut header, b"\nBinary: Built with ")?; append(&mut header, &self.bindings.binary.compiler.0)?;
        append(&mut header, b" ")?; append(&mut header, &self.bindings.binary.version.0)?;
        append(&mut header, b" for ")?; append(&mut header, &self.bindings.binary.operating_system.0)?;
        append(&mut header, b"/")?; append(&mut header, &self.bindings.binary.architecture.0)?;
        append(&mut header, b"\nLog line format: [IWEF]mmdd hh:mm:ss.uuuuuu threadid file:line] msg\n")?;
        let result = file.source_file_write(&header); data.bytes = data.bytes.wrapping_add(result.count as u64);
        Ok(result.error)
    }
}
struct FileSyncTarget(Arc<dyn SourceKlogFile>);
impl SourceKlogSyncTarget for FileSyncTarget { fn source_sync(&self) -> Result<(), SourceRetainedError> { self.0.source_sync() } }
impl SourceKlogSyncBuffer for SourceKlogFileBuffer {
    fn source_flush_file(&self) -> Option<Arc<dyn SourceKlogSyncTarget>> {
        let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner());
        data.file.as_ref()?;
        if let Some(writer) = data.writer.as_mut() {
            // Source bufio errors are ignored here. Native binding/allocation
            // gaps remain fatal diagnostics, never a fabricated Go I/O error.
            if let Err(error) = writer.flush() { std::panic::panic_any(error); }
        }
        data.file.as_ref().map(|file| Arc::new(FileSyncTarget(Arc::clone(file))) as Arc<dyn SourceKlogSyncTarget>)
    }
    fn source_write_buffer(&self, runtime: &UtilityRuntimeContext, flags: &SourceKlogOutputFlags,
        bytes: &[u8], exit: &mut dyn FnMut(SourceRetainedError) -> FormatResult) -> FormatResult<SourceKlogWrite>
    {
        let rotate = { let data = self.data.lock().unwrap_or_else(|e| e.into_inner());
            data.bytes.wrapping_add(bytes.len() as u64) >= data.max_bytes };
        if rotate {
            let now = (self.bindings.now)(runtime)?;
            if let Some(error) = self.rotate(runtime, flags, &now, false)? { exit(error)?; }
        }
        let result = {
            let mut data = self.data.lock().unwrap_or_else(|e| e.into_inner());
            let writer = data.writer.as_mut().unwrap_or_else(|| crate::utility_error_nil::nil_error_receiver());
            let result = writer.write(bytes)?; data.bytes = data.bytes.wrapping_add(result.count as u64); result
        };
        // A source exit callback may re-enter flushAll and this very buffer.
        // logging.mu stays with the caller, but our native guard must be gone.
        if let Some(error) = result.error.as_ref() { exit(error.source_fmt_copy())?; }
        Ok(result)
    }
}
impl SourceKlogOutputWriter for SourceKlogFileBuffer {
    fn source_write(&self, _bytes: &[u8]) -> Result<usize, SourceRetainedError> {
        // syncBuffer.Write needs its actual logging owner and source exit
        // policy; the output dispatcher uses its declared sync-buffer view.
        std::panic::panic_any(missing("klog syncBuffer Write without logging owner"))
    }
    fn source_sync_buffer(&self) -> Option<&dyn SourceKlogSyncBuffer> { Some(self) }
}
pub fn file_factory(bindings: Arc<SourceKlogFileBindings>) -> SourceKlogFileFactory {
    Arc::new(move |runtime, severity, flags, files| {
        let interval = if flags.flush_interval == 0 { 5_000_000_000 } else { flags.flush_interval };
        runtime.run_source_klog_flush_daemon(interval)?;
        let now = (bindings.now)(runtime)?;
        for index in (0..=severity as usize).rev() {
            if files[index].is_some() { break; }
            let severity = [SourceKlogSeverity::Info, SourceKlogSeverity::Warning, SourceKlogSeverity::Error, SourceKlogSeverity::Fatal][index];
            let buffer = Arc::new(SourceKlogFileBuffer::new(severity, Arc::clone(&bindings), calculate_max_size(flags)));
            if let Some(error) = buffer.rotate(runtime, flags, &now, true)? { return Ok(Err(error)); }
            files[index] = Some(buffer as SourceKlogWriter);
        }
        Ok(Ok(()))
    })
}
fn missing(operation: &'static str) -> SourceKlogFormatError { SourceKlogFormatError::Unbound { source_type: None, operation } }
