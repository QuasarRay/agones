//! Source time.Time wall/monotonic values and explicit location ownership.
// Adapted from Go1.26.5 time/time.go, zoneinfo.go and log/slog/value.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
// Native timezone loading/local initialization, full word/layout/GC/ABI and
// runtime clock bindings remain separate. No location/provider registry.
use std::{borrow::Cow, sync::{Arc, OnceLock}};
use derive_deftly::Deftly;
use crate::{utility_format_copy::{SourceFmtSharedString, SourceFmtValueCopy},
    utility_format_arguments::SourcePrintfValue,
    utility_format_methods::{SourceStringer, SourceGoStringer, SourcePrintfRecursion, write_source_value},
    utility_format_collections::{SourcePrintfStructFields, write_source_struct},
    utility_format_pointer::{SourcePrintfAddress, SourceAddressKind},
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_print::SourcePrintfMode,
    utility_http::{HeaderAllocationProfile, SourceUnicodeProfile, SourceSliceAllocationError, NativeSourceCollectionContext, complete_source_collection},
    utility_klog_dispatch::{SourceKlogFormatError, FormatResult},
    utility_declarations::derive_deftly_template_SourcePrintfInterfaces};

pub(crate) const HAS_MONOTONIC: u64 = 1 << 63;
const NSEC_MASK: u64 = (1 << 30) - 1;
const NSEC_SHIFT: u32 = 30;
pub(crate) const UNIX_TO_INTERNAL: i64 = (1969 * 365 + 1969 / 4 - 1969 / 100 + 1969 / 400) * 86400;
pub(crate) const WALL_TO_INTERNAL: i64 = (1884 * 365 + 1884 / 4 - 1884 / 100 + 1884 / 400) * 86400;
pub(crate) const ABSOLUTE_YEARS: u64 = 292277022400;
pub(crate) const INTERNAL_TO_ABSOLUTE: i64 = ((ABSOLUTE_YEARS as i128 * 146097 / 400 + 306) * 86400) as i64;

pub struct SourceTimeZone { pub name: SourceFmtSharedString, pub offset: i64 }
pub type SourceTimeZoneLookup = Arc<dyn Fn(i64) -> FormatResult<SourceTimeZone> + Send + Sync>;
pub type SourceTimeLocationName = Arc<dyn Fn() -> FormatResult<SourceFmtSharedString> + Send + Sync>;
pub type SourceTimeLocalIdentity = Arc<dyn Fn() -> bool + Send + Sync>;
enum LocationData {
    Utc,
    Fixed { name: SourceFmtSharedString, offset: i64 },
    Bound { name: SourceTimeLocationName, lookup: SourceTimeZoneLookup, is_current_local: SourceTimeLocalIdentity },
}
pub struct SourceTimeLocation { data: LocationData }
impl SourceTimeLocation {
    pub fn utc() -> Arc<Self> {
        static UTC: OnceLock<Arc<SourceTimeLocation>> = OnceLock::new();
        Arc::clone(UTC.get_or_init(|| Arc::new(Self { data: LocationData::Utc })))
    }
    /// This is the source's specific FixedZone cache, never a general
    /// name-to-type/provider/location loader. Named zones remain distinct.
    pub fn fixed_zone(name: SourceFmtSharedString, offset: i64, profile: HeaderAllocationProfile) -> Arc<Self> {
        let offset = if profile.word_bits() == 32 { i64::from(offset as i32) } else { offset };
        let hour = offset / 60 / 60;
        if name.0.is_empty() && (-12..=14).contains(&hour) && hour * 3600 == offset {
            static UNNAMED: OnceLock<Box<[Arc<SourceTimeLocation>]>> = OnceLock::new();
            let values = UNNAMED.get_or_init(|| (-12..=14).map(|hour| Arc::new(Self {
                data: LocationData::Fixed { name: SourceFmtSharedString(Arc::from(&b""[..])), offset: hour * 3600 },
            })).collect());
            return Arc::clone(&values[(hour + 12) as usize]);
        }
        Arc::new(Self { data: LocationData::Fixed { name, offset } })
    }
    /// Concrete native/source location effects belong to the existing CGP
    /// timezone owner. These callbacks retain that actual compiled location;
    /// no guessed zone name, offset, UTC fallback or new manager is inserted.
    pub fn bound(name: SourceTimeLocationName, lookup: SourceTimeZoneLookup,
        is_current_local: SourceTimeLocalIdentity) -> Arc<Self>
    { Arc::new(Self { data: LocationData::Bound { name, lookup, is_current_local } }) }
    pub fn name(&self) -> FormatResult<SourceFmtSharedString> {
        match &self.data { LocationData::Utc => Ok(SourceFmtSharedString(Arc::from(&b"UTC"[..]))),
            LocationData::Fixed { name, .. } => Ok(name.source_fmt_copy()), LocationData::Bound { name, .. } => name() }
    }
    pub fn lookup(&self, unix: i64) -> FormatResult<SourceTimeZone> {
        match &self.data {
            LocationData::Utc => Ok(SourceTimeZone { name: self.name()?, offset: 0 }),
            LocationData::Fixed { name, offset } => Ok(SourceTimeZone { name: name.source_fmt_copy(), offset: *offset }),
            LocationData::Bound { lookup, .. } => lookup(unix),
        }
    }
    pub(crate) fn is_current_local(&self) -> bool {
        match &self.data { LocationData::Bound { is_current_local, .. } => is_current_local(), _ => false }
    }
}

