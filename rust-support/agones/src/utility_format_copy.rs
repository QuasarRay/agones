//! Explicit scalar, aggregate and reference-header copies for map snapshots.
// Adapted from Go reflect/{map,value}.go and fmt/print.go. Copyright The Go
// Authors. BSD-3-Clause; see ../GO-LICENSE. Native allocation/ABI/GC profiles
// remain separate. This is typed data copying, not an erased Go object model.
use std::{cmp::Ordering, sync::Arc};
use derive_deftly::Deftly;
use crate::{
    utility_declarations::{derive_deftly_template_SourcePrintfInterfaces, derive_deftly_template_SourceFmtValueCopy},
    utility_format_arguments::{SourcePrintfValue, SourcePrintfString, SourcePrintfNil},
    utility_format_collections::{SourcePrintfArray, SourcePrintfSlice, SourcePrintfStruct},
    utility_format_pointer::{SourcePrintfAddress, SourcePrintfPointer, SourceAddressKind},
    utility_format_map::{SourceFmtMapKey, SourceFmtInterfaceKey, SourceFmtComplexKey},
    utility_format_adapters::{SourcePrintfFormatter, SourcePrintfGoStringer, SourcePrintfStringer},
    utility_format_methods::SourcePrintfRecursion,
    utility_format_print::SourcePrintfMode,
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_text::write_text,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

/// A source map value copy freezes scalar/aggregate data while preserving
/// declared reference aliases. Implementations must not run value methods.
/// No blanket Clone or borrowed-T fallback can deep-copy a slice/map/error
/// or retain an alias to a map slot that the source would have copied.
pub trait SourceFmtValueCopy: Sized { fn source_fmt_copy(&self) -> Self; }
macro_rules! scalar_copies {
    ($($ty:ty),+ $(,)?) => { $(impl SourceFmtValueCopy for $ty {
        fn source_fmt_copy(&self) -> Self { *self }
    })+ };
}
scalar_copies!(bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, SourceAddressKind);
impl SourceFmtValueCopy for &str { fn source_fmt_copy(&self) -> Self { *self } }
impl SourceFmtValueCopy for SourcePrintfString<'_> { fn source_fmt_copy(&self) -> Self { Self(self.0) } }
impl SourceFmtValueCopy for SourcePrintfNil { fn source_fmt_copy(&self) -> Self { Self } }
impl<Value: SourceFmtValueCopy> SourceFmtValueCopy for Option<Value> {
    fn source_fmt_copy(&self) -> Self { self.as_ref().map(SourceFmtValueCopy::source_fmt_copy) }
}
impl<Value: SourceFmtValueCopy, const LENGTH: usize> SourceFmtValueCopy for [Value; LENGTH] {
    fn source_fmt_copy(&self) -> Self { std::array::from_fn(|index| self[index].source_fmt_copy()) }
}
impl SourceFmtValueCopy for SourcePrintfAddress<'_> {
    fn source_fmt_copy(&self) -> Self { Self { address: self.address, source_type: self.source_type, kind: self.kind } }
}
impl<Value> SourceFmtValueCopy for SourcePrintfPointer<'_, Value> {
    fn source_fmt_copy(&self) -> Self { Self { target: self.target, source_type: self.source_type } }
}
impl<Value> SourceFmtValueCopy for SourcePrintfSlice<'_, Value> {
    fn source_fmt_copy(&self) -> Self { Self { values: self.values, source_type: self.source_type } }
}
impl<Value: SourceFmtValueCopy> SourceFmtValueCopy for SourcePrintfStruct<Value> {
    fn source_fmt_copy(&self) -> Self { Self(self.0.source_fmt_copy()) }
}
impl<Value: SourceFmtValueCopy> SourceFmtValueCopy for SourceFmtInterfaceKey<'_, Value> {
    fn source_fmt_copy(&self) -> Self { Self { value: self.value.source_fmt_copy(),
        source_type_token: self.source_type_token, declared_interface_type: self.declared_interface_type } }
}
impl<Real: SourceFmtValueCopy> SourceFmtValueCopy for SourceFmtComplexKey<Real> {
    fn source_fmt_copy(&self) -> Self { Self { real: self.real.source_fmt_copy(), imaginary: self.imaginary.source_fmt_copy() } }
}
impl<Value: SourceFmtValueCopy, Function: Copy> SourceFmtValueCopy for SourcePrintfFormatter<Value, Function> {
    fn source_fmt_copy(&self) -> Self { Self { value: self.value.source_fmt_copy(), format: self.format } }
}
macro_rules! string_method_copies {
    ($($adapter:ident),+ $(,)?) => { $(impl<Value: SourceFmtValueCopy, Function: Copy> SourceFmtValueCopy for $adapter<Value, Function> {
        fn source_fmt_copy(&self) -> Self { Self { value: self.value.source_fmt_copy(), method: self.method } }
    })+ };
}
string_method_copies!(SourcePrintfGoStringer, SourcePrintfStringer);

/// Immutable string backing can survive replacement/deletion of its map slot.
/// A source copy shares bytes rather than cloning a native String allocation.
/// Arc hosting itself is not source header/GC/allocation identity evidence.
#[derive(PartialEq, Eq, Hash, Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
pub struct SourceFmtSharedString(pub Arc<[u8]>);
impl SourceFmtValueCopy for SourceFmtSharedString {
    fn source_fmt_copy(&self) -> Self { Self(Arc::clone(&self.0)) }
}
impl SourceFmtMapKey for SourceFmtSharedString {
    fn source_key_cmp(&self, other: &Self) -> Ordering { self.0.as_ref().cmp(other.0.as_ref()) }
}
impl SourcePrintfValue for SourceFmtSharedString {
    fn source_type(&self) -> &str { "string" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_text(output, &self.0, verb, state, profile) }
}

/// MapRange array values own the copied array and are non-addressable.
/// Borrowed SourcePrintfArray remains a separate addressability projection;
/// copying that borrowed view is deliberately not a map value copy.
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces, SourceFmtValueCopy)]
pub struct SourceFmtOwnedArray<'a, Value, const LENGTH: usize> {
    pub values: [Value; LENGTH],
    pub source_type: &'a str,
}
impl<Value: SourcePrintfValue, const LENGTH: usize> SourcePrintfValue for SourceFmtOwnedArray<'_, Value, LENGTH> {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { self.source_type }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { SourcePrintfArray { values: &self.values, source_type: self.source_type, source_addressable: false }
        .write_source(verb, state, profile, mode, recursion, output) }
}
impl<Value: SourceFmtMapKey, const LENGTH: usize> SourceFmtMapKey for SourceFmtOwnedArray<'_, Value, LENGTH> {
    fn source_key_cmp(&self, other: &Self) -> Ordering { self.values.source_key_cmp(&other.values) }
}

impl SourceFmtValueCopy for () { fn source_fmt_copy(&self) -> Self { () } }
macro_rules! tuple_copies {
    ($( $index:tt => $ty:ident ),+ $(,)?) => {
        impl<$($ty: SourceFmtValueCopy),+> SourceFmtValueCopy for ($($ty,)+) {
            fn source_fmt_copy(&self) -> Self { ($(self.$index.source_fmt_copy(),)+) }
        }
    };
}
tuple_copies!(0 => A);
tuple_copies!(0 => A, 1 => B);
tuple_copies!(0 => A, 1 => B, 2 => C);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J, 10 => K);
tuple_copies!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J, 10 => K, 11 => L);
