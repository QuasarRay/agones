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
        print_arguments, write_nil, append_rune},
    utility_format_integer::{SourcePrintfIntegerValue, write_integer, write_number},
    utility_format_text::write_text,
    utility_format_bool::write_bool,
    utility_format_methods::{SourcePrintfMethodViews, SourceErrorMethod, write_source_value},
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces,
    utility_error_format::{source_error_bytes, canonical_error_fields, CanonicalErrorFields},
    utility_error_view::source_error_type,
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

pub trait SourcePrintfValue: SourcePrintfMethodViews + Sized {
    fn source_type(&self) -> &str;
    fn source_integer(&self) -> SourcePrintfInteger { SourcePrintfInteger::Other }
    fn source_is_nil(&self) -> bool { false }
    fn source_is_nil_pointer(&self) -> bool { false }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { None }
    /// Base/reflection formatting without user method dispatch. The shared
    /// method helper calls this directly while suppressing recursive methods.
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>;
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
        _: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_text(output, self.0, verb, state, profile) }
}
impl SourcePrintfValue for &str {
    fn source_type(&self) -> &str { "string" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_text(output, self.as_bytes(), verb, state, profile) }
}
impl SourcePrintfValue for String {
    fn source_type(&self) -> &str { "string" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_text(output, self.as_bytes(), verb, state, profile) }
}
impl SourcePrintfValue for bool {
    fn source_type(&self) -> &str { "bool" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile,
        _: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_bool(output, *self, verb, state) }
}
macro_rules! integer_values {
    ($($ty:ty),+ $(,)?) => { $(
        impl SourcePrintfValue for $ty {
            fn source_type(&self) -> &str { <Self as SourcePrintfIntegerValue>::SOURCE_TYPE }
            fn source_integer(&self) -> SourcePrintfInteger { self.source_count() }
            fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
                _: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
            { write_integer(output, self.source_bits(), <Self as SourcePrintfIntegerValue>::SIGNED,
                <Self as SourcePrintfIntegerValue>::SOURCE_TYPE, verb, state, profile) }
        }
    )+ };
}
integer_values!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

macro_rules! no_source_methods {
    ($($ty:ty),+ $(,)?) => { $(impl SourcePrintfMethodViews for $ty {})+ };
}
no_source_methods!(String, &str, bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfNil;
impl SourcePrintfValue for SourcePrintfNil {
    fn source_type(&self) -> &str { "" }
    fn source_is_nil(&self) -> bool { true }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile,
        _: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_nil(output, verb, state) }
}

