//! Concrete native method-data adapters for source formatting interfaces.
//! Closures belong to individual source values; they do not register runtime
//! providers, types, plugins or lifecycle components.
use std::{borrow::Cow, error::Error};
use crate::{
    errors::BoxError, runtime::SourceLogType,
    utility_format_arguments::SourcePrintfValue,
    utility_format_methods::{SourcePrintfMethodViews, SourcePrintfInterfaces, SourceFormatState,
        SourceFormatter, SourceGoStringer, SourceStringer, SourceErrorMethod},
    utility_format_print::SourcePrintfMode,
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts},
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

pub struct SourcePrintfFormatter<Value, Function> { pub value: Value, pub format: Function }
impl<Value: SourcePrintfValue, Function> SourceFormatter for SourcePrintfFormatter<Value, Function>
where Function: for<'a, 'b, 'c> Fn(&'a Value, &'b mut SourceFormatState<'c>, char) -> Result<(), SourceSliceAllocationError>,
{
    fn source_format(&self, state: &mut SourceFormatState<'_>, verb: char) -> Result<(), SourceSliceAllocationError> {
        (self.format)(&self.value, state, verb)
    }
}
impl<Value: SourcePrintfValue, Function> SourcePrintfMethodViews for SourcePrintfFormatter<Value, Function>
where Function: for<'a, 'b, 'c> Fn(&'a Value, &'b mut SourceFormatState<'c>, char) -> Result<(), SourceSliceAllocationError>,
{
    fn source_methods(&self) -> SourcePrintfInterfaces<'_> {
        SourcePrintfInterfaces { formatter: Some(self), ..self.value.source_methods() }
    }
}
impl<Value: SourcePrintfValue, Function> SourcePrintfValue for SourcePrintfFormatter<Value, Function>
where Function: for<'a, 'b, 'c> Fn(&'a Value, &'b mut SourceFormatState<'c>, char) -> Result<(), SourceSliceAllocationError>,
{
    fn source_type(&self) -> &str { self.value.source_type() }
    fn source_integer(&self) -> SourcePrintfInteger { self.value.source_integer() }
    fn source_is_nil(&self) -> bool { self.value.source_is_nil() }
    fn source_is_nil_pointer(&self) -> bool { self.value.source_is_nil_pointer() }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { self.value.source_error_view() }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { self.value.write_source(verb, state, profile, mode, output) }
    fn into_source_error(self) -> Option<BoxError> { self.value.into_source_error() }
}

macro_rules! string_method_adapter {
    ($name:ident, $interface:ident, $method:ident, $view:ident) => {
        pub struct $name<Value, Function> { pub value: Value, pub method: Function }
        impl<Value: SourcePrintfValue, Function> $interface for $name<Value, Function>
        where Function: for<'a> Fn(&'a Value) -> Cow<'a, [u8]>,
        {
            fn $method(&self) -> Cow<'_, [u8]> { (self.method)(&self.value) }
        }
        impl<Value: SourcePrintfValue, Function> SourcePrintfMethodViews for $name<Value, Function>
        where Function: for<'a> Fn(&'a Value) -> Cow<'a, [u8]>,
        {
            fn source_methods(&self) -> SourcePrintfInterfaces<'_> {
                SourcePrintfInterfaces { $view: Some(self), ..self.value.source_methods() }
            }
        }
        impl<Value: SourcePrintfValue, Function> SourcePrintfValue for $name<Value, Function>
        where Function: for<'a> Fn(&'a Value) -> Cow<'a, [u8]>,
        {
            fn source_type(&self) -> &str { self.value.source_type() }
            fn source_integer(&self) -> SourcePrintfInteger { self.value.source_integer() }
            fn source_is_nil(&self) -> bool { self.value.source_is_nil() }
            fn source_is_nil_pointer(&self) -> bool { self.value.source_is_nil_pointer() }
            fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { self.value.source_error_view() }
            fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
                mode: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
            { self.value.write_source(verb, state, profile, mode, output) }
            fn into_source_error(self) -> Option<BoxError> { self.value.into_source_error() }
        }
    };
}
string_method_adapter!(SourcePrintfGoStringer, SourceGoStringer, source_go_string, go_stringer);
string_method_adapter!(SourcePrintfStringer, SourceStringer, source_string, stringer);

/// A typed native error keeps its original Box allocation through formatting
/// and erasure into the existing canonical factory. Its declared source type,
/// methods and base/reflection body come from the real source-derived value.
pub struct SourcePrintfOwnedError<Value> { error: Box<Value> }
impl<Value> SourcePrintfOwnedError<Value> {
    pub fn new(error: Box<Value>) -> Self { Self { error } }
    pub fn original(&self) -> &Value { &self.error }
    pub fn into_original(self) -> Box<Value> { self.error }
}
impl<Value> SourceErrorMethod for SourcePrintfOwnedError<Value>
where Value: SourceErrorMethod,
{
    fn source_error_message(&self) -> Cow<'_, [u8]> { self.error.source_error_message() }
}
impl<Value> SourcePrintfMethodViews for SourcePrintfOwnedError<Value>
where Value: SourcePrintfValue + SourceErrorMethod,
{
    fn source_methods(&self) -> SourcePrintfInterfaces<'_> {
        SourcePrintfInterfaces { error: Some(self), ..self.error.source_methods() }
    }
}
impl<Value> SourcePrintfValue for SourcePrintfOwnedError<Value>
where Value: SourcePrintfValue + SourceErrorMethod + Error + SourceLogType + Send + Sync + 'static,
{
    fn source_type(&self) -> &str { Value::SOURCE_LOG_TYPE }
    fn source_integer(&self) -> SourcePrintfInteger { self.error.source_integer() }
    // A typed error interface is non-nil even when its receiver pointer is nil.
    fn source_is_nil_pointer(&self) -> bool { self.error.source_is_nil_pointer() }
    fn source_error_view(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { Some(self.error.as_ref()) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { self.error.write_source(verb, state, profile, mode, output) }
    fn into_source_error(self) -> Option<BoxError> {
        let original: BoxError = self.error;
        Some(original)
    }
}
