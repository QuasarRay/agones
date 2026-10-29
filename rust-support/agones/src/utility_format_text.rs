//! Concrete Go printf rendering for source string operands, including bytes
//! that cannot be represented by a Rust str. Other operand kinds still need
//! their own source formatting paths; this module does not emulate Go objects.
// Adapted from Go fmt/print.go, fmt/format.go and strconv/quote.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use crate::{
    utility_format_scan::{SourcePrintfPlan, SourcePrintfStep, SourcePrintfOutcome,
        SourcePrintfInteger, SourcePrintfFlags, SourcePrintfResolvedCounts, SourcePrintfCountDiagnostic,
        decode_source_rune},
    utility_go_quote::{append, source_can_backquote, try_source_go_quote_with_ascii},
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

pub struct SourceTextPrintf {
    bytes: Vec<u8>,
    plan: SourcePrintfPlan,
}
impl SourceTextPrintf {
    pub fn bytes(&self) -> &[u8] { &self.bytes }
    pub fn plan(&self) -> &SourcePrintfPlan { &self.plan }
    pub(crate) fn into_parts(self) -> (Vec<u8>, SourcePrintfPlan) { (self.bytes, self.plan) }
}

/// Each argument has source kind string, with its original byte contents.
/// Stars therefore consume a present argument but report a bad count. The
/// scanner's separate integer-kind resolver serves integer-bearing paths.
pub fn try_source_text_printf(format: &[u8], arguments: &[&[u8]], profile: SourceUnicodeProfile)
    -> Result<SourceTextPrintf, SourceSliceAllocationError>
{
    let plan = SourcePrintfPlan::scan(format, arguments.len());
    let mut output = Vec::new();
    for step in plan.steps() {
        let directive = match step {
            SourcePrintfStep::Literal(span) => { append(&mut output, &format[span.clone()])?; continue; }
            SourcePrintfStep::Directive(directive) => directive,
        };
        let state = directive.resolve_counts(|_| SourcePrintfInteger::Other);
        for diagnostic in &state.diagnostics {
            append(&mut output, match diagnostic {
                SourcePrintfCountDiagnostic::BadWidth => b"%!(BADWIDTH)",
                SourcePrintfCountDiagnostic::BadPrecision => b"%!(BADPREC)",
            })?;
        }
        match directive.outcome {
            SourcePrintfOutcome::Percent => append(&mut output, b"%")?,
            SourcePrintfOutcome::NoVerb => append(&mut output, b"%!(NOVERB)")?,
            SourcePrintfOutcome::BadIndex | SourcePrintfOutcome::MissingArgument => {
                append(&mut output, b"%!")?;
                append_rune(&mut output, directive.verb.expect("scanner supplies a verb for index/missing outcomes"))?;
                append(&mut output, if directive.outcome == SourcePrintfOutcome::BadIndex { b"(BADINDEX)" } else { b"(MISSING)" })?;
            }
            SourcePrintfOutcome::Value { argument } => {
                write_text(&mut output, arguments[argument], directive.verb.expect("scanner supplies a value verb"), &state, profile)?;
            }
        }
    }
    if let Some(extra) = plan.extra_arguments() {
        append(&mut output, b"%!(EXTRA ")?;
        // Source clearflags happens once before rendering trailing operands.
        let state = SourcePrintfResolvedCounts { flags: SourcePrintfFlags::default(), width: None,
            precision: None, diagnostics: Vec::new() };
        for (offset, index) in extra.enumerate() {
            if offset != 0 { append(&mut output, b", ")?; }
            append(&mut output, b"string=")?;
            write_text(&mut output, arguments[index], 'v', &state, profile)?;
        }
        append(&mut output, b")")?;
    }
    Ok(SourceTextPrintf { bytes: output, plan })
}

fn write_text(output: &mut Vec<u8>, value: &[u8], verb: char, state: &SourcePrintfResolvedCounts,
    profile: SourceUnicodeProfile) -> Result<(), SourceSliceAllocationError>
{
    match verb {
        'T' => write_plain(output, b"string", state),
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

fn write_plain(output: &mut Vec<u8>, value: &[u8], state: &SourcePrintfResolvedCounts)
    -> Result<(), SourceSliceAllocationError>
{
    pad(output, truncate_runes(value, state.precision), state)
}

fn truncate_runes(value: &[u8], precision: Option<i64>) -> &[u8] {
    let Some(mut remaining) = precision else { return value; };
    let mut cursor = 0;
    while cursor < value.len() {
        if remaining == 0 { return &value[..cursor]; }
        cursor += decode_source_rune(&value[cursor..]).1;
        remaining -= 1;
    }
    value
}

fn rune_count(mut value: &[u8]) -> usize {
    let mut count = 0;
    while !value.is_empty() { value = &value[decode_source_rune(value).1..]; count += 1; }
    count
}

fn pad(output: &mut Vec<u8>, value: &[u8], state: &SourcePrintfResolvedCounts)
    -> Result<(), SourceSliceAllocationError>
{
    let width = state.width.unwrap_or(0) as usize;
    let padding = if width == 0 { 0 } else { width.saturating_sub(rune_count(value)) };
    if !state.flags.minus { write_padding(output, padding, state.flags.zero)?; }
    append(output, value)?;
    if state.flags.minus { write_padding(output, padding, false)?; }
    Ok(())
}

fn write_padding(output: &mut Vec<u8>, count: usize, zero: bool) -> Result<(), SourceSliceAllocationError> {
    let length = output.len().checked_add(count).ok_or(SourceSliceAllocationError::NativeAddressSpace)?;
    if length > isize::MAX as usize { return Err(SourceSliceAllocationError::NativeAddressSpace); }
    output.try_reserve(count).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
    output.resize(length, if zero { b'0' } else { b' ' });
    Ok(())
}

fn append_rune(output: &mut Vec<u8>, rune: char) -> Result<(), SourceSliceAllocationError> {
    let mut bytes = [0; 4];
    append(output, rune.encode_utf8(&mut bytes).as_bytes())
}
