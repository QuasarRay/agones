//! Typed printer-state input and canonical source fmt/errors result objects.
// Adapted from Go fmt/errors.go. Copyright The Go Authors. BSD-3-Clause;
// see ../GO-LICENSE. Raw scanning is in utility_format_scan; full rendering
// and source prefix-format lowering remain required.
use std::{error::Error, fmt};
use derive_deftly::Deftly;
use crate::{errors::BoxError, utility_declarations::derive_deftly_template_SourceErrorOrigin};

/// Native bridge errors describe inconsistent normalized printer data. They
/// are not fmt.Errorf results for invalid Go format strings or operands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceFormatInputError {
    ArgumentIndex { occurrence: usize, index: usize, arguments: usize },
    ImplicitOrder { occurrence: usize, previous: usize, index: usize },
}
impl fmt::Display for SourceFormatInputError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgumentIndex { occurrence, index, arguments } => write!(out, "source printer wrap occurrence {occurrence} indexes argument {index} outside {arguments} arguments"),
            Self::ImplicitOrder { occurrence, previous, index } => write!(out, "source printer implicit wrap occurrence {occurrence} uses {index} after {previous}"),
        }
    }
}
impl Error for SourceFormatInputError {}

/// Retain printer %w occurrences separately from typed error operands. None
/// denotes a non-error operand, including nil interface data. A typed nil
/// error receiver remains a concrete error operand in its interface adapter.
pub struct SourceFormat {
    rendered: String,
    operands: Vec<Option<BoxError>>,
    wrapped_positions: Vec<usize>,
    reordered: bool,
}
impl SourceFormat {
    /// Scan the original format bytes for wrapping/argument control. Rendered
    /// output is still supplied by the caller; this does not render Go values
    /// or apply Agones' prefix to the format before parsing it.
    pub fn scanned(rendered: impl Into<String>, format: &[u8], operands: Vec<Option<BoxError>>) -> Self {
        let (wrapped_positions, reordered) = crate::utility_format_scan::SourcePrintfPlan::scan(format, operands.len()).into_wrapping();
        Self { rendered: rendered.into(), operands, wrapped_positions, reordered }
    }
    pub fn indexed(rendered: impl Into<String>, operands: Vec<Option<BoxError>>, wrapped_positions: Vec<usize>, reordered: bool)
        -> Result<Self, SourceFormatInputError>
    {
        for (occurrence, &index) in wrapped_positions.iter().enumerate() {
            if index >= operands.len() { return Err(SourceFormatInputError::ArgumentIndex { occurrence, index, arguments: operands.len() }); }
            if !reordered && occurrence != 0 && wrapped_positions[occurrence - 1] >= index {
                return Err(SourceFormatInputError::ImplicitOrder { occurrence, previous: wrapped_positions[occurrence - 1], index });
            }
        }
        Ok(Self { rendered: rendered.into(), operands, wrapped_positions, reordered })
    }
    /// Explicit compatibility projection for already rendered unique operands
    /// in argument order. Indexed/repeated/invalid operands use indexed above.
    pub fn from_rendered_errors(rendered: impl Into<String>, wrapped: Vec<BoxError>) -> Self {
        let wrapped_positions = (0..wrapped.len()).collect();
        Self { rendered: rendered.into(), operands: wrapped.into_iter().map(Some).collect(), wrapped_positions, reordered: false }
    }
    pub fn rendered(&self) -> &str { &self.rendered }
    pub(crate) fn into_error(self, message: String) -> BoxError {
        let Self { mut operands, mut wrapped_positions, reordered, .. } = self;
        match wrapped_positions.len() {
            0 => Box::new(MessageError(message)),
            1 => Box::new(WrappedError { message, source: operands[wrapped_positions[0]].take() }),
            _ => {
                if reordered { wrapped_positions.sort_unstable(); }
                let mut errors: Option<Vec<BoxError>> = None;
                let mut previous = None;
                for index in wrapped_positions {
                    if previous == Some(index) { continue; }
                    previous = Some(index);
                    if let Some(error) = operands[index].take() { errors.get_or_insert_with(Vec::new).push(error); }
                }
                // The result remains wrapErrors after deduplication even if
                // only one or no valid error operand survives. None is nil.
                Box::new(FormattedError { message, wrapped: errors })
            }
        }
    }
}

#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "errors", source_struct = "errorString", source_log_type = "*errors.errorString")]
pub(crate) struct MessageError(pub(crate) String);
impl fmt::Display for MessageError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&self.0) }
}
impl Error for MessageError {}

#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "fmt", source_struct = "wrapError", source_log_type = "*fmt.wrapError")]
pub(crate) struct WrappedError { message: String, source: Option<BoxError> }
impl WrappedError {
    pub(crate) fn new(prefix: String, source: BoxError) -> Self {
        let message = format!("{prefix}: {source}");
        Self { message, source: Some(source) }
    }
}
impl fmt::Display for WrappedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&self.message) }
}
impl Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { self.source.as_ref().map(|source| source.as_ref() as &(dyn Error + 'static)) }
}

#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "fmt", source_struct = "wrapErrors", source_log_type = "*fmt.wrapErrors")]
pub(crate) struct FormattedError { message: String, wrapped: Option<Vec<BoxError>> }
impl FormattedError {
    pub(crate) fn wrapped(&self) -> Option<&[BoxError]> { self.wrapped.as_deref() }
}
impl fmt::Display for FormattedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&self.message) }
}
// Unwrap() []error must not become Rust's single source() even when len==1.
impl Error for FormattedError {}

/// Outer None means this value has no multiple-error method. Some(None) is
/// the source nil []error; Some(Some(...)) retains its current elements.
/// Mutable source slice/interface aliasing still requires its native bridge.
pub fn source_wrapped_errors(error: &(dyn Error + 'static)) -> Option<Option<&[BoxError]>> {
    error.downcast_ref::<FormattedError>().map(FormattedError::wrapped)
}
