//! Source klog file naming, directories, user initialization and file effects.
// klog/v2/klog_file{,_others,_windows}.go, Google Inc./Kubernetes Authors,
// Apache-2.0. Go1.26.5 filepath/os/user/hostname contracts, Go Authors,
// BSD-3-Clause (../GO-LICENSE). Full native/platform/error/GC/ABI profiles
// remain required; these are actual compiled effect values, not a registry.
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, Ordering}};
#[cfg(unix)]
use std::path::PathBuf;
use cgp::prelude::*;
use crate::{utility_runtime::UtilityRuntimeContext, utility_klog::SourceKlogSeverity,
    utility_klog_output::SourceKlogOutputFlags,
    utility_klog_file::{SourceKlogCreatedFile, SourceKlogFileBindings, SourceKlogBinaryInfo, native_open_file},
    utility_klog_path::SourceKlogPathFlavor, utility_klog_dispatch::FormatResult,
    utility_error_slice::{SourceRetainedError, SourceErrorSliceIndex},
    utility_format_copy::{SourceFmtSharedString, SourceFmtValueCopy},
    utility_callback_slice::SourceCallbackSlice, utility_time_values::SourceGoTime,
    utility_http::HeaderAllocationProfile, utility_go_quote::append};

struct SourceKlogOnce { done: AtomicBool, lock: Mutex<()> }
impl Default for SourceKlogOnce { fn default() -> Self { Self { done: AtomicBool::new(false), lock: Mutex::new(()) } } }
struct MarkDone<'a>(&'a AtomicBool);
impl Drop for MarkDone<'_> { fn drop(&mut self) { self.0.store(true, Ordering::Release); } }
impl SourceKlogOnce {
    fn run(&self, initialize: impl FnOnce() -> FormatResult) -> FormatResult {
        if self.done.load(Ordering::Acquire) { return Ok(()); }
        let _lock = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        if self.done.load(Ordering::Acquire) { return Ok(()); }
        let _done = MarkDone(&self.done); initialize()
    }
}
struct Directory(SourceFmtSharedString);
impl Default for Directory { fn default() -> Self { Self(string(b"")) } }
impl SourceFmtValueCopy for Directory { fn source_fmt_copy(&self) -> Self { Self(self.0.source_fmt_copy()) } }
pub enum SourceKlogUserBindings {
    UnixCurrent(Arc<dyn Fn() -> Result<SourceFmtSharedString, SourceRetainedError> + Send + Sync>),
    WindowsEnvironment(Arc<dyn Fn(&[u8]) -> SourceFmtSharedString + Send + Sync>),
}
pub struct SourceKlogFileSystem {
    pub hostname: Arc<dyn Fn() -> Result<SourceFmtSharedString, SourceRetainedError> + Send + Sync>,
    pub temp_directory: Arc<dyn Fn() -> FormatResult<SourceFmtSharedString> + Send + Sync>,
    pub open: Arc<dyn Fn(&[u8], bool) -> FormatResult<SourceKlogCreatedFile> + Send + Sync>,
    pub remove: Arc<dyn Fn(&[u8]) -> Result<(), SourceRetainedError> + Send + Sync>,
    pub symlink: Arc<dyn Fn(&[u8], &[u8]) -> Result<(), SourceRetainedError> + Send + Sync>,
}
pub struct SourceKlogFileEnvironment {
    program: RwLock<SourceFmtSharedString>, pub host: Arc<RwLock<SourceFmtSharedString>>,
    username: RwLock<SourceFmtSharedString>, username_once: SourceKlogOnce, user: SourceKlogUserBindings,
    directories: Mutex<SourceCallbackSlice<Directory>>, directories_once: SourceKlogOnce,
    flavor: SourceKlogPathFlavor, filesystem: SourceKlogFileSystem,
}
pub struct SourceKlogNamedFile { pub result: SourceKlogCreatedFile, pub filename: SourceFmtSharedString }
impl SourceKlogFileEnvironment {
    pub fn initialize(arguments: &[SourceFmtSharedString], flavor: SourceKlogPathFlavor, user: SourceKlogUserBindings,
        filesystem: SourceKlogFileSystem, profile: HeaderAllocationProfile) -> Self
    {
        let argument = arguments.first().unwrap_or_else(|| crate::utility_panic::unwind_source_runtime_error(
            crate::utility_panic::SourceBoundsError { x: 0, y: 0, signed: true, code: crate::utility_panic::SourceBoundsCode::Index }));
        let program = SourceFmtSharedString(Arc::from(flavor.base(&argument.0)));
        let mut host = string(b"unknownhost");
        if let Ok(value) = (filesystem.hostname)() { host = short_hostname(&value); }
        Self { program: RwLock::new(program), host: Arc::new(RwLock::new(host)),
            username: RwLock::new(string(b"unknownuser")), username_once: SourceKlogOnce::default(), user,
            directories: Mutex::new(SourceCallbackSlice::nil(profile, 2)), directories_once: SourceKlogOnce::default(),
            flavor, filesystem }
    }
    pub fn username(&self) -> FormatResult<SourceFmtSharedString> {
        self.username_once.run(|| {
            match &self.user {
                SourceKlogUserBindings::UnixCurrent(current) => {
                    if let Ok(value) = current() { *self.username.write().unwrap_or_else(|e| e.into_inner()) = value; }
                }
                SourceKlogUserBindings::WindowsEnvironment(getenv) => {
                    let user = getenv(b"USERNAME");
                    if user.0.is_empty() { return Ok(()); }
                    let mut user = user.0.to_vec();
                    for byte in &mut user { if *byte == b'\\' { *byte = b'_'; } }
                    let domain = getenv(b"USERDOMAIN");
                    let bytes = if domain.0.is_empty() { user } else {
                        let mut bytes = domain.0.to_vec(); append(&mut bytes, b"_")?; append(&mut bytes, &user)?; bytes
                    };
                    *self.username.write().unwrap_or_else(|e| e.into_inner()) = SourceFmtSharedString(Arc::from(bytes));
                }
            }
            Ok(())
        })?;
        Ok(self.username.read().unwrap_or_else(|e| e.into_inner()).source_fmt_copy())
    }
    fn create_log_directories(&self, flags: &SourceKlogOutputFlags) -> FormatResult {
        if !flags.log_dir.0.is_empty() {
            self.directories.lock().unwrap_or_else(|e| e.into_inner()).try_append(Directory(flags.log_dir.source_fmt_copy()))?;
        }
        let temporary = (self.filesystem.temp_directory)()?;
        self.directories.lock().unwrap_or_else(|e| e.into_inner()).try_append(Directory(temporary))?;
        Ok(())
    }
    pub fn log_name(&self, runtime: &UtilityRuntimeContext, tag: &[u8], time: &SourceGoTime)
        -> FormatResult<(SourceFmtSharedString, SourceFmtSharedString)>
    {
        // Match argument evaluation before fmt: program, host, user, and then
        // each separate Year/Month/Day/Hour/Minute/Second lookup.
        let program = self.program.read().unwrap_or_else(|e| e.into_inner()).source_fmt_copy();
        let host = self.host.read().unwrap_or_else(|e| e.into_inner()).source_fmt_copy();
        let user = self.username()?;
        let year = time.date()?.0; let month = time.date()?.1; let day = time.date()?.2;
        let hour = time.clock()?.0; let minute = time.clock()?.1; let second = time.clock()?.2;
        // The existing header settings own the same source global pid.
        let pid = runtime.source_klog_state.output.header_settings().pid;
        let pid = if runtime.source_klog_state.profile().word_bits() == 32 { i64::from(pid as i32) } else { pid };
        let mut name = Vec::new();
        for part in [&program.0, &host.0, &user.0] { append(&mut name, part)?; append(&mut name, b".")?; }
        append(&mut name, b"log.")?; append(&mut name, tag)?; append(&mut name, b".")?;
        let mut values = [year, month, day, hour, minute, second, pid];
        if runtime.source_klog_state.profile().word_bits() == 32 { values = values.map(|value| i64::from(value as i32)); }
        let output = crate::utility_format_integer::try_source_integer_printf(
            b"%04d%02d%02d-%02d%02d%02d.%d", &values, runtime.source_unicode_profile)?;
        let (bytes, _) = output.into_parts(); append(&mut name, &bytes)?;
        let mut link = self.program.read().unwrap_or_else(|e| e.into_inner()).0.to_vec();
        append(&mut link, b".")?; append(&mut link, tag)?;
        Ok((SourceFmtSharedString(Arc::from(name)), SourceFmtSharedString(Arc::from(link))))
    }
    pub fn create(&self, runtime: &UtilityRuntimeContext, flags: &SourceKlogOutputFlags,
        severity: SourceKlogSeverity, time: &SourceGoTime, startup: bool) -> FormatResult<SourceKlogNamedFile>
    {
        if !flags.log_file.0.is_empty() {
            let result = (self.filesystem.open)(&flags.log_file.0, startup)?;
            if result.error.is_none() { return Ok(SourceKlogNamedFile { result, filename: flags.log_file.source_fmt_copy() }); }
            let error = formatted_error(runtime, b"log: unable to create log: ", result.error.as_ref())?;
            return Ok(failed_file(error));
        }
        self.directories_once.run(|| self.create_log_directories(flags))?;
        let directories = self.directories.lock().unwrap_or_else(|e| e.into_inner()).source_fmt_copy();
        if directories.len() == 0 { return Ok(failed_file(message_error(b"log: no log dirs".to_vec()))); }
        let tag: &[u8] = [b"INFO".as_slice(), b"WARNING".as_slice(), b"ERROR".as_slice(), b"FATAL".as_slice()][severity as usize];
        let (name, link) = self.log_name(runtime, tag, time)?;
        let mut last_error = None;
        for index in 0..directories.len() {
            let directory = directories.get(SourceErrorSliceIndex::unsigned(index as u64)).0;
            let filename = self.flavor.join(&[&directory.0, &name.0])?;
            let result = (self.filesystem.open)(&filename, startup)?;
            if result.error.is_none() {
                let symlink = self.flavor.join(&[&directory.0, &link.0])?;
                let _ = (self.filesystem.remove)(&symlink);
                let _ = (self.filesystem.symlink)(&name.0, &symlink);
                return Ok(SourceKlogNamedFile { result, filename: SourceFmtSharedString(Arc::from(filename)) });
            }
            last_error = result.error;
        }
        Ok(failed_file(formatted_error(runtime, b"log: cannot create log: ", last_error.as_ref())?))
    }
}
fn string(bytes: &[u8]) -> SourceFmtSharedString { SourceFmtSharedString(Arc::from(bytes)) }
pub fn short_hostname(hostname: &SourceFmtSharedString) -> SourceFmtSharedString {
    let end = hostname.0.iter().position(|byte| *byte == b'.').unwrap_or(hostname.0.len());
    SourceFmtSharedString(Arc::from(&hostname.0[..end]))
}
fn message_error(bytes: Vec<u8>) -> SourceRetainedError {
    SourceRetainedError::from_original(Box::new(crate::utility_error_format::MessageError(crate::utility_error_format::SourceErrorMessage::Bytes(bytes))))
}
fn formatted_error(runtime: &UtilityRuntimeContext, prefix: &[u8], error: Option<&SourceRetainedError>) -> FormatResult<SourceRetainedError> {
    let mut message = prefix.to_vec();
    if let Some(error) = error {
        crate::utility_format_arguments::write_source_error_operand(error, 'v', runtime.source_unicode_profile,
            crate::utility_format_print::SourcePrintfMode::Sprintf, &mut message)?;
    } else { append(&mut message, b"<nil>")?; }
    Ok(message_error(message))
}
fn failed_file(error: SourceRetainedError) -> SourceKlogNamedFile {
    SourceKlogNamedFile { result: SourceKlogCreatedFile { file: None, error: Some(error) }, filename: string(b"") }
}
pub struct SourceKlogFileInitialization {
    pub arguments: Vec<SourceFmtSharedString>, pub flavor: SourceKlogPathFlavor, pub user: SourceKlogUserBindings,
    pub filesystem: SourceKlogFileSystem,
    pub now: Arc<dyn Fn(&UtilityRuntimeContext) -> FormatResult<SourceGoTime> + Send + Sync>,
    pub compiler: SourceFmtSharedString, pub version: SourceFmtSharedString,
    pub operating_system: SourceFmtSharedString, pub architecture: SourceFmtSharedString,
}
#[cgp_component(SourceKlogFileInitializer)]
pub trait CanInitializeSourceKlogFiles {
    fn initialize_source_klog_files(&self, input: SourceKlogFileInitialization) -> FormatResult;
}
#[cgp_new_provider]
impl SourceKlogFileInitializer<UtilityRuntimeContext> for InitializeOwnedSourceKlogFiles {
    fn initialize_source_klog_files(runtime: &UtilityRuntimeContext, input: SourceKlogFileInitialization) -> FormatResult {
        let environment = Arc::new(SourceKlogFileEnvironment::initialize(&input.arguments, input.flavor, input.user,
            input.filesystem, runtime.source_klog_state.profile()));
        let binary = SourceKlogBinaryInfo { host: Arc::clone(&environment.host), compiler: input.compiler, version: input.version,
            operating_system: input.operating_system, architecture: input.architecture };
        let bindings = SourceKlogFileBindings { now: input.now, binary, create: Arc::new(move |runtime, flags, severity, time, startup|
            environment.create(runtime, flags, severity, time, startup).map(|created| created.result)) };
        runtime.source_klog_state.output.set_file_bindings(Arc::new(bindings));
        Ok(())
    }
}

