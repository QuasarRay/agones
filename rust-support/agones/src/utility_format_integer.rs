//! Source integer formatting over concrete native scalar slices. Named source
//! types can supply the same compile-time kind/name declaration; no value
//! erasure, reflection hierarchy or runtime provider selection is introduced.
// Adapted from Go fmt/print.go and fmt/format.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE.
use crate::{
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts},
    utility_format_print::{SourcePrintfArguments, SourcePrintfOutput, print_arguments, pad, write_padding, append_rune},
    utility_go_quote::{append, try_source_go_quote_rune},
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

/// Signed bits are sign-extended before becoming uint64, as in printArg.
/// isize/usize are native int/uint projections; actual Go target word/ABI
/// binding and named-type method sets remain explicit required work.
pub trait SourcePrintfIntegerValue: Copy {
    const SOURCE_TYPE: &'static str;
    const SIGNED: bool;
    fn source_bits(self) -> u64;
    fn source_count(self) -> SourcePrintfInteger;
}
macro_rules! integer_kinds {
    (signed; $( $rust:ty => $source:literal ),* $(,)?) => { $(
        impl SourcePrintfIntegerValue for $rust {
            const SOURCE_TYPE: &'static str = $source;
            const SIGNED: bool = true;
            fn source_bits(self) -> u64 { self as u64 }
            fn source_count(self) -> SourcePrintfInteger { SourcePrintfInteger::Signed(self as i64) }
        }
    )* };
    (unsigned; $( $rust:ty => $source:literal ),* $(,)?) => { $(
        impl SourcePrintfIntegerValue for $rust {
            const SOURCE_TYPE: &'static str = $source;
            const SIGNED: bool = false;
            fn source_bits(self) -> u64 { self as u64 }
            fn source_count(self) -> SourcePrintfInteger { SourcePrintfInteger::Unsigned(self as u64) }
        }
    )* };
}
integer_kinds!(signed; i8 => "int8", i16 => "int16", i32 => "int32", i64 => "int64", isize => "int");
integer_kinds!(unsigned; u8 => "uint8", u16 => "uint16", u32 => "uint32", u64 => "uint64", usize => "uint");

struct IntegerArguments<'a, Value>(&'a [Value]);
impl<Value: SourcePrintfIntegerValue> SourcePrintfArguments for IntegerArguments<'_, Value> {
    fn len(&self) -> usize { self.0.len() }
    fn type_name(&self, _: usize) -> &str { Value::SOURCE_TYPE }
    fn integer(&self, index: usize) -> SourcePrintfInteger { self.0[index].source_count() }
    fn write_value(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        write_integer(output, self.0[index].source_bits(), Value::SIGNED, Value::SOURCE_TYPE, verb, state, profile)
    }
}

pub fn try_source_integer_printf<Value: SourcePrintfIntegerValue>(format: &[u8], arguments: &[Value],
    profile: SourceUnicodeProfile) -> Result<SourcePrintfOutput, SourceSliceAllocationError>
{
    print_arguments(format, &IntegerArguments(arguments), profile)
}

fn write_integer(output: &mut Vec<u8>, bits: u64, signed: bool, type_name: &str, verb: char,
    state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile) -> Result<(), SourceSliceAllocationError>
{
    match verb {
        'v' if state.flags.sharp_v && !signed => {
            let mut state = state.value_state(); state.flags.sharp = true;
            write_number(output, bits, false, 16, 'v', false, &state)
        }
        'v' | 'd' => write_number(output, bits, signed, 10, verb, false, state),
        'b' => write_number(output, bits, signed, 2, verb, false, state),
        'o' | 'O' => write_number(output, bits, signed, 8, verb, false, state),
        'x' | 'X' => write_number(output, bits, signed, 16, verb, verb == 'X', state),
        'c' | 'q' => {
            let rune = u32::try_from(bits).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            if verb == 'q' {
                let quoted = try_source_go_quote_rune(rune, profile, state.flags.plus)?;
                pad(output, quoted.as_bytes(), state)
            } else {
                let mut bytes = [0; 4]; pad(output, rune.encode_utf8(&mut bytes).as_bytes(), state)
            }
        }
        'U' => write_unicode(output, bits, state, profile),
        _ => {
            append(output, b"%!")?; append_rune(output, verb)?; append(output, b"(")?;
            append(output, type_name.as_bytes())?; append(output, b"=")?;
            write_integer(output, bits, signed, type_name, 'v', state, profile)?;
            append(output, b")")
        }
    }
}

fn digits(mut value: u64, base: u64, upper: bool, storage: &mut [u8; 64]) -> &[u8] {
    let alphabet = if upper { b"0123456789ABCDEF" } else { b"0123456789abcdef" };
    let mut start = storage.len();
    loop {
        start -= 1;
        storage[start] = alphabet[(value % base) as usize];
        value /= base;
        if value == 0 { return &storage[start..]; }
    }
}

fn write_number(output: &mut Vec<u8>, bits: u64, signed: bool, base: u64, verb: char,
    upper: bool, state: &SourcePrintfResolvedCounts) -> Result<(), SourceSliceAllocationError>
{
    let negative = signed && (bits as i64) < 0;
    let magnitude = if negative { bits.wrapping_neg() } else { bits };
    let sign = if negative { Some(b'-') } else if state.flags.plus { Some(b'+') }
        else if state.flags.space { Some(b' ') } else { None };
    let precision = match state.precision {
        Some(0) if magnitude == 0 => return write_padding(output, state.width.unwrap_or(0) as usize, false),
        Some(precision) => precision,
        None if state.flags.zero && !state.flags.minus && state.width.is_some() => state.width.unwrap() - if sign.is_some() { 1 } else { 0 },
        None => 0,
    };
    let mut storage = [0; 64];
    let number = digits(magnitude, base, upper, &mut storage);
    let zeros = (precision.max(0) as usize).saturating_sub(number.len());
    let mut encoded = Vec::new();
    if let Some(sign) = sign { append(&mut encoded, &[sign])?; }
    // O prefixes the complete octal representation, including any sharp zero.
    if verb == 'O' { append(&mut encoded, b"0o")?; }
    if state.flags.sharp {
        match base {
            2 => append(&mut encoded, b"0b")?,
            8 if zeros == 0 && number[0] != b'0' => append(&mut encoded, b"0")?,
            16 => append(&mut encoded, if upper { b"0X" } else { b"0x" })?,
            _ => {}
        }
    }
    write_padding(&mut encoded, zeros, true)?;
    append(&mut encoded, number)?;
    let mut state = state.value_state(); state.flags.zero = false;
    pad(output, &encoded, &state)
}

fn write_unicode(output: &mut Vec<u8>, bits: u64, state: &SourcePrintfResolvedCounts,
    profile: SourceUnicodeProfile) -> Result<(), SourceSliceAllocationError>
{
    let mut storage = [0; 64];
    let number = digits(bits, 16, true, &mut storage);
    let precision = state.precision.unwrap_or(4).max(4) as usize;
    let mut encoded = Vec::new();
    append(&mut encoded, b"U+")?;
    write_padding(&mut encoded, precision.saturating_sub(number.len()), true)?;
    append(&mut encoded, number)?;
    if state.flags.sharp {
        if let Some(rune) = u32::try_from(bits).ok().and_then(char::from_u32).filter(|rune| profile.is_print(*rune as u32)) {
            // Source %#U inserts the literal printable rune between quotes.
            append(&mut encoded, b" '")?; append_rune(&mut encoded, rune)?; append(&mut encoded, b"'")?;
        }
    }
    let mut state = state.value_state(); state.flags.zero = false;
    pad(output, &encoded, &state)
}