#[derive(Default, Deftly)]
#[derive_deftly(SourcePrintfInterfaces)]
#[deftly(source_stringer, source_go_stringer, source_klog_json_encoder)]
pub struct SourceGoTime { pub(crate) wall: u64, pub(crate) ext: i64, pub(crate) loc: Option<Arc<SourceTimeLocation>> }
impl SourceFmtValueCopy for SourceGoTime {
    fn source_fmt_copy(&self) -> Self { Self { wall: self.wall, ext: self.ext, loc: self.loc.as_ref().map(Arc::clone) } }
}
impl SourceGoTime {
    /// Exact field bits supplied by the concrete source/native declaration.
    /// This does not assert that Rust fields have the Go ABI or GC layout.
    pub fn from_parts(wall: u64, ext: i64, location: Option<Arc<SourceTimeLocation>>) -> Self { Self { wall, ext, loc: location } }
    pub fn nanosecond(&self) -> i32 { (self.wall & NSEC_MASK) as i32 }
    pub fn has_monotonic(&self) -> bool { self.wall & HAS_MONOTONIC != 0 }
    pub fn seconds(&self) -> i64 {
        if self.has_monotonic() { WALL_TO_INTERNAL.wrapping_add((self.wall << 1 >> (NSEC_SHIFT + 1)) as i64) } else { self.ext }
    }
    pub fn unix(&self) -> i64 { self.seconds().wrapping_sub(UNIX_TO_INTERNAL) }
    pub fn unix_milli(&self) -> i64 { self.unix().wrapping_mul(1000).wrapping_add(i64::from(self.nanosecond()) / 1_000_000) }
    pub fn unix_micro(&self) -> i64 { self.unix().wrapping_mul(1_000_000).wrapping_add(i64::from(self.nanosecond()) / 1000) }
    pub fn unix_nano(&self) -> i64 { self.unix().wrapping_mul(1_000_000_000).wrapping_add(i64::from(self.nanosecond())) }
    pub fn is_zero(&self) -> bool { self.wall == 0 && self.ext == 0 }
    pub fn equal(&self, other: &Self) -> bool {
        if self.wall & other.wall & HAS_MONOTONIC != 0 { self.ext == other.ext }
        else { self.seconds() == other.seconds() && self.nanosecond() == other.nanosecond() }
    }
    pub fn compare(&self, other: &Self) -> i32 {
        let (left, right) = if self.wall & other.wall & HAS_MONOTONIC != 0 { (self.ext, other.ext) }
            else if self.seconds() == other.seconds() { (i64::from(self.nanosecond()), i64::from(other.nanosecond())) }
            else { (self.seconds(), other.seconds()) };
        if left < right { -1 } else if left > right { 1 } else { 0 }
    }
    pub fn before(&self, other: &Self) -> bool { self.compare(other) < 0 }
    pub fn after(&self, other: &Self) -> bool { self.compare(other) > 0 }
    pub fn strip_monotonic(&mut self) {
        if self.has_monotonic() { self.ext = self.seconds(); self.wall &= NSEC_MASK; }
    }
    /// The exact Round(d<=0) branch used by slog TimeValue's Round(0).
    pub fn without_monotonic(&self) -> Self { let mut time = self.source_fmt_copy(); time.strip_monotonic(); time }
    pub fn set_monotonic(&mut self, monotonic: i64) {
        if !self.has_monotonic() {
            if self.ext < WALL_TO_INTERNAL || self.ext > WALL_TO_INTERNAL + (1_i64 << 33) - 1 { return; }
            self.wall |= HAS_MONOTONIC | ((self.ext - WALL_TO_INTERNAL) as u64) << NSEC_SHIFT;
        }
        self.ext = monotonic;
    }
    fn add_seconds(&mut self, delta: i64) {
        if self.has_monotonic() {
            let packed = (self.wall << 1 >> (NSEC_SHIFT + 1)) as i64;
            let next = packed.wrapping_add(delta);
            if (0..=(1_i64 << 33) - 1).contains(&next) {
                self.wall = self.wall & NSEC_MASK | (next as u64) << NSEC_SHIFT | HAS_MONOTONIC; return;
            }
            self.strip_monotonic();
        }
        let sum = self.ext.wrapping_add(delta);
        self.ext = if (sum > self.ext) == (delta > 0) { sum }
            else if delta > 0 { i64::MAX } else { -i64::MAX };
    }
    pub fn add(&self, duration: i64) -> Self {
        let mut time = self.source_fmt_copy();
        let mut seconds = duration / 1_000_000_000;
        let mut nanos = time.nanosecond() + (duration % 1_000_000_000) as i32;
        if nanos >= 1_000_000_000 { seconds += 1; nanos -= 1_000_000_000; }
        else if nanos < 0 { seconds -= 1; nanos += 1_000_000_000; }
        time.wall = time.wall & !NSEC_MASK | nanos as u64; time.add_seconds(seconds);
        if time.has_monotonic() {
            let next = time.ext.wrapping_add(duration);
            if duration < 0 && next > time.ext || duration > 0 && next < time.ext { time.strip_monotonic(); }
            else { time.ext = next; }
        }
        time
    }
    pub fn sub(&self, other: &Self) -> i64 {
        if self.wall & other.wall & HAS_MONOTONIC != 0 {
            let delta = self.ext.wrapping_sub(other.ext);
            return if delta < 0 && self.ext > other.ext { i64::MAX }
                else if delta > 0 && self.ext < other.ext { i64::MIN } else { delta };
        }
        let delta = self.seconds().wrapping_sub(other.seconds()).wrapping_mul(1_000_000_000)
            .wrapping_add(i64::from(self.nanosecond() - other.nanosecond()));
        if other.add(delta).equal(self) { delta } else if self.before(other) { i64::MIN } else { i64::MAX }
    }
    pub fn set_location(&mut self, location: Option<Arc<SourceTimeLocation>>) {
        self.strip_monotonic();
        self.loc = location.filter(|loc| !Arc::ptr_eq(loc, &SourceTimeLocation::utc()));
    }
    pub fn utc(&self) -> Self { let mut time = self.source_fmt_copy(); time.set_location(None); time }
    pub fn in_location(&self, location: Option<Arc<SourceTimeLocation>>) -> Self {
        if location.is_none() {
            crate::utility_format_methods::SourcePrintfPanic::raise("time: missing Location in call to Time.In");
        }
        let mut time = self.source_fmt_copy(); time.set_location(location); time
    }
    pub fn location(&self) -> Arc<SourceTimeLocation> { self.loc.as_ref().map_or_else(SourceTimeLocation::utc, Arc::clone) }
    /// Unix's local location is supplied by its existing CGP owner. The UTC
    /// helper below is used only for the location-independent slog fit test.
    pub fn from_unix(mut seconds: i64, mut nanos: i64, local: Arc<SourceTimeLocation>) -> Self {
        if nanos < 0 || nanos >= 1_000_000_000 {
            let quotient = nanos / 1_000_000_000; seconds = seconds.wrapping_add(quotient);
            nanos -= quotient * 1_000_000_000;
            if nanos < 0 { nanos += 1_000_000_000; seconds = seconds.wrapping_sub(1); }
        }
        Self { wall: nanos as u64, ext: seconds.wrapping_add(UNIX_TO_INTERNAL), loc: Some(local) }
    }
    pub fn unix_utc(seconds: i64, nanos: i64) -> Self {
        let mut time = Self::from_unix(seconds, nanos, SourceTimeLocation::utc()); time.loc = None; time
    }
    pub(crate) fn location_abs(&self) -> FormatResult<(SourceTimeZone, u64)> {
        let zone = self.location().lookup(self.unix())?;
        let absolute = self.unix().wrapping_add(zone.offset).wrapping_add(UNIX_TO_INTERNAL.wrapping_add(INTERNAL_TO_ABSOLUTE)) as u64;
        Ok((zone, absolute))
    }
}
pub(crate) fn finish_time<Value>(result: FormatResult<Value>) -> Value {
    match result { Ok(value) => value,
        Err(SourceKlogFormatError::Allocation(error)) => complete_source_collection(&NativeSourceCollectionContext, Err(error)),
        Err(error) => std::panic::panic_any(error) }
}
impl SourceStringer for SourceGoTime {
    fn source_string(&self) -> Cow<'_, [u8]> { Cow::Owned(finish_time(self.try_string())) }
}
impl SourceGoStringer for SourceGoTime {
    fn source_go_string(&self) -> Cow<'_, [u8]> { Cow::Owned(finish_time(self.try_go_string(SourceUnicodeProfile::Go1265))) }
}
impl SourcePrintfValue for SourceGoTime {
    const SOURCE_DEREFERENCE_AT_ROOT: bool = true;
    fn source_type(&self) -> &str { "time.Time" }
    fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    { write_source_struct(self, verb, state, profile, mode, recursion, output) }
}
impl SourcePrintfStructFields for SourceGoTime {
    fn source_field_count(&self) -> usize { 3 }
    fn source_field_name(&self, index: usize) -> &str { match index { 0 => "wall", 1 => "ext", _ => "loc" } }
    fn source_field_interfaceable(&self, _: usize) -> bool { false }
    fn source_write_field(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        match index {
            0 => write_source_value(&self.wall, verb, state, profile, mode, recursion, output),
            1 => write_source_value(&self.ext, verb, state, profile, mode, recursion, output),
            _ => write_source_value(&SourcePrintfAddress {
                address: self.loc.as_ref().map_or(0, |loc| Arc::as_ptr(loc) as usize as u64),
                source_type: "*time.Location", kind: SourceAddressKind::Pointer,
            }, verb, state, profile, mode, recursion, output),
        }
    }
}
