//! One source printf control loop for statically typed native operand views.
// Adapted from Go fmt/print.go and fmt/format.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. This is a pure borrowed data path, with
// no erased value containers, runtime provider registry or lifecycle owner.
use crate::{
    utility_format_scan::{SourcePrintfPlan, SourcePrintfStep, SourcePrintfOutcome,
        SourcePrintfInteger, SourcePrintfFlags, SourcePrintfResolvedCounts, SourcePrintfCountDiagnostic,
        decode_source_rune},
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

/// Concrete views supply source scalar kind/value operations. The Sized
/// bound on print_arguments keeps selection static at every caller.
pub(crate) trait SourcePrintfArguments {
    fn len(&self) -> usize;
    fn type_name(&self, index: usize) -> &str;
    fn integer(&self, index: usize) -> SourcePrintfInteger;
    fn write_value(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        profile: SourceUnicodeProfile, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>;
}

pub struct SourcePrintfOutput { bytes: Vec<u8>, plan: SourcePrintfPlan }
impl SourcePrintfOutput {
    pub fn bytes(&self) -> &[u8] { &self.bytes }
    pub fn plan(&self) -> &SourcePrintfPlan { &self.plan }
    pub(crate) fn into_parts(self) -> (Vec<u8>, SourcePrintfPlan) { (self.bytes, self.plan) }
}

pub(crate) fn print_arguments<Arguments: SourcePrintfArguments>(format: &[u8], arguments: &Arguments,
    profile: SourceUnicodeProfile) -> Result<SourcePrintfOutput, SourceSliceAllocationError>
{
    let plan = SourcePrintfPlan::scan(format, arguments.len());
    let mut output = Vec::new();
    for step in plan.steps() {
        let directive = match step {
            SourcePrintfStep::Literal(span) => { append(&mut output, &format[span.clone()])?; continue; }
            SourcePrintfStep::Directive(directive) => directive,
        };
        let state = directive.resolve_counts(|index| arguments.integer(index));
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
                append_rune(&mut output, directive.verb.expect("scanner supplies an index/missing verb"))?;
                append(&mut output, if directive.outcome == SourcePrintfOutcome::BadIndex { b"(BADINDEX)" } else { b"(MISSING)" })?;
            }
            SourcePrintfOutcome::Value { argument } => {
                let verb = directive.verb.expect("scanner supplies a value verb");
                // printArg handles source type names before value dispatch.
                if verb == 'T' { write_plain(&mut output, arguments.type_name(argument).as_bytes(), &state)?; }
                else { arguments.write_value(argument, verb, &state, profile, &mut output)?; }
            }
        }
    }
    if let Some(extra) = plan.extra_arguments() {
        append(&mut output, b"%!(EXTRA ")?;
        let state = SourcePrintfResolvedCounts { flags: SourcePrintfFlags::default(), width: None,
            precision: None, diagnostics: Vec::new() };
        for (offset, index) in extra.enumerate() {
            if offset != 0 { append(&mut output, b", ")?; }
            append(&mut output, arguments.type_name(index).as_bytes())?;
            append(&mut output, b"=")?;
            arguments.write_value(index, 'v', &state, profile, &mut output)?;
        }
        append(&mut output, b")")?;
    }
    Ok(SourcePrintfOutput { bytes: output, plan })
}

pub(crate) fn write_plain(output: &mut Vec<u8>, value: &[u8], state: &SourcePrintfResolvedCounts)
    -> Result<(), SourceSliceAllocationError>
{
    pad(output, truncate_runes(value, state.precision), state)
}

pub(crate) fn truncate_runes(value: &[u8], precision: Option<i64>) -> &[u8] {
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

pub(crate) fn pad(output: &mut Vec<u8>, value: &[u8], state: &SourcePrintfResolvedCounts)
    -> Result<(), SourceSliceAllocationError>
{
    let width = state.width.unwrap_or(0) as usize;
    let padding = if width == 0 { 0 } else { width.saturating_sub(rune_count(value)) };
    if !state.flags.minus { write_padding(output, padding, state.flags.zero)?; }
    append(output, value)?;
    if state.flags.minus { write_padding(output, padding, false)?; }
    Ok(())
}

pub(crate) fn write_padding(output: &mut Vec<u8>, count: usize, zero: bool) -> Result<(), SourceSliceAllocationError> {
    let length = output.len().checked_add(count).ok_or(SourceSliceAllocationError::NativeAddressSpace)?;
    if length > isize::MAX as usize { return Err(SourceSliceAllocationError::NativeAddressSpace); }
    output.try_reserve(count).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
    output.resize(length, if zero { b'0' } else { b' ' });
    Ok(())
}

pub(crate) fn append_rune(output: &mut Vec<u8>, rune: char) -> Result<(), SourceSliceAllocationError> {
    let mut bytes = [0; 4];
    append(output, rune.encode_utf8(&mut bytes).as_bytes())
}
