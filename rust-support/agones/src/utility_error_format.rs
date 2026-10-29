//! Typed printer-state input and canonical source fmt/errors result objects.
// Adapted from Go fmt/errors.go. Copyright The Go Authors. BSD-3-Clause;
// see ../GO-LICENSE. Raw scanning is in utility_format_scan and concrete
// scalar rendering in utility_format_text/utility_format_integer; full Go
// value and formatting-interface support remains required.
use std::{borrow::Cow, error::Error, fmt};
use derive_deftly::Deftly;
use crate::{errors::BoxError, utility_declarations::derive_deftly_template_SourceErrorOrigin};

/// Go strings retain arbitrary bytes. Display is the native Rust UTF8 view;
/// source-aware consumers use as_bytes without replacing invalid contents.
#[derive(Debug)]
pub(crate) enum SourceErrorMessage { Text(String), Bytes(Vec<u8>) }
impl SourceErrorMessage {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        match self { Self::Text(text) => text.as_bytes(), Self::Bytes(bytes) => bytes }
    }
    fn text(&self) -> Cow<'_, str> {
        match self { Self::Text(text) => Cow::Borrowed(text), Self::Bytes(bytes) => String::from_utf8_lossy(bytes) }
    }
}
impl From<String> for SourceErrorMessage { fn from(text: String) -> Self { Self::Text(text) } }
impl fmt::Display for SourceErrorMessage {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&self.text()) }
}

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
    rendered: SourceErrorMessage,
    operands: Vec<Option<BoxError>>,
    wrapped_positions: Vec<usize>,
    reordered: bool,
    origin_prefixed: bool,
}
impl SourceFormat {
    /// Concrete byte renderer for Go string arguments. Other argument kinds
    /// and source formatting interfaces remain separate required paths.
    pub fn text(format: &[u8], arguments: &[&[u8]], profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<Self, crate::utility_http::SourceSliceAllocationError>
    {
        let output = crate::utility_format_text::try_source_text_printf(format, arguments, profile)?;
        Ok(Self::from_scalar_output(output, arguments.len()))
    }
    pub fn integers<Value: crate::utility_format_integer::SourcePrintfIntegerValue>(format: &[u8], arguments: &[Value],
        profile: crate::utility_http::SourceUnicodeProfile) -> Result<Self, crate::utility_http::SourceSliceAllocationError>
    {
        let output = crate::utility_format_integer::try_source_integer_printf(format, arguments, profile)?;
        Ok(Self::from_scalar_output(output, arguments.len()))
    }
    pub fn booleans(format: &[u8], arguments: &[bool], profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<Self, crate::utility_http::SourceSliceAllocationError>
    {
        let output = crate::utility_format_bool::try_source_bool_printf(format, arguments, profile)?;
        Ok(Self::from_scalar_output(output, arguments.len()))
    }
    /// Render borrowed tuple fields, then move their original errors into
    /// argument-indexed ownership. No error payload is cloned or reboxed.
    pub fn arguments<Arguments: crate::utility_format_arguments::SourcePrintfOwnedArguments>(format: &[u8],
        arguments: Arguments, profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<Self, crate::utility_http::SourceSliceAllocationError>
    {
        let output = crate::utility_format_arguments::try_source_arguments_printf(format, &arguments, profile,
            crate::utility_format_print::SourcePrintfMode::Errorf)?;
        let (rendered, plan) = output.into_parts();
        let (wrapped_positions, reordered) = plan.into_wrapping();
        let operands = arguments.into_error_operands();
        Ok(Self { rendered: SourceErrorMessage::Bytes(rendered), operands, wrapped_positions, reordered, origin_prefixed: false })
    }
    fn from_scalar_output(output: crate::utility_format_print::SourcePrintfOutput, arguments: usize) -> Self {
        let (rendered, plan) = output.into_parts();
        let (wrapped_positions, reordered) = plan.into_wrapping();
        Self { rendered: SourceErrorMessage::Bytes(rendered),
            operands: (0..arguments).map(|_| None).collect(), wrapped_positions, reordered, origin_prefixed: false }
    }
    /// Scan the original format bytes for wrapping/argument control. Rendered
    /// output is still supplied by the caller; this does not render Go values
    /// or apply Agones' prefix to the format before parsing it.
    pub fn scanned(rendered: impl Into<String>, format: &[u8], operands: Vec<Option<BoxError>>) -> Self {
        let (wrapped_positions, reordered) = crate::utility_format_scan::SourcePrintfPlan::scan(format, operands.len()).into_wrapping();
        Self { rendered: SourceErrorMessage::Text(rendered.into()), operands, wrapped_positions, reordered, origin_prefixed: false }
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
        Ok(Self { rendered: SourceErrorMessage::Text(rendered.into()), operands, wrapped_positions, reordered, origin_prefixed: false })
    }
    /// Explicit compatibility projection for already rendered unique operands
    /// in argument order. Indexed/repeated/invalid operands use indexed above.
    pub fn from_rendered_errors(rendered: impl Into<String>, wrapped: Vec<BoxError>) -> Self {
        let wrapped_positions = (0..wrapped.len()).collect();
        Self { rendered: SourceErrorMessage::Text(rendered.into()), operands: wrapped.into_iter().map(Some).collect(), wrapped_positions, reordered: false, origin_prefixed: false }
    }
    pub fn rendered(&self) -> Cow<'_, str> { self.rendered.text() }
    pub fn rendered_bytes(&self) -> &[u8] { self.rendered.as_bytes() }
    pub(crate) fn into_error(mut self, origin: &crate::errors::Errors) -> BoxError {
        if !self.origin_prefixed {
            let mut prefix = origin.prefix("").into_bytes();
            if !prefix.is_empty() {
                prefix.extend_from_slice(self.rendered.as_bytes());
                self.rendered = SourceErrorMessage::Bytes(prefix);
            }
        }
        self.into_rendered_error()
    }
    pub(crate) fn into_rendered_error(self) -> BoxError {
        let Self { rendered: message, mut operands, mut wrapped_positions, reordered, .. } = self;
        match wrapped_positions.len() {
            0 => Box::new(MessageError(message)),
            1 => Box::new(WrappedError { message, source: operands[wrapped_positions[0]].take()
                .map(crate::utility_error_slice::SourceRetainedError::from_original) }),
            _ => {
                if reordered { wrapped_positions.sort_unstable(); }
                let mut errors = crate::utility_error_slice::SourceErrorSlice::nil();
                let mut previous = None;
                for index in wrapped_positions {
                    if previous == Some(index) { continue; }
                    previous = Some(index);
                    if let Some(error) = operands[index].take() {
                        errors.append(Some(crate::utility_error_slice::SourceRetainedError::from_original(error)));
                    }
                }
                // The result remains wrapErrors after deduplication even if
                // only one or no valid operand survives. The empty header
                // remains nil until the first valid error is appended.
                Box::new(FormattedError { message, wrapped: errors })
            }
        }
    }
}

#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "errors", source_struct = "errorString", source_log_type = "*errors.errorString")]
pub(crate) struct MessageError(pub(crate) SourceErrorMessage);
impl fmt::Display for MessageError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.0, out) }
}
impl Error for MessageError {}

