//! klog value-method ordering and recovery over retained argument data.
// Adapted from klog/v2/internal/serialize/keyvalues{,_slog}.go, Kubernetes
// Authors, Apache-2.0, and Go encoding/json/encode.go, Go Authors, BSD-3-Clause
// (../GO-LICENSE). Full time/native/ABI and arbitrary JSON type bindings and
// native buffer/heap/ABI profiles remain required. No registry or lifecycle.
use std::{fmt, panic::{catch_unwind, AssertUnwindSafe}, sync::Arc};
use derive_deftly::Deftly;
use crate::{utility_error_handlers::{SourceErrorArgument, SourceErrorArguments},
    utility_error_slice::SourceRetainedError,
    utility_format_copy::{SourceFmtSharedString, SourceFmtValueCopy},
    utility_format_arguments::SourcePrintfValue,
    utility_format_collections::SourcePrintfSlice,
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_methods::write_recovered_value,
    utility_float_reference::{try_source_float, SourceFloatWidth},
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
    utility_go_quote::append,
    utility_klog_values::{string_bytes, write_string_value},
    utility_panic::SourceRecoveredPanic,
    utility_declarations::derive_deftly_template_SourceKlogAdapter};

pub trait SourceKlogTextWriter {
    fn source_write_text(&self, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>;
}
pub trait SourceKlogMarshaler { fn source_marshal_log(&self) -> SourceErrorArgument; }
/// A concrete type's Encoder.Encode operation, including the final LF.
/// Source errors carry their formatting methods. Allocation/binding failures
/// are separate from returned Go JSON errors and never become error strings.
pub trait SourceKlogJsonEncoder {
    fn source_encode_json(&self, output: &mut Vec<u8>, profile: SourceUnicodeProfile)
        -> Result<(), SourceKlogEncodeError>;
}
pub enum SourceKlogEncodeError {
    Source(SourceRetainedError),
    Allocation(SourceSliceAllocationError),
    Unbound,
}
impl From<SourceSliceAllocationError> for SourceKlogEncodeError {
    fn from(value: SourceSliceAllocationError) -> Self { Self::Allocation(value) }
}
#[derive(Default)]
pub struct SourceKlogInterfaces<'a> {
    pub text_writer: Option<&'a dyn SourceKlogTextWriter>,
    pub marshaler: Option<&'a dyn SourceKlogMarshaler>,
    pub json_encoder: Option<&'a dyn SourceKlogJsonEncoder>,
    pub slog_value: Option<&'a crate::utility_slog_values::SourceSlogValue>,
    pub log_valuer: Option<&'a dyn crate::utility_slog_values::SourceSlogLogValuer>,
}

#[derive(Clone, Debug)]
pub enum SourceKlogFormatError {
    Allocation(SourceSliceAllocationError),
    Unbound { source_type: Option<String>, operation: &'static str },
}
impl From<SourceSliceAllocationError> for SourceKlogFormatError {
    fn from(value: SourceSliceAllocationError) -> Self { Self::Allocation(value) }
}
impl fmt::Display for SourceKlogFormatError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::Allocation(error) => fmt::Display::fmt(error, output),
            Self::Unbound { source_type, operation } => write!(output, "unbound source {operation} for {source_type:?}") }
    }
}
impl std::error::Error for SourceKlogFormatError {}
pub type FormatResult<Value = ()> = Result<Value, SourceKlogFormatError>;
pub type SourceKlogAnyToStringHook = Arc<dyn Fn(&SourceErrorArgument) -> Vec<u8> + Send + Sync>;

