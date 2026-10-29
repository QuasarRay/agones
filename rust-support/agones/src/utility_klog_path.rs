//! Required byte-preserving filepath Base/Join/Clean operations for klog.
// Go1.26.5 path/filepath and internal/filepathlite. Copyright The Go Authors,
// BSD-3-Clause (../GO-LICENSE). Native string aliases/allocation/word/ABI and
// platform filesystem semantics remain required; this is no general path API.
use crate::{utility_klog_dispatch::FormatResult, utility_go_quote::append, utility_http::reserve_source_array};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceKlogPathFlavor { Unix, Windows, Plan9 }
impl SourceKlogPathFlavor {
    fn separator(self) -> u8 { if self == Self::Windows { b'\\' } else { b'/' } }
    fn is_separator(self, byte: u8) -> bool { byte == b'/' || self == Self::Windows && byte == b'\\' }
    fn volume(self, path: &[u8]) -> usize {
        if self != Self::Windows { return 0; }
        if path.len() >= 2 && path[1] == b':' { return 2; }
        if path.first().is_none_or(|byte| !self.is_separator(*byte)) { return 0; }
        if prefix_fold(path, br"\\.\UNC") { return unc_length(path, 8); }
        if [br"\\.".as_slice(), br"\\?".as_slice(), br"\??".as_slice()].iter().any(|prefix| prefix_fold(path, prefix)) {
            if path.len() == 3 { return 3; }
            return path[4..].iter().position(|byte| self.is_separator(*byte)).map_or(path.len(), |index| 4 + index);
        }
        if path.len() >= 2 && self.is_separator(path[1]) { return unc_length(path, 2); }
        0
    }
    fn from_slash(self, mut path: Vec<u8>) -> Vec<u8> {
        if self == Self::Windows { for byte in &mut path { if *byte == b'/' { *byte = b'\\'; } } }
        path
    }
    pub fn base(self, original: &[u8]) -> Vec<u8> {
        if original.is_empty() { return b".".to_vec(); }
        let mut end = original.len();
        while end > 0 && self.is_separator(original[end - 1]) { end -= 1; }
        let path = &original[..end]; let path = &path[self.volume(path)..];
        let start = path.iter().rposition(|byte| self.is_separator(*byte)).map_or(0, |index| index + 1);
        if start == path.len() { vec![self.separator()] } else { path[start..].to_vec() }
    }
    pub fn join(self, elements: &[&[u8]]) -> FormatResult<Vec<u8>> {
        let mut joined = Vec::new();
        if self != Self::Windows {
            let Some(first) = elements.iter().position(|element| !element.is_empty()) else { return Ok(joined); };
            for (index, element) in elements[first..].iter().enumerate() {
                if index > 0 { append(&mut joined, b"/")?; } append(&mut joined, element)?;
            }
        } else {
            let mut last = 0;
            for &element in elements {
                let mut element = element;
                if joined.is_empty() {}
                else if self.is_separator(last) {
                    while element.first().is_some_and(|byte| self.is_separator(*byte)) { element = &element[1..]; }
                    if joined.len() == 1 && element.starts_with(b"??") && (element.len() == 2 || self.is_separator(element[2])) {
                        append(&mut joined, br".\")?;
                    }
                } else if last != b':' { append(&mut joined, br"\")?; last = b'\\'; }
                if !element.is_empty() { append(&mut joined, element)?; last = element[element.len() - 1]; }
            }
            if joined.is_empty() { return Ok(joined); }
        }
        self.clean(&joined)
    }
    pub fn clean(self, original: &[u8]) -> FormatResult<Vec<u8>> {
        let volume = self.volume(original); let path = &original[volume..];
        if path.is_empty() {
            if volume > 1 && self.is_separator(original[0]) && self.is_separator(original[1]) {
                return Ok(self.from_slash(original.to_vec()));
            }
            let mut output = original.to_vec(); append(&mut output, b".")?; return Ok(output);
        }
        let rooted = self.is_separator(path[0]); let mut output = LazyPath { path, bytes: None, written: 0 };
        let (mut read, mut dotdot) = (0, 0);
        if rooted { output.push(self.separator())?; read = 1; dotdot = 1; }
        while read < path.len() {
            if self.is_separator(path[read]) { read += 1; }
            else if path[read] == b'.' && (read + 1 == path.len() || self.is_separator(path[read + 1])) { read += 1; }
            else if path[read] == b'.' && path.get(read + 1) == Some(&b'.')
                && (read + 2 == path.len() || self.is_separator(path[read + 2])) {
                read += 2;
                if output.written > dotdot {
                    output.written -= 1;
                    while output.written > dotdot && !self.is_separator(output.at(output.written)) { output.written -= 1; }
                } else if !rooted {
                    if output.written > 0 { output.push(self.separator())?; }
                    output.push(b'.')?; output.push(b'.')?; dotdot = output.written;
                }
            } else {
                if rooted && output.written != 1 || !rooted && output.written != 0 { output.push(self.separator())?; }
                while read < path.len() && !self.is_separator(path[read]) { output.push(path[read])?; read += 1; }
            }
        }
        if output.written == 0 { output.push(b'.')?; }
        // Source postClean observes whether lazybuf allocated and scans its
        // backing, including bytes beyond written. Don't replace this with a
        // normalized-string comparison or truncate the backing prematurely.
        if self == Self::Windows && volume == 0 {
            if let Some(bytes) = &output.bytes {
                let colon = bytes.iter().take_while(|byte| !self.is_separator(**byte)).any(|byte| *byte == b':');
                let local_device = bytes.len() >= 3 && self.is_separator(bytes[0]) && bytes[1..3] == *b"??";
                if colon { output.prepend(br".\")?; }
                else if local_device { output.prepend(br"\.")?; }
            }
        }
        let mut result = original[..volume].to_vec(); append(&mut result, output.value())?;
        Ok(self.from_slash(result))
    }
}
struct LazyPath<'a> { path: &'a [u8], bytes: Option<Vec<u8>>, written: usize }
impl LazyPath<'_> {
    fn at(&self, index: usize) -> u8 { match &self.bytes { Some(bytes) => bytes[index], None => self.path[index] } }
    fn push(&mut self, byte: u8) -> FormatResult {
        if self.bytes.is_none() {
            if self.written < self.path.len() && self.path[self.written] == byte { self.written += 1; return Ok(()); }
            let mut bytes = reserve_source_array::<u8>(self.path.len() as u64)?; bytes.resize(self.path.len(), 0);
            bytes[..self.written].copy_from_slice(&self.path[..self.written]); self.bytes = Some(bytes);
        }
        self.bytes.as_mut().unwrap()[self.written] = byte; self.written += 1; Ok(())
    }
    fn prepend(&mut self, prefix: &[u8]) -> FormatResult {
        let mut bytes = Vec::new(); append(&mut bytes, prefix)?; append(&mut bytes, self.bytes.as_ref().unwrap())?;
        self.bytes = Some(bytes); self.written += prefix.len(); Ok(())
    }
    fn value(&self) -> &[u8] { match &self.bytes { Some(bytes) => &bytes[..self.written], None => &self.path[..self.written] } }
}
fn prefix_fold(path: &[u8], prefix: &[u8]) -> bool {
    let flavor = SourceKlogPathFlavor::Windows;
    path.len() >= prefix.len() && path[..prefix.len()].iter().zip(prefix).all(|(a, b)|
        if flavor.is_separator(*b) { flavor.is_separator(*a) } else { a.to_ascii_uppercase() == b.to_ascii_uppercase() })
        && (path.len() == prefix.len() || flavor.is_separator(path[prefix.len()]))
}
fn unc_length(path: &[u8], prefix: usize) -> usize {
    let mut count = 0;
    for (index, byte) in path.iter().enumerate().skip(prefix) {
        if SourceKlogPathFlavor::Windows.is_separator(*byte) { count += 1; if count == 2 { return index; } }
    }
    path.len()
}