#[cfg(unix)]
fn native_path(bytes: &[u8]) -> PathBuf {
    use std::os::unix::ffi::OsStrExt;
    PathBuf::from(std::ffi::OsStr::from_bytes(bytes))
}
#[cfg(unix)]
pub fn native_unix_file_system(
    hostname: Arc<dyn Fn() -> Result<SourceFmtSharedString, SourceRetainedError> + Send + Sync>,
    android: bool) -> SourceKlogFileSystem
{
    SourceKlogFileSystem { hostname,
        temp_directory: Arc::new(move || {
            use std::os::unix::ffi::OsStrExt;
            let directory = std::env::var_os("TMPDIR");
            Ok(directory.filter(|value| !value.is_empty()).map_or_else(
                || string(if android { b"/data/local/tmp" } else { b"/tmp" }),
                |value| string(value.as_os_str().as_bytes())))
        }),
        open: Arc::new(|name, startup| Ok(native_open_file(&native_path(name), startup))),
        remove: Arc::new(|name| {
            let path = native_path(name);
            let first = match crate::utility_fs_error::retry_interrupted(|| std::fs::remove_file(&path)) {
                Ok(()) => return Ok(()), Err(error) => error,
            };
            let error = match crate::utility_fs_error::retry_interrupted(|| std::fs::remove_dir(&path)) {
                Ok(()) => return Ok(()),
                Err(error) if error.raw_os_error() == Some(libc::ENOTDIR) => first,
                Err(error) => error,
            };
            Err(crate::utility_fs_error::native_path_error(b"remove", name, error))
        }),
        symlink: Arc::new(|name, link| crate::utility_fs_error::retry_interrupted(||
            std::os::unix::fs::symlink(native_path(name), native_path(link)))
            .map_err(|error| crate::utility_fs_error::native_link_error(b"symlink", name, link, error))),
    }
}
fn native_error(error: std::io::Error) -> SourceRetainedError { crate::utility_fs_error::native_io_error(error) }

