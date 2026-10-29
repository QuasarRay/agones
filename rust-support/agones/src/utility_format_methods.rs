//! Source formatting interfaces, borrowed state and method panic recovery.
// Adapted from Go fmt/print.go. Copyright The Go Authors. BSD-3-Clause;
// see ../GO-LICENSE. Interface references belong to each actual value, with
// no global method/provider registry or alternate lifecycle manager.
use std::{borrow::Cow, error::Error, panic::{catch_unwind, AssertUnwindSafe}};
use crate::{
    utility_format_arguments::SourcePrintfValue,
    utility_format_print::{SourcePrintfArguments, SourcePrintfMode, append_rune, write_plain, write_nil},
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_text::write_text,
    utility_error_format::source_error_message,
    utility_panic::SourceRecoveredPanic,
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

pub trait SourceFormatter {
    fn source_format(&self, state: &mut SourceFormatState<'_>, verb: char) -> Result<(), SourceSliceAllocationError>;
}
pub trait SourceGoStringer { fn source_go_string(&self) -> Cow<'_, [u8]>; }
pub trait SourceErrorMethod { fn source_error_message(&self) -> Cow<'_, [u8]>; }
pub trait SourceStringer { fn source_string(&self) -> Cow<'_, [u8]>; }

#[derive(Default)]
pub struct SourcePrintfInterfaces<'a> {
    pub formatter: Option<&'a dyn SourceFormatter>,
    pub go_stringer: Option<&'a dyn SourceGoStringer>,
    pub error: Option<&'a dyn SourceErrorMethod>,
    pub stringer: Option<&'a dyn SourceStringer>,
    pub error_methods: crate::utility_error_query::SourceErrorMethods<'a>,
    pub klog: crate::utility_klog_dispatch::SourceKlogInterfaces<'a>,
}
pub trait SourcePrintfMethodViews {
    fn source_methods(&self) -> SourcePrintfInterfaces<'_> { SourcePrintfInterfaces::default() }
}

#[derive(Clone, Copy, Debug)]
pub struct SourcePrintfRecursion {
    pub depth: usize,
    pub erroring: bool,
    pub panicking: bool,
    pub can_interface: bool,
}
impl Default for SourcePrintfRecursion {
    fn default() -> Self { Self { depth: 0, erroring: false, panicking: false, can_interface: true } }
}
impl SourcePrintfRecursion {
    pub fn child(self, interfaceable: bool) -> Self {
        Self { depth: self.depth + 1, can_interface: self.can_interface && interfaceable, ..self }
    }
}

pub struct SourceFormatState<'a> {
    output: &'a mut Vec<u8>,
    printer: SourcePrintfResolvedCounts,
    profile: SourceUnicodeProfile,
}
impl SourceFormatState<'_> {
    pub fn width(&self) -> (i64, bool) { (self.printer.width.unwrap_or(0), self.printer.width.is_some()) }
    pub fn precision(&self) -> (i64, bool) { (self.printer.precision.unwrap_or(0), self.printer.precision.is_some()) }
    pub fn flag(&self, flag: char) -> bool {
        let flags = self.printer.flags;
        match flag { '-' => flags.minus, '+' => flags.plus || flags.plus_v,
            '#' => flags.sharp || flags.sharp_v, ' ' => flags.space, '0' => flags.zero, _ => false }
    }
    pub fn flag_code(&self, flag: i64) -> bool {
        u32::try_from(flag).ok().and_then(char::from_u32).is_some_and(|flag| self.flag(flag))
    }
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, SourceSliceAllocationError> {
        append(self.output, bytes)?; Ok(bytes.len())
    }
    pub fn write_string(&mut self, bytes: &[u8]) -> Result<usize, SourceSliceAllocationError> { self.write(bytes) }
    /// Fprintf to this State creates a separate printer, then writes its bytes.
    pub fn printf<Arguments: SourcePrintfArguments>(&mut self, format: &[u8], arguments: &Arguments)
        -> Result<usize, SourceSliceAllocationError>
    {
        let result = crate::utility_format_arguments::try_source_arguments_printf(format, arguments, self.profile, SourcePrintfMode::Sprintf)?;
        self.write(result.bytes())
    }
}

pub fn write_source_value<Value: SourcePrintfValue>(value: &Value, verb: char,
    state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile, mode: SourcePrintfMode,
    recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    if value.source_is_nil() {
        if recursion.depth == 0 { return write_nil(output, verb, state); }
        if state.flags.sharp_v {
            append(output, value.source_nil_interface_type().as_bytes())?; return append(output, b"(nil)");
        }
        return append(output, b"<nil>");
    }
    // printArg handles p/T before any user methods. T normally exits in the
    // shared control loop, but remains correct for a direct panic-value view.
    if recursion.depth == 0 && verb == 'T' { return write_plain(output, value.source_type().as_bytes(), state); }
    if recursion.depth == 0 && verb == 'p' { return value.write_source(verb, state, profile, mode, recursion, output); }
    if recursion.erroring || !recursion.can_interface {
        return value.write_source(verb, state, profile, mode, recursion, output);
    }
    let methods = value.source_methods();
    if verb == 'w' && (methods.error.is_none() || mode != SourcePrintfMode::Errorf) {
        return bad_verb(value, verb, state, profile, mode, recursion, output);
    }
    // w becomes v only inside handleMethods. A reflection fallback still
    // receives the original verb from printArg, including sharp-w.
    let method_verb = if verb == 'w' { 'v' } else { verb };
    let nil_pointer = value.source_is_nil_pointer();
    if let Some(formatter) = methods.formatter {
        return call_method("Format", method_verb, nil_pointer, recursion.panicking, profile, output, |output| {
            let mut frame = SourceFormatState { output, printer: state.value_state(), profile };
            formatter.source_format(&mut frame, method_verb)
        });
    }
    if state.flags.sharp_v {
        if let Some(stringer) = methods.go_stringer {
            return call_method("GoString", method_verb, nil_pointer, recursion.panicking, profile, output, |output| {
                let text = stringer.source_go_string(); write_plain(output, &text, state)
            });
        }
    } else if matches!(method_verb, 'v' | 's' | 'x' | 'X' | 'q') {
        if let Some(error) = methods.error {
            return call_method("Error", method_verb, nil_pointer, recursion.panicking, profile, output, |output| {
                let text = error.source_error_message(); write_text(output, &text, method_verb, state, profile)
            });
        }
        if let Some(stringer) = methods.stringer {
            return call_method("String", method_verb, nil_pointer, recursion.panicking, profile, output, |output| {
                let text = stringer.source_string(); write_text(output, &text, method_verb, state, profile)
            });
        }
    }
    value.write_source(verb, state, profile, mode, recursion, output)
}

pub(crate) fn bad_verb<Value: SourcePrintfValue>(value: &Value, verb: char, state: &SourcePrintfResolvedCounts,
    profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
    output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    append(output, b"%!")?; append_rune(output, verb)?; append(output, b"(")?;
    append(output, value.source_type().as_bytes())?; append(output, b"=")?;
    // erroring=true suppresses all methods during the recursive v. The
    // concrete value's fallback is separate from method dispatch.
    value.write_source('v', state, profile, mode, SourcePrintfRecursion { depth: 0, erroring: true, ..recursion }, output)?;
    append(output, b")")
}

fn call_method(method: &str, verb: char, nil_pointer: bool, panicking: bool, profile: SourceUnicodeProfile,
    output: &mut Vec<u8>, call: impl FnOnce(&mut Vec<u8>) -> Result<(), SourceSliceAllocationError>)
    -> Result<(), SourceSliceAllocationError>
{
    match catch_unwind(AssertUnwindSafe(|| call(output))) {
        Ok(result) => result,
        Err(payload) => {
            // Existing partial writes remain. Nil receivers are checked before
            // the nested-panic branch, matching catchPanic's source ordering.
            if nil_pointer { return append(output, b"<nil>"); }
            if panicking { std::panic::resume_unwind(payload); }
            append(output, b"%!")?; append_rune(output, verb)?; append(output, b"(PANIC=")?;
            append(output, method.as_bytes())?; append(output, b" method: ")?;
            write_panic_value(SourceRecoveredPanic::new(payload), profile, output)?;
            append(output, b")")
        }
    }
}

trait PanicValue: Send {
    fn source_type(&self) -> &str;
    fn source_error(&self) -> Option<&(dyn Error + Send + Sync + 'static)>;
    fn write_panic_value(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        panicking: bool, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>;
}
struct TypedPanicValue<Value>(Value);
impl<Value: SourcePrintfValue + Send + 'static> PanicValue for TypedPanicValue<Value> {
    fn source_type(&self) -> &str { self.0.source_type() }
    fn source_error(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { self.0.source_error_view() }
    fn write_panic_value(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        panicking: bool, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError> {
        write_source_value(&self.0, verb, state, profile, SourcePrintfMode::Sprintf,
            SourcePrintfRecursion { panicking, ..SourcePrintfRecursion::default() }, output)
    }
}
/// Per-panic native value data preserves its formatting interfaces. The
/// original Any payload is retained on nested rethrow, without a registry.
pub struct SourcePrintfPanic { value: Box<dyn PanicValue> }
impl SourcePrintfPanic {
    pub fn new<Value: SourcePrintfValue + Send + 'static>(value: Value) -> Self {
        Self { value: Box::new(TypedPanicValue(value)) }
    }
    pub fn raise<Value: SourcePrintfValue + Send + 'static>(value: Value) -> ! {
        crate::utility_panic::unwind_source_panic(Self::new(value))
    }
    pub fn source_type(&self) -> &str { self.value.source_type() }
    pub fn error(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { self.value.source_error() }
    pub fn render(&self, profile: SourceUnicodeProfile, go_syntax: bool) -> Result<Vec<u8>, SourceSliceAllocationError> {
        let mut state = SourcePrintfResolvedCounts::default(); state.flags.sharp_v = go_syntax;
        let mut output = Vec::new(); self.value.write_panic_value('v', &state, profile, false, &mut output)?; Ok(output)
    }
}

fn write_panic_value(panic: SourceRecoveredPanic, profile: SourceUnicodeProfile, output: &mut Vec<u8>)
    -> Result<(), SourceSliceAllocationError>
{
    if let Some(value) = panic.payload().downcast_ref::<SourcePrintfPanic>() {
        return value.value.write_panic_value('v', &SourcePrintfResolvedCounts::default(), profile, true, output);
    }
    if let Some(value) = panic.payload().downcast_ref::<crate::utility_error_alias::SourceSharedError>() {
        return write_source_value(value, 'v', &SourcePrintfResolvedCounts::default(), profile, SourcePrintfMode::Sprintf,
            SourcePrintfRecursion { panicking: true, ..SourcePrintfRecursion::default() }, output);
    }
    if let Some(error) = panic.error() {
        if let Some(bytes) = source_error_message(error) { return append(output, &bytes); }
        // Existing native errors without source method views retain their
        // Display bridge. A nested panic propagates its original payload.
        return append(output, error.to_string().as_bytes());
    }
    macro_rules! primitive_payloads {
        ($($ty:ty),+ $(,)?) => { $(
            if let Some(value) = panic.payload().downcast_ref::<$ty>() {
                return write_source_value(value, 'v', &SourcePrintfResolvedCounts::default(), profile, SourcePrintfMode::Sprintf,
                    SourcePrintfRecursion { panicking: true, ..SourcePrintfRecursion::default() }, output);
            }
        )+ };
    }
    primitive_payloads!(String, &'static str, bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);
    // Opaque native payloads have no source reflection data yet. Preserve the
    // existing native recovery view; full arbitrary/nil/Goexit profiles remain.
    append(output, panic.to_string().as_bytes())
}

/// A new Sprintf printer for a recovered value, rather than fmt's nested
/// panicking printer. klog's recovery uses %s, including its bad-verb output
/// for non-string values. Unmapped native payloads remain explicit.
pub(crate) fn write_recovered_value(panic: &SourceRecoveredPanic, verb: char,
    profile: SourceUnicodeProfile, output: &mut Vec<u8>) -> Option<Result<(), SourceSliceAllocationError>>
{
    let state = SourcePrintfResolvedCounts::default();
    if let Some(value) = panic.payload().downcast_ref::<SourcePrintfPanic>() {
        return Some(value.value.write_panic_value(verb, &state, profile, false, output));
    }
    if let Some(error) = panic.error() {
        if crate::utility_error_view::source_error_type(error).is_none() { return None; }
        return Some(crate::utility_format_arguments::write_source_error_operand(error, verb, profile, SourcePrintfMode::Sprintf, output));
    }
    macro_rules! primitive_payloads {
        ($($ty:ty),+ $(,)?) => { $(
            if let Some(value) = panic.payload().downcast_ref::<$ty>() {
                return Some(write_source_value(value, verb, &state, profile, SourcePrintfMode::Sprintf,
                    SourcePrintfRecursion::default(), output));
            }
        )+ };
    }
    primitive_payloads!(String, &'static str, bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64,
        crate::utility_format_copy::SourceFmtSharedString);
    None
}
