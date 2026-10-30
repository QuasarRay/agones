//! Source fs.PathError, os.LinkError/SyscallError and Unix errno values.
// Go1.26.5 io/fs, os, syscall, errors and io. Copyright The Go Authors,
// BSD-3-Clause (../GO-LICENSE). Native object/type/ABI/GC, other platform
// errno data and complete filesystem/poll/descriptor profiles remain required.
use std::{borrow::Cow, error::Error, fmt, sync::{Arc, OnceLock, RwLock}};
use derive_deftly::Deftly;
use crate::{utility_error_alias::{SourceSharedError, source_error_original},
    utility_error_slice::SourceRetainedError, utility_format_copy::{SourceFmtSharedString, SourceFmtValueCopy},
    utility_error_query::{SourceErrorUnwrapOne, SourceErrorTimeout, SourceErrorTemporary, SourceErrorIs,
        SourceErrorIsTarget, SourceErrorCursor, try_source_error_timeout},
    utility_format_arguments::{SourcePrintfValue, write_source_error_value},
    utility_format_methods::{SourceErrorMethod, SourcePrintfMethodViews, SourcePrintfInterfaces, SourcePrintfRecursion, write_source_value},
    utility_format_collections::{SourcePrintfStructFields, write_source_struct},
    utility_format_pointer::write_source_address, utility_format_integer::write_integer,
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts}, utility_format_print::SourcePrintfMode,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError, NativeSourceCollectionContext, complete_source_collection},
    utility_go_quote::append, utility_klog_dispatch::SourceKlogFormatError,
    utility_declarations::{derive_deftly_template_SourceErrorOrigin, derive_deftly_template_SourcePrintfInterfaces,
        derive_deftly_template_SourceFsPointerError, derive_deftly_template_SourceNilErrorPointer,
        derive_deftly_template_SourceFmtValueCopy}};

pub struct SourcePathErrorFields { pub op: SourceFmtSharedString, pub path: SourceFmtSharedString, pub error: Option<SourceRetainedError> }
pub struct SourceLinkErrorFields { pub op: SourceFmtSharedString, pub old: SourceFmtSharedString, pub new: SourceFmtSharedString, pub error: Option<SourceRetainedError> }
pub struct SourceSyscallErrorFields { pub syscall: SourceFmtSharedString, pub error: Option<SourceRetainedError> }
#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceFsPointerError)]
#[deftly(source_package = "io/fs", source_struct = "PathError", source_log_type = "*fs.PathError",
    source_error_method, source_error_unwrap_one, source_error_timeout)]
pub struct SourcePathError { pub fields: RwLock<SourcePathErrorFields> }
#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceFsPointerError)]
#[deftly(source_package = "os", source_struct = "LinkError", source_log_type = "*os.LinkError",
    source_error_method, source_error_unwrap_one)]
pub struct SourceLinkError { pub fields: RwLock<SourceLinkErrorFields> }
#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceFsPointerError)]
#[deftly(source_package = "os", source_struct = "SyscallError", source_log_type = "*os.SyscallError",
    source_error_method, source_error_unwrap_one, source_error_timeout)]
