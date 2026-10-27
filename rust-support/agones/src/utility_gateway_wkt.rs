//! Text-shaped protobuf well-known types in the source JSONPb profile.
//! Source: vendored protojson/well_known_types.go and internal/strs/strings.go.
// Timestamp parsing also adapts Go time/format*. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE.
use prost_reflect::{DynamicMessage, ReflectMessage, Value};
use crate::errors::{BoxError, Errors};

const ERRORS: Errors = Errors::from_package("google.golang.org/protobuf/encoding/protojson");
const MAX_DURATION_SECONDS: i64 = 315_576_000_000;
const MIN_TIMESTAMP_SECONDS: i64 = -62_135_596_800;
const MAX_TIMESTAMP_SECONDS: i64 = 253_402_300_799;

pub(crate) fn marshal_text(message: &DynamicMessage) -> Result<String, BoxError> {
    match message.descriptor().full_name() {
        "google.protobuf.Duration" => {
            let (seconds, nanos) = seconds_nanos(message)?;
            if !(-MAX_DURATION_SECONDS..=MAX_DURATION_SECONDS).contains(&seconds) { return Err(ERRORS.New("Duration: seconds out of range")); }
            if !(-999_999_999..=999_999_999).contains(&nanos) { return Err(ERRORS.New("Duration: nanos out of range")); }
            if seconds > 0 && nanos < 0 || seconds < 0 && nanos > 0 { return Err(ERRORS.New("Duration: signs of seconds and nanos do not match")); }
            let sign = if seconds < 0 || nanos < 0 { "-" } else { "" };
            Ok(trim_fraction(format!("{sign}{}.{:09}", seconds.abs(), nanos.abs())) + "s")
        }
        "google.protobuf.Timestamp" => {
            let (seconds, nanos) = seconds_nanos(message)?;
            if !(MIN_TIMESTAMP_SECONDS..=MAX_TIMESTAMP_SECONDS).contains(&seconds) { return Err(ERRORS.New("Timestamp: seconds out of range")); }
            if !(0..=999_999_999).contains(&nanos) { return Err(ERRORS.New("Timestamp: nanos out of range")); }
            let time = chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, nanos as u32)
                .ok_or_else(|| ERRORS.New("Timestamp: value out of range"))?;
            Ok(trim_fraction(format!("{}.{nanos:09}", time.format("%Y-%m-%dT%H:%M:%S"))) + "Z")
        }
        "google.protobuf.FieldMask" => {
            let paths = message.get_field_by_name("paths").ok_or_else(|| ERRORS.New("FieldMask.paths is absent"))?;
            let Value::List(paths) = paths.as_ref() else { return Err(ERRORS.New("invalid FieldMask.paths")); };
            let mut result = Vec::with_capacity(paths.len());
            for path in paths {
                let Value::String(path) = path else { return Err(ERRORS.New("invalid FieldMask path")); };
                if !valid_full_name(path) { return Err(ERRORS.New(format!("FieldMask.paths contains invalid path: {path:?}"))); }
                let camel = camel_case(path);
                if snake_case(&camel) != *path { return Err(ERRORS.New(format!("FieldMask.paths contains irreversible value {path:?}"))); }
                result.push(camel);
            }
            Ok(result.join(","))
        }
        _ => Err(ERRORS.New("message has no text-shaped JSON mapping")),
    }
}
pub(crate) fn unmarshal_text(message: &mut DynamicMessage, text: &str) -> Result<(), BoxError> {
    match message.descriptor().full_name() {
        "google.protobuf.Duration" => {
            let (seconds, nanos) = parse_duration(text)?;
            if !(-MAX_DURATION_SECONDS..=MAX_DURATION_SECONDS).contains(&seconds) { return Err(ERRORS.New("Duration value out of range")); }
            set_seconds_nanos(message, seconds, nanos);
        }
        "google.protobuf.Timestamp" => {
            let (seconds, nanos) = parse_timestamp(text)?;
            if !(MIN_TIMESTAMP_SECONDS..=MAX_TIMESTAMP_SECONDS).contains(&seconds) { return Err(ERRORS.New("Timestamp value out of range")); }
            set_seconds_nanos(message, seconds, nanos);
        }
        "google.protobuf.FieldMask" => {
            let text = text.trim();
            if text.is_empty() { return Ok(()); }
            let field = message.descriptor().get_field_by_name("paths").expect("FieldMask.paths");
            let Value::List(paths) = message.get_field_mut(&field) else { return Err(ERRORS.New("invalid FieldMask.paths")); };
            for path in text.split(',') {
                let snake = snake_case(path);
                if path.contains('_') || !valid_full_name(&snake) { return Err(ERRORS.New(format!("FieldMask.paths contains invalid path: {path:?}"))); }
                paths.push(Value::String(snake));
            }
        }
        _ => return Err(ERRORS.New("message has no text-shaped JSON mapping")),
    }
    Ok(())
}
fn seconds_nanos(message: &DynamicMessage) -> Result<(i64, i32), BoxError> {
    let seconds = message.get_field_by_name("seconds").ok_or_else(|| ERRORS.New("missing seconds descriptor"))?;
    let nanos = message.get_field_by_name("nanos").ok_or_else(|| ERRORS.New("missing nanos descriptor"))?;
    match (seconds.as_ref(), nanos.as_ref()) {
        (Value::I64(seconds), Value::I32(nanos)) => Ok((*seconds, *nanos)),
        _ => Err(ERRORS.New("invalid seconds/nanos field kinds")),
    }
}
fn set_seconds_nanos(message: &mut DynamicMessage, seconds: i64, nanos: i32) {
    message.set_field_by_name("seconds", Value::I64(seconds));
    message.set_field_by_name("nanos", Value::I32(nanos));
}
fn trim_fraction(mut text: String) -> String {
    for _ in 0..2 { if text.ends_with("000") { text.truncate(text.len() - 3); } }
    if text.ends_with(".000") { text.truncate(text.len() - 4); }
    text
}
fn parse_duration(text: &str) -> Result<(i64, i32), BoxError> {
    let invalid = || ERRORS.New(format!("invalid Duration value {text:?}"));
    if text.len() < 2 { return Err(invalid()); }
    let mut input = text.strip_suffix('s').ok_or_else(invalid)?;
    let negative = input.starts_with('-');
    if input.starts_with(['+', '-']) { input = &input[1..]; }
    let bytes = input.as_bytes();
    let first = *bytes.first().ok_or_else(invalid)?;
    let count = if first == b'0' { 1 } else if (b'1'..=b'9').contains(&first) {
        bytes.iter().take_while(|byte| byte.is_ascii_digit()).count()
    } else if first == b'.' { 0 } else { return Err(invalid()); };
    let seconds: i64 = if count == 0 { 0 } else { input[..count].parse().map_err(|_| invalid())? };
    let mut nanos = 0_i32;
    if count < input.len() {
        let fraction = input[count..].strip_prefix('.').ok_or_else(invalid)?;
        if fraction.len() > 9 || !fraction.bytes().all(|byte| byte.is_ascii_digit()) { return Err(invalid()); }
        // Source parseDuration accepts an empty fractional part, including .s.
        for byte in fraction.bytes() { nanos = nanos * 10 + i32::from(byte - b'0'); }
        for _ in fraction.len()..9 { nanos *= 10; }
    }
    Ok(if negative { (-seconds, -nanos) } else { (seconds, nanos) })
}
fn parse_timestamp(text: &str) -> Result<(i64, i32), BoxError> {
    // Go 1.26.5 time.Parse first tries parseRFC3339 and then its layout parser.
    // The fallback accepts a one-digit hour, a comma fraction, and offsets of
    // exactly 24 hours or 60 minutes. chrono is used only for Gregorian dates.
    let invalid = || ERRORS.New(format!("invalid Timestamp value {text:?}"));
    let input = text.as_bytes(); let mut position = 0;
    fn digits(input: &[u8], position: &mut usize, minimum: usize, maximum: usize) -> Option<u32> {
        let start = *position; let mut value = 0;
        while *position < input.len() && *position - start < maximum && input[*position].is_ascii_digit() {
            value = value * 10 + u32::from(input[*position] - b'0'); *position += 1;
        }
        (*position - start >= minimum).then_some(value)
    }
    fn literal(input: &[u8], position: &mut usize, byte: u8) -> bool {
        if input.get(*position) != Some(&byte) { return false; }
        *position += 1; true
    }
    let year = digits(input, &mut position, 4, 4).ok_or_else(invalid)?;
    if !literal(input, &mut position, b'-') { return Err(invalid()); }
    let month = digits(input, &mut position, 2, 2).ok_or_else(invalid)?;
    if !literal(input, &mut position, b'-') { return Err(invalid()); }
    let day = digits(input, &mut position, 2, 2).ok_or_else(invalid)?;
    if !literal(input, &mut position, b'T') { return Err(invalid()); }
    let hour = digits(input, &mut position, 1, 2).ok_or_else(invalid)?;
    if !literal(input, &mut position, b':') { return Err(invalid()); }
    let minute = digits(input, &mut position, 2, 2).ok_or_else(invalid)?;
    if !literal(input, &mut position, b':') { return Err(invalid()); }
    let second = digits(input, &mut position, 2, 2).ok_or_else(invalid)?;
    let date = chrono::NaiveDate::from_ymd_opt(year as i32, month, day).ok_or_else(invalid)?;
    let time = date.and_hms_opt(hour, minute, second).ok_or_else(invalid)?;
    let mut nanos = 0_i32;
    if input.get(position).is_some_and(|byte| matches!(byte, b'.' | b',')) {
        let period = input[position] == b'.'; position += 1;
        let start = position;
        while let Some(byte) = input.get(position).filter(|byte| byte.is_ascii_digit()) {
            if position - start < 9 { nanos = nanos * 10 + i32::from(*byte - b'0'); }
            position += 1;
        }
        let count = position - start;
        if count == 0 { return Err(invalid()); }
        if period && count > 9 { return Err(ERRORS.New("invalid Timestamp fractional precision")); }
        for _ in count..9 { nanos *= 10; }
    }
    let offset = if literal(input, &mut position, b'Z') { 0_i64 } else {
        let sign = *input.get(position).ok_or_else(invalid)?; position += 1;
        if sign != b'+' && sign != b'-' { return Err(invalid()); }
        let hours = digits(input, &mut position, 2, 2).ok_or_else(invalid)?;
        if !literal(input, &mut position, b':') { return Err(invalid()); }
        let minutes = digits(input, &mut position, 2, 2).ok_or_else(invalid)?;
        if hours > 24 || minutes > 60 { return Err(invalid()); }
        let value = i64::from(hours * 60 + minutes) * 60;
        if sign == b'-' { -value } else { value }
    };
    if position != input.len() { return Err(invalid()); }
    Ok((time.and_utc().timestamp() - offset, nanos))
}
fn valid_full_name(name: &str) -> bool {
    name.split('.').all(|part| {
        let mut bytes = part.bytes();
        bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
            && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    })
}
fn camel_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len()); let mut underscore = false;
    for byte in name.bytes() {
        if byte != b'_' { result.push(char::from(if underscore { byte.to_ascii_uppercase() } else { byte })); }
        underscore = byte == b'_';
    }
    result
}
fn snake_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    // Invalid non-ASCII identifiers are rejected by valid_full_name afterwards.
    for character in name.chars() {
        if character.is_ascii_uppercase() { result.push('_'); result.push(character.to_ascii_lowercase()); }
        else { result.push(character); }
    }
    result
}
