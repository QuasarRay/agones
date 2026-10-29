//! Source klog header, output, fatal and flush control.
// klog/v2/klog.go and internal/buffer/buffer.go, Google Inc./Kubernetes
// Authors, Apache-2.0. Native caller/clock/file creation/rotation/flush daemon,
// native stack/exit/pool/GC/ABI/race/configuration profiles remain required.
use std::{io::Write, sync::{Arc, Mutex, RwLock, atomic::{AtomicI64, AtomicU32, Ordering}}, time::Duration};
use crate::{utility_runtime::UtilityRuntimeContext,
    utility_klog::{SourceKlogSeverity, LogWriter}, utility_logr::add_int,
    utility_klog_header::{SourceKlogCallSite, SourceKlogHeaderBindings, source_file, format_header, append_body},
    utility_klog_dispatch::{FormatResult, SourceKlogFormatError},
    utility_time_values::SourceGoTime,
    utility_error_slice::SourceRetainedError, utility_format_copy::{SourceFmtSharedString, SourceFmtValueCopy},
    utility_http::HeaderAllocationProfile, utility_go_quote::append,
    utility_tls::CanExitSourceProcess, utility_scope::CanSpawnUtilityTask,
    utility_format_methods::SourceStringer, utility_slog_values::SourceSlogDuration};

pub trait SourceKlogSyncTarget: Send + Sync { fn source_sync(&self) -> Result<(), SourceRetainedError>; }
pub trait SourceKlogSyncBuffer: Send + Sync {
    /// Source Flush's ignored result followed by the actual current file.
    /// None means this syncBuffer currently has a nil file.
    fn source_flush_file(&self) -> Option<Arc<dyn SourceKlogSyncTarget>>;
}
pub trait SourceKlogOutputWriter: Send + Sync {
    fn source_write(&self, bytes: &[u8]) -> Result<usize, SourceRetainedError>;
    fn source_sync_buffer(&self) -> Option<&dyn SourceKlogSyncBuffer> { None }
}
pub struct NativeKlogWriter<Writer: Write + Send> { writer: Mutex<Writer> }
impl<Writer: Write + Send> NativeKlogWriter<Writer> { pub fn new(writer: Writer) -> Self { Self { writer: Mutex::new(writer) } } }
impl<Writer: Write + Send> SourceKlogOutputWriter for NativeKlogWriter<Writer> {
    fn source_write(&self, bytes: &[u8]) -> Result<usize, SourceRetainedError> {
        self.writer.lock().unwrap_or_else(|e| e.into_inner()).write(bytes)
            .map_err(|error| SourceRetainedError::from_original(Box::new(error)))
    }
}
pub type SourceKlogWriter = Arc<dyn SourceKlogOutputWriter>;
pub type SourceKlogFileFactory = Arc<dyn Fn(&UtilityRuntimeContext, SourceKlogSeverity,
    &mut [Option<SourceKlogWriter>; 4]) -> Result<(), SourceRetainedError> + Send + Sync>;
