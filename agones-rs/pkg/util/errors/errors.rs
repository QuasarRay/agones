// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2, pkg/util/errors/errors.go.
// Copyright Contributors to Agones a Series of LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case)]
use std::{
    any::type_name,
    error::Error,
    fmt,
};
use derive_deftly::Deftly;
use crate::utility_declarations::derive_deftly_template_SourceErrorOrigin;

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "agones.dev/agones/pkg/util/errors", source_struct = "Errors")]
pub struct Errors {
    module: &'static str,
    struct_name: &'static str,
}

/// Source type identity comes from a declaration, rather than a Rust type
/// spelling. Pointer implementations retain the underlying defined Go type.
pub trait SourceErrorOrigin {
    const SOURCE_PACKAGE: &'static str;
    const SOURCE_STRUCT: &'static str;
}
impl<T: SourceErrorOrigin + ?Sized> SourceErrorOrigin for &T {
    const SOURCE_PACKAGE: &'static str = T::SOURCE_PACKAGE;
    const SOURCE_STRUCT: &'static str = T::SOURCE_STRUCT;
}
impl<T: SourceErrorOrigin + ?Sized> SourceErrorOrigin for &mut T {
    const SOURCE_PACKAGE: &'static str = T::SOURCE_PACKAGE;
    const SOURCE_STRUCT: &'static str = T::SOURCE_STRUCT;
}
impl<T: SourceErrorOrigin + ?Sized> SourceErrorOrigin for Box<T> {
    const SOURCE_PACKAGE: &'static str = T::SOURCE_PACKAGE;
    const SOURCE_STRUCT: &'static str = T::SOURCE_STRUCT;
}
impl<T: SourceErrorOrigin + ?Sized> SourceErrorOrigin for std::sync::Arc<T> {
    const SOURCE_PACKAGE: &'static str = T::SOURCE_PACKAGE;
    const SOURCE_STRUCT: &'static str = T::SOURCE_STRUCT;
}
impl<T: SourceErrorOrigin + ?Sized> SourceErrorOrigin for *const T {
    const SOURCE_PACKAGE: &'static str = T::SOURCE_PACKAGE;
    const SOURCE_STRUCT: &'static str = T::SOURCE_STRUCT;
}
impl<T: SourceErrorOrigin + ?Sized> SourceErrorOrigin for *mut T {
    const SOURCE_PACKAGE: &'static str = T::SOURCE_PACKAGE;
    const SOURCE_STRUCT: &'static str = T::SOURCE_STRUCT;
}
macro_rules! unnamed_origins {
    ($($ty:ty),* $(,)?) => { $(impl SourceErrorOrigin for $ty {
        const SOURCE_PACKAGE: &'static str = "";
        const SOURCE_STRUCT: &'static str = "";
    })* };
}
unnamed_origins!((), bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, str, String);

/// The caller performs its typed formatting once and retains every `%w`
/// operand in source argument order. This value does not parse a Go format
/// string; complete Go format lowering remains an explicit adoption task.
pub struct SourceFormat {
    pub rendered: String,
    pub wrapped: Vec<BoxError>,
}

impl Errors {
    /// Used by `errors_from_package!()`.
    pub const fn from_package(module: &'static str) -> Self {
        Self {
            module,
            struct_name: "",
        }
    }

    /// Rust equivalent of Go's FromStruct.
    pub fn from_struct<T: 'static>() -> Self {
        let name = type_name::<T>();