pub struct SourceSyscallError { pub fields: RwLock<SourceSyscallErrorFields> }
pub(crate) trait SourceFsErrorFields: SourcePrintfValue {
    fn string_count(&self) -> usize;
    fn string_name(&self, index: usize) -> &'static str;
    fn string_field(&self, index: usize) -> SourceFmtSharedString;
    fn error_field(&self) -> Option<SourceRetainedError>;
}
impl SourceFsErrorFields for SourcePathError {
    fn string_count(&self) -> usize { 2 }
    fn string_name(&self, index: usize) -> &'static str { if index == 0 { "Op" } else { "Path" } }
    fn string_field(&self, index: usize) -> SourceFmtSharedString {
        let fields = self.fields.read().unwrap_or_else(|e| e.into_inner());
        if index == 0 { fields.op.source_fmt_copy() } else { fields.path.source_fmt_copy() }
    }
    fn error_field(&self) -> Option<SourceRetainedError> { self.fields.read().unwrap_or_else(|e| e.into_inner()).error.as_ref().map(SourceFmtValueCopy::source_fmt_copy) }
}
impl SourceFsErrorFields for SourceLinkError {
    fn string_count(&self) -> usize { 3 }
    fn string_name(&self, index: usize) -> &'static str { ["Op", "Old", "New"][index] }
    fn string_field(&self, index: usize) -> SourceFmtSharedString {
        let fields = self.fields.read().unwrap_or_else(|e| e.into_inner());
        match index { 0 => fields.op.source_fmt_copy(), 1 => fields.old.source_fmt_copy(), _ => fields.new.source_fmt_copy() }
    }
    fn error_field(&self) -> Option<SourceRetainedError> { self.fields.read().unwrap_or_else(|e| e.into_inner()).error.as_ref().map(SourceFmtValueCopy::source_fmt_copy) }
}
impl SourceFsErrorFields for SourceSyscallError {
    fn string_count(&self) -> usize { 1 }
    fn string_name(&self, _: usize) -> &'static str { "Syscall" }
    fn string_field(&self, _: usize) -> SourceFmtSharedString { self.fields.read().unwrap_or_else(|e| e.into_inner()).syscall.source_fmt_copy() }
    fn error_field(&self) -> Option<SourceRetainedError> { self.fields.read().unwrap_or_else(|e| e.into_inner()).error.as_ref().map(SourceFmtValueCopy::source_fmt_copy) }
}
pub(crate) fn pointer_message<Value: SourceFsErrorFields>(value: &Value) -> Cow<'_, [u8]> {
    // All string reads precede the direct Err.Error call. Release each native
    // guard before calling a source method, which may mutate the outer fields.
    // Concat arguments are evaluated before allocating the outer result.
    let mut parts: [Option<SourceFmtSharedString>; 3] = std::array::from_fn(|_| None);
    for index in 0..value.string_count() {
        parts[index] = Some(value.string_field(index));
    }
    let error = value.error_field().unwrap_or_else(|| crate::utility_error_nil::nil_error_receiver());
    let message = crate::utility_error_format::source_error_message(error.original()).unwrap_or_else(||
        std::panic::panic_any(SourceKlogFormatError::Unbound { source_type: crate::utility_error_view::source_error_type(error.original()).map(str::to_owned),
            operation: "filesystem wrapper direct Err.Error method" }));
    let mut bytes = Vec::new();
    for (index, part) in parts[..value.string_count()].iter().enumerate() {
        if index > 0 { complete_source_collection(&NativeSourceCollectionContext, append(&mut bytes, b" ")); }
        complete_source_collection(&NativeSourceCollectionContext, append(&mut bytes, &part.as_ref().unwrap().0));
    }
    complete_source_collection(&NativeSourceCollectionContext, append(&mut bytes, b": "));
    complete_source_collection(&NativeSourceCollectionContext, append(&mut bytes, &message));
    Cow::Owned(bytes)
}
pub(crate) fn pointer_timeout<Value: SourceFsErrorFields>(value: &Value) -> bool {
    try_source_error_timeout(value.error_field().map(SourceErrorCursor::retained)).unwrap_or_else(|error| std::panic::panic_any(error))
}
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
struct PointerFields<'a, Value: SourceFsErrorFields>(&'a Value);
impl<Value: SourceFsErrorFields> SourcePrintfValue for PointerFields<'_, Value> {
    fn source_type(&self) -> &str { self.0.source_type().strip_prefix('*').unwrap_or(self.0.source_type()) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl<Value: SourceFsErrorFields> SourcePrintfStructFields for PointerFields<'_, Value> {
    fn source_field_count(&self) -> usize { self.0.string_count() + 1 }
    fn source_field_name(&self, index: usize) -> &str { if index < self.0.string_count() { self.0.string_name(index) } else { "Err" } }
    fn source_field_interfaceable(&self, _: usize) -> bool { true }
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if index < self.0.string_count() { return write_source_value(&self.0.string_field(index), verb, state, profile, mode, recursion, output); }
        match self.0.error_field() {
            Some(error) => write_source_error_value(&error, verb, state, profile, mode, recursion, output),
            None => crate::utility_error_handlers::SourceErrorArgument::from_error(None)
                .try_write_recursive(verb, state, profile, mode, recursion, output).expect("nil interface has a source format view"),
        }
    }
}
pub(crate) fn write_pointer<Value: SourceFsErrorFields>(value: &Value, verb: char, state: &SourcePrintfResolvedCounts,
    profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
    output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    if recursion.depth == 0 && verb != 'p' {
        append(output, b"&")?; return write_source_struct(&PointerFields(value), verb, state, profile, mode, recursion.child(true), output);
    }
    write_source_address(value, value as *const Value as usize as u64, verb, state, profile, mode, recursion, output)
}
fn string(bytes: &[u8]) -> SourceFmtSharedString { SourceFmtSharedString(Arc::from(bytes)) }
fn retained<Value: Error + SourcePrintfValue + SourceErrorMethod + crate::runtime::SourceLogType + Send + Sync + 'static>(value: Value) -> SourceRetainedError {
    retain_box(Box::new(value))
}
fn retain_box<Value: Error + SourcePrintfValue + SourceErrorMethod + crate::runtime::SourceLogType + Send + Sync + 'static>(value: Box<Value>) -> SourceRetainedError {
    SourceRetainedError::from_original(SourceSharedError::from_original(value).into_error())
}
/// An existing Box can enter SourcePrintfOwnedError without moving or
/// reboxing its pointer payload when printf later transfers a wrapped error.
pub fn boxed_path_error(op: &[u8], path: &[u8], error: Option<SourceRetainedError>) -> Box<SourcePathError> {
    Box::new(SourcePathError { fields: RwLock::new(SourcePathErrorFields { op: string(op), path: string(path), error }) })
}
pub fn boxed_link_error(op: &[u8], old: &[u8], new: &[u8], error: Option<SourceRetainedError>) -> Box<SourceLinkError> {
    Box::new(SourceLinkError { fields: RwLock::new(SourceLinkErrorFields { op: string(op), old: string(old), new: string(new), error }) })
}
pub fn path_error(op: &[u8], path: &[u8], error: Option<SourceRetainedError>) -> SourceRetainedError {
    retain_box(boxed_path_error(op, path, error))
}
pub fn link_error(op: &[u8], old: &[u8], new: &[u8], error: Option<SourceRetainedError>) -> SourceRetainedError {
    retain_box(boxed_link_error(op, old, new, error))
}
pub fn new_syscall_error(syscall: &[u8], error: Option<SourceRetainedError>) -> Option<SourceRetainedError> {
    error.map(|error| retained(SourceSyscallError { fields: RwLock::new(SourceSyscallErrorFields { syscall: string(syscall), error: Some(error) }) }))
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum SourceFsSentinel { Invalid, Permission, Exist, NotExist, Closed, Unsupported, Eof, ShortWrite }
pub fn sentinel(kind: SourceFsSentinel) -> SourceRetainedError {
    static ERRORS: OnceLock<[SourceRetainedError; 8]> = OnceLock::new();
    ERRORS.get_or_init(|| [b"invalid argument".as_slice(), b"permission denied", b"file already exists",
        b"file does not exist", b"file already closed", b"unsupported operation", b"EOF", b"short write"]
        .map(|message| SourceRetainedError::from_original(Box::new(crate::utility_error_format::MessageError(
            crate::utility_error_format::SourceErrorMessage::Bytes(message.to_vec()))))))[kind as usize].source_fmt_copy()
}
pub fn sentinel_target(kind: SourceFsSentinel) -> SourceErrorIsTarget<'static> {
    SourceErrorIsTarget::declared(SourceErrorCursor::retained(sentinel(kind)),
        Some(crate::utility_error_query::pointer_equal::<crate::utility_error_format::MessageError>))
}
fn target_is(target: &SourceErrorIsTarget<'_>, kind: SourceFsSentinel) -> bool {
    let Some(target) = target.error().and_then(|error| error.downcast_ref::<crate::utility_error_format::MessageError>()) else { return false; };
    let canonical = sentinel(kind);
    let canonical = source_error_original(canonical.original()).downcast_ref::<crate::utility_error_format::MessageError>().unwrap();
    std::ptr::eq(target, canonical)
}
fn os_underlying_error(error: Option<SourceErrorCursor<'_>>) -> Option<SourceErrorCursor<'_>> {
    let error = error?;
    let original = error.original();
    if original.is::<SourcePathError>() || original.is::<SourceLinkError>() || original.is::<SourceSyscallError>()
        || original.is::<SourceNilPathError>() || original.is::<SourceNilLinkError>() || original.is::<SourceNilSyscallError>() {
        return unwrapped(original).unwrap().map(SourceErrorCursor::retained);
    }
    Some(error)
}
/// Legacy os predicates unwrap one known os wrapper, then inspect only errno
/// or the exact canonical target. They do not traverse arbitrary Unwrap/Is.
pub fn try_os_is(error: Option<SourceErrorCursor<'_>>, kind: SourceFsSentinel) -> Result<bool, crate::utility_error_query::SourceErrorBindingError> {
    let Some(error) = os_underlying_error(error) else { return Ok(false); };
    let target = SourceErrorIsTarget::declared(error.source_fmt_copy(),
        Some(crate::utility_error_query::pointer_equal::<crate::utility_error_format::MessageError>));
    if target_is(&target, kind) { return Ok(true); }
    if let Some(errno) = error.original().downcast_ref::<SourceUnixErrno>() { return Ok(errno.source_error_is(&sentinel_target(kind))); }
    let source_type = crate::utility_error_view::source_error_type(error.original());
    if source_type.is_none() || source_type == Some("syscall.Errno") {
        return Err(crate::utility_error_query::SourceErrorBindingError { source_type });
    }
    Ok(false)
}
pub fn try_os_is_exist(error: Option<SourceErrorCursor<'_>>) -> Result<bool, crate::utility_error_query::SourceErrorBindingError> { try_os_is(error, SourceFsSentinel::Exist) }
pub fn try_os_is_not_exist(error: Option<SourceErrorCursor<'_>>) -> Result<bool, crate::utility_error_query::SourceErrorBindingError> { try_os_is(error, SourceFsSentinel::NotExist) }
pub fn try_os_is_permission(error: Option<SourceErrorCursor<'_>>) -> Result<bool, crate::utility_error_query::SourceErrorBindingError> { try_os_is(error, SourceFsSentinel::Permission) }
pub fn try_os_is_timeout(error: Option<SourceErrorCursor<'_>>) -> Result<bool, crate::utility_error_query::SourceErrorBindingError> { try_source_error_timeout(os_underlying_error(error)) }
pub struct SourceUnixErrnoProfile {
    pub word_bits: u8, pub message: fn(u64) -> Option<&'static [u8]>,
    pub permission: &'static [u64], pub exist: &'static [u64], pub not_exist: &'static [u64], pub unsupported: &'static [u64],
    pub timeout: &'static [u64], pub temporary: &'static [u64],
}
pub static LINUX_AMD64: SourceUnixErrnoProfile = SourceUnixErrnoProfile { word_bits: 64,
    message: crate::utility_fs_errno_linux::message, permission: &[1, 13], exist: &[17, 39], not_exist: &[2],
    unsupported: &[38, 95], timeout: &[11, 110], temporary: &[4, 24, 23] };
#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces)]
#[deftly(source_package = "syscall", source_struct = "Errno", source_log_type = "syscall.Errno",
    source_error_method, source_error_is, source_error_timeout, source_error_temporary)]
pub struct SourceUnixErrno { bits: u64, profile: &'static SourceUnixErrnoProfile }
impl SourceUnixErrno {
    pub fn new(bits: u64, profile: &'static SourceUnixErrnoProfile) -> Self {
        Self { bits: if profile.word_bits == 32 { u64::from(bits as u32) } else { bits }, profile }
    }
    pub fn bits(&self) -> u64 { self.bits }
    pub fn into_error(self) -> SourceRetainedError { retained(self) }
    fn signed(&self) -> i64 { if self.profile.word_bits == 32 { i64::from(self.bits as i32) } else { self.bits as i64 } }
}
/// Source errnoErr's nil and common interface-allocation branches for this
/// actual target profile. Other targets keep their own required code data.
pub fn linux_amd64_errno_error(code: u64) -> Option<SourceRetainedError> {
    if code == 0 { return None; }
    if let Some(index) = [11, 22, 2].iter().position(|value| *value == code) {
        static COMMON: OnceLock<[SourceRetainedError; 3]> = OnceLock::new();
        return Some(COMMON.get_or_init(|| [11, 22, 2].map(|code| SourceUnixErrno::new(code, &LINUX_AMD64).into_error()))[index].source_fmt_copy());
    }
    Some(SourceUnixErrno::new(code, &LINUX_AMD64).into_error())
}
impl PartialEq for SourceUnixErrno { fn eq(&self, other: &Self) -> bool { self.bits == other.bits } }
impl SourceFmtValueCopy for SourceUnixErrno { fn source_fmt_copy(&self) -> Self { Self { bits: self.bits, profile: self.profile } } }
impl SourceErrorMethod for SourceUnixErrno {
    fn source_error_message(&self) -> Cow<'_, [u8]> {
        if self.signed() >= 0 { if let Some(message) = (self.profile.message)(self.bits) { if !message.is_empty() { return Cow::Borrowed(message); } } }
        Cow::Owned(format!("errno {}", self.signed()).into_bytes())
    }
}
impl SourceErrorIs for SourceUnixErrno {
    fn source_error_is(&self, target: &SourceErrorIsTarget<'_>) -> bool {
        [(SourceFsSentinel::Permission, self.profile.permission), (SourceFsSentinel::Exist, self.profile.exist),
            (SourceFsSentinel::NotExist, self.profile.not_exist), (SourceFsSentinel::Unsupported, self.profile.unsupported)]
            .into_iter().any(|(kind, codes)| target_is(target, kind) && codes.contains(&self.bits))
    }
}
impl SourceErrorTimeout for SourceUnixErrno { fn source_error_timeout(&self) -> bool { self.profile.timeout.contains(&self.bits) } }
impl SourceErrorTemporary for SourceUnixErrno {
    fn source_error_temporary(&self) -> bool { self.profile.temporary.contains(&self.bits) || self.source_error_timeout() }
}
impl fmt::Display for SourceUnixErrno { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&String::from_utf8_lossy(&self.source_error_message())) } }
impl fmt::Debug for SourceUnixErrno { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self, out) } }
impl Error for SourceUnixErrno {}
impl SourcePrintfValue for SourceUnixErrno {
    fn source_type(&self) -> &str { "syscall.Errno" }
    fn source_integer(&self) -> SourcePrintfInteger { SourcePrintfInteger::Unsigned(self.bits) }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { Some(self) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_integer(output, self.bits, false, "syscall.Errno", verb, state, profile) }
    fn into_source_error(self) -> Option<crate::errors::BoxError> { Some(self.into_error().into_error()) }
}
pub(crate) fn native_io_error(error: std::io::Error) -> SourceRetainedError {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        if let Some(code) = error.raw_os_error() {
            let code = code as i64 as u64;
            return linux_amd64_errno_error(code).unwrap_or_else(|| SourceUnixErrno::new(0, &LINUX_AMD64).into_error());
        }
    }
    // Unsupported native errors remain actual data, with no guessed source
    // errno, invented type/method set, or Display string as source evidence.
    SourceRetainedError::from_original(Box::new(error))
}
fn native_name_error(names: &[&[u8]], error: std::io::Error) -> SourceRetainedError {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    if error.raw_os_error().is_none() && error.kind() == std::io::ErrorKind::InvalidInput
        && names.iter().any(|name| name.contains(&0)) {
        // This is the concrete BytePtrFromString failure, not a guess from
        // an arbitrary native InvalidInput error or its Display text.
        return linux_amd64_errno_error(22).unwrap();
    }
    native_io_error(error)
}
pub(crate) fn native_path_error(op: &[u8], path: &[u8], error: std::io::Error) -> SourceRetainedError {
    path_error(op, path, Some(native_name_error(&[path], error)))
}
pub(crate) fn native_link_error(op: &[u8], old: &[u8], new: &[u8], error: std::io::Error) -> SourceRetainedError {
    link_error(op, old, new, Some(native_name_error(&[old, new], error)))
}
pub(crate) fn retry_interrupted<Value>(mut operation: impl FnMut() -> std::io::Result<Value>) -> std::io::Result<Value> {
    loop {
        match operation() {
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            result => return result,
        }
    }
}
pub(crate) fn methods(error: &(dyn Error + 'static)) -> Option<SourcePrintfInterfaces<'_>> {
    macro_rules! declared { ($($ty:ty),+) => { $(if let Some(value) = error.downcast_ref::<$ty>() { return Some(value.source_methods()); })+ }; }
    declared!(SourcePathError, SourceLinkError, SourceSyscallError, SourceUnixErrno,
        SourceNilPathError, SourceNilLinkError, SourceNilSyscallError);
    None
}
pub(crate) fn unwrapped(error: &(dyn Error + 'static)) -> Option<Option<SourceRetainedError>> { methods(error)?.error_methods.unwrap_one.map(SourceErrorUnwrapOne::source_unwrap_one) }
pub(crate) fn source_type(error: &(dyn Error + 'static)) -> Option<&'static str> {
    macro_rules! declared { ($($ty:ty),+) => { $(if error.is::<$ty>() { return Some(<$ty as crate::runtime::SourceLogType>::SOURCE_LOG_TYPE); })+ }; }
    declared!(SourcePathError, SourceLinkError, SourceSyscallError, SourceUnixErrno, SourceNilPathError, SourceNilLinkError, SourceNilSyscallError);
    None
}
pub(crate) fn write_value(error: &(dyn Error + 'static), verb: char, state: &SourcePrintfResolvedCounts,
    profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
    output: &mut Vec<u8>) -> Option<Result<(), SourceSliceAllocationError>>
{
    macro_rules! declared { ($($ty:ty),+) => { $(if let Some(value) = error.downcast_ref::<$ty>() {
        return Some(write_source_value(value, verb, state, profile, mode, recursion, output)); })+ }; }
    declared!(SourcePathError, SourceLinkError, SourceSyscallError, SourceUnixErrno, SourceNilPathError, SourceNilLinkError, SourceNilSyscallError);
    None
}

#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceNilErrorPointer, SourceFmtValueCopy)]
#[deftly(source_package = "io/fs", source_struct = "PathError", source_log_type = "*fs.PathError",
    source_error_method, source_error_unwrap_one, source_error_timeout, source_nil_error_pointer,
    source_error_target = "crate::utility_fs_error::SourcePathError")]
pub struct SourceNilPathError;
#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceNilErrorPointer, SourceFmtValueCopy)]
#[deftly(source_package = "os", source_struct = "LinkError", source_log_type = "*os.LinkError",
    source_error_method, source_error_unwrap_one, source_nil_error_pointer,
    source_error_target = "crate::utility_fs_error::SourceLinkError")]
pub struct SourceNilLinkError;
#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin, SourcePrintfInterfaces, SourceNilErrorPointer, SourceFmtValueCopy)]
#[deftly(source_package = "os", source_struct = "SyscallError", source_log_type = "*os.SyscallError",
    source_error_method, source_error_unwrap_one, source_error_timeout, source_nil_error_pointer,
    source_error_target = "crate::utility_fs_error::SourceSyscallError")]
pub struct SourceNilSyscallError;
impl SourceErrorUnwrapOne for SourceNilPathError { fn source_unwrap_one(&self) -> Option<SourceRetainedError> { crate::utility_error_nil::nil_error_receiver() } }
impl SourceErrorUnwrapOne for SourceNilLinkError { fn source_unwrap_one(&self) -> Option<SourceRetainedError> { crate::utility_error_nil::nil_error_receiver() } }
impl SourceErrorUnwrapOne for SourceNilSyscallError { fn source_unwrap_one(&self) -> Option<SourceRetainedError> { crate::utility_error_nil::nil_error_receiver() } }
impl SourceErrorTimeout for SourceNilPathError { fn source_error_timeout(&self) -> bool { crate::utility_error_nil::nil_error_receiver() } }
impl SourceErrorTimeout for SourceNilSyscallError { fn source_error_timeout(&self) -> bool { crate::utility_error_nil::nil_error_receiver() } }