/// Own a canonical source error without wrapping/reboxing its original Box.
/// The constructor's Err returns unknown native input unchanged; it is a
/// bridge classification result, not a source fmt error. Arbitrary source
/// error/Formatter/GoStringer data still needs its concrete value adapter.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
#[deftly(source_error_method)]
pub struct SourcePrintfError { error: BoxError, source_type: &'static str }
impl SourcePrintfError {
    pub fn canonical(error: BoxError) -> Result<Self, BoxError> {
        if canonical_error_fields(error.as_ref()).is_some() {
            if let Some(source_type) = source_error_type(error.as_ref()) { return Ok(Self { error, source_type }); }
        }
        Err(error)
    }
    pub fn original(&self) -> &(dyn Error + Send + Sync + 'static) { self.error.as_ref() }
    pub fn into_original(self) -> BoxError { self.error }
}
impl SourcePrintfValue for SourcePrintfError {
    fn source_type(&self) -> &str { self.source_type }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { Some(self.error.as_ref()) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if verb == 'p' { return write_error_pointer(output, self.error.as_ref(), self.source_type, verb, state, profile); }
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
                match index { $($index => write_source_value(&self.$index, verb, state, profile, mode, false, output),)+
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
        if index == 0 { write_source_value(&self.0, verb, state, profile, mode, false, output) }
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
    // This is the retained native allocation's address. Full Go heap/word/
    // pointer/ABI binding remains required; no source-address evidence exists.
    let address = error as *const (dyn Error + 'static) as *const () as usize as u64;
    let mut number = state.value_state();
    if verb == 'v' && state.flags.sharp_v {
        append(output, b"(")?; append(output, source_type.as_bytes())?; append(output, b")(")?;
        number.flags.sharp = true;
        write_number(output, address, false, 16, 'v', false, &number)?;
        append(output, b")")
    } else if verb == 'v' || verb == 'p' {
        number.flags.sharp = !state.flags.sharp;
        write_number(output, address, false, 16, 'v', false, &number)
    } else if matches!(verb, 'b' | 'o' | 'd' | 'x' | 'X') {
        write_integer(output, address, false, source_type, verb, state, profile)
    } else {
        append(output, b"%!")?; append_rune(output, verb)?; append(output, b"(")?;
        append(output, source_type.as_bytes())?; append(output, b"=")?;
        if canonical_error_fields(error).is_some() {
            write_canonical_reflection(output, error, source_type, 'v', state, profile)?;
        } else { append(output, format!("{error:?}").as_bytes())?; }
        append(output, b")")
    }
}

fn write_canonical_reflection(output: &mut Vec<u8>, error: &(dyn Error + 'static), source_type: &str,
    verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile) -> Result<(), SourceSliceAllocationError>
{
    let fields = canonical_error_fields(error).expect("canonical error adapter retains canonical fields");
    append(output, b"&")?;
    if state.flags.sharp_v { append(output, source_type.strip_prefix('*').unwrap_or(source_type).as_bytes())?; }
    append(output, b"{")?;
    let (message, field) = match &fields {
        CanonicalErrorFields::Message(message) => (*message, b"s".as_slice()),
        CanonicalErrorFields::Wrapped { message, .. } | CanonicalErrorFields::Multiple { message, .. } => (*message, b"msg".as_slice()),
    };
    field_name(output, field, state)?; write_text(output, message, verb, state, profile)?;
    match fields {
        CanonicalErrorFields::Message(_) => {}
        CanonicalErrorFields::Wrapped { source, .. } => {
            field_separator(output, state)?; field_name(output, b"err", state)?;
            if let Some(source) = source { write_nested_error(output, source, verb, state, profile)?; }
            else { append(output, if state.flags.sharp_v { b"error(nil)" } else { b"<nil>" })?; }
        }
        CanonicalErrorFields::Multiple { errors, .. } => {
            field_separator(output, state)?; field_name(output, b"errs", state)?;
            if state.flags.sharp_v {
                append(output, b"[]error")?;
                if errors.is_none() { append(output, b"(nil)")?; append(output, b"}")?; return Ok(()); }
            }
            append(output, if state.flags.sharp_v { b"{" } else { b"[" })?;
            for (index, error) in errors.unwrap_or(&[]).iter().enumerate() {
                if index != 0 { field_separator(output, state)?; }
                write_nested_error(output, error.as_ref(), verb, state, profile)?;
            }
            append(output, if state.flags.sharp_v { b"}" } else { b"]" })?;
        }
    }
    append(output, b"}")
}
fn field_name(output: &mut Vec<u8>, name: &[u8], state: &SourcePrintfResolvedCounts) -> Result<(), SourceSliceAllocationError> {
    if state.flags.sharp_v || state.flags.plus_v { append(output, name)?; append(output, b":")?; }
    Ok(())
}
fn field_separator(output: &mut Vec<u8>, state: &SourcePrintfResolvedCounts) -> Result<(), SourceSliceAllocationError> {
    append(output, if state.flags.sharp_v { b", " } else { b" " })
}
fn write_nested_error(output: &mut Vec<u8>, error: &(dyn Error + 'static), verb: char,
    state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile)
    -> Result<(), SourceSliceAllocationError>
{
    if let Some(source_type) = source_error_type(error).filter(|name| name.starts_with('*')) {
        return write_error_pointer(output, error, source_type, verb, state, profile);
    }
    // Native errors without a source reflection view retain an explicit
    // Debug fallback. Their source field/method/panic profiles remain required.
    append(output, format!("{error:?}").as_bytes())
}
