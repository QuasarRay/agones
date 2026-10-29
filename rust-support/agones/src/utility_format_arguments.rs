//! Heterogeneous native tuples for Go variadic formatting. Each field keeps
//! its Rust type and original ownership; macro expansion supplies indexed
//! operations without erased value containers or a runtime provider registry.
// Adapted from Go fmt/print.go and fmt/errors.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE.
use std::{borrow::Cow, error::Error};
use derive_deftly::Deftly;
use crate::{
    errors::BoxError,
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts},
    utility_format_print::{SourcePrintfArguments, SourcePrintfOutput, SourcePrintfMode,
        print_arguments, write_nil},
    utility_format_integer::{SourcePrintfIntegerValue, write_integer},
    utility_format_pointer::write_source_address,
    utility_format_text::write_text,
    utility_format_bool::write_bool,
    utility_format_methods::{SourcePrintfMethodViews, SourcePrintfRecursion, SourceErrorMethod, write_source_value},
    utility_format_collections::{SourcePrintfStructFields, write_source_struct},
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces,
    utility_error_format::{source_error_bytes, canonical_error_fields, CanonicalErrorFields},
    utility_error_view::source_error_type,
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

pub trait SourcePrintfValue: SourcePrintfMethodViews + Sized {
    const SOURCE_KIND_UINT8: bool = false;
    const SOURCE_DEREFERENCE_AT_ROOT: bool = false;
    fn source_type(&self) -> &str;
    fn source_integer(&self) -> SourcePrintfInteger { SourcePrintfInteger::Other }
    fn source_is_nil(&self) -> bool { false }
    fn source_nil_interface_type(&self) -> &str { "interface {}" }
    fn source_is_nil_pointer(&self) -> bool { false }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { None }
    fn source_uint8(&self) -> Option<u8> { None }
    fn source_contiguous_bytes(_: &[Self]) -> Option<&[u8]> { None }
    /// Base/reflection formatting without user method dispatch. The shared
    /// method helper calls this directly while suppressing recursive methods.
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>;
    /// Non-errors are dropped after rendering. Error adapters transfer the
    /// existing Box once; repeated w occurrences retain only argument indexes.
    fn into_source_error(self) -> Option<BoxError> { None }
}

/// Explicit Go string-kind view, distinct from a source []byte operand.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfString<'a>(pub &'a [u8]);
impl SourcePrintfValue for SourcePrintfString<'_> {
    fn source_type(&self) -> &str { "string" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_text(output, self.0, verb, state, profile) }
}
impl SourcePrintfValue for &str {
    fn source_type(&self) -> &str { "string" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_text(output, self.as_bytes(), verb, state, profile) }
}
impl SourcePrintfValue for String {
    fn source_type(&self) -> &str { "string" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_text(output, self.as_bytes(), verb, state, profile) }
}
impl SourcePrintfValue for bool {
    fn source_type(&self) -> &str { "bool" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_bool(output, *self, verb, state) }
}
macro_rules! integer_values {
    ($($ty:ty),+ $(,)?) => { $(
        impl SourcePrintfValue for $ty {
            fn source_type(&self) -> &str { <Self as SourcePrintfIntegerValue>::SOURCE_TYPE }
            fn source_integer(&self) -> SourcePrintfInteger { self.source_count() }
            fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
                _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
            { write_integer(output, self.source_bits(), <Self as SourcePrintfIntegerValue>::SIGNED,
                <Self as SourcePrintfIntegerValue>::SOURCE_TYPE, verb, state, profile) }
        }
    )+ };
}
integer_values!(i8, i16, i32, i64, isize, u16, u32, u64, usize);
impl SourcePrintfValue for u8 {
    const SOURCE_KIND_UINT8: bool = true;
    fn source_type(&self) -> &str { "uint8" }
    fn source_integer(&self) -> SourcePrintfInteger { self.source_count() }
    fn source_uint8(&self) -> Option<u8> { Some(*self) }
    fn source_contiguous_bytes(values: &[Self]) -> Option<&[u8]> { Some(values) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_integer(output, u64::from(*self), false, "uint8", verb, state, profile) }
}

