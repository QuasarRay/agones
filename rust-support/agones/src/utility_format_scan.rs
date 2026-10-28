//! Raw Go printf directives, operand consumption and wrapping positions.
// Adapted from Go fmt/print.go. Copyright The Go Authors. BSD-3-Clause;
// see ../GO-LICENSE. This module scans bytes; value rendering has its own owner.
use std::ops::Range;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SourcePrintfFlags {
    pub sharp: bool, pub zero: bool, pub plus: bool, pub minus: bool, pub space: bool,
    pub sharp_v: bool, pub plus_v: bool,
}

/// A star retains the consumed operand, including a present non-integer.
/// None means no operand was available, so the argument cursor did not move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourcePrintfCount { Absent, Literal(u32), Argument(Option<usize>) }

/// The integer-kind adapter includes named Go integer types. It must classify
/// values by their source kind, rather than invoke Error/String/Formatter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourcePrintfInteger { Signed(i64), Unsigned(u64), Other }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourcePrintfCountDiagnostic { BadWidth, BadPrecision }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePrintfResolvedCounts {
    pub flags: SourcePrintfFlags,
    pub width: Option<i64>,
    pub precision: Option<i64>,
    /// The printer emits these before the verb result, in this order.
    pub diagnostics: Vec<SourcePrintfCountDiagnostic>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourcePrintfOutcome {
    Value { argument: usize }, Percent, BadIndex, MissingArgument, NoVerb,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePrintfDirective {
    pub span: Range<usize>,
    pub flags: SourcePrintfFlags,
    pub width: SourcePrintfCount,
    pub precision: SourcePrintfCount,
    pub verb: Option<char>,
    pub outcome: SourcePrintfOutcome,
}
impl SourcePrintfDirective {
    /// Resolve stars in source evaluation order without formatting their values.
    /// The accepted star range fits both Go32 and Go64; all other magnitudes
    /// are rejected by intFromArg's 1e6 bound even if their native cast fits.
    pub fn resolve_counts(&self, mut integer: impl FnMut(usize) -> SourcePrintfInteger) -> SourcePrintfResolvedCounts {
        let mut flags = self.flags;
        let mut diagnostics = Vec::new();
        let mut width = count_value(self.width, &mut integer);
        if matches!(self.width, SourcePrintfCount::Argument(_)) && width.is_none() {
            diagnostics.push(SourcePrintfCountDiagnostic::BadWidth);
        }
        if let Some(value) = width.as_mut() {
            if *value < 0 { *value = -*value; flags.minus = true; flags.zero = false; }
        }
        let mut precision = count_value(self.precision, &mut integer);
        if matches!(self.precision, SourcePrintfCount::Argument(_)) {
            if precision.is_some_and(|value| value < 0) { precision = None; }
            if precision.is_none() { diagnostics.push(SourcePrintfCountDiagnostic::BadPrecision); }
        }
        SourcePrintfResolvedCounts { flags, width, precision, diagnostics }
    }
}

fn count_value(count: SourcePrintfCount, integer: &mut impl FnMut(usize) -> SourcePrintfInteger) -> Option<i64> {
    match count {
        SourcePrintfCount::Absent | SourcePrintfCount::Argument(None) => None,
        // parsenum checks the bound before each digit. Its final digit can
        // leave a literal larger than 1e6; intFromArg checks after conversion.
        SourcePrintfCount::Literal(value) => Some(i64::from(value)),
        SourcePrintfCount::Argument(Some(index)) => match integer(index) {
            SourcePrintfInteger::Signed(value) if (-1_000_000..=1_000_000).contains(&value) => Some(value),
            SourcePrintfInteger::Unsigned(value) if value <= 1_000_000 => Some(value as i64),
            _ => None,
        },
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourcePrintfStep { Literal(Range<usize>), Directive(SourcePrintfDirective) }

/// Byte ranges refer to the caller's original format, which may contain
/// invalid UTF8. No Go value methods run while building the plan. This is
/// printer control data, not a general Go object or runtime provider system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePrintfPlan {
    steps: Vec<SourcePrintfStep>,
    wrapped_positions: Vec<usize>,
    reordered: bool,
    next_argument: usize,
    extra_arguments: Option<Range<usize>>,
}
impl SourcePrintfPlan {
    pub fn scan(format: &[u8], arguments: usize) -> Self {
        let mut scanner = Scanner { format, arguments, cursor: 0, argument: 0, reordered: false, good_index: true };
        let mut steps = Vec::new();
        let mut wrapped_positions = Vec::new();
        while scanner.cursor < format.len() {
            let start = scanner.cursor;
            while scanner.cursor < format.len() && format[scanner.cursor] != b'%' { scanner.cursor += 1; }
            if start < scanner.cursor { steps.push(SourcePrintfStep::Literal(start..scanner.cursor)); }
            if scanner.cursor == format.len() { break; }
            let directive = scanner.directive();
            if let (Some('w'), SourcePrintfOutcome::Value { argument }) = (directive.verb, directive.outcome) {
                // doPrintf records %w before printArg's error-type check.
                wrapped_positions.push(argument);
            }
            let no_verb = directive.outcome == SourcePrintfOutcome::NoVerb;
            steps.push(SourcePrintfStep::Directive(directive));
            if no_verb { break; }
        }
        let extra_arguments = (!scanner.reordered && scanner.argument < arguments).then_some(scanner.argument..arguments);
        Self { steps, wrapped_positions, reordered: scanner.reordered,
            next_argument: scanner.argument, extra_arguments }
    }
    pub fn steps(&self) -> &[SourcePrintfStep] { &self.steps }
    pub fn wrapped_positions(&self) -> &[usize] { &self.wrapped_positions }
    pub fn reordered(&self) -> bool { self.reordered }
    pub fn next_argument(&self) -> usize { self.next_argument }
    pub fn extra_arguments(&self) -> Option<Range<usize>> { self.extra_arguments.clone() }
    pub(crate) fn into_wrapping(self) -> (Vec<usize>, bool) { (self.wrapped_positions, self.reordered) }
}

struct Scanner<'a> {
    format: &'a [u8], arguments: usize, cursor: usize, argument: usize,
    reordered: bool, good_index: bool,
}
impl Scanner<'_> {
    fn directive(&mut self) -> SourcePrintfDirective {
        let start = self.cursor;
        self.cursor += 1;
        self.good_index = true;
        let mut flags = SourcePrintfFlags::default();
        while let Some(byte) = self.format.get(self.cursor) {
            match *byte {
                b'#' => flags.sharp = true, b'0' => flags.zero = true,
                b'+' => flags.plus = true, b'-' => flags.minus = true, b' ' => flags.space = true,
                _ => break,
            }
            self.cursor += 1;
        }

        // The general path has the same argument/flag behavior as Go's
        // ASCII-lowercase fast path, without duplicating the scanner bodies.
        let mut after_index = self.argument_number();
        let width = if self.format.get(self.cursor) == Some(&b'*') {
            self.cursor += 1;
            after_index = false;
            SourcePrintfCount::Argument(self.consume_star())
        } else {
            let number = self.number(self.format.len());
            if after_index && number.is_some() { self.good_index = false; }
            number.map_or(SourcePrintfCount::Absent, SourcePrintfCount::Literal)
        };

        let mut precision = SourcePrintfCount::Absent;
        // A terminal '.' is itself the verb. An empty precision before a
        // following byte is zero, including when parsenum rejects a number.
        if self.cursor < self.format.len().saturating_sub(1) && self.format[self.cursor] == b'.' {
            self.cursor += 1;
            if after_index { self.good_index = false; }
            after_index = self.argument_number();
            precision = if self.format.get(self.cursor) == Some(&b'*') {
                self.cursor += 1;
                after_index = false;
                SourcePrintfCount::Argument(self.consume_star())
            } else { SourcePrintfCount::Literal(self.number(self.format.len()).unwrap_or(0)) };
        }
        if !after_index { self.argument_number(); }

        let (verb, outcome) = if self.cursor >= self.format.len() {
            (None, SourcePrintfOutcome::NoVerb)
        } else {
            let (verb, size) = decode_verb(&self.format[self.cursor..]);
            self.cursor += size;
            let outcome = if verb == '%' { SourcePrintfOutcome::Percent }
            else if !self.good_index { SourcePrintfOutcome::BadIndex }
            else if self.argument >= self.arguments { SourcePrintfOutcome::MissingArgument }
            else {
                let argument = self.argument;
                self.argument += 1;
                if verb == 'v' || verb == 'w' {
                    flags.sharp_v = flags.sharp; flags.sharp = false;
                    flags.plus_v = flags.plus; flags.plus = false;
                }
                SourcePrintfOutcome::Value { argument }
            };
            (Some(verb), outcome)
        };
        SourcePrintfDirective { span: start..self.cursor, flags, width, precision, verb, outcome }
    }

    fn consume_star(&mut self) -> Option<usize> {
        if self.argument >= self.arguments { return None; }
        let argument = self.argument;
        self.argument += 1;
        Some(argument)
    }

    fn number(&mut self, end: usize) -> Option<u32> {
        let (number, next) = parse_number(self.format, self.cursor, end);
        self.cursor = next;
        number
    }

    /// Return syntactic index presence even for an out-of-range number.
    /// Bad index syntax consumes through ']' when one exists, otherwise '['.
    fn argument_number(&mut self) -> bool {
        if self.format.get(self.cursor) != Some(&b'[') { return false; }
        self.reordered = true;
        let start = self.cursor;
        let tail = &self.format[start..];
        if tail.len() >= 3 {
            if let Some(close) = tail[1..].iter().position(|byte| *byte == b']').map(|offset| offset + 1) {
                let (number, next) = parse_number(tail, 1, close);
                self.cursor += close + 1;
                if let Some(number) = number.filter(|_| next == close) {
                    if let Some(index) = number.checked_sub(1).map(|value| value as usize).filter(|&index| index < self.arguments) {
                        self.argument = index;
                    } else { self.good_index = false; }
                    return true;
                }
                self.good_index = false;
                return false;
            }
        }
        self.cursor += 1;
        self.good_index = false;
        false
    }
}

fn parse_number(format: &[u8], start: usize, end: usize) -> (Option<u32>, usize) {
    if start >= end { return (None, end); }
    let mut cursor = start;
    let mut number = 0;
    while cursor < end && format[cursor].is_ascii_digit() {
        if number > 1_000_000 { return (None, end); }
        number = number * 10 + u32::from(format[cursor] - b'0');
        cursor += 1;
    }
    ((cursor > start).then_some(number), cursor)
}

fn decode_verb(bytes: &[u8]) -> (char, usize) {
    let size = match bytes[0] { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => 0 };
    if size != 0 && size <= bytes.len() {
        if let Ok(text) = std::str::from_utf8(&bytes[..size]) {
            if let Some(rune) = text.chars().next() { return (rune, size); }
        }
    }
    ('\u{fffd}', 1)
}