pub struct SourceKlogFormatter {
    pub profile: SourceUnicodeProfile,
    pub any_to_string_hook: Option<SourceKlogAnyToStringHook>,
    pub slog_stack: Option<crate::utility_slog_values::SourceSlogStack>,
}
impl SourceKlogFormatter {
    pub fn new(profile: SourceUnicodeProfile) -> Self { Self { profile, any_to_string_hook: None, slog_stack: None } }
    pub fn format_kvs(&self, output: &mut Vec<u8>, groups: &[SourceErrorArguments]) -> FormatResult {
        crate::utility_klog_values::format_kvs(output, groups, |output, key, value| self.kv_format(output, key, value))
    }
    fn unbound(&self, value: &SourceErrorArgument, operation: &'static str) -> SourceKlogFormatError {
        SourceKlogFormatError::Unbound { source_type: value.source_type().map(str::to_owned), operation }
    }
    fn printf(&self, output: &mut Vec<u8>, value: &SourceErrorArgument, verb: char,
        state: &SourcePrintfResolvedCounts) -> FormatResult
    {
        match catch_unwind(AssertUnwindSafe(|| value.try_write(verb, state, self.profile, output))) {
            Ok(result) => { result.ok_or_else(|| self.unbound(value, "fmt operand"))??; Ok(()) }
            Err(payload) => match payload.downcast::<SourceKlogFormatError>() {
                Ok(error) => Err(*error), Err(payload) => std::panic::resume_unwind(payload),
            }
        }
    }
    fn panic_string(&self, panic: SourceRecoveredPanic) -> FormatResult<Vec<u8>> {
        if let Some(error) = panic.payload().downcast_ref::<SourceKlogFormatError>() { return Err(error.clone()); }
        let mut output = Vec::new(); append(&mut output, b"<panic: ")?;
        write_recovered_value(&panic, 's', self.profile, &mut output).ok_or_else(|| SourceKlogFormatError::Unbound {
            source_type: panic.source_type().map(str::to_owned), operation: "recovered panic fmt %s" })??;
        append(&mut output, b">")?; Ok(output)
    }
    fn recovered_string(&self, call: impl FnOnce() -> Vec<u8>) -> FormatResult<Vec<u8>> {
        match catch_unwind(AssertUnwindSafe(call)) {
            Ok(value) => Ok(value), Err(payload) => self.panic_string(SourceRecoveredPanic::new(payload)),
        }
    }
    fn marshal_value(&self, marshaler: &dyn SourceKlogMarshaler) -> FormatResult<SourceErrorArgument> {
        match catch_unwind(AssertUnwindSafe(|| marshaler.source_marshal_log())) {
            Ok(value) => Ok(value),
            Err(payload) => Ok(SourceErrorArgument::from_value(SourceFmtSharedString(Arc::from(
                self.panic_string(SourceRecoveredPanic::new(payload))?)))),
        }
    }
    /// Source Go1.21+ order: the concrete slog.Value precedes Stringer, and
    /// raw LogValuer follows Marshaler. These use actual method/type views.
    pub fn kv_format(&self, output: &mut Vec<u8>, key: &SourceErrorArgument, value: &SourceErrorArgument)
        -> FormatResult<Vec<u8>>
    {
        append(output, b" ")?;
        let mut rendered_key = Vec::new();
        if let Some(key) = string_bytes(key) { append(&mut rendered_key, key)?; }
        else { self.printf(&mut rendered_key, key, 's', &SourcePrintfResolvedCounts::default())?; }
        append(output, &rendered_key)?;
        let methods = value.source_methods();
        if let Some(writer) = methods.klog.text_writer {
            append(output, b"=")?;
            match catch_unwind(AssertUnwindSafe(|| writer.source_write_text(output))) {
                Ok(result) => result?,
                Err(payload) => {
                    // The source keeps partial writer bytes. The panic marker
                    // is inserted inside literal quotes without Go quoting.
                    let marker = self.panic_string(SourceRecoveredPanic::new(payload))?;
                    append(output, b"\"")?; append(output, &marker)?; append(output, b"\"")?;
                }
            }
        } else if let Some(value) = methods.klog.slog_value {
            append(output, b"=")?; self.generate_slog_json(output, value)?;
        } else if let Some(stringer) = methods.stringer {
            let value = self.recovered_string(|| stringer.source_string().into_owned())?;
            write_string_value(output, &value, self.profile)?;
        } else if let Some(value) = string_bytes(value) {
            write_string_value(output, value, self.profile)?;
        } else if let Some(error) = methods.error {
            let value = self.recovered_string(|| error.source_error_message().into_owned())?;
            write_string_value(output, &value, self.profile)?;
        } else if let Some(marshaler) = methods.klog.marshaler {
            let value = self.marshal_value(marshaler)?;
            if let Some(value) = string_bytes(&value) { write_string_value(output, value, self.profile)?; }
            else { self.format_any(output, &value)?; } // no second MarshalLog
        } else if methods.klog.log_valuer.is_some() {
            let resolved = crate::utility_slog_values::SourceSlogValue::any_value(value.source_fmt_copy()).resolve(self)?;
            if let Some(bytes) = resolved.string_bytes() { write_string_value(output, bytes, self.profile)?; }
            else { append(output, b"=")?; self.generate_slog_json(output, &resolved)?; }
        } else if value.downcast_ref::<SourcePrintfSlice<'static, u8>>()
            .is_some_and(|bytes| bytes.source_type == "[]uint8") {
            append(output, b"=")?;
            let mut state = SourcePrintfResolvedCounts::default(); state.flags.plus = true;
            // This is the concrete borrowed []byte declaration, not every
            // named byte-slice type or a source mutable backing/GC binding.
            self.printf(output, value, 'q', &state)?;
        } else { self.format_any(output, value)?; }
        Ok(rendered_key)
    }
    fn format_any(&self, output: &mut Vec<u8>, value: &SourceErrorArgument) -> FormatResult {
        if let Some(hook) = &self.any_to_string_hook {
            let rendered = hook(value); // source hook panics propagate
            if rendered.contains(&b'\n') { write_string_value(output, &rendered, self.profile)?; }
            else { append(output, b"=")?; append(output, &rendered)?; }
        } else { append(output, b"=")?; self.format_as_json(output, value)?; }
        Ok(())
    }
    /// JSON generation differs from KV dispatch: Marshaler recurses, the
    /// hook is excluded, and Error is invoked directly without recovery.
    pub fn generate_json(&self, output: &mut Vec<u8>, value: &SourceErrorArgument) -> FormatResult {
        let methods = value.source_methods();
        if let Some(value) = methods.klog.slog_value { self.generate_slog_json(output, value) }
        else if let Some(stringer) = methods.stringer {
            let value = self.recovered_string(|| stringer.source_string().into_owned())?;
            self.quote(output, &value)
        } else if let Some(marshaler) = methods.klog.marshaler {
            self.generate_json(output, &self.marshal_value(marshaler)?)
        } else if methods.klog.log_valuer.is_some() {
            // This source branch peels Any after Resolve. In particular a
            // group becomes []Attr here, whereas the concrete Value branch
            // above serializes its attributes as a JSON object.
            let resolved = crate::utility_slog_values::SourceSlogValue::any_value(value.source_fmt_copy()).resolve(self)?;
            self.generate_json(output, &resolved.any())
        } else if let Some(value) = string_bytes(value) { self.quote(output, value) }
        else if let Some(error) = methods.error { self.quote(output, &error.source_error_message()) }
        else { self.format_as_json(output, value) }
    }
    fn generate_slog_json(&self, output: &mut Vec<u8>, value: &crate::utility_slog_values::SourceSlogValue) -> FormatResult {
        if let Some(group) = value.group_ref() {
            append(output, b"{")?;
            for index in 0..group.len() {
                let attr = group.get(index);
                if index != 0 { append(output, b",")?; }
                self.quote(output, &attr.key.0)?; append(output, b":")?;
                self.generate_slog_json(output, &attr.value)?;
            }
            append(output, b"}")?; Ok(())
        } else if value.kind() == crate::utility_slog_values::SourceSlogKind::LOG_VALUER {
            self.generate_slog_json(output, &value.resolve(self)?)
        } else { self.generate_json(output, &value.any()) }
    }
    fn quote(&self, output: &mut Vec<u8>, value: &[u8]) -> FormatResult {
        append(output, crate::utility_go_quote::try_source_go_quote(value, self.profile)?.as_bytes())?; Ok(())
    }
    fn internal_error(&self, output: &mut Vec<u8>, error: &SourceRetainedError) -> FormatResult {
        if crate::utility_error_view::source_error_type(error).is_none() {
            return Err(SourceKlogFormatError::Unbound { source_type: None, operation: "JSON error fmt operand" });
        }
        append(output, b"\"<internal error: ")?;
        crate::utility_format_arguments::write_source_error_operand(error, 'v', self.profile,
            crate::utility_format_print::SourcePrintfMode::Sprintf, output)?;
        append(output, b">\"")?; Ok(())
    }
    fn format_as_json(&self, output: &mut Vec<u8>, value: &SourceErrorArgument) -> FormatResult {
        let start = output.len();
        if value.source_type().is_none() && !value.is_nil() { return Err(self.unbound(value, "JSON encoder")); }
        if let Some(encoder) = value.source_methods().klog.json_encoder {
            match encoder.source_encode_json(output, self.profile) {
                Ok(()) if output.len() > start && output.last() == Some(&b'\n') => {
                    output.truncate(output.len() - 1); Ok(())
                }
                Ok(()) => Err(self.unbound(value, "JSON Encoder.Encode success/final LF binding")),
                Err(SourceKlogEncodeError::Source(error)) => { output.truncate(start); self.internal_error(output, &error) }
                Err(SourceKlogEncodeError::Allocation(error)) => Err(error.into()),
                Err(SourceKlogEncodeError::Unbound) => Err(self.unbound(value, "JSON encoder")),
            }
        } else if value.is_nil() { append(output, b"null")?; Ok(()) }
        else if let Some(string) = string_bytes(value) {
            // Encoder uses JSON escaping, including HTML and invalid UTF8;
            // generateJSON's string branch above uses strconv.Quote instead.
            crate::utility_http_headers::append_json_string(output, string)?; Ok(())
        } else if let Some(boolean) = value.downcast_ref::<bool>() {
            append(output, if *boolean { b"true" } else { b"false" })?; Ok(())
        } else if integer_argument(value) {
            self.printf(output, value, 'd', &SourcePrintfResolvedCounts::default())
        } else if let Some(float) = value.downcast_ref::<f64>() { self.float_json(output, *float, SourceFloatWidth::Binary64) }
        else if let Some(float) = value.downcast_ref::<f32>() { self.float_json(output, f64::from(*float), SourceFloatWidth::Binary32) }
        else { Err(self.unbound(value, "JSON encoder")) }
    }
    fn float_json(&self, output: &mut Vec<u8>, value: f64, width: SourceFloatWidth) -> FormatResult {
        if !value.is_finite() {
            // The source's UnsupportedValueError.Error has no custom user
            // method: json: unsupported value: + its FormatFloat string.
            append(output, b"\"<internal error: json: unsupported value: ")?;
            append(output, &try_source_float(value, width, b'g', -1)?)?;
            append(output, b">\"")?; return Ok(());
        }
        let abs = value.abs();
        let exponent = if width == SourceFloatWidth::Binary32 {
            let abs = abs as f32; abs != 0.0 && (abs < 1e-6_f32 || abs >= 1e21_f32)
        } else { abs != 0.0 && (abs < 1e-6 || abs >= 1e21) };
        let mut bytes = try_source_float(value, width, if exponent { b'e' } else { b'f' }, -1)?;
        let len = bytes.len();
        if exponent && len >= 4 && &bytes[len - 4..len - 1] == b"e-0" {
            bytes[len - 2] = bytes[len - 1]; bytes.truncate(len - 1);
        }
        append(output, &bytes)?; Ok(())
    }
}
fn integer_argument(value: &SourceErrorArgument) -> bool {
    macro_rules! kinds { ($($ty:ty),+) => { false $(|| value.downcast_ref::<$ty>().is_some())+ }; }
    kinds!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize)
}

