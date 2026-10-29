//! Concrete bool operand view for the shared source printf control loop.
// Adapted from Go fmt/print.go and fmt/format.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE.
use crate::{
    utility_format_scan::{SourcePrintfInteger, SourcePrintfResolvedCounts},
    utility_format_print::{SourcePrintfArguments, SourcePrintfOutput, print_arguments, pad, append_rune},
    utility_go_quote::append,
    utility_http::{SourceUnicodeProfile, SourceSliceAllocationError},
};

struct BoolArguments<'a>(&'a [bool]);
impl SourcePrintfArguments for BoolArguments<'_> {
    fn len(&self) -> usize { self.0.len() }
    fn type_name(&self, _: usize) -> &str { "bool" }
    fn integer(&self, _: usize) -> SourcePrintfInteger { SourcePrintfInteger::Other }
    fn write_value(&self, index: usize, verb: char, state: &SourcePrintfResolvedCounts,
        _: SourceUnicodeProfile, output: &mut Vec<u8>) -> Result<(), SourceSliceAllocationError>
    {
        let value: &[u8] = if self.0[index] { b"true" } else { b"false" };
        match verb {
            't' | 'v' => pad(output, value, state),
            _ => {
                append(output, b"%!")?; append_rune(output, verb)?; append(output, b"(bool=")?;
                // fmtBoolean ignores precision but retains width/zero/minus.
                pad(output, value, state)?;
                append(output, b")")
            }
        }
    }
}

pub fn try_source_bool_printf(format: &[u8], arguments: &[bool], profile: SourceUnicodeProfile)
    -> Result<SourcePrintfOutput, SourceSliceAllocationError>
{
    print_arguments(format, &BoolArguments(arguments), profile)
}