macro_rules! no_source_methods {
    ($($ty:ty),+ $(,)?) => { $(impl SourcePrintfMethodViews for $ty {})+ };
}
no_source_methods!(String, &str, bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfNil;
impl SourcePrintfValue for SourcePrintfNil {
    fn source_type(&self) -> &str { "" }
    fn source_is_nil(&self) -> bool { true }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_nil(output, verb, state) }
}

/// Own a canonical source error without wrapping/reboxing its original Box.
/// The constructor's Err returns unknown native input unchanged; it is a
/// bridge classification result, not a source fmt error. Arbitrary source
/// error/Formatter/GoStringer data still needs its concrete value adapter.
pub struct SourcePrintfError { error: BoxError, source_type: &'static str }
impl SourcePrintfError {
    pub fn canonical(error: BoxError) -> Result<Self, BoxError> {
        if canonical_error_fields(error.as_ref()).is_some() {
            if let Some(source_type) = source_error_type(error.as_ref()) { return Ok(Self { error, source_type }); }
        }
        Err(error)
    }
    pub fn original(&self) -> &(dyn Error + Send + Sync + 'static) { self.error.as_ref() }
    pub(crate) fn source_type_name(&self) -> &'static str { self.source_type }
    pub fn into_original(self) -> BoxError { self.error }
}
impl SourcePrintfMethodViews for SourcePrintfError {
    fn source_methods(&self) -> crate::utility_format_methods::SourcePrintfInterfaces<'_> {
        crate::utility_format_methods::SourcePrintfInterfaces {
            error: Some(self),
            error_methods: crate::utility_error_query::canonical_query_methods(self.original())
                .expect("canonical error constructor binds its source method set"),
            ..crate::utility_format_methods::SourcePrintfInterfaces::default()
        }
    }
}
impl SourcePrintfValue for SourcePrintfError {
    fn source_type(&self) -> &str { self.source_type }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { Some(self.error.as_ref()) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if verb == 'p' || recursion.depth > 0 { return write_error_pointer(output, self.error.as_ref(), self.source_type, verb, state, profile); }
        write_canonical_reflection(output, self.error.as_ref(), self.source_type, verb, state, profile)
    }
    fn into_source_error(self) -> Option<BoxError> { Some(self.error) }
}
impl SourceErrorMethod for SourcePrintfError {
    fn source_error_message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(source_error_bytes(self.error.as_ref()).expect("canonical error retains its message"))
    }
}

pub trait SourcePrintfOwnedArguments: SourcePrintfArguments + Sized {
    /// Return len() entries in the same argument order as the borrowed view.
    fn into_error_operands(self) -> Vec<Option<BoxError>>;
}
impl SourcePrintfArguments for () {
    fn len(&self) -> usize { 0 }
    fn type_name(&self, index: usize) -> &str { panic!("source printf index {index} outside empty arguments") }
    fn integer(&self, index: usize) -> SourcePrintfInteger { panic!("source printf index {index} outside empty arguments") }
    fn write_value(&self, index: usize, _: char, _: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile,
        _: SourcePrintfMode, _: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { panic!("source printf index {index} outside empty arguments") }
}
impl SourcePrintfOwnedArguments for () { fn into_error_operands(self) -> Vec<Option<BoxError>> { Vec::new() } }

macro_rules! tuple_arguments {
    ($( $index:tt => $ty:ident $field:ident ),+ $(,)?) => {
        impl<$($ty: SourcePrintfValue),+> SourcePrintfArguments for ($($ty,)+) {
            fn len(&self) -> usize { [$(stringify!($ty)),+].len() }
            fn type_name(&self, index: usize) -> &str {
                match index { $($index => self.$index.source_type(),)+ _ => panic!("source printf tuple index {index} out of range") }
            }
            fn integer(&self, index: usize) -> SourcePrintfInteger {
                match index { $($index => self.$index.source_integer(),)+ _ => panic!("source printf tuple index {index} out of range") }
            }
            fn is_nil(&self, index: usize) -> bool {
                match index { $($index => self.$index.source_is_nil(),)+ _ => panic!("source printf tuple index {index} out of range") }
            }
            fn write_value(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
                mode: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
            {
                match index { $($index => write_source_value(&self.$index, verb, state, profile, mode, SourcePrintfRecursion::default(), output),)+
                    _ => panic!("source printf tuple index {index} out of range") }
            }
        }
        impl<$($ty: SourcePrintfValue),+> SourcePrintfOwnedArguments for ($($ty,)+) {
            fn into_error_operands(self) -> Vec<Option<BoxError>> {
                let ($($field,)+) = self;
                vec![$($field.into_source_error(),)+]
            }
        }
    };
}
tuple_arguments!(0 => A a);
tuple_arguments!(0 => A a, 1 => B b);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d, 4 => E e);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d, 4 => E e, 5 => F f);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d, 4 => E e, 5 => F f, 6 => G g);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d, 4 => E e, 5 => F f, 6 => G g, 7 => H h);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d, 4 => E e, 5 => F f, 6 => G g, 7 => H h, 8 => I i);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d, 4 => E e, 5 => F f, 6 => G g, 7 => H h, 8 => I i, 9 => J j);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d, 4 => E e, 5 => F f, 6 => G g, 7 => H h, 8 => I i, 9 => J j, 10 => K k);
tuple_arguments!(0 => A a, 1 => B b, 2 => C c, 3 => D d, 4 => E e, 5 => F f, 6 => G g, 7 => H h, 8 => I i, 9 => J j, 10 => K k, 11 => L l);