/// Method closures belong to this actual argument, retaining its original
/// typed value. derive-deftly forwards the existing formatting/value views.
#[derive(Deftly)]
#[derive_deftly(SourceKlogAdapter)]
#[deftly(source_klog_text_writer)]
pub struct SourceKlogTextWriterValue<Value, Function>
where Value: SourcePrintfValue, Function: Fn(&Value, &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>,
{ pub value: Value, pub method: Function }
impl<Value, Function> SourceKlogTextWriter for SourceKlogTextWriterValue<Value, Function>
where Value: SourcePrintfValue, Function: Fn(&Value, &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>,
{
    fn source_write_text(&self, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError> { (self.method)(&self.value, output) }
}
#[derive(Deftly)]
#[derive_deftly(SourceKlogAdapter)]
#[deftly(source_klog_marshaler)]
pub struct SourceKlogMarshalerValue<Value, Function>
where Value: SourcePrintfValue, Function: Fn(&Value) -> SourceErrorArgument,
{ pub value: Value, pub method: Function }
impl<Value, Function> SourceKlogMarshaler for SourceKlogMarshalerValue<Value, Function>
where Value: SourcePrintfValue, Function: Fn(&Value) -> SourceErrorArgument,
{
    fn source_marshal_log(&self) -> SourceErrorArgument { (self.method)(&self.value) }
}
#[derive(Deftly)]
#[derive_deftly(SourceKlogAdapter)]
#[deftly(source_klog_json_encoder)]
pub struct SourceKlogJsonEncoderValue<Value, Function>
where Value: SourcePrintfValue, Function: Fn(&Value, &mut Vec<u8>, SourceUnicodeProfile) -> Result<(), SourceKlogEncodeError>,
{ pub value: Value, pub method: Function }
impl<Value, Function> SourceKlogJsonEncoder for SourceKlogJsonEncoderValue<Value, Function>
where Value: SourcePrintfValue, Function: Fn(&Value, &mut Vec<u8>, SourceUnicodeProfile) -> Result<(), SourceKlogEncodeError>,
{
    fn source_encode_json(&self, output: &mut Vec<u8>, profile: SourceUnicodeProfile) -> Result<(), SourceKlogEncodeError>
    { (self.method)(&self.value, output, profile) }
}
