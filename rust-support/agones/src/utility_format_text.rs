//! Concrete Go printf rendering for source string operands, including bytes
//! that cannot be represented by a Rust str. Other operand kinds still need
//! their own source formatting paths; this module does not emulate Go objects.
// Adapted from Go fmt/print.go, fmt/format.go and strconv/quote.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use crate::{
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts},
    utility_format_print::{SourcePrintfArguments, SourcePrintfMode, print_arguments, write_plain, truncate_runes, pad, append_rune},
    utility_go_quote::{append, source_can_backquote, try_source_go_quote_with_ascii},
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

pub use crate::utility_format_print::SourcePrintfOutput as SourceTextPrintf;

/// Each argument has source kind string, with its original byte contents.
/// Stars therefore consume a present argument but report a bad count. The
/// scanner's separate integer-kind resolver serves integer-bearing paths.
pub fn try_source_text_printf(format: &[u8], arguments: &[&[u8]], profile: SourceUnicodeProfile)
    -> Result<SourceTextPrintf, SourceSliceAllocationError>
{
    print_arguments(format, &TextArguments(arguments), profile, SourcePrintfMode::Sprintf)
}

struct TextArguments<'a>(&'a [&'a [u8]]);
impl SourcePrintfArguments for TextArguments<'_> {
    fn len(&self) -> usize { self.0.len() }
    fn type_name(&self, _: usize) -> &str { "string" }
    fn integer(&self, _: usize) -> SourcePrintfInteger { SourcePrintfInteger::Other }
    fn write_value(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, _: SourcePrintfMode, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        write_text(output, self.0[index], verb, state, profile)
    }
}

pub(crate) fn write_text(output: &mut Vec<u8>, value: &[u8], verb: char, state: &SourcePrintfResolvedCounts,
    profile: SourceUnicodeProfile) -> Result<(), SourceSliceAllocationError>
{
    match verb {
        'v' if !state.flags.sharp_v => write_plain(output, value, state),
        's' => write_plain(output, value, state),
        'q' | 'v' => {
            let value = truncate_runes(value, state.precision);
            if state.flags.sharp && source_can_backquote(value) {
                let mut quoted = Vec::new();
                append(&mut quoted, b"`")?; append(&mut quoted, value)?; append(&mut quoted, b"`")?;
                pad(output, &quoted, state)
            } else {
                let quoted = try_source_go_quote_with_ascii(value, profile, state.flags.plus)?;
                pad(output, quoted.as_bytes(), state)
            }
        }
        'x' | 'X' => {
            let digits = if verb == 'x' { b"0123456789abcdefx" } else { b"0123456789ABCDEFX" };
            let length = state.precision.map_or(value.len(), |precision| value.len().min(precision as usize));
            let mut encoded = Vec::new();
            if length != 0 && state.flags.sharp { append(&mut encoded, &[b'0', digits[16]])?; }
            for (index, byte) in value[..length].iter().copied().enumerate() {
                if index != 0 && state.flags.space {
                    append(&mut encoded, b" ")?;
                    if state.flags.sharp { append(&mut encoded, &[b'0', digits[16]])?; }
                }
                append(&mut encoded, &[digits[(byte >> 4) as usize], digits[(byte & 15) as usize]])?;
            }
            pad(output, &encoded, state)
        }
        _ => {
            // badVerb recursively prints the original string with v and the
            // current printer state. Its width/precision/flags are retained.
            append(output, b"%!")?; append_rune(output, verb)?; append(output, b"(string=")?;
            write_text(output, value, 'v', state, profile)?;
            append(output, b")")
        }
    }
}