/// Recursive native argument lists remove the convenience tuple arity limit.
pub struct SourcePrintfCons<Head, Tail>(pub Head, pub Tail);
impl<Head: SourcePrintfValue, Tail: SourcePrintfArguments> SourcePrintfArguments for SourcePrintfCons<Head, Tail> {
    fn len(&self) -> usize { 1 + self.1.len() }
    fn type_name(&self, index: usize) -> &str { if index == 0 { self.0.source_type() } else { self.1.type_name(index - 1) } }
    fn integer(&self, index: usize) -> SourcePrintfInteger { if index == 0 { self.0.source_integer() } else { self.1.integer(index - 1) } }
    fn is_nil(&self, index: usize) -> bool { if index == 0 { self.0.source_is_nil() } else { self.1.is_nil(index - 1) } }
    fn write_value(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if index == 0 { write_source_value(&self.0, verb, state, profile, mode, SourcePrintfRecursion::default(), output) }
        else { self.1.write_value(index - 1, verb, state, profile, mode, output) }
    }
}
impl<Head: SourcePrintfValue, Tail: SourcePrintfOwnedArguments> SourcePrintfOwnedArguments for SourcePrintfCons<Head, Tail> {
    fn into_error_operands(self) -> Vec<Option<BoxError>> {
        let mut errors = vec![self.0.into_source_error()];
        errors.extend(self.1.into_error_operands()); errors
    }
}

pub fn try_source_arguments_printf<Arguments: SourcePrintfArguments>(format: &[u8], arguments: &Arguments,
    profile: SourceUnicodeProfile, mode: SourcePrintfMode) -> Result<SourcePrintfOutput, SourceSliceAllocationError>
{ print_arguments(format, arguments, profile, mode) }