/// Linux/Android hostname effect: source uname first, Android fallback, and
/// one bounded /proc read with one trailing LF removed. Native returned error
/// classes still need the declared source PathError/errno/EOF bindings.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn native_linux_hostname(android: bool) -> Result<SourceFmtSharedString, SourceRetainedError> {
    use std::io::Read;
    let mut data = std::mem::MaybeUninit::<libc::utsname>::zeroed();
    // The output storage is initialized even if uname fails; no pointer is
    // retained after the call, and nodename is read within its actual array.
    let success = unsafe { libc::uname(data.as_mut_ptr()) } == 0;
    let data = unsafe { data.assume_init() };
    let bytes: Vec<u8> = data.nodename.iter().map(|byte| *byte as u8).collect();
    let name = bytes.iter().position(|byte| *byte == 0).map_or_else(Vec::new, |end| bytes[..end].to_vec());
    if success && !name.is_empty() && name.len() < 64 { return Ok(SourceFmtSharedString(Arc::from(name))); }
    if android { return Ok(if name.is_empty() { string(b"localhost") } else { SourceFmtSharedString(Arc::from(name)) }); }
    let mut file = crate::utility_fs_error::retry_interrupted(|| std::fs::File::open("/proc/sys/kernel/hostname"))
        .map_err(|error| crate::utility_fs_error::native_path_error(b"open", b"/proc/sys/kernel/hostname", error))?;
    let mut bytes = [0_u8; 512];
    let mut count = crate::utility_fs_error::retry_interrupted(|| file.read(&mut bytes))
        .map_err(|error| crate::utility_fs_error::native_path_error(b"read", b"/proc/sys/kernel/hostname", error))?;
    if count == 0 { return Err(crate::utility_fs_error::sentinel(crate::utility_fs_error::SourceFsSentinel::Eof)); }
    if bytes[count - 1] == b'\n' { count -= 1; }
    Ok(string(&bytes[..count]))
}
/// Native cached username projection for the selected Linux cgo lookup
/// profile. This is not the complete os/user.Current *User/type/cache API;
/// callers needing another source build profile must bind that actual effect.
#[cfg(target_os = "linux")]
pub fn native_linux_cached_username() -> Result<SourceFmtSharedString, SourceRetainedError> {
    static CURRENT: std::sync::OnceLock<Result<SourceFmtSharedString, SourceRetainedError>> = std::sync::OnceLock::new();
    let value = CURRENT.get_or_init(|| {
        // Source userBuffer.initialSize uses sysconf, 1024 for -1, otherwise
        // clamps an unreasonable size to the source 1-MiB retry limit.
        let size = unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) };
        let mut size = if size == -1 { 1024 } else if size <= 0 || size > 1 << 20 { 1 << 20 } else { size as usize };
        loop {
            let mut bytes = vec![0_u8; size];
            let mut entry = std::mem::MaybeUninit::<libc::passwd>::zeroed();
            let mut found: *mut libc::passwd = std::ptr::null_mut();
            // getpwuid_r writes entry and points its strings into this live
            // buffer. Copy pw_name before releasing either local allocation.
            let error = unsafe { libc::getpwuid_r(libc::getuid(), entry.as_mut_ptr(), bytes.as_mut_ptr().cast(), bytes.len(), &mut found) };
            if error == libc::ERANGE {
                size *= 2;
                if size > 1 << 20 { return Err(native_error(std::io::Error::other("internal buffer exceeds 1048576 bytes"))); }
                continue;
            }
            if error != 0 { return Err(native_error(std::io::Error::from_raw_os_error(error))); }
            if found.is_null() { return Err(native_error(std::io::ErrorKind::NotFound.into())); }
            let entry = unsafe { entry.assume_init() };
            let name = if entry.pw_name.is_null() { &b""[..] } else { unsafe { std::ffi::CStr::from_ptr(entry.pw_name).to_bytes() } };
            return Ok(string(name));
        }
    });
    match value { Ok(name) => Ok(name.source_fmt_copy()), Err(error) => Err(error.source_fmt_copy()) }
}