        match name.rsplit_once("::") {
            Some((module, name)) => Self {
                module,
                struct_name: name.split('<').next().unwrap_or(name),
            },

            // Primitive / unnamed-like type.
            None => Self::default(),
        }
    }

    pub fn FromPackage(caller_function: Option<&'static str>) -> Self {
        Self { module: caller_function.map(package_from_func_slice).unwrap_or_default(), struct_name: "" }
    }

    pub fn FromStruct<T: SourceErrorOrigin + ?Sized>(_: Option<&T>, caller_function: Option<&'static str>) -> Self {
        if T::SOURCE_PACKAGE.is_empty() { return Self::FromPackage(caller_function); }
        Self { module: T::SOURCE_PACKAGE, struct_name: T::SOURCE_STRUCT }
    }

    pub fn New(&self, message: impl Into<String>) -> BoxError { self.new(message) }

    pub fn Errorf(&self, format: SourceFormat) -> BoxError {
        if format.wrapped.is_empty() { return self.New(format.rendered); }
        Box::new(FormattedError { message: self.prefix(&format.rendered), wrapped: format.wrapped })
    }

    pub fn Wrap(&self, error: Option<BoxError>, message: &str) -> Option<BoxError> {
        error.map(|source| Box::new(WrappedError { message: self.prefix(message), source }) as BoxError)
    }

    pub fn Wrapf(&self, error: Option<BoxError>, format: impl FnOnce() -> String) -> Option<BoxError> {
        // Source Wrapf returns before formatting when err is nil. Keeping the
        // formatting closure lazy also preserves Stringer/effect ordering.
        error.map(|source| Box::new(WrappedError { message: self.prefix(&format()), source }) as BoxError)
    }

    /// Creates a new prefixed error.
    pub fn new(&self, msg: impl Into<String>) -> BoxError {
        Box::new(MessageError(self.prefix(&msg.into())))
    }

    /// Rust equivalent of Errorf.
    ///
    /// Usage:
    /// `errs.errorf(format_args!("invalid value: {value}"))`
    pub fn errorf(&self, args: fmt::Arguments<'_>) -> BoxError {
        self.new(args.to_string())
    }

    /// Wraps an existing error while preserving its source chain.
    pub fn wrap<E>(
        &self,
        err: E,
        msg: impl AsRef<str>,
    ) -> BoxError
    where
        E: Error + Send + Sync + 'static,
    {
        Box::new(WrappedError {
            message: self.prefix(msg.as_ref()),
            source: Box::new(err),
        })
    }

    /// Formatted equivalent of Wrapf.
    pub fn wrapf<E>(
        &self,
        err: E,
        args: fmt::Arguments<'_>,
    ) -> BoxError
    where
        E: Error + Send + Sync + 'static,
    {
        self.wrap(err, args.to_string())
    }

    /// Equivalent to Go's `Wrap(nil, ...) == nil`.
    pub fn wrap_opt<E>(
        &self,
        err: Option<E>,
        msg: impl AsRef<str>,
    ) -> Option<BoxError>
    where
        E: Error + Send + Sync + 'static,
    {
        err.map(|err| self.wrap(err, msg))
    }

    /// Equivalent to Go's nil-preserving Wrapf.
    pub fn wrapf_opt<E>(
        &self,
        err: Option<E>,
        args: fmt::Arguments<'_>,
    ) -> Option<BoxError>
    where
        E: Error + Send + Sync + 'static,
    {
        err.map(|err| self.wrapf(err, args))
    }

    pub fn prefix(&self, msg: &str) -> String {
        match (self.module.is_empty(), self.struct_name.is_empty()) {
            (true, true) => msg.to_owned(),
            (false, true) => format!("{}: {msg}", self.module),
            (true, false) => format!("{}: {msg}", self.struct_name),
            (false, false) => {
                format!("{}.{}: {msg}", self.module, self.struct_name)
            }
        }
    }
}

pub fn packageFromFunc(name: &str) -> String {
    package_from_func_slice(name).to_owned()
}
fn package_from_func_slice(name: &str) -> &str {
    let segment = name.rfind('/').map_or(0, |index| index + 1);
    name[segment..].find('.').map_or(name, |dot| &name[..segment + dot])
}

/// The native declaration/call-site adapter supplies source function names.
/// Missing PC/function entries retain runtime.Caller/FuncForPC's empty result.
pub fn callerPackage(frames: &[Option<&str>], skip: isize) -> String {
    usize::try_from(skip).ok().and_then(|index| frames.get(index)).and_then(|name| *name)
        .map(packageFromFunc).unwrap_or_default()
}

#[derive(Debug)]
struct FormattedError { message: String, wrapped: Vec<BoxError> }
impl fmt::Display for FormattedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&self.message) }
}
impl Error for FormattedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if self.wrapped.len() == 1 { Some(self.wrapped[0].as_ref()) } else { None }
    }
}

/// Source errors.As/errors.Is can traverse both Unwrap() error and
/// Unwrap() []error. Rust's standard source() represents only the first form.
pub fn contains_error<T: Error + 'static>(error: &(dyn Error + 'static)) -> bool {
    if error.is::<T>() { return true; }
    if let Some(formatted) = error.downcast_ref::<FormattedError>() {
        return formatted.wrapped.iter().any(|source| contains_error::<T>(source.as_ref()));
    }
    error.source().is_some_and(contains_error::<T>)
}

#[derive(Debug)]
struct MessageError(String);

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for MessageError {}

#[derive(Debug)]
struct WrappedError {
    message: String,
    source: BoxError,
}

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.source)
    }
}

impl Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

/// Exact Rust analogue of Go's `FromPackage()`.
///
/// `module_path!()` is expanded at the caller, so this captures the package/module
/// in which the macro is invoked.
#[macro_export]
macro_rules! errors_from_package {
    ($source_package:expr) => {
        $crate::errors::Errors::from_package($source_package)
    };
    () => {
        $crate::errors::Errors::from_package(module_path!())
    };
}

/// Convenience equivalent of Go's `FromStruct(c)`.
#[macro_export]
macro_rules! errors_from_struct {
    ($ty:ty) => {
        $crate::errors::Errors::from_struct::<$ty>()
    };
}