fn write_error_pointer(output: &mut Vec<u8>, error: &(dyn Error + 'static), source_type: &str,
    verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile) -> Result<(), SourceSliceAllocationError>
{
    let error = crate::utility_error_alias::source_error_original(error);
    // This is the retained native allocation's address. Full Go heap/word/
    // pointer/ABI binding remains required; no source-address evidence exists.
    let address = error as *const (dyn Error + 'static) as *const () as usize as u64;
    let value = CanonicalPointerValue { error, source_type };
    write_source_address(&value, address, verb, state, profile, SourcePrintfMode::Sprintf,
        SourcePrintfRecursion { depth: 1, can_interface: false, ..SourcePrintfRecursion::default() }, output)
}

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
struct CanonicalPointerValue<'a> { error: &'a (dyn Error + 'static), source_type: &'a str }
impl SourcePrintfValue for CanonicalPointerValue<'_> {
    fn source_type(&self) -> &str { self.source_type }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if recursion.depth == 0 && verb != 'p' {
            if canonical_error_fields(self.error).is_some() {
                return write_canonical_reflection(output, self.error, self.source_type, verb, state, profile);
            }
            return append(output, format!("{:?}", self.error).as_bytes());
        }
        let error = crate::utility_error_alias::source_error_original(self.error);
        let address = error as *const (dyn Error + 'static) as *const () as usize as u64;
        write_source_address(self, address, verb, state, profile, mode, recursion, output)
    }
}

fn write_canonical_reflection(output: &mut Vec<u8>, error: &(dyn Error + 'static), source_type: &str,
    verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile) -> Result<(), SourceSliceAllocationError>
{
    append(output, b"&")?;
    let fields = CanonicalStructFields {
        fields: canonical_error_fields(error).expect("canonical error adapter retains canonical fields"),
        source_type: source_type.strip_prefix('*').unwrap_or(source_type),
    };
    write_source_struct(&fields, verb, state, profile, SourcePrintfMode::Sprintf,
        SourcePrintfRecursion { depth: 1, can_interface: false, ..SourcePrintfRecursion::default() }, output)
}

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
struct CanonicalStructFields<'a> { fields: CanonicalErrorFields<'a>, source_type: &'a str }
impl SourcePrintfValue for CanonicalStructFields<'_> {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { self.source_type }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl SourcePrintfStructFields for CanonicalStructFields<'_> {
    fn source_field_count(&self) -> usize { if matches!(&self.fields, CanonicalErrorFields::Message(_)) { 1 } else { 2 } }
    fn source_field_name(&self, index: usize) -> &str {
        match (index, &self.fields) {
            (0, CanonicalErrorFields::Message(_)) => "s",
            (0, _) => "msg",
            (1, CanonicalErrorFields::Wrapped { .. }) => "err",
            (1, CanonicalErrorFields::Multiple { .. }) => "errs",
            _ => panic!("canonical source field index out of range"),
        }
    }
    fn source_field_interfaceable(&self, _: usize) -> bool { false }
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, _: SourcePrintfMode, _: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if index == 0 {
            let message = match &self.fields {
                CanonicalErrorFields::Message(message) | CanonicalErrorFields::Wrapped { message, .. }
                    | CanonicalErrorFields::Multiple { message, .. } => *message,
            };
            return write_text(output, message, verb, state, profile);
        }
        match (index, &self.fields) {
            (1, CanonicalErrorFields::Wrapped { source, .. }) => {
                if let Some(source) = source { write_nested_error(output, *source, verb, state, profile)?; }
                else { append(output, if state.flags.sharp_v { b"error(nil)" } else { b"<nil>" })?; }
            }
            (1, CanonicalErrorFields::Multiple { errors, .. }) => {
                errors.write_source(verb, state, profile, SourcePrintfMode::Sprintf,
                    SourcePrintfRecursion { depth: 2, can_interface: false, ..SourcePrintfRecursion::default() }, output)?;
            }
            _ => panic!("canonical source field index out of range"),
        }
        Ok(())
    }
}
fn write_nested_error(output: &mut Vec<u8>, error: &(dyn Error + 'static), verb: char,
    state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile)
    -> Result<(), SourceSliceAllocationError>
{
    write_source_error_value(error, verb, state, profile, SourcePrintfMode::Sprintf,
        SourcePrintfRecursion { depth: 2, can_interface: false, ..SourcePrintfRecursion::default() }, output)
}

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
#[deftly(source_error_method)]
struct CanonicalBorrowedError<'a> { error: &'a (dyn Error + 'static), source_type: &'static str }
impl SourceErrorMethod for CanonicalBorrowedError<'_> {
    fn source_error_message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(source_error_bytes(self.error).expect("canonical borrowed error retains its message"))
    }
}
impl SourcePrintfValue for CanonicalBorrowedError<'_> {
    fn source_type(&self) -> &str { self.source_type }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if verb == 'p' || recursion.depth > 0 { write_error_pointer(output, self.error, self.source_type, verb, state, profile) }
        else { write_canonical_reflection(output, self.error, self.source_type, verb, state, profile) }
    }
}

