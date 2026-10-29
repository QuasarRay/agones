//! Typed map-key ordering and source map rendering over captured native pairs.
// Adapted from Go internal/fmtsort/sort.go, cmp/cmp.go and fmt/print.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use std::cmp::Ordering;
use derive_deftly::Deftly;
use crate::{
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces,
    utility_format_arguments::{SourcePrintfValue, SourcePrintfString},
    utility_format_methods::{SourcePrintfRecursion, write_source_value},
    utility_format_pointer::{SourcePrintfAddress, SourceAddressKind, SourcePrintfPointer, write_source_address},
    utility_format_collections::{SourcePrintfArray, SourcePrintfStruct},
    utility_format_adapters::{SourcePrintfFormatter, SourcePrintfGoStringer, SourcePrintfStringer, SourcePrintfOwnedError},
    utility_format_methods::{SourcePrintfMethodViews, SourcePrintfInterfaces},
    utility_format_scan::SourcePrintfInteger,
    utility_format_print::SourcePrintfMode,
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_sort::source_stable_sort,
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError, reserve_source_array},
};

/// Comparison is source data access only: no Error/Stringer/Formatter calls.
/// Every supplied map has one coherent Go key type. Arbitrary interface/type
/// identity and source pointer/word bindings still need their native profile.
pub trait SourceFmtMapKey { fn source_key_cmp(&self, other: &Self) -> Ordering; }
macro_rules! ordered_keys {
    ($($ty:ty),+ $(,)?) => { $(impl SourceFmtMapKey for $ty {
        fn source_key_cmp(&self, other: &Self) -> Ordering { self.cmp(other) }
    })+ };
}
ordered_keys!(bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, String);
impl SourceFmtMapKey for str { fn source_key_cmp(&self, other: &Self) -> Ordering { self.as_bytes().cmp(other.as_bytes()) } }
impl SourceFmtMapKey for SourcePrintfString<'_> { fn source_key_cmp(&self, other: &Self) -> Ordering { self.0.cmp(other.0) } }
macro_rules! float_keys {
    ($($ty:ty),+ $(,)?) => { $(impl SourceFmtMapKey for $ty {
        fn source_key_cmp(&self, other: &Self) -> Ordering {
            if self.is_nan() { return if other.is_nan() { Ordering::Equal } else { Ordering::Less }; }
            if other.is_nan() { return Ordering::Greater; }
            if self < other { Ordering::Less } else if self > other { Ordering::Greater } else { Ordering::Equal }
        }
    })+ };
}
float_keys!(f32, f64);
impl<Key: SourceFmtMapKey, const LENGTH: usize> SourceFmtMapKey for [Key; LENGTH] {
    fn source_key_cmp(&self, other: &Self) -> Ordering {
        for (left, right) in self.iter().zip(other) {
            let order = left.source_key_cmp(right); if order != Ordering::Equal { return order; }
        }
        Ordering::Equal
    }
}
impl<Key: SourceFmtMapKey, const LENGTH: usize> SourceFmtMapKey for SourcePrintfArray<'_, Key, LENGTH> {
    fn source_key_cmp(&self, other: &Self) -> Ordering { self.values.source_key_cmp(other.values) }
}
impl SourceFmtMapKey for SourcePrintfAddress<'_> {
    fn source_key_cmp(&self, other: &Self) -> Ordering {
        match self.kind {
            SourceAddressKind::Pointer | SourceAddressKind::UnsafePointer | SourceAddressKind::Channel => self.address.cmp(&other.address),
            _ => panic!("bad type in source map-key comparison: {}", self.source_type),
        }
    }
}
impl<Value> SourceFmtMapKey for SourcePrintfPointer<'_, Value> {
    fn source_key_cmp(&self, other: &Self) -> Ordering {
        let address = |value: Option<&Value>| value.map_or(0, |value| value as *const Value as usize as u64);
        address(self.target).cmp(&address(other.target))
    }
}
impl<Value: SourceFmtMapKey + ?Sized> SourceFmtMapKey for &Value {
    fn source_key_cmp(&self, other: &Self) -> Ordering { <Value as SourceFmtMapKey>::source_key_cmp(*self, *other) }
}
impl<Value: SourceFmtMapKey> SourceFmtMapKey for SourcePrintfStruct<Value> {
    fn source_key_cmp(&self, other: &Self) -> Ordering { self.0.source_key_cmp(&other.0) }
}
macro_rules! method_keys {
    ($($adapter:ident),+ $(,)?) => { $(
        impl<Value: SourceFmtMapKey, Function> SourceFmtMapKey for $adapter<Value, Function> {
            fn source_key_cmp(&self, other: &Self) -> Ordering { self.value.source_key_cmp(&other.value) }
        }
    )+ };
}
method_keys!(SourcePrintfFormatter, SourcePrintfGoStringer, SourcePrintfStringer);
impl<Value: SourceFmtMapKey> SourceFmtMapKey for SourcePrintfOwnedError<Value> {
    fn source_key_cmp(&self, other: &Self) -> Ordering { self.original().source_key_cmp(other.original()) }
}

