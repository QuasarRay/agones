//! Pinned Go single-rune uppercase mapping for HTTP method overrides.
// Adapted from Go strings.ToUpper/Map and unicode.ToUpper/convertCase.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use bytes::Bytes;
use cgp::prelude::*;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SourceUnicodeProfile { #[default] Go1265, Go1271 }
#[cgp_getter]
pub trait HasSourceUnicodeProfile { fn source_unicode_profile(&self) -> &SourceUnicodeProfile; }
#[cgp_component(SourceUppercaseMapper)]
pub trait CanUppercaseSourceString { fn uppercase_source_string(&self, input: &[u8]) -> Bytes; }
#[cgp_new_provider]
impl<Context: HasSourceUnicodeProfile> SourceUppercaseMapper<Context> for MapSourceUppercase {
    fn uppercase_source_string(context: &Context, input: &[u8]) -> Bytes { source_to_upper(input, *context.source_unicode_profile()) }
}

#[derive(Clone, Copy)]
struct CaseRange { low: u32, high: u32, upper: i32 }
static GO1265: OnceLock<Vec<CaseRange>> = OnceLock::new();
static GO1271: OnceLock<Vec<CaseRange>> = OnceLock::new();
#[derive(Clone, Copy)]
struct PrintRange { low: u32, high: u32, stride: u32 }
static GO1265_PRINT: OnceLock<Vec<Vec<PrintRange>>> = OnceLock::new();
static GO1271_PRINT: OnceLock<Vec<Vec<PrintRange>>> = OnceLock::new();
impl SourceUnicodeProfile {
    fn case_ranges(self) -> &'static [CaseRange] {
        match self {
            Self::Go1265 => GO1265.get_or_init(|| read_case_ranges(include_str!("../source-data/unicode/go1.26.5/tables.go"))),
            Self::Go1271 => GO1271.get_or_init(|| read_case_ranges(include_str!("../source-data/unicode/go1.27.1/tables.go"))),
        }
    }
    /// Go's printable set is L/M/N/P/S plus ASCII space. Read the same pinned
    /// immutable category data used by this source version, not Rust Unicode.
    pub fn is_print(self, rune: u32) -> bool {
        if rune <= 0x7f { return (0x20..=0x7e).contains(&rune); }
        if rune > 0x10ffff { return false; }
        let categories = match self {
            Self::Go1265 => GO1265_PRINT.get_or_init(|| read_print_ranges(include_str!("../source-data/unicode/go1.26.5/tables.go"))),
            Self::Go1271 => GO1271_PRINT.get_or_init(|| read_print_ranges(include_str!("../source-data/unicode/go1.27.1/tables.go"))),
        };
        categories.iter().any(|ranges| {
            let index = ranges.partition_point(|range| range.high < rune);
            ranges.get(index).is_some_and(|range| rune >= range.low && (rune - range.low) % range.stride == 0)
        })
    }
}
/// Project only numeric category initializer rows into an immutable lookup.
/// No Go body is evaluated or emitted by this data reader.
fn read_print_ranges(source: &str) -> Vec<Vec<PrintRange>> {
    ["L", "M", "N", "P", "S"].into_iter().map(|name| {
        let marker = format!("var _{name} = &RangeTable{{");
        let (_, body) = source.split_once(marker.as_str()).expect("pinned Go print category");
        let mut ranges = Vec::new();
        for line in body.lines() {
            let line = line.trim();
            if line == "}" { return ranges; }
            let Some(row) = line.strip_prefix('{').and_then(|line| line.split_once('}').map(|(row, _)| row)) else { continue; };
            let mut fields = row.split(',').map(|field| {
                let field = field.trim();
                if let Some(hex) = field.strip_prefix("0x") { u32::from_str_radix(hex, 16).expect("pinned print scalar") }
                else { field.parse::<u32>().expect("pinned print stride") }
            });
            ranges.push(PrintRange { low: fields.next().expect("print low"), high: fields.next().expect("print high"), stride: fields.next().expect("print stride") });
        }
        panic!("unterminated pinned Go print category")
    }).collect()
}
/// Read only the numeric CaseRanges data in the immutable, tracked upstream
/// artifact. This is a disposable typed lookup cache, not a Go evaluator or a
/// Go-to-Rust emitter. No production body is obtained from Go text at runtime.
fn read_case_ranges(source: &str) -> Vec<CaseRange> {
    let (_, body) = source.split_once("var _CaseRanges = []CaseRange{").expect("pinned Go CaseRanges declaration");
    let mut ranges = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line == "}" { return ranges; }
        if line.is_empty() || line.starts_with("//") { continue; }
        let line = line.strip_prefix('{').expect("pinned CaseRange row");
        let (low, line) = line.split_once(',').expect("pinned CaseRange low");
        let (high, line) = line.split_once(',').expect("pinned CaseRange high");
        let delta = line.trim().strip_prefix("d{").expect("pinned CaseRange deltas").split(',').next().expect("pinned uppercase delta").trim();
        let number = |value: &str| u32::from_str_radix(value.trim().trim_start_matches("0x"), 16).expect("pinned CaseRange scalar");
        let upper = if delta == "UpperLower" { 0x110000 } else { delta.parse::<i32>().expect("pinned uppercase delta") };
        ranges.push(CaseRange { low: number(low), high: number(high), upper });
    }
    panic!("unterminated pinned Go CaseRanges data")
}
fn uppercase_rune(rune: u32, ranges: &[CaseRange]) -> u32 {
    if rune <= 0x7f { return if (b'a' as u32..=b'z' as u32).contains(&rune) { rune-32 } else { rune }; }
    let (mut low, mut high) = (0, ranges.len());
    while low < high {
        let middle = low + (high-low)/2; let range = ranges[middle];
        if rune < range.low { high = middle; }
        else if rune > range.high { low = middle+1; }
        else {
            return if range.upper > 0x10ffff { range.low + ((rune-range.low) & !1) }
                else { (rune as i64 + range.upper as i64) as u32 };
        }
    }
    rune
}
pub fn source_to_upper(input: &[u8], profile: SourceUnicodeProfile) -> Bytes {
    if input.iter().all(u8::is_ascii) { return Bytes::from(input.iter().map(u8::to_ascii_uppercase).collect::<Vec<_>>()); }
    let ranges = profile.case_ranges();
    let mut result = Vec::new(); let mut offset = 0;
    while offset < input.len() {
        let first = input[offset];
        let width = match first { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => 0 };
        let decoded = (width != 0 && offset+width <= input.len()).then(|| &input[offset..offset+width])
            .and_then(|bytes| std::str::from_utf8(bytes).ok()).and_then(|value| value.chars().next());
        // Go range replaces each invalid UTF8 byte separately. A valid U+FFFD
        // remains one rune, and case conversion never expands to several runes.
        let (rune, consumed) = decoded.map_or((0xfffd, 1), |value| (value as u32, width));
        let mapped = char::from_u32(uppercase_rune(rune, ranges)).unwrap_or('\u{fffd}');
        let mut encoded = [0; 4]; result.extend_from_slice(mapped.encode_utf8(&mut encoded).as_bytes());
        offset += consumed;
    }
    Bytes::from(result)
}