#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "fmt", source_struct = "wrapError", source_log_type = "*fmt.wrapError")]
pub(crate) struct WrappedError { message: SourceErrorMessage, source: Option<crate::utility_error_slice::SourceRetainedError> }
impl WrappedError {
    pub(crate) fn new(prefix: String, source: BoxError) -> Self {
        let mut message = prefix.into_bytes();
        message.extend_from_slice(b": ");
        if let Some(bytes) = source_error_message(source.as_ref()) { message.extend_from_slice(&bytes); }
        else { message.extend_from_slice(source.to_string().as_bytes()); }
        let message = SourceErrorMessage::Bytes(message);
        Self { message, source: Some(crate::utility_error_slice::SourceRetainedError::from_original(source)) }
    }
    pub(crate) fn unwrapped(&self) -> Option<crate::utility_error_slice::SourceRetainedError> {
        self.source.as_ref().map(crate::utility_format_copy::SourceFmtValueCopy::source_fmt_copy)
    }
}
impl fmt::Display for WrappedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.message, out) }
}
impl Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { self.source.as_ref().map(|source| source as &(dyn Error + 'static)) }
}

#[derive(Debug, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "fmt", source_struct = "wrapErrors", source_log_type = "*fmt.wrapErrors")]
pub(crate) struct FormattedError { message: SourceErrorMessage, wrapped: crate::utility_error_slice::SourceErrorSlice }
impl FormattedError {
    pub(crate) fn wrapped(&self) -> crate::utility_error_slice::SourceErrorSlice {
        crate::utility_format_copy::SourceFmtValueCopy::source_fmt_copy(&self.wrapped)
    }
}
impl fmt::Display for FormattedError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.message, out) }
}
// Unwrap() []error must not become Rust's single source() even when len==1.
impl Error for FormattedError {}

