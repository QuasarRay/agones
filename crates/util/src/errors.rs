use std::{
    any::type_name,
    error::Error,
    fmt,
};

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Errors {
    module: &'static str,
    struct_name: &'static str,
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

    fn prefix(&self, msg: &str) -> String {
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