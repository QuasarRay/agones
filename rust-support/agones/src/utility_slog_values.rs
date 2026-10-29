//! Source slog Value/Attr data and resolution used by the klog serializer.
// Adapted from Go1.26.5 log/slog/{value,attr}.go and time/time.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE. These are typed
// source values, not a general Go object interpreter or lifecycle manager.
// Full time/native timezone, interface equality/type/word/heap/ABI and caller-frame
// bindings remain required. A missing frame/format binding is explicit.
use std::{borrow::Cow, panic::{catch_unwind, AssertUnwindSafe}, sync::Arc};
use derive_deftly::Deftly;
use crate::{utility_error_handlers::{SourceErrorArgument, SourceErrorArguments},
    utility_error_slice::SourceErrorSliceIndex,
    utility_callback_slice::SourceCallbackSlice,
    utility_format_copy::{SourceFmtSharedString, SourceFmtValueCopy},
    utility_format_arguments::{SourcePrintfValue, SourcePrintfError},
    utility_format_methods::{SourceStringer, SourcePrintfRecursion, write_source_value},
    utility_format_collections::{SourcePrintfStructFields, write_source_struct},
    utility_format_integer::write_integer,
    utility_format_pointer::{SourcePrintfAddress, SourceAddressKind, write_source_address},
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts},
    utility_format_print::SourcePrintfMode,
    utility_float_reference::{try_source_float, SourceFloatWidth},
    utility_time_values::{SourceGoTime, SourceTimeLocation},
    utility_http::{HeaderAllocationProfile, SourceUnicodeProfile, SourceSliceAllocationError,
        NativeSourceCollectionContext, complete_source_collection},
    utility_go_quote::append,
    utility_klog_dispatch::{SourceKlogFormatter, SourceKlogFormatError, FormatResult,
        SourceKlogJsonEncoder, SourceKlogEncodeError},
    utility_declarations::{derive_deftly_template_SourcePrintfInterfaces,
        derive_deftly_template_SourceCallbackSlice, derive_deftly_template_SourceKlogAdapter}};

fn allocation_profile() -> HeaderAllocationProfile {
    if usize::BITS == 32 { HeaderAllocationProfile::GO126_32 } else { HeaderAllocationProfile::GO126_64 }
}
fn finish<Value>(result: FormatResult<Value>) -> Value {
    match result {
        Ok(value) => value,
        Err(SourceKlogFormatError::Allocation(error)) => complete_source_collection(&NativeSourceCollectionContext, Err(error)),
        Err(error) => std::panic::panic_any(error), // explicit native binding failure
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
#[deftly(source_stringer, source_klog_json_encoder)]
pub struct SourceSlogKind(pub i64);
impl SourceSlogKind {
    pub const ANY: Self = Self(0); pub const BOOL: Self = Self(1);
    pub const DURATION: Self = Self(2); pub const FLOAT64: Self = Self(3);
    pub const INT64: Self = Self(4); pub const STRING: Self = Self(5);
    pub const TIME: Self = Self(6); pub const UINT64: Self = Self(7);
    pub const GROUP: Self = Self(8); pub const LOG_VALUER: Self = Self(9);
    pub fn string(self) -> &'static [u8] {
        match self.0 { 0 => b"Any", 1 => b"Bool", 2 => b"Duration", 3 => b"Float64", 4 => b"Int64",
            5 => b"String", 6 => b"Time", 7 => b"Uint64", 8 => b"Group", 9 => b"LogValuer", _ => b"<unknown slog.Kind>" }
    }
}
impl SourceFmtValueCopy for SourceSlogKind { fn source_fmt_copy(&self) -> Self { *self } }
impl SourceStringer for SourceSlogKind { fn source_string(&self) -> Cow<'_, [u8]> { Cow::Borrowed(self.string()) } }
impl SourcePrintfValue for SourceSlogKind {
    fn source_type(&self) -> &str { "slog.Kind" }
    fn source_integer(&self) -> SourcePrintfInteger { SourcePrintfInteger::Signed(self.0) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_integer(output, self.0 as u64, true, "slog.Kind", verb, state, profile) }
}

#[derive(Clone, Copy, Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
#[deftly(source_stringer, source_klog_json_encoder)]
pub struct SourceSlogDuration(pub i64);
impl SourceFmtValueCopy for SourceSlogDuration { fn source_fmt_copy(&self) -> Self { *self } }
impl SourceStringer for SourceSlogDuration {
    fn source_string(&self) -> Cow<'_, [u8]> { Cow::Owned(duration_string(self.0)) }
}
impl SourcePrintfValue for SourceSlogDuration {
    fn source_type(&self) -> &str { "time.Duration" }
    fn source_integer(&self) -> SourcePrintfInteger { SourcePrintfInteger::Signed(self.0) }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_integer(output, self.0 as u64, true, "time.Duration", verb, state, profile) }
}

enum ValueData {
    Kind(SourceSlogKind), String(SourceFmtSharedString), Group(SourceSlogAttrs),
    Any(SourceErrorArgument), KindValue(SourceSlogKind),
    TimeLocation(Option<Arc<SourceTimeLocation>>), TimeTime(SourceGoTime),
}
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
#[deftly(source_stringer, source_slog_value, source_klog_json_encoder)]
pub struct SourceSlogValue { num: u64, data: ValueData }
impl Default for SourceSlogValue {
    fn default() -> Self { Self { num: 0, data: ValueData::Any(SourceErrorArgument::default()) } }
}
impl SourceFmtValueCopy for SourceSlogValue {
    fn source_fmt_copy(&self) -> Self {
        let data = match &self.data {
            ValueData::Kind(kind) => ValueData::Kind(*kind), ValueData::KindValue(kind) => ValueData::KindValue(*kind),
            ValueData::String(value) => ValueData::String(value.source_fmt_copy()),
            ValueData::Group(value) => ValueData::Group(value.source_fmt_copy()),
            ValueData::Any(value) => ValueData::Any(value.source_fmt_copy()),
            ValueData::TimeLocation(value) => ValueData::TimeLocation(value.as_ref().map(Arc::clone)),
            ValueData::TimeTime(value) => ValueData::TimeTime(value.source_fmt_copy()),
        };
        Self { num: self.num, data }
    }
}
impl SourceSlogValue {
    pub fn string_value(value: SourceFmtSharedString) -> Self { Self { num: value.0.len() as u64, data: ValueData::String(value) } }
    pub fn int64_value(value: i64) -> Self { Self { num: value as u64, data: ValueData::Kind(SourceSlogKind::INT64) } }
    pub fn uint64_value(value: u64) -> Self { Self { num: value, data: ValueData::Kind(SourceSlogKind::UINT64) } }
    pub fn float64_value(value: f64) -> Self { Self { num: value.to_bits(), data: ValueData::Kind(SourceSlogKind::FLOAT64) } }
    pub fn bool_value(value: bool) -> Self { Self { num: u64::from(value), data: ValueData::Kind(SourceSlogKind::BOOL) } }
    pub fn duration_value(value: i64) -> Self { Self { num: value as u64, data: ValueData::Kind(SourceSlogKind::DURATION) } }
    pub fn time_value(value: &SourceGoTime) -> Self {
        if value.is_zero() { return Self { num: 0, data: ValueData::TimeLocation(None) }; }
        let nanos = value.unix_nano();
        // The intermediate Unix time is used only for instant equality. Its
        // temporary Local pointer cannot affect that comparison; final Time
        // reconstruction explicitly restores the retained source location.
        if value.equal(&SourceGoTime::unix_utc(0, nanos)) {
            Self { num: nanos as u64, data: ValueData::TimeLocation(Some(value.location())) }
        } else { Self { num: 0, data: ValueData::TimeTime(value.without_monotonic()) } }
    }
    pub fn group_value(attrs: SourceSlogAttrs) -> Self {
        let mut empty = 0;
        for index in 0..attrs.len() { if attrs.get(index).value.is_empty_group() { empty += 1; } }
        let attrs = if empty == 0 { attrs } else {
            let mut filtered = SourceSlogAttrs::make(0, (attrs.len() - empty) as i64, attrs.allocation_profile());
            // The source performs a second range pass; do not turn this into
            // a snapshot/dedup or resolve any value methods while filtering.
            for index in 0..attrs.len() {
                let attr = attrs.get(index);
                if !attr.value.is_empty_group() { filtered.append(attr); }
            }
            filtered
        };
        Self { num: attrs.len() as u64, data: ValueData::Group(attrs) }
    }
    pub fn any_value(value: SourceErrorArgument) -> Self {
        if value.source_type().is_some() {
            if let Some(value) = value.downcast_ref::<Self>() { return value.source_fmt_copy(); }
            if let Some(value) = value.downcast_ref::<SourceFmtSharedString>() { return Self::string_value(value.source_fmt_copy()); }
            if let Some(bytes) = crate::utility_klog_values::string_bytes(&value) {
                return Self::string_value(SourceFmtSharedString(Arc::from(bytes)));
            }
            macro_rules! signed { ($($ty:ty),+) => { $(if let Some(value) = value.downcast_ref::<$ty>() { return Self::int64_value(*value as i64); })+ }; }
            macro_rules! unsigned { ($($ty:ty),+) => { $(if let Some(value) = value.downcast_ref::<$ty>() { return Self::uint64_value(*value as u64); })+ }; }
            signed!(i8, i16, i32, i64, isize); unsigned!(u8, u16, u32, u64, usize);
            if let Some(value) = value.downcast_ref::<bool>() { return Self::bool_value(*value); }
            if let Some(value) = value.downcast_ref::<f64>() { return Self::float64_value(*value); }
            if let Some(value) = value.downcast_ref::<f32>() { return Self::float64_value(f64::from(*value)); }
            if let Some(value) = value.downcast_ref::<SourceSlogDuration>() { return Self::duration_value(value.0); }
            if let Some(value) = value.downcast_ref::<SourceGoTime>() { return Self::time_value(value); }
            if let Some(value) = value.downcast_ref::<SourceSlogAttrs>() { return Self::group_value(value.source_fmt_copy()); }
            if let Some(value) = value.downcast_ref::<SourceSlogKind>() { return Self { num: 0, data: ValueData::KindValue(*value) }; }
        }
        Self { num: 0, data: ValueData::Any(value) }
    }
    pub fn kind(&self) -> SourceSlogKind {
        match &self.data {
            ValueData::Kind(kind) => *kind, ValueData::String(_) => SourceSlogKind::STRING,
            ValueData::Group(_) => SourceSlogKind::GROUP, ValueData::KindValue(_) => SourceSlogKind::ANY,
            ValueData::TimeLocation(_) | ValueData::TimeTime(_) => SourceSlogKind::TIME,
            ValueData::Any(value) => if value.source_methods().klog.log_valuer.is_some() {
                SourceSlogKind::LOG_VALUER } else { SourceSlogKind::ANY },
        }
    }
    pub fn string_bytes(&self) -> Option<&[u8]> { match &self.data { ValueData::String(value) => Some(&value.0), _ => None } }
    pub fn group_ref(&self) -> Option<&SourceSlogAttrs> { match &self.data { ValueData::Group(value) => Some(value), _ => None } }
    pub fn is_empty_group(&self) -> bool { self.group_ref().is_some_and(|group| group.len() == 0) }
    pub fn any(&self) -> SourceErrorArgument {
        match &self.data {
            ValueData::Any(value) => value.source_fmt_copy(),
            ValueData::KindValue(value) => SourceErrorArgument::from_value(*value),
            ValueData::String(value) => SourceErrorArgument::copy_value(value),
            ValueData::Group(value) => SourceErrorArgument::copy_value(value),
            ValueData::TimeLocation(_) | ValueData::TimeTime(_) => SourceErrorArgument::from_value(self.time()),
            ValueData::Kind(kind) => match *kind {
                SourceSlogKind::INT64 => SourceErrorArgument::from_value(self.num as i64),
                SourceSlogKind::UINT64 => SourceErrorArgument::from_value(self.num),
                SourceSlogKind::BOOL => SourceErrorArgument::from_value(self.num == 1),
                SourceSlogKind::FLOAT64 => SourceErrorArgument::from_value(f64::from_bits(self.num)),
                SourceSlogKind::DURATION => SourceErrorArgument::from_value(SourceSlogDuration(self.num as i64)),
                _ => bad_kind(b"bad kind: ", *kind),
            },
        }
    }
    fn require(&self, expected: SourceSlogKind) {
        if self.kind() != expected {
            let mut message = b"Value kind is ".to_vec(); message.extend_from_slice(self.kind().string());
            message.extend_from_slice(b", not "); message.extend_from_slice(expected.string()); raise_bytes(message);
        }
    }
    pub fn int64(&self) -> i64 { self.require(SourceSlogKind::INT64); self.num as i64 }
    pub fn uint64(&self) -> u64 { self.require(SourceSlogKind::UINT64); self.num }
    pub fn bool(&self) -> bool { self.require(SourceSlogKind::BOOL); self.num == 1 }
    pub fn float64(&self) -> f64 { self.require(SourceSlogKind::FLOAT64); f64::from_bits(self.num) }
    pub fn duration(&self) -> i64 { self.require(SourceSlogKind::DURATION); self.num as i64 }
    pub fn time(&self) -> SourceGoTime {
        self.require(SourceSlogKind::TIME);
        match &self.data {
            ValueData::TimeLocation(None) => SourceGoTime::default(),
            ValueData::TimeLocation(Some(location)) => SourceGoTime::unix_utc(0, self.num as i64).in_location(Some(Arc::clone(location))),
            ValueData::TimeTime(value) => value.source_fmt_copy(),
            _ => unreachable!("KindTime has an actual time representation"),
        }
    }
    pub fn group(&self) -> SourceSlogAttrs {
        match &self.data { ValueData::Group(value) => value.source_fmt_copy(), _ => raise_bytes(b"Group: bad kind".to_vec()) }
    }
    pub fn resolve(&self, formatter: &SourceKlogFormatter) -> FormatResult<Self> {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut value = self.source_fmt_copy();
            for _ in 0..100 {
                if value.kind() != SourceSlogKind::LOG_VALUER { return Ok(value); }
                let next = match &value.data {
                    ValueData::Any(arg) => arg.source_methods().klog.log_valuer.expect("KindLogValuer retains its method").source_log_value(),
                    _ => unreachable!("only an actual retained LogValuer has KindLogValuer"),
                };
                value = next;
            }
            // A terminal value returned by call 100 still reaches this error.
            let original = self.any();
            let mut message = b"LogValue called too many times on Value of type ".to_vec();
            original.try_write('T', &SourcePrintfResolvedCounts::default(), formatter.profile, &mut message)
                .ok_or_else(|| missing(&original, "LogValue original type"))??;
            error_value(message)
        }));
        match result {
            Ok(value) => value,
            Err(payload) => {
                let payload = match payload.downcast::<SourceKlogFormatError>() {
                    Ok(error) => return Err(*error), Err(payload) => payload,
                };
                drop(payload);
                // The source discards the recovered payload and obtains
                // stack(3,5). Missing native frames must not become a fake
                // '(no stack)' result or a differently shaped Rust trace.
                let capture = formatter.slog_stack.as_ref().ok_or_else(|| SourceKlogFormatError::Unbound {
                    source_type: None, operation: "slog Resolve runtime.Callers/Frames binding" })?;
                let mut message = b"LogValue panicked\n".to_vec();
                append_stack(&mut message, capture, 3, 5, formatter.profile)?;
                error_value(message)
            }
        }
    }
    pub fn try_string(&self, profile: SourceUnicodeProfile) -> FormatResult<Vec<u8>> {
        let mut output = Vec::new();
        match self.kind() {
            SourceSlogKind::STRING => append(&mut output, self.string_bytes().expect("string kind"))?,
            SourceSlogKind::INT64 => write_integer(&mut output, self.num, true, "int64", 'd', &SourcePrintfResolvedCounts::default(), profile)?,
            SourceSlogKind::UINT64 => write_integer(&mut output, self.num, false, "uint64", 'd', &SourcePrintfResolvedCounts::default(), profile)?,
            SourceSlogKind::FLOAT64 => append(&mut output, &try_source_float(f64::from_bits(self.num), SourceFloatWidth::Binary64, b'g', -1)?)?,
            SourceSlogKind::BOOL => append(&mut output, if self.num == 1 { b"true" } else { b"false" })?,
            SourceSlogKind::DURATION => append(&mut output, &duration_string(self.num as i64))?,
            SourceSlogKind::TIME => append(&mut output, &self.time().try_string()?)?,
            SourceSlogKind::GROUP | SourceSlogKind::ANY | SourceSlogKind::LOG_VALUER => {
                let value = self.any();
                value.try_write('v', &SourcePrintfResolvedCounts::default(), profile, &mut output)
                    .ok_or_else(|| missing(&value, "slog Value.String operand"))??;
            }
            kind => bad_kind(b"bad kind: ", kind),
        }
        Ok(output)
    }
    /// Interface equality is supplied by the declared type/comparability
    /// binding, including its source panic for a non-comparable dynamic type.
    /// Never substitute native Arc equality, Debug strings or PartialEq.
    pub fn try_equal_with(&self, other: &Self,
        interface_equal: &mut impl FnMut(&SourceErrorArgument, &SourceErrorArgument) -> FormatResult<bool>) -> FormatResult<bool>
    {
        let kind = self.kind();
        if kind != other.kind() { return Ok(false); }
        match kind {
            SourceSlogKind::INT64 | SourceSlogKind::UINT64 | SourceSlogKind::BOOL | SourceSlogKind::DURATION => Ok(self.num == other.num),
            SourceSlogKind::STRING => Ok(self.string_bytes() == other.string_bytes()),
            SourceSlogKind::FLOAT64 => Ok(f64::from_bits(self.num) == f64::from_bits(other.num)),
            SourceSlogKind::TIME => Ok(self.time().equal(&other.time())),
            SourceSlogKind::ANY | SourceSlogKind::LOG_VALUER => interface_equal(&self.any(), &other.any()),
            SourceSlogKind::GROUP => {
                let left = self.group_ref().expect("group kind"); let right = other.group_ref().expect("group kind");
                if left.len() != right.len() { return Ok(false); }
                for index in 0..left.len() {
                    if !left.get(index).try_equal_with(&right.get(index), interface_equal)? { return Ok(false); }
                }
                Ok(true)
            }
            kind => bad_kind(b"bad kind: ", kind),
        }
    }
}
fn missing(value: &SourceErrorArgument, operation: &'static str) -> SourceKlogFormatError {
    SourceKlogFormatError::Unbound { source_type: value.source_type().map(str::to_owned), operation }
}
fn raise_bytes(message: Vec<u8>) -> ! { crate::utility_format_methods::SourcePrintfPanic::raise(SourceFmtSharedString(Arc::from(message))) }
fn bad_kind(prefix: &[u8], kind: SourceSlogKind) -> ! {
    let mut message = prefix.to_vec(); message.extend_from_slice(kind.string()); raise_bytes(message)
}
fn error_value(message: Vec<u8>) -> FormatResult<SourceSlogValue> {
    let error = crate::errors::Errors::default().new_bytes(&message)?;
    let error = SourcePrintfError::canonical(error).unwrap_or_else(|_| unreachable!("new_bytes creates errors.errorString"));
    Ok(SourceSlogValue::any_value(SourceErrorArgument::from_value(error)))
}
impl SourceStringer for SourceSlogValue {
    fn source_string(&self) -> Cow<'_, [u8]> { Cow::Owned(finish(self.try_string(SourceUnicodeProfile::Go1265))) }
}

pub trait SourceSlogLogValuer { fn source_log_value(&self) -> SourceSlogValue; }
#[derive(Deftly)]
#[derive_deftly(SourceKlogAdapter)]
#[deftly(source_slog_log_valuer)]
pub struct SourceSlogLogValuerValue<Value, Function>
where Value: SourcePrintfValue, Function: Fn(&Value) -> SourceSlogValue,
{ pub value: Value, pub method: Function }
impl<Value, Function> SourceSlogLogValuer for SourceSlogLogValuerValue<Value, Function>
where Value: SourcePrintfValue, Function: Fn(&Value) -> SourceSlogValue,
{
    fn source_log_value(&self) -> SourceSlogValue { (self.method)(&self.value) }
}

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
#[deftly(source_stringer, source_klog_json_encoder)]
pub struct SourceSlogAttr { pub key: SourceFmtSharedString, pub value: SourceSlogValue }
impl Default for SourceSlogAttr {
    fn default() -> Self { Self { key: SourceFmtSharedString(Arc::from(&b""[..])), value: SourceSlogValue::default() } }
}
impl SourceFmtValueCopy for SourceSlogAttr {
    fn source_fmt_copy(&self) -> Self { Self { key: self.key.source_fmt_copy(), value: self.value.source_fmt_copy() } }
}
impl SourceSlogAttr {
    pub fn new(key: SourceFmtSharedString, value: SourceSlogValue) -> Self { Self { key, value } }
    pub fn group(key: SourceFmtSharedString, arguments: SourceErrorArguments) -> Self {
        Self { key, value: SourceSlogValue::group_value(args_to_attr_slice(arguments)) }
    }
    pub fn is_empty(&self) -> bool {
        self.key.0.is_empty() && self.value.num == 0
            && matches!(&self.value.data, ValueData::Any(value) if value.is_nil())
    }
    pub fn try_equal_with(&self, other: &Self,
        interface_equal: &mut impl FnMut(&SourceErrorArgument, &SourceErrorArgument) -> FormatResult<bool>) -> FormatResult<bool>
    {
        if self.key.0 != other.key.0 { return Ok(false); }
        self.value.try_equal_with(&other.value, interface_equal)
    }
}
impl SourceStringer for SourceSlogAttr {
    fn source_string(&self) -> Cow<'_, [u8]> {
        let mut output = Vec::new();
        complete_source_collection(&NativeSourceCollectionContext, append(&mut output, &self.key.0));
        complete_source_collection(&NativeSourceCollectionContext, append(&mut output, b"="));
        let value = finish(self.value.try_string(SourceUnicodeProfile::Go1265));
        complete_source_collection(&NativeSourceCollectionContext, append(&mut output, &value)); Cow::Owned(output)
    }
}
#[derive(Deftly)]
#[derive_deftly(SourceCallbackSlice, SourcePrintfInterfaces)]
#[deftly(source_slice_words = "5", source_slice_value = "SourceSlogAttr", source_klog_json_encoder)]
pub struct SourceSlogAttrs { slice: SourceCallbackSlice<SourceSlogAttr> }
impl Default for SourceSlogAttrs { fn default() -> Self { Self::nil_with_profile(allocation_profile()) } }

pub fn args_to_attr(arguments: SourceErrorArguments) -> (SourceSlogAttr, SourceErrorArguments) {
    let first = arguments.get(0); // the source requires a nonempty slice
    let tail = |skip| arguments.slice2(SourceErrorSliceIndex::unsigned(skip), SourceErrorSliceIndex::unsigned(arguments.len() as u64));
    if let Some(key) = crate::utility_klog_values::string_bytes(&first) {
        if arguments.len() == 1 {
            return (SourceSlogAttr::new(SourceFmtSharedString(Arc::from(&b"!BADKEY"[..])), SourceSlogValue::any_value(first)),
                SourceErrorArguments::nil_with_profile(arguments.allocation_profile()));
        }
        let key = if let Some(key) = first.downcast_ref::<SourceFmtSharedString>() { key.source_fmt_copy() }
            else { SourceFmtSharedString(Arc::from(key)) };
        return (SourceSlogAttr::new(key, SourceSlogValue::any_value(arguments.get(1))), tail(2));
    }
    if first.source_type().is_some() {
        if let Some(attr) = first.downcast_ref::<SourceSlogAttr>() { return (attr.source_fmt_copy(), tail(1)); }
    }
    (SourceSlogAttr::new(SourceFmtSharedString(Arc::from(&b"!BADKEY"[..])), SourceSlogValue::any_value(first)), tail(1))
}
pub fn args_to_attr_slice(mut arguments: SourceErrorArguments) -> SourceSlogAttrs {
    let mut attrs = SourceSlogAttrs::nil_with_profile(arguments.allocation_profile());
    while arguments.len() > 0 {
        let (attr, rest) = args_to_attr(arguments); attrs.append(attr); arguments = rest;
    }
    attrs
}

/// Frames are supplied by the existing CGP effect owner at the recovery
/// call site. Its callback receives Callers(skip+1) and the nFrames+1 slots.
/// This data interface does not claim native Rust frames are Go frames.
pub struct SourceSlogFrame { pub function: SourceFmtSharedString, pub file: SourceFmtSharedString, pub line: i64 }
pub struct SourceSlogCapturedFrames { pub pc_count: usize, pub frames: Vec<SourceSlogFrame> }
pub type SourceSlogStack = Arc<dyn Fn(i32, usize) -> FormatResult<SourceSlogCapturedFrames> + Send + Sync>;
fn append_stack(output: &mut Vec<u8>, capture: &SourceSlogStack, skip: i32, count: usize,
    profile: SourceUnicodeProfile) -> FormatResult
{
    let captured = capture(skip + 1, count + 1)?;
    if captured.pc_count > count + 1 { return Err(SourceKlogFormatError::Unbound {
        source_type: None, operation: "slog runtime.Callers returned PC count" }); }
    if captured.pc_count == 0 { append(output, b"(no stack)")?; return Ok(()); }
    // CallersFrames expands inlined calls; logical frames can outnumber the
    // captured program counters. Limit output using Next's more condition,
    // represented by the remaining frames, rather than the PC slice length.
    let frames = captured.frames;
    // Frames.Next may return a zero Frame even after Callers returned PCs.
    if frames.is_empty() { append(output, b"called from  (:0)\n")?; return Ok(()); }
    for (index, frame) in frames.iter().enumerate() {
        append(output, b"called from ")?; append(output, &frame.function.0)?;
        append(output, b" (")?; append(output, &frame.file.0)?; append(output, b":")?;
        write_integer(output, frame.line as u64, true, "int", 'd', &SourcePrintfResolvedCounts::default(), profile)?;
        append(output, b")\n")?;
        if index + 1 == frames.len() { break; }
        if index + 1 >= count { append(output, b"(rest of stack elided)\n")?; break; }
    }
    Ok(())
}

// time.Duration.String writes into a 32-byte backwards buffer, including
// wrapping negation of MinInt64 and trimming fraction zeros before units.
fn duration_string(value: i64) -> Vec<u8> {
    let mut buffer = [0_u8; 32]; let mut end = buffer.len();
    let mut magnitude = value as u64;
    if value < 0 { magnitude = magnitude.wrapping_neg(); }
    end -= 1; buffer[end] = b's';
    if magnitude < 1_000_000_000 {
        end -= 1;
        let precision;
        if magnitude == 0 { buffer[end] = b'0'; return buffer[end..].to_vec(); }
        else if magnitude < 1_000 { precision = 0; buffer[end] = b'n'; }
        else if magnitude < 1_000_000 { precision = 3; end -= 1; buffer[end..end + 2].copy_from_slice("µ".as_bytes()); }
        else { precision = 6; buffer[end] = b'm'; }
        (end, magnitude) = duration_fraction(&mut buffer, end, magnitude, precision);
        end = duration_integer(&mut buffer, end, magnitude);
    } else {
        (end, magnitude) = duration_fraction(&mut buffer, end, magnitude, 9);
        end = duration_integer(&mut buffer, end, magnitude % 60); magnitude /= 60;
        if magnitude > 0 {
            end -= 1; buffer[end] = b'm'; end = duration_integer(&mut buffer, end, magnitude % 60); magnitude /= 60;
            if magnitude > 0 { end -= 1; buffer[end] = b'h'; end = duration_integer(&mut buffer, end, magnitude); }
        }
    }
    if value < 0 { end -= 1; buffer[end] = b'-'; }
    buffer[end..].to_vec()
}
fn duration_fraction(buffer: &mut [u8; 32], mut end: usize, mut value: u64, precision: u32) -> (usize, u64) {
    let mut print = false;
    for _ in 0..precision {
        let digit = value % 10; print |= digit != 0;
        if print { end -= 1; buffer[end] = digit as u8 + b'0'; } value /= 10;
    }
    if print { end -= 1; buffer[end] = b'.'; } (end, value)
}
fn duration_integer(buffer: &mut [u8; 32], mut end: usize, mut value: u64) -> usize {
    if value == 0 { end -= 1; buffer[end] = b'0'; }
    while value > 0 { end -= 1; buffer[end] = (value % 10) as u8 + b'0'; value /= 10; } end
}

impl SourcePrintfValue for SourceSlogValue {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { "slog.Value" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl SourcePrintfStructFields for SourceSlogValue {
    fn source_field_count(&self) -> usize { 2 }
    fn source_field_name(&self, index: usize) -> &str { if index == 0 { "num" } else { "any" } }
    fn source_field_interfaceable(&self, _: usize) -> bool { false }
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if index == 0 { return write_source_value(&self.num, verb, state, profile, mode, recursion, output); }
        match &self.data {
            ValueData::Kind(kind) => write_source_value(kind, verb, state, profile, mode, recursion.child(true), output),
            ValueData::KindValue(kind) => write_integer(output, kind.0 as u64, true, "slog.kind", verb, state, profile),
            ValueData::String(value) => write_source_value(&SourcePrintfAddress {
                address: value.0.as_ptr() as usize as u64, source_type: "slog.stringptr", kind: SourceAddressKind::Pointer,
            }, verb, state, profile, mode, recursion.child(true), output),
            ValueData::Group(value) => write_source_value(&SourcePrintfAddress {
                address: value.slice.native_backing_address(), source_type: "slog.groupptr", kind: SourceAddressKind::Pointer,
            }, verb, state, profile, mode, recursion.child(true), output),
            ValueData::Any(value) => match value.try_write_recursive(verb, state, profile, mode, recursion.child(true), output) {
                Some(result) => result,
                None => std::panic::panic_any(missing(value, "slog.Value private any reflection binding")),
            },
            ValueData::TimeLocation(location) => write_source_value(&SourcePrintfAddress {
                address: location.as_ref().map_or(0, |value| Arc::as_ptr(value) as usize as u64),
                source_type: "slog.timeLocation", kind: SourceAddressKind::Pointer,
            }, verb, state, profile, mode, recursion.child(true), output),
            ValueData::TimeTime(value) => write_source_value(&SlogTimeTime(value), verb, state, profile, mode, recursion.child(true), output),
        }
    }
}
impl SourcePrintfValue for SourceSlogAttr {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { "slog.Attr" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl SourcePrintfStructFields for SourceSlogAttr {
    fn source_field_count(&self) -> usize { 2 }
    fn source_field_name(&self, index: usize) -> &str { if index == 0 { "Key" } else { "Value" } }
    fn source_field_interfaceable(&self, _: usize) -> bool { true }
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if index == 0 { write_source_value(&self.key, verb, state, profile, mode, recursion, output) }
        else { write_source_value(&self.value, verb, state, profile, mode, recursion, output) }
    }
}
impl SourcePrintfValue for SourceSlogAttrs {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { "[]slog.Attr" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if recursion.depth == 0 && verb == 'p' {
            return write_source_address(self, self.slice.native_backing_address(), verb, state, profile, mode, recursion, output);
        }
        if state.flags.sharp_v {
            append(output, b"[]slog.Attr")?;
            if self.is_nil() { return append(output, b"(nil)"); }
        }
        append(output, if state.flags.sharp_v { b"{" } else { b"[" })?;
        for index in 0..self.len() {
            if index != 0 { append(output, if state.flags.sharp_v { b", " } else { b" " })?; }
            // Copy the visited element/header immediately before its methods,
            // with no backing lock held while user formatting can re-enter.
            let attr = self.get(index);
            write_source_value(&attr, verb, state, profile, mode, recursion.child(true), output)?;
        }
        append(output, if state.flags.sharp_v { b"}" } else { b"]" })
    }
}
// encoding/json sees only exported fields: slog.Value is {}, while Attr is
// {"Key":...,"Value":{}}. This is intentionally distinct from klog's own
// generateJSON group-object conversion and ignores formatting methods.
impl SourceKlogJsonEncoder for SourceSlogValue {
    fn source_encode_json(&self, output: &mut Vec<u8>, _: SourceUnicodeProfile) -> Result<(), SourceKlogEncodeError>
    { append(output, b"{}\n")?; Ok(()) }
}
fn encode_attr(output: &mut Vec<u8>, attr: &SourceSlogAttr) -> Result<(), SourceSliceAllocationError> {
    append(output, b"{\"Key\":")?;
    crate::utility_http_headers::append_json_string(output, &attr.key.0)?;
    append(output, b",\"Value\":{}}")
}
impl SourceKlogJsonEncoder for SourceSlogAttr {
    fn source_encode_json(&self, output: &mut Vec<u8>, _: SourceUnicodeProfile) -> Result<(), SourceKlogEncodeError>
    { encode_attr(output, self)?; append(output, b"\n")?; Ok(()) }
}
impl SourceKlogJsonEncoder for SourceSlogAttrs {
    fn source_encode_json(&self, output: &mut Vec<u8>, _: SourceUnicodeProfile) -> Result<(), SourceKlogEncodeError> {
        if self.is_nil() { append(output, b"null\n")?; return Ok(()); }
        append(output, b"[")?;
        for index in 0..self.len() {
            if index != 0 { append(output, b",")?; }
            encode_attr(output, &self.get(index))?;
        }
        append(output, b"]\n")?; Ok(())
    }
}
macro_rules! scalar_json {
    ($($ty:ty),+) => { $(impl SourceKlogJsonEncoder for $ty {
        fn source_encode_json(&self, output: &mut Vec<u8>, profile: SourceUnicodeProfile) -> Result<(), SourceKlogEncodeError> {
            write_integer(output, self.0 as u64, true, self.source_type(), 'd', &SourcePrintfResolvedCounts::default(), profile)?;
            append(output, b"\n")?; Ok(())
        }
    })+ };
}
scalar_json!(SourceSlogKind, SourceSlogDuration);

// slog.timeTime is a distinct defined type with time.Time's underlying
// fields and no inherited Time.String/GoString/MarshalJSON method set.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
struct SlogTimeTime<'a>(&'a SourceGoTime);
impl SourcePrintfValue for SlogTimeTime<'_> {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { "slog.timeTime" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl SourcePrintfStructFields for SlogTimeTime<'_> {
    fn source_field_count(&self) -> usize { self.0.source_field_count() }
    fn source_field_name(&self, index: usize) -> &str { self.0.source_field_name(index) }
    fn source_field_interfaceable(&self, _: usize) -> bool { false }
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { self.0.source_write_field(index, verb, state, profile, mode, recursion, output) }
}
