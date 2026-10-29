//! Source klog caller path selection and fixed-width byte header.
// klog/v2/klog.go and internal/buffer/buffer.go; Google Inc./Kubernetes
// Authors, Apache-2.0. Native caller/clock/layout/allocator profiles remain
// required; a missing caller binding is not runtime.Caller returning false.
use std::sync::Arc;
use crate::{utility_runtime::UtilityRuntimeContext, utility_time_values::SourceGoTime,
    utility_format_copy::SourceFmtSharedString, utility_go_quote::append,
    utility_klog_dispatch::FormatResult,
    utility_http::SourceSliceAllocationError,
    utility_panic::{SourceBoundsError, SourceBoundsCode, unwind_source_runtime_error}};

pub struct SourceKlogCallSite { pub file: SourceFmtSharedString, pub line: i64 }
pub type SourceKlogCaller = Arc<dyn Fn(&UtilityRuntimeContext, i64) -> FormatResult<Option<SourceKlogCallSite>> + Send + Sync>;
pub type SourceKlogNow = Arc<dyn Fn(&UtilityRuntimeContext) -> FormatResult<SourceGoTime> + Send + Sync>;
#[derive(Clone)]
pub struct SourceKlogHeaderBindings { pub caller: SourceKlogCaller, pub now: SourceKlogNow }
pub(crate) fn source_file(site: Option<SourceKlogCallSite>, add_directory: bool) -> SourceKlogCallSite {
    let Some(mut site) = site else { return SourceKlogCallSite { file: SourceFmtSharedString(Arc::from(&b"???"[..])), line: 1 }; };
    if let Some(slash) = site.file.0.iter().rposition(|byte| *byte == b'/') {
        let from = if add_directory {
            site.file.0[..slash].iter().rposition(|byte| *byte == b'/').map_or(slash + 1, |previous| previous + 1)
        } else { slash + 1 };
        site.file = SourceFmtSharedString(Arc::from(&site.file.0[from..]));
    }
    site
}
fn digit(index: i64) -> u8 {
    if index as u64 >= 10 {
        unwind_source_runtime_error(SourceBoundsError { x: index, y: 10, signed: true, code: SourceBoundsCode::Index });
    }
    b"0123456789"[index as usize]
}
fn two_digits(buffer: &mut [u8; 64], index: usize, mut value: i64) {
    buffer[index + 1] = digit(value % 10); value /= 10; buffer[index] = digit(value % 10);
}
fn n_digits(buffer: &mut [u8; 64], width: usize, index: usize, mut value: i64, padding: u8) {
    let mut remaining = width;
    while remaining > 0 && value > 0 { remaining -= 1; buffer[index + remaining] = digit(value % 10); value /= 10; }
    while remaining > 0 { remaining -= 1; buffer[index + remaining] = padding; }
}
fn some_digits(buffer: &mut [u8; 64], index: usize, mut value: i64) -> usize {
    let mut from = buffer.len();
    loop { from -= 1; buffer[from] = digit(value % 10); value /= 10; if value == 0 { break; } }
    let count = buffer.len() - from; buffer.copy_within(from.., index); count
}
pub(crate) fn format_header(output: &mut Vec<u8>, mut severity: i32, site: &SourceKlogCallSite,
    time: &SourceGoTime, pid: i64) -> FormatResult {
    let line = site.line.max(0);
    if severity > 3 { severity = 0; }
    if severity < 0 { unwind_source_runtime_error(SourceBoundsError {
        x: i64::from(severity), y: 4, signed: true, code: SourceBoundsCode::Index }); }
    // Source Date and Clock each compute their own local absolute seconds.
    let (_, month, day) = time.date()?; let (hour, minute, second) = time.clock()?;
    let mut buffer = [0_u8; 64]; buffer[0] = b"IWEF"[severity as usize];
    two_digits(&mut buffer, 1, month); two_digits(&mut buffer, 3, day); buffer[5] = b' ';
    two_digits(&mut buffer, 6, hour); buffer[8] = b':'; two_digits(&mut buffer, 9, minute);
    buffer[11] = b':'; two_digits(&mut buffer, 12, second); buffer[14] = b'.';
    n_digits(&mut buffer, 6, 15, i64::from(time.nanosecond()) / 1000, b'0'); buffer[21] = b' ';
    // nDigits intentionally discards high PID digits, and a negative PID
    // produces seven spaces. A generic width formatter changes this behavior.
    n_digits(&mut buffer, 7, 22, pid, b' '); buffer[29] = b' ';
    append(output, &buffer[..30])?; append(output, &site.file.0)?;
    buffer[0] = b':'; let count = some_digits(&mut buffer, 1, line);
    buffer[count + 1] = b']'; buffer[count + 2] = b' ';
    append(output, &buffer[..count + 3])?; Ok(())
}
pub(crate) fn append_body(output: &mut Vec<u8>, body: &[u8]) -> Result<(), SourceSliceAllocationError> {
    append(output, body)?;
    if output.last() != Some(&b'\n') { append(output, b"\n")?; }
    Ok(())
}
