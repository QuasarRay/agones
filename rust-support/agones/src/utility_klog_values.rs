//! Source klog WithValues and format-before-deduplication control.
// Adapted from pinned klog/v2/internal/serialize/keyvalues.go and the Go1.21+
// keyvalues_slog.go. Copyright 2021/2023 The Kubernetes Authors, Apache-2.0.
// Value dispatch is supplied by the actual caller; this is not an object,
// formatter/provider registry or an application lifecycle.
use crate::{utility_error_handlers::{SourceErrorArgument, SourceErrorArguments},
    utility_format_copy::{SourceFmtValueCopy, SourceFmtSharedString},
    utility_http::{HeaderAllocationProfile, SourceSliceAllocationError, SourceUnicodeProfile,
        NativeSourceCollectionContext, complete_source_collection, reserve_source_array},
    utility_go_quote::append};

fn add_source_int(profile: HeaderAllocationProfile, left: i64, right: i64) -> i64 {
    let sum = left.wrapping_add(right);
    if profile.word_bits() == 32 { i64::from(sum as i32) } else { sum }
}
/// Empty new values preserve the original header. Otherwise the returned
/// sink gets a fresh backing, preserving argument references and padding odd
/// total length with the literal source string (MISSING).
pub(crate) fn with_values(old: &SourceErrorArguments, new: &SourceErrorArguments,
    profile: HeaderAllocationProfile) -> SourceErrorArguments
{
    if new.len() == 0 { return old.source_fmt_copy(); }
    let mut length = add_source_int(profile, old.len() as i64, new.len() as i64);
    let missing = length % 2 != 0;
    if missing { length = add_source_int(profile, length, 1); }
    let mut values = SourceErrorArguments::make(0, length, profile);
    for index in 0..old.len() { values.append(old.get(index)); }
    for index in 0..new.len() { values.append(new.get(index)); }
    if missing { values.append(SourceErrorArgument::from_value("(MISSING)")); }
    values
}

#[derive(Clone, Copy)]
struct Interval { start: usize, end: usize }
struct Existing { key: Vec<u8>, interval: Interval }

/// Every value formatter runs, including for overwritten or identical pairs.
/// The formatter returns the actual rendered key bytes used for equality.
/// The source's first identical pair keeps its position; a changed later value
/// removes the earlier bytes and takes the later position.
pub(crate) fn format_kvs(output: &mut Vec<u8>, groups: &[SourceErrorArguments],
    mut format: impl FnMut(&mut Vec<u8>, &SourceErrorArgument, &SourceErrorArgument)
        -> Result<Vec<u8>, SourceSliceAllocationError>) -> Result<(), SourceSliceAllocationError>
{
    // These are the source's two initial capacity-32 scratch slices. Native
    // Vec storage is a projection, not source stack/escape/allocation evidence.
    let mut existing = reserve_source_array::<Existing>(32)?;
    let mut obsolete = reserve_source_array::<Interval>(32)?;
    for values in groups {
        for index in (0..values.len()).step_by(2) {
            let key = values.get(index);
            let value = if index + 1 < values.len() { values.get(index + 1) }
                else { SourceErrorArgument::from_value("(MISSING)") };
            let start = output.len();
            let key = format(output, &key, &value)?;
            let interval = Interval { start, end: output.len() };
            if let Some(position) = existing.iter().position(|entry| entry.key == key) {
                let previous = existing[position].interval;
                if output[previous.start..previous.end] == output[interval.start..interval.end] {
                    obsolete.try_reserve(1).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
                    obsolete.push(interval);
                } else {
                    // Source BinarySearchFunc uses the start index only.
                    let mut low = 0; let mut high = obsolete.len();
                    while low < high {
                        let middle = low + (high - low) / 2;
                        if obsolete[middle].start < previous.start { low = middle + 1; } else { high = middle; }
                    }
                    obsolete.try_reserve(1).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
                    obsolete.insert(low, previous);
                    existing[position].interval = interval;
                }
            } else {
                existing.try_reserve(1).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
                existing.push(Existing { key, interval });
            }
        }
    }
    if let Some(first) = obsolete.first().copied() {
        let original_length = output.len();
        let mut from = first.end;
        let mut destination = first.start;
        for interval in &obsolete[1..] {
            if from == interval.start { from = interval.end; continue; }
            output.copy_within(from..interval.start, destination);
            destination += interval.start - from;
            from = interval.end;
        }
        output.copy_within(from..original_length, destination);
        destination += original_length - from;
        output.truncate(destination);
    }
    Ok(())
}

/// writeStringValue preserves raw bytes and the exact source multiline
/// delimiters. A final LF does not produce an extra empty indented line.
pub(crate) fn write_string_value(output: &mut Vec<u8>, value: &[u8], profile: SourceUnicodeProfile)
    -> Result<(), SourceSliceAllocationError>
{
    if !value.contains(&b'\n') {
        append(output, b"=")?;
        return append(output, crate::utility_go_quote::try_source_go_quote(value, profile)?.as_bytes());
    }
    append(output, b"=<\n")?;
    let mut tail = value;
    while let Some(index) = tail.iter().position(|byte| *byte == b'\n') {
        append(output, b"\t")?; append(output, &tail[..=index])?;
        tail = &tail[index + 1..];
    }
    if tail.is_empty() { append(output, b" >") }
    else { append(output, b"\t")?; append(output, tail)?; append(output, b"\n >") }
}

fn string_bytes(value: &SourceErrorArgument) -> Option<&[u8]> {
    if let Some(value) = value.downcast_ref::<SourceFmtSharedString>() { return Some(&value.0); }
    if let Some(value) = value.downcast_ref::<&'static str>() { return Some(value.as_bytes()); }
    if let Some(value) = value.downcast_ref::<String>() { return Some(value.as_bytes()); }
    None
}
/// The apimachinery logPanic caller has already converted every field to a Go
/// string (panic, panicGoValue and stacktrace). It uses this concrete branch;
/// arbitrary klog value/Stringer/Marshaler/slog dispatch remains separate.
pub(crate) fn format_panic_strings(output: &mut Vec<u8>, values: SourceErrorArguments, profile: SourceUnicodeProfile)
    -> Result<(), SourceSliceAllocationError>
{
    format_kvs(output, &[values], |output, key, value| {
        let key = string_bytes(key).expect("logPanic supplies concrete string keys");
        let value = string_bytes(value).expect("logPanic supplies preformatted string values");
        append(output, b" ")?; append(output, key)?;
        write_string_value(output, value, profile)?;
        let mut copied = reserve_source_array::<u8>(key.len() as u64)?;
        copied.extend_from_slice(key); Ok(copied)
    })
}
pub(crate) fn panic_fields(profile: HeaderAllocationProfile, fields: &[(&'static str, &[u8])]) -> SourceErrorArguments {
    let length = fields.len().checked_mul(2).and_then(|length| i64::try_from(length).ok());
    let length = complete_source_collection(&NativeSourceCollectionContext, length.ok_or(SourceSliceAllocationError::MakeCapacity));
    let mut values = SourceErrorArguments::make(0, length, profile);
    for (key, value) in fields {
        values.append(SourceErrorArgument::from_value(*key));
        values.append(SourceErrorArgument::from_value(SourceFmtSharedString(std::sync::Arc::from(*value))));
    }
    with_values(&SourceErrorArguments::nil_with_profile(profile), &values, profile)
}