/// None means no multiple-error method. A returned header preserves nil,
/// length/capacity and live mutable backing without copying its elements.
pub fn source_wrapped_errors(error: &(dyn Error + 'static)) -> Option<crate::utility_error_slice::SourceErrorSlice> {
    let error = crate::utility_error_alias::source_error_original(error);
    error.downcast_ref::<FormattedError>().map(FormattedError::wrapped)
}

/// The outer Option distinguishes no single Unwrap method from Unwrap()==nil.
pub fn source_unwrapped_error(error: &(dyn Error + 'static)) -> Option<Option<crate::utility_error_slice::SourceRetainedError>> {
    let error = crate::utility_error_alias::source_error_original(error);
    error.downcast_ref::<WrappedError>().map(WrappedError::unwrapped)
}

/// The canonical objects expose their source byte messages without calling
/// user Error/Display methods. Other error kinds require their own adapter.
pub fn source_error_bytes(error: &(dyn Error + 'static)) -> Option<&[u8]> {
    let error = crate::utility_error_alias::source_error_original(error);
    if let Some(error) = error.downcast_ref::<MessageError>() { return Some(error.0.as_bytes()); }
    if let Some(error) = error.downcast_ref::<WrappedError>() { return Some(error.message.as_bytes()); }
    error.downcast_ref::<FormattedError>().map(|error| error.message.as_bytes())
}

/// A copied source error can supply its declared Error method without a
/// native Display fallback. Canonical immutable messages remain borrowed.
pub fn source_error_message(error: &(dyn Error + 'static)) -> Option<Cow<'_, [u8]>> {
    if let Some(error) = error.downcast_ref::<crate::utility_error_slice::SourceRetainedError>() {
        return source_error_message(error.original());
    }
    if let Some(error) = error.downcast_ref::<crate::utility_error_alias::SourceSharedError>() {
        return Some(crate::utility_format_methods::SourceErrorMethod::source_error_message(error));
    }
    source_error_bytes(error).map(Cow::Borrowed)
}

/// Borrowed fields for the three canonical source structs, for their concrete
/// reflection formatter. This does not add a general Go value hierarchy.
pub(crate) enum CanonicalErrorFields<'a> {
    Message(&'a [u8]),
    Wrapped { message: &'a [u8], source: Option<&'a (dyn Error + 'static)> },
    Multiple { message: &'a [u8], errors: crate::utility_error_slice::SourceErrorSlice },
}
pub(crate) fn canonical_error_fields(error: &(dyn Error + 'static)) -> Option<CanonicalErrorFields<'_>> {
    let error = crate::utility_error_alias::source_error_original(error);
    if let Some(error) = error.downcast_ref::<MessageError>() { return Some(CanonicalErrorFields::Message(error.0.as_bytes())); }
    if let Some(error) = error.downcast_ref::<WrappedError>() {
        return Some(CanonicalErrorFields::Wrapped { message: error.message.as_bytes(), source: error.source() });
    }
    error.downcast_ref::<FormattedError>().map(|error| CanonicalErrorFields::Multiple {
        message: error.message.as_bytes(), errors: error.wrapped() })
}

impl crate::errors::Errors {
    pub fn new_bytes(&self, message: &[u8]) -> Result<BoxError, crate::utility_http::SourceSliceAllocationError> {
        let mut prefixed = self.prefix("").into_bytes();
        crate::utility_go_quote::append(&mut prefixed, message)?;
        Ok(Box::new(MessageError(SourceErrorMessage::Bytes(prefixed))))
    }

    /// Rust-only typed string bridge: the source Errorf prefix is part of the
    /// format before scanning, so percent bytes in the prefix also participate.
    /// It does not invoke a runtime value/provider registry or a second owner.
    pub fn errorf_text(&self, format: &[u8], arguments: &[&[u8]], profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<BoxError, crate::utility_http::SourceSliceAllocationError>
    {
        let mut prefixed = self.prefix("").into_bytes();
        crate::utility_go_quote::append(&mut prefixed, format)?;
        let mut result = SourceFormat::text(&prefixed, arguments, profile)?;
        result.origin_prefixed = true;
        Ok(self.Errorf(result))
    }

    pub fn errorf_integers<Value: crate::utility_format_integer::SourcePrintfIntegerValue>(&self, format: &[u8],
        arguments: &[Value], profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<BoxError, crate::utility_http::SourceSliceAllocationError>
    {
        let mut prefixed = self.prefix("").into_bytes();
        crate::utility_go_quote::append(&mut prefixed, format)?;
        let mut result = SourceFormat::integers(&prefixed, arguments, profile)?;
        result.origin_prefixed = true;
        Ok(self.Errorf(result))
    }

    pub fn errorf_booleans(&self, format: &[u8], arguments: &[bool], profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<BoxError, crate::utility_http::SourceSliceAllocationError>
    {
        let mut prefixed = self.prefix("").into_bytes();
        crate::utility_go_quote::append(&mut prefixed, format)?;
        let mut result = SourceFormat::booleans(&prefixed, arguments, profile)?;
        result.origin_prefixed = true;
        Ok(self.Errorf(result))
    }

    pub fn errorf_arguments<Arguments: crate::utility_format_arguments::SourcePrintfOwnedArguments>(&self, format: &[u8],
        arguments: Arguments, profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<BoxError, crate::utility_http::SourceSliceAllocationError>
    {
        let mut prefixed = self.prefix("").into_bytes();
        crate::utility_go_quote::append(&mut prefixed, format)?;
        let mut result = SourceFormat::arguments(&prefixed, arguments, profile)?;
        result.origin_prefixed = true;
        Ok(self.Errorf(result))
    }

    /// Source Wrapf returns on nil before parsing or rendering its format.
    /// Sprintf's result receives the origin prefix as data, then the separate
    /// final %s: %w factory captures that message and the original error.
    pub fn wrapf_text(&self, error: Option<BoxError>, format: &[u8], arguments: &[&[u8]],
        profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<Option<BoxError>, crate::utility_http::SourceSliceAllocationError>
    {
        let Some(error) = error else { return Ok(None); };
        let formatted = crate::utility_format_text::try_source_text_printf(format, arguments, profile)?;
        self.wrap_source_formatted(error, formatted.bytes()).map(Some)
    }

    pub fn wrapf_integers<Value: crate::utility_format_integer::SourcePrintfIntegerValue>(&self, error: Option<BoxError>,
        format: &[u8], arguments: &[Value], profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<Option<BoxError>, crate::utility_http::SourceSliceAllocationError>
    {
        let Some(error) = error else { return Ok(None); };
        let formatted = crate::utility_format_integer::try_source_integer_printf(format, arguments, profile)?;
        self.wrap_source_formatted(error, formatted.bytes()).map(Some)
    }

    pub fn wrapf_booleans(&self, error: Option<BoxError>, format: &[u8], arguments: &[bool],
        profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<Option<BoxError>, crate::utility_http::SourceSliceAllocationError>
    {
        let Some(error) = error else { return Ok(None); };
        let formatted = crate::utility_format_bool::try_source_bool_printf(format, arguments, profile)?;
        self.wrap_source_formatted(error, formatted.bytes()).map(Some)
    }

    pub fn wrapf_arguments<Arguments: crate::utility_format_print::SourcePrintfArguments>(&self, error: Option<BoxError>,
        format: &[u8], arguments: Arguments, profile: crate::utility_http::SourceUnicodeProfile)
        -> Result<Option<BoxError>, crate::utility_http::SourceSliceAllocationError>
    {
        let Some(error) = error else { return Ok(None); };
        let formatted = crate::utility_format_arguments::try_source_arguments_printf(format, &arguments, profile,
            crate::utility_format_print::SourcePrintfMode::Sprintf)?;
        self.wrap_source_formatted(error, formatted.bytes()).map(Some)
    }

    fn wrap_source_formatted(&self, error: BoxError, formatted: &[u8])
        -> Result<BoxError, crate::utility_http::SourceSliceAllocationError>
    {
        let mut message = self.prefix("").into_bytes();
        crate::utility_go_quote::append(&mut message, formatted)?;
        crate::utility_go_quote::append(&mut message, b": ")?;
        if let Some(bytes) = source_error_message(error.as_ref()) { crate::utility_go_quote::append(&mut message, &bytes)?; }
        else { crate::utility_go_quote::append(&mut message, error.to_string().as_bytes())?; }
        // The final format has exactly one w, independently of any w that
        // Sprintf encountered while formatting the annotation's scalar values.
        let result = SourceFormat { rendered: SourceErrorMessage::Bytes(message),
            operands: vec![Some(error)], wrapped_positions: vec![0], reordered: false, origin_prefixed: true };
        Ok(self.Errorf(result))
    }
}