/// Source complex-key comparison is independent of numeric text rendering.
pub struct SourceFmtComplexKey<Real> { pub real: Real, pub imaginary: Real }
impl<Real: SourceFmtMapKey> SourceFmtMapKey for SourceFmtComplexKey<Real> {
    fn source_key_cmp(&self, other: &Self) -> Ordering {
        let real = self.real.source_key_cmp(&other.real);
        if real != Ordering::Equal { real } else { self.imaginary.source_key_cmp(&other.imaginary) }
    }
}

/// The identity token is the source reflect.Type object-address projection,
/// never a name/hash-derived ordering. Actual source/native type-address and
/// word binding remains required. None is a nil declared interface value.
pub struct SourceFmtInterfaceKey<'a, Value> {
    pub value: Option<Value>,
    pub source_type_token: u64,
    pub declared_interface_type: &'a str,
}
impl<Value: SourceFmtMapKey> SourceFmtMapKey for SourceFmtInterfaceKey<'_, Value> {
    fn source_key_cmp(&self, other: &Self) -> Ordering {
        match (&self.value, &other.value) {
            (None, None) => Ordering::Equal, (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(left), Some(right)) => {
                let types = self.source_type_token.cmp(&other.source_type_token);
                if types != Ordering::Equal { types } else { left.source_key_cmp(right) }
            }
        }
    }
}
impl<Value: SourcePrintfValue> SourcePrintfMethodViews for SourceFmtInterfaceKey<'_, Value> {
    fn source_methods(&self) -> SourcePrintfInterfaces<'_> {
        self.value.as_ref().map_or_else(SourcePrintfInterfaces::default, |value| value.source_methods())
    }
}
impl<Value: SourcePrintfValue> SourcePrintfValue for SourceFmtInterfaceKey<'_, Value> {
    fn source_type(&self) -> &str { self.value.as_ref().map_or("", |value| value.source_type()) }
    fn source_integer(&self) -> SourcePrintfInteger {
        self.value.as_ref().map_or(SourcePrintfInteger::Other, |value| value.source_integer())
    }
    fn source_is_nil(&self) -> bool { self.value.is_none() }
    fn source_nil_interface_type(&self) -> &str { self.declared_interface_type }
    fn source_is_nil_pointer(&self) -> bool { self.value.as_ref().is_some_and(|value| value.source_is_nil_pointer()) }
    fn source_error_view(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        self.value.as_ref().and_then(|value| value.source_error_view())
    }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        match &self.value {
            Some(value) if recursion.depth == 0 => value.write_source(verb, state, profile, mode, recursion, output),
            Some(value) => write_source_value(value, verb, state, profile, mode, recursion.child(true), output),
            None => crate::utility_format_methods::write_source_value(self, verb, state, profile, mode, recursion, output),
        }
    }
    fn into_source_error(self) -> Option<crate::errors::BoxError> { self.value.and_then(|value| value.into_source_error()) }
}