pub type SourceKlogStacks = Arc<dyn Fn(bool) -> FormatResult<Vec<u8>> + Send + Sync>;
pub type SourceKlogExitFunction = Arc<dyn Fn(SourceRetainedError) + Send + Sync>;
#[derive(Clone)]
pub struct SourceKlogHeaderSettings {
    pub skip_headers: bool, pub add_dir_header: bool, pub pid: i64,
    pub time: Option<Arc<RwLock<SourceGoTime>>>, pub bindings: Option<SourceKlogHeaderBindings>,
}
impl Default for SourceKlogHeaderSettings {
    fn default() -> Self { Self { skip_headers: false, add_dir_header: false, pid: i64::from(std::process::id()), time: None, bindings: None } }
}
pub struct SourceKlogOutputFlags {
    pub to_stderr: bool, pub also_to_stderr: bool, pub legacy_stderr_threshold_behavior: bool,
    pub stderr_threshold: i32, pub also_stderr_threshold: i32, pub one_output: bool,
    pub log_file: SourceFmtSharedString,
}
impl Clone for SourceKlogOutputFlags {
    fn clone(&self) -> Self { Self { to_stderr: self.to_stderr, also_to_stderr: self.also_to_stderr,
        legacy_stderr_threshold_behavior: self.legacy_stderr_threshold_behavior, stderr_threshold: self.stderr_threshold,
        also_stderr_threshold: self.also_stderr_threshold, one_output: self.one_output, log_file: self.log_file.source_fmt_copy() } }
}
impl Default for SourceKlogOutputFlags {
    fn default() -> Self { Self { to_stderr: true, also_to_stderr: false, legacy_stderr_threshold_behavior: true,
        stderr_threshold: 2, also_stderr_threshold: 0, one_output: false, log_file: SourceFmtSharedString(Arc::from(&b""[..])) } }
}
struct OutputData {
    flags: SourceKlogOutputFlags, files: [Option<SourceKlogWriter>; 4],
    create_files: Option<SourceKlogFileFactory>, trace: Option<SourceKlogCallSite>, stacks: Option<SourceKlogStacks>,
    log_exit_function: Option<SourceKlogExitFunction>, statistics: [[i64; 2]; 3],
}
pub struct SourceKlogPrepared {
    pub bytes: Vec<u8>, pub site: Option<SourceKlogCallSite>, pub source_header_bound: bool,
}
pub struct SourceKlogOutputState {
    // Header fields are source initialization-time variables. This native
    // guard is not a claim about Go's unsynchronized read/write race profile.
    header: Mutex<SourceKlogHeaderSettings>,
    // This is the source logging.mu output owner. Source output invokes
    // writers/trace/exit hooks while holding it; do not silently unlock them.
    output: Mutex<OutputData>, profile: HeaderAllocationProfile,
    // os.Stderr is independent of logging.mu. In particular, timeoutFlush
    // must print its timeout while the flush worker may still hold that mu.
    stderr: RwLock<SourceKlogWriter>, fatal_no_stacks: AtomicU32, exit_flush_timeout: AtomicI64,
}
impl SourceKlogOutputState {
    pub fn new(profile: HeaderAllocationProfile) -> Self {
        Self { header: Mutex::new(SourceKlogHeaderSettings::default()), profile,
            stderr: RwLock::new(Arc::new(NativeKlogWriter::new(std::io::stderr()))),
            fatal_no_stacks: AtomicU32::new(0), exit_flush_timeout: AtomicI64::new(10_000_000_000),
            output: Mutex::new(OutputData { flags: SourceKlogOutputFlags::default(), files: std::array::from_fn(|_| None),
                create_files: None, trace: None, stacks: None, log_exit_function: None, statistics: [[0; 2]; 3] }) }
    }
    pub fn set_header_settings(&self, settings: SourceKlogHeaderSettings) -> SourceKlogHeaderSettings {
        std::mem::replace(&mut *self.header.lock().unwrap_or_else(|e| e.into_inner()), settings)
    }
    pub fn header_settings(&self) -> SourceKlogHeaderSettings { self.header.lock().unwrap_or_else(|e| e.into_inner()).clone() }
    pub fn set_flags(&self, flags: SourceKlogOutputFlags) -> SourceKlogOutputFlags {
        let mut data = self.output.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.flags, flags)
    }
    pub fn log_to_stderr(&self, enabled: bool) { self.output.lock().unwrap_or_else(|e| e.into_inner()).flags.to_stderr = enabled; }
    pub fn set_output(&self, writer: Option<SourceKlogWriter>) -> [Option<SourceKlogWriter>; 4] {
        let mut data = self.output.lock().unwrap_or_else(|e| e.into_inner());
        let mut previous = std::array::from_fn(|_| None);
        for severity in (0..4).rev() { previous[severity] = std::mem::replace(&mut data.files[severity], writer.as_ref().map(Arc::clone)); }
        previous
    }
    pub fn set_output_for_severity(&self, severity: SourceKlogSeverity, writer: Option<SourceKlogWriter>) -> Option<SourceKlogWriter> {
        let mut data = self.output.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.files[severity as usize], writer)
    }
    pub fn set_stderr(&self, writer: SourceKlogWriter) -> SourceKlogWriter {
        std::mem::replace(&mut *self.stderr.write().unwrap_or_else(|e| e.into_inner()), writer)
    }
    fn write_stderr(&self, bytes: &[u8]) {
        let stderr = Arc::clone(&self.stderr.read().unwrap_or_else(|e| e.into_inner()));
        let _ = stderr.source_write(bytes);
    }
    /// Exit/Exitln/Exitf set this source global permanently before logging.
    /// Fatal does not reset it for later calls.
    pub fn mark_exit_without_stacks(&self) { self.fatal_no_stacks.store(1, Ordering::SeqCst); }
    pub fn set_exit_flush_timeout(&self, nanoseconds: i64) -> i64 {
        // This native cell does not establish the source unguarded variable's
        // concurrent mutation/race profile.
        self.exit_flush_timeout.swap(nanoseconds, Ordering::SeqCst)
    }
    pub fn set_file_factory(&self, factory: Option<SourceKlogFileFactory>) -> Option<SourceKlogFileFactory> {
        let mut data = self.output.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.create_files, factory)
    }
    pub fn set_trace(&self, site: Option<SourceKlogCallSite>, stacks: Option<SourceKlogStacks>)
        -> (Option<SourceKlogCallSite>, Option<SourceKlogStacks>)
    {
        let mut data = self.output.lock().unwrap_or_else(|e| e.into_inner());
        (std::mem::replace(&mut data.trace, site), std::mem::replace(&mut data.stacks, stacks))
    }
    pub fn set_log_exit_function(&self, callback: Option<SourceKlogExitFunction>) -> Option<SourceKlogExitFunction> {
        let mut data = self.output.lock().unwrap_or_else(|e| e.into_inner()); std::mem::replace(&mut data.log_exit_function, callback)
    }
    /// Strict source header path. Caller is invoked before timeNow even when
    /// skip_headers is true or buffer.Time overrides the returned clock.
    pub fn prepare_source(&self, runtime: &UtilityRuntimeContext, severity: SourceKlogSeverity, depth: i64,
        body: &[u8]) -> FormatResult<SourceKlogPrepared>
    {
        let bindings = self.header.lock().unwrap_or_else(|e| e.into_inner()).bindings.clone()
            .ok_or_else(|| missing("klog runtime.Caller/timeNow bindings"))?;
        let site = (bindings.caller)(runtime, add_int(self.profile, depth, 3))?;
        let add_directory = self.header.lock().unwrap_or_else(|e| e.into_inner()).add_dir_header;
        let site = source_file(site, add_directory);
        let now = (bindings.now)(runtime)?;
        let settings = self.header_settings();
        let mut output = Vec::new();
        if !settings.skip_headers {
            let now = settings.time.as_ref().map_or_else(|| now.source_fmt_copy(),
                |time| time.read().unwrap_or_else(|e| e.into_inner()).source_fmt_copy());
            let pid = if self.profile.word_bits() == 32 { i64::from(settings.pid as i32) } else { settings.pid };
            format_header(&mut output, severity as i32, &site, &now, pid)?;
        }
        append_body(&mut output, body)?;
        Ok(SourceKlogPrepared { bytes: output, site: Some(site), source_header_bound: true })
    }
    pub(crate) fn prepare_native(&self, runtime: &UtilityRuntimeContext, severity: SourceKlogSeverity, depth: i64,
        body: &[u8]) -> FormatResult<SourceKlogPrepared>
    {
        if self.header.lock().unwrap_or_else(|e| e.into_inner()).bindings.is_some() {
            return self.prepare_source(runtime, severity, depth, body);
        }
        // Preserve the previous explicit native body-only projection until
        // actual caller/clock bindings exist. This does not claim skip_headers
        // or pretend a missing native binding is a source Caller failure.
        let mut bytes = Vec::new(); append_body(&mut bytes, body)?;
        Ok(SourceKlogPrepared { bytes, site: None, source_header_bound: false })
    }
    pub(crate) fn write_prepared(&self, runtime: &UtilityRuntimeContext, severity: SourceKlogSeverity,
        logger: Option<LogWriter>, mut prepared: SourceKlogPrepared, depth: i64, also_to_stderr: bool) -> FormatResult
    {
        let mut data = self.output.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(trace) = data.trace.as_ref().filter(|trace| trace.line > 0) {
            let site = prepared.site.as_ref().ok_or_else(|| missing("klog trace source caller binding"))?;
            let base = site.file.0.iter().rposition(|byte| *byte == b'/').map_or(0, |slash| slash + 1);
            if trace.file.0.as_ref() == &site.file.0[base..] && trace.line == site.line {
                let stacks = data.stacks.as_ref().ok_or_else(|| missing("klog trace stack binding"))?;
                append(&mut prepared.bytes, &stacks(false)?)?;
            }
        }
        let bytes = &prepared.bytes;
        if let Some(logger) = logger {
            if let Some(write) = logger.write_klog_buffer { write(bytes); }
            else {
                let bytes = if bytes.last() == Some(&b'\n') { &bytes[..bytes.len() - 1] } else { bytes.as_slice() };
                let logger = logger.logger.with_call_depth(add_int(self.profile, depth, 3));
                let values = crate::utility_error_handlers::SourceErrorArguments::nil_with_profile(self.profile);
                if severity == SourceKlogSeverity::Error { logger.error(runtime, None, bytes, values); }
                else { logger.info(runtime, bytes, values); }
            }
        } else if data.flags.to_stderr {
            if data.flags.legacy_stderr_threshold_behavior || severity as i32 >= data.flags.stderr_threshold {
                self.write_stderr(bytes);
            }
        } else {
            if also_to_stderr || data.flags.also_to_stderr && severity as i32 >= data.flags.also_stderr_threshold
                || !data.flags.also_to_stderr && severity as i32 >= data.flags.stderr_threshold
            { self.write_stderr(bytes); }
            let single_file = !data.flags.log_file.0.is_empty();
            let selected = if single_file { SourceKlogSeverity::Info } else { severity };
            if data.files[selected as usize].is_none() {
                let create = data.create_files.as_ref().cloned().ok_or_else(|| missing("klog createFiles/rotation/flush-daemon binding"))?;
                match create(runtime, selected, &mut data.files) {
                    Err(error) => { self.write_stderr(bytes); exit_error(self, runtime, &mut data, error)?; }
                    Ok(()) if data.files[selected as usize].is_none() => return Err(missing("klog createFiles success/file binding")),
                    Ok(()) => (),
                }
            }
            if single_file || data.flags.one_output {
                write_file(&data, selected, bytes);
            } else {
                for index in (0..=severity as usize).rev() {
                    let writer = data.files[index].as_ref().unwrap_or_else(|| crate::utility_error_nil::nil_error_receiver());
                    let _ = writer.source_write(bytes);
                }
            }
        }
        if severity == SourceKlogSeverity::Fatal {
            if self.fatal_no_stacks.load(Ordering::SeqCst) > 0 {
                drop(data);
                timeout_flush(runtime, self.exit_flush_timeout.load(Ordering::SeqCst))?;
                runtime.exit_source_process(1);
                // The real CGP process effect does not return. The source
                // substituted-exit profile continues with unlocked reads and
                // an eventual second Unlock; do not invent a safe relock or
                // claim that returning here implements that profile.
                return Err(missing("klog returning OsExit(1)/unlocked continuation profile"));
            }
            let stacks = data.stacks.as_ref().cloned().ok_or_else(|| missing("klog fatal current/all-goroutine stack binding"))?;
            if !data.flags.to_stderr { self.write_stderr(&stacks(false)?); }
            let trace = stacks(true)?;
            // A file write can invoke exit_error; fatal must still reach its
            // own flush and status, so install the source no-op exit hook.
            data.log_exit_function = Some(Arc::new(|_| ()));
            for index in (0..4).rev() {
                if let Some(file) = &data.files[index] { let _ = file.source_write(&trace); }
            }
            drop(data);
            timeout_flush(runtime, self.exit_flush_timeout.load(Ordering::SeqCst))?;
            runtime.exit_source_process(255);
            // Fatal has no severityStats slot, including when OsExit returns.
            return Ok(());
        }
        let statistics = &mut data.statistics[severity as usize];
        statistics[0] = statistics[0].wrapping_add(1); statistics[1] = statistics[1].wrapping_add(bytes.len() as i64);
        Ok(())
    }
    pub fn statistics(&self, severity: SourceKlogSeverity) -> Option<(i64, i64)> {
        if severity == SourceKlogSeverity::Fatal { return None; }
        let data = self.output.lock().unwrap_or_else(|e| e.into_inner()); let counts = data.statistics[severity as usize];
        Some((counts[0], counts[1]))
    }
    pub fn flush(&self, runtime: &UtilityRuntimeContext) {
        let files = {
            let data = self.output.lock().unwrap_or_else(|e| e.into_inner());
            flush_all(runtime, &data)
        };
        // Source lockAndFlushAll releases logging.mu before Sync, unlike exit.
        sync_all(files);
    }
}
fn write_file(data: &OutputData, severity: SourceKlogSeverity, bytes: &[u8]) {
    let writer = data.files[severity as usize].as_ref().unwrap_or_else(|| crate::utility_error_nil::nil_error_receiver());
    let _ = writer.source_write(bytes);
}
struct FilesToSync { files: [Option<Arc<dyn SourceKlogSyncTarget>>; 4], count: usize }
fn flush_all(runtime: &UtilityRuntimeContext, data: &OutputData) -> FilesToSync {
    let mut files = FilesToSync { files: std::array::from_fn(|_| None), count: 0 };
    for severity in (0..4).rev() {
        if let Some(buffer) = data.files[severity].as_ref().and_then(|writer| writer.source_sync_buffer()) {
            if let Some(file) = buffer.source_flush_file() {
                files.files[files.count] = Some(file); files.count += 1;
            }
        }
    }
    runtime.source_klog_state.flush_backing_logger();
    files
}
fn sync_all(files: FilesToSync) {
    for file in files.files[..files.count].iter().flatten() { let _ = file.source_sync(); }
}
fn exit_error(output: &SourceKlogOutputState, runtime: &UtilityRuntimeContext, data: &mut OutputData, error: SourceRetainedError) -> FormatResult {
    let mut message = b"log: exiting because of error: ".to_vec();
    crate::utility_format_arguments::write_source_error_operand(&error, 's', runtime.source_unicode_profile,
        crate::utility_format_print::SourcePrintfMode::Sprintf, &mut message)?;
    append(&mut message, b"\n")?; output.write_stderr(&message);
    if let Some(exit) = &data.log_exit_function { exit(error); return Ok(()); }
    sync_all(flush_all(runtime, data));
    runtime.exit_source_process(2);
    Ok(())
}
/// Source FlushAndExit shares the same timed worker as Fatal and Exit.
pub fn flush_and_exit(runtime: &UtilityRuntimeContext, nanoseconds: i64, status: i32) -> FormatResult {
    timeout_flush(runtime, nanoseconds)?;
    runtime.exit_source_process(status);
    Ok(())
}
pub fn timeout_flush(runtime: &UtilityRuntimeContext, nanoseconds: i64) -> FormatResult {
    let (done, completion) = std::sync::mpsc::sync_channel(1);
    let parent = runtime.clone();
    let work = Box::new(move || {
        parent.source_klog_state.output.flush(&parent);
        // The source completion channel has capacity one. A timeout must not
        // block or cancel the worker when it eventually finishes flushing.
        let _ = done.send(true);
        Ok(())
    });
    if let Err(_unstarted) = runtime.spawn_blocking_utility_task_or_return("klog timeoutFlush", work) {
        return Err(missing("klog timeoutFlush live utility task owner"));
    }
    let timeout = Duration::from_nanos(nanoseconds.max(0) as u64);
    match completion.recv_timeout(timeout) {
        Ok(_) => Ok(()),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            let mut diagnostic = b"klog: Flush took longer than ".to_vec();
            append(&mut diagnostic, &SourceSlogDuration(nanoseconds).source_string())?;
            append(&mut diagnostic, b"\n")?;
            runtime.source_klog_state.output.write_stderr(&diagnostic);
            Ok(())
        }
        // A source goroutine panic is process-fatal via the CGP parent policy;
        // losing the sender is not successful Flush completion.
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Err(missing("klog flush worker completion/panic/exit profile")),
    }
}
fn missing(operation: &'static str) -> SourceKlogFormatError { SourceKlogFormatError::Unbound { source_type: None, operation } }