/// Source interface elements retain their concrete declared format views.
/// The existing opaque native Debug path remains explicitly unbound source
/// data; it is not used to assign a fabricated source type or method set.
pub(crate) fn write_source_error_value(mut error: &(dyn Error + 'static), verb: char,
    state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile, mode: SourcePrintfMode,
    recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    loop {
        if let Some(retained) = error.downcast_ref::<crate::utility_error_slice::SourceRetainedError>() { error = retained.original(); continue; }
        if let Some(shared) = error.downcast_ref::<crate::utility_error_alias::SourceSharedError>() {
            return write_source_value(shared, verb, state, profile, mode, recursion, output);
        }
        if let Some(runtime) = error.downcast_ref::<crate::utility_panic::SourceRuntimeErrorEnvelope>() { error = runtime.error(); continue; }
        break;
    }
    if canonical_error_fields(error).is_some() {
        let value = CanonicalBorrowedError { error, source_type: source_error_type(error).expect("canonical error has source type") };
        return write_source_value(&value, verb, state, profile, mode, recursion, output);
    }
    macro_rules! runtime_values {
        ($($ty:ty),+ $(,)?) => { $(if let Some(value) = error.downcast_ref::<$ty>() {
            return write_source_value(value, verb, state, profile, mode, recursion, output);
        })+ };
    }
    runtime_values!(crate::utility_panic::SourceRuntimeErrorString, crate::utility_panic::SourceNilMapAssignment,
        crate::utility_panic::SourceBoundsError, crate::utility_error_nil::SourceNilStatusError,
        crate::utility_error_nil::SourceNilTraceError, crate::source::workerqueue::traceError,
        crate::utility_time_json_error::SourceTimeMarshalerError);
    if let Some(source_type) = source_error_type(error).filter(|name| name.starts_with('*')) {
        return write_error_pointer(output, error, source_type, verb, state, profile);
    }
    // Native errors without a source reflection view retain an explicit
    // Debug fallback. Their source field/method/panic profiles remain required.
    append(output, format!("{error:?}").as_bytes())
}

/// Default Error operand formatting for source Wrap/Wrapf and Logrus Sprint.
/// Declared source values retain method precedence and fmt panic recovery.
/// Opaque native errors preserve their old Display bridge, without source
/// method/receiver/panic/refinement credit; they are not reflection objects.
pub(crate) fn write_source_error_operand(error: &(dyn Error + 'static), verb: char,
    profile: SourceUnicodeProfile, mode: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    let original = error;
    let mut value = error;
    loop {
        if let Some(retained) = value.downcast_ref::<crate::utility_error_slice::SourceRetainedError>() { value = retained.original(); continue; }
        if value.is::<crate::utility_error_alias::SourceSharedError>() { break; }
        if let Some(runtime) = value.downcast_ref::<crate::utility_panic::SourceRuntimeErrorEnvelope>() { value = runtime.error(); continue; }
        break;
    }
    let declared = value.is::<crate::utility_error_alias::SourceSharedError>() || canonical_error_fields(value).is_some()
        || value.is::<crate::utility_panic::SourceRuntimeErrorString>()
        || value.is::<crate::utility_panic::SourceNilMapAssignment>()
        || value.is::<crate::utility_panic::SourceBoundsError>()
        || value.is::<crate::utility_error_nil::SourceNilStatusError>()
        || value.is::<crate::utility_error_nil::SourceNilTraceError>()
        || value.is::<crate::source::workerqueue::traceError>()
        || value.is::<crate::utility_time_json_error::SourceTimeMarshalerError>();
    if declared {
        return write_source_error_value(original, verb, &SourcePrintfResolvedCounts::default(), profile, mode,
            SourcePrintfRecursion::default(), output);
    }
    if let Some(message) = crate::utility_error_format::source_error_message(original) { return append(output, &message); }
    append(output, original.to_string().as_bytes())
}
