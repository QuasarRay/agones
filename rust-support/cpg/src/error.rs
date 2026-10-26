//! Source exception values at native build-effect boundaries.
//!
//! Preserve class, absent/empty messages, causes and initializer failures as
//! data. A Rust I/O display string is not the source exception's identity.
use std::{fmt, io, path::Path, sync::Arc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrowableCategory { Exception, Error, VirtualMachineError, LinkageError, Interrupted }

#[derive(Clone, Debug)]
pub struct Error {
    pub class: &'static str,
    pub message: Option<String>,
    pub category: ThrowableCategory,
    pub cause: Option<Arc<Error>>,
    pub suppressed: Vec<Error>,
    foreign: Option<Arc<cpg_rs::Error>>,
}

impl Error {
    pub fn exception(class: &'static str, message: impl Into<Option<String>>) -> Self {
        Self { class, message: message.into(), category: ThrowableCategory::Exception,
            cause: None, suppressed: Vec::new(), foreign: None }
    }

    pub fn with_cause(mut self, cause: Error) -> Self { self.cause = Some(Arc::new(cause)); self }
    pub fn source_exception_class(&self) -> &'static str { self.class }
    pub fn is_java_error(&self) -> bool { self.category != ThrowableCategory::Exception && self.category != ThrowableCategory::Interrupted }
    pub fn is_nonfatal(&self) -> bool { matches!(self.category, ThrowableCategory::Exception | ThrowableCategory::Error) }
    pub fn is_io_exception(&self) -> bool {
        matches!(self.class,
            "java.io.IOException" | "java.io.FileNotFoundException" | "java.io.EOFException"
            | "java.nio.file.FileSystemException" | "java.nio.file.NoSuchFileException"
            | "java.nio.file.AccessDeniedException" | "java.nio.file.FileAlreadyExistsException"
            | "java.nio.file.NotDirectoryException" | "java.nio.file.DirectoryNotEmptyException"
            | "java.nio.file.FileSystemLoopException" | "java.nio.channels.ClosedChannelException"
            | "java.nio.charset.MalformedInputException" | "java.nio.charset.UnmappableCharacterException")
    }

    pub fn java_to_string(&self) -> String {
        match &self.message {
            Some(message) => format!("{}: {message}", self.class),
            None => self.class.to_owned(),
        }
    }

    #[allow(non_snake_case)]
    pub fn SourceAssertion(message: String) -> Self {
        let mut error = Self::exception("java.lang.AssertionError", Some(message));
        error.category = ThrowableCategory::Error;
        error
    }
    #[allow(non_snake_case)]
    pub fn IllegalArgument(message: String) -> Self { Self::exception("java.lang.IllegalArgumentException", Some(message)) }
    #[allow(non_snake_case)]
    pub fn Invalid(message: String) -> Self { Self::IllegalArgument(message) }
    #[allow(non_snake_case)]
    pub fn Runtime(message: String) -> Self { Self::exception("java.lang.RuntimeException", Some(message)) }
    pub fn out_of_memory() -> Self {
        let mut error = Self::exception("java.lang.OutOfMemoryError", Some("Java heap space".into()));
        error.category = ThrowableCategory::VirtualMachineError;
        error
    }
    #[allow(non_snake_case)]
    pub fn Io(error: io::Error) -> Self { Self::stream_io(error) }

    pub fn malformed_input(length: usize) -> Self {
        Self::exception("java.nio.charset.MalformedInputException", Some(format!("Input length = {length}")))
    }

    pub fn stream_io(error: io::Error) -> Self {
        Self::exception("java.io.IOException", Some(os_reason(&error)))
    }

    pub fn process_start(program: &str, error: io::Error) -> Self {
        let detail = match error.raw_os_error() {
            Some(code) => format!("error={code}, {}", os_reason(&error)),
            None => os_reason(&error),
        };
        Self::exception("java.io.IOException", Some(format!("Cannot run program \"{program}\": {detail}")))
    }

    pub fn file_stream_open(path: &Path, error: io::Error) -> Self {
        Self::exception("java.io.FileNotFoundException", Some(format!("{} ({})", path.display(), os_reason(&error))))
    }

    pub fn nio(path: &Path, error: io::Error, opening_directory: bool) -> Self {
        let class = match error.kind() {
            io::ErrorKind::NotFound => "java.nio.file.NoSuchFileException",
            io::ErrorKind::PermissionDenied => "java.nio.file.AccessDeniedException",
            io::ErrorKind::AlreadyExists => "java.nio.file.FileAlreadyExistsException",
            io::ErrorKind::NotADirectory if opening_directory => "java.nio.file.NotDirectoryException",
            _ => "java.nio.file.FileSystemException",
        };
        let message = if class == "java.nio.file.FileSystemException" {
            format!("{}: {}", path.display(), os_reason(&error))
        } else { path.display().to_string() };
        Self::exception(class, Some(message))
    }

    pub fn unchecked_io(cause: Error) -> Self {
        Self::exception("java.io.UncheckedIOException", Some(cause.java_to_string())).with_cause(cause)
    }

    pub fn initializer_failure(cause: Error) -> Self {
        if cause.is_java_error() { return cause; }
        let mut error = Self::exception("java.lang.ExceptionInInitializerError", None);
        error.category = ThrowableCategory::LinkageError;
        error.with_cause(cause)
    }

    pub fn failed_class(name: &str, initial_failure: &Error) -> Self {
        let mut error = Self::exception("java.lang.NoClassDefFoundError", Some(format!("Could not initialize class {name}")));
        error.category = ThrowableCategory::LinkageError;
        error.with_cause(initial_failure.clone())
    }

    pub fn initialization_snapshot(failure: &Error, thread_name: &str) -> Self {
        // OpenJDK 21 retains a new EIIE diagnostic, rather than retaining the
        // first throwable as the later NoClassDefFoundError's direct cause.
        let mut snapshot = Self::exception("java.lang.ExceptionInInitializerError",
            Some(format!("Exception {} [in thread \"{thread_name}\"]", failure.java_to_string())));
        snapshot.category = ThrowableCategory::LinkageError;
        snapshot
    }
}

fn os_reason(error: &io::Error) -> String {
    let text = error.to_string();
    match error.raw_os_error() {
        Some(code) => text.strip_suffix(&format!(" (os error {code})")).unwrap_or(&text).to_owned(),
        None => text,
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&self.java_to_string()) }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(cause) = &self.cause { Some(cause.as_ref()) }
        else { self.foreign.as_ref().map(|error| error.as_ref() as &(dyn std::error::Error + 'static)) }
    }
}
impl From<io::Error> for Error { fn from(error: io::Error) -> Self { Self::stream_io(error) } }
impl From<cpg_rs::Error> for Error {
    fn from(error: cpg_rs::Error) -> Self {
        let class = error.source_exception_class().unwrap_or("java.lang.RuntimeException");
        let mut source = Self::exception(class, Some(error.to_string()));
        if class.ends_with("Error") { source.category = ThrowableCategory::Error; }
        source.foreign = Some(Arc::new(error));
        source
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self { Self::IllegalArgument(error.to_string()) }
}
impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Self::exception("java.util.zip.ZipException", Some(error.to_string()))
    }
}
