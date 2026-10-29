//! Source float/complex verbs over the selected multiprecision conversion path.
// Adapted from Go fmt/print.go and fmt/format.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. Conversion is authored, not executed evidence;
// optimized Dragonbox/fixed paths and native allocation profiles remain open.
use derive_deftly::Deftly;
use crate::{
    utility_declarations::{derive_deftly_template_SourcePrintfInterfaces, derive_deftly_template_SourceFmtValueCopy},
    utility_format_arguments::SourcePrintfValue,
    utility_format_methods::SourcePrintfRecursion,
    utility_format_print::{SourcePrintfMode, pad, write_padding, append_rune},
    utility_format_scan::SourcePrintfResolvedCounts,
    utility_float_reference::{SourceFloatWidth, try_source_float, repeat},
    utility_format_map::SourceFmtMapKey,
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

macro_rules! float_values {
    ($( $ty:ty => $name:literal, $width:ident ),+ $(,)?) => { $(
        impl SourcePrintfValue for $ty {
            fn source_type(&self) -> &str { $name }
            fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile,
                _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
            { write_float(output, f64::from(*self), SourceFloatWidth::$width, $name, verb, state) }
        }
    )+ };
}
float_values!(f32 => "float32", Binary32, f64 => "float64", Binary64);

pub(crate) fn write_float(output: &mut Vec<u8>, value: f64, width: SourceFloatWidth, source_type: &str,
    verb: char, state: &SourcePrintfResolvedCounts) -> Result<(), SourceSliceAllocationError>
{
    let (format, default_precision) = match verb {
        'v' => (b'g', -1), 'b' | 'g' | 'G' | 'x' | 'X' => (verb as u8, -1),
        'f' | 'e' | 'E' => (verb as u8, 6), 'F' => (b'f', 6),
        _ => {
            append(output, b"%!")?; append_rune(output, verb)?; append(output, b"(")?;
            append(output, source_type.as_bytes())?; append(output, b"=")?;
            write_float(output, value, width, source_type, 'v', state)?;
            return append(output, b")");
        }
    };
    let precision = state.precision.map_or(default_precision, |precision| precision as i32);
    let converted = try_source_float(value, width, format, precision)?;
    let mut number = Vec::new();
    if !matches!(converted.first(), Some(b'-' | b'+')) { append(&mut number, b"+")?; }
    append(&mut number, &converted)?;
    if state.flags.space && number[0] == b'+' && !state.flags.plus { number[0] = b' '; }
    if number[1] == b'I' || number[1] == b'N' {
        let mut special = state.value_state(); special.flags.zero = false;
        if number[1] == b'N' && !state.flags.space && !state.flags.plus { return pad(output, &number[1..], &special); }
        return pad(output, &number, &special);
    }
    if state.flags.sharp && format != b'b' {
        let mut digits = if matches!(format, b'g' | b'G' | b'x') {
            if precision == -1 { 6 } else { precision }
        } else { 0 };
        let mut tail = Vec::new(); let mut decimal_point = false; let mut nonzero = false; let mut index = 1;
        while index < number.len() {
            let digit = number[index];
            if digit == b'.' { decimal_point = true; }
            else if matches!(digit, b'p' | b'P') || (matches!(digit, b'e' | b'E') && !matches!(format, b'x' | b'X')) {
                append(&mut tail, &number[index..])?; number.truncate(index);
            } else {
                if digit != b'0' { nonzero = true; }
                if nonzero { digits -= 1; }
            }
            index += 1;
        }
        if !decimal_point {
            if number.len() == 2 && number[1] == b'0' { digits -= 1; }
            append(&mut number, b".")?;
        }
        if digits > 0 { repeat(&mut number, b'0', digits as usize)?; }
        append(&mut number, &tail)?;
    }
    if state.flags.plus || number[0] != b'+' {
        if state.flags.zero && !state.flags.minus && state.width.is_some_and(|width| width > number.len() as i64) {
            append(output, &number[..1])?;
            write_padding(output, state.width.unwrap() as usize - number.len(), true)?;
            return append(output, &number[1..]);
        }
        pad(output, &number, state)
    } else { pad(output, &number[1..], state) }
}

#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces, SourceFmtValueCopy)]
pub struct SourceComplex64 { pub real: f32, pub imaginary: f32 }
#[derive(Deftly)]
#[derive_deftly(SourcePrintfInterfaces, SourceFmtValueCopy)]
pub struct SourceComplex128 { pub real: f64, pub imaginary: f64 }

macro_rules! complex_values {
    ($( $ty:ty => $name:literal, $width:ident ),+ $(,)?) => { $(
        impl SourcePrintfValue for $ty {
            fn source_type(&self) -> &str { $name }
            fn write_source(&self, verb: char, state: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile,
                _: SourcePrintfMode, _: SourcePrintfRecursion, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
            { write_complex(output, f64::from(self.real), f64::from(self.imaginary), SourceFloatWidth::$width, $name, verb, state) }
        }
        impl SourceFmtMapKey for $ty {
            fn source_key_cmp(&self, other: &Self) -> std::cmp::Ordering {
                let real = self.real.source_key_cmp(&other.real);
                if real != std::cmp::Ordering::Equal { real } else { self.imaginary.source_key_cmp(&other.imaginary) }
            }
        }
    )+ };
}
complex_values!(SourceComplex64 => "complex64", Binary32, SourceComplex128 => "complex128", Binary64);

fn write_complex(output: &mut Vec<u8>, real: f64, imaginary: f64, width: SourceFloatWidth,
    source_type: &str, verb: char, state: &SourcePrintfResolvedCounts) -> Result<(), SourceSliceAllocationError>
{
    if !matches!(verb, 'v' | 'b' | 'g' | 'G' | 'x' | 'X' | 'f' | 'F' | 'e' | 'E') {
        append(output, b"%!")?; append_rune(output, verb)?; append(output, b"(")?;
        append(output, source_type.as_bytes())?; append(output, b"=")?;
        write_complex(output, real, imaginary, width, source_type, 'v', state)?;
        return append(output, b")");
    }
    append(output, b"(")?;
    let component_type = if width == SourceFloatWidth::Binary32 { "float32" } else { "float64" };
    write_float(output, real, width, component_type, verb, state)?;
    let mut imaginary_state = state.value_state(); imaginary_state.flags.plus = true;
    write_float(output, imaginary, width, component_type, verb, &imaginary_state)?;
    append(output, b"i)")
}