macro_rules! tuple_keys {
    ($( $index:tt => $ty:ident ),+ $(,)?) => {
        impl<$($ty: SourceFmtMapKey),+> SourceFmtMapKey for ($($ty,)+) {
            fn source_key_cmp(&self, other: &Self) -> Ordering {
                $(let order = self.$index.source_key_cmp(&other.$index); if order != Ordering::Equal { return order; })+
                Ordering::Equal
            }
        }
    };
}
impl SourceFmtMapKey for () { fn source_key_cmp(&self, _: &Self) -> Ordering { Ordering::Equal } }
tuple_keys!(0 => A);
tuple_keys!(0 => A, 1 => B);
tuple_keys!(0 => A, 1 => B, 2 => C);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J, 10 => K);
tuple_keys!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J, 10 => K, 11 => L);

/// Entries are captured key/value pairs in source iteration order, including
/// distinct NaN-key entries. They remain paired while sorting; no key lookup
/// is performed during output. The original map/storage owner is external.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourcePrintfMap<'a, Key, Value> {
    pub entries: Option<&'a [(Key, Value)]>,
    pub source_type: &'a str,
    pub native_address: u64,
}
impl<Key: SourceFmtMapKey + SourcePrintfValue, Value: SourcePrintfValue> SourcePrintfValue for SourcePrintfMap<'_, Key, Value> {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { self.source_type }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        if begin_source_map(self, self.entries.is_none(), self.native_address,
            verb, state, profile, mode, recursion, output)? { return Ok(()); }
        let entries = self.entries.unwrap_or(&[]);
        let mut order = reserve_source_array::<usize>(entries.len() as u64)?;
        order.extend(0..entries.len());
        // Stable ties retain the captured iteration order, including all NaNs
        // and equal signed zeros. Source scratch/allocation schedules remain.
        source_stable_sort(&mut order, |&left, &right| entries[left].0.source_key_cmp(&entries[right].0));
        write_source_map_pairs(order.into_iter().map(|index| (&entries[index].0, &entries[index].1)),
            verb, state, profile, mode, recursion, output)
    }
}

/// Framing precedes fmtsort capture in the source. A pointer result or sharp
/// nil result finishes before a range or any key/value copy is requested.
pub(crate) fn begin_source_map<Value: SourcePrintfValue>(value: &Value, nil: bool, address: u64,
    verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
    mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<bool, SourceSliceAllocationError>
{
    if recursion.depth == 0 && verb == 'p' {
        write_source_address(value, if nil { 0 } else { address }, verb, state, profile, mode, recursion, output)?;
        return Ok(true);
    }
    if state.flags.sharp_v {
        append(output, value.source_type().as_bytes())?;
        if nil { append(output, b"(nil)")?; return Ok(true); }
        append(output, b"{")?;
    } else { append(output, b"map[")?; }
    Ok(false)
}

pub(crate) fn write_source_map_pairs<'a, Key: SourcePrintfValue + 'a, Value: SourcePrintfValue + 'a>(
    entries: impl IntoIterator<Item = (&'a Key, &'a Value)>, verb: char,
    state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile, mode: SourcePrintfMode,
    recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
{
    for (position, (key, value)) in entries.into_iter().enumerate() {
        if position != 0 { append(output, if state.flags.sharp_v { b", " } else { b" " })?; }
        write_source_value(key, verb, state, profile, mode, recursion.child(true), output)?;
        append(output, b":")?;
        write_source_value(value, verb, state, profile, mode, recursion.child(true), output)?;
    }
    append(output, if state.flags.sharp_v { b"}" } else { b"]" })
}
