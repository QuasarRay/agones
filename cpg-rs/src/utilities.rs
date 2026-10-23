//! Native equivalents of the handwritten io.shiftleft.utils utilities and Implicits.
use crate::{Error, Result};
use std::{collections::HashSet, ffi::OsString, iter::Peekable, path::{Path, PathBuf}, sync::{Arc, OnceLock}};

pub const DEFAULT_MAX_STRING_LENGTH: i32 = 1024;
pub const DEFAULT_INITIAL_SIZE: i32 = 64 * 1024;

/// Arc ownership preserves the reference reuse which is the purpose of the source interner.
pub trait StringInterner {
    fn intern(&mut self, value: Arc<str>) -> Arc<str>;
    fn intern_nullable(&mut self, value: Option<Arc<str>>) -> Result<Option<Arc<str>>> {
        value.map(|value| self.intern(value)).map(Some)
            .ok_or_else(|| Error::Invalid("null StringInterner input".into()))
    }
}
pub struct NoopStringInterner;
impl StringInterner for NoopStringInterner {
    fn intern(&mut self, value: Arc<str>) -> Arc<str> { value }
    fn intern_nullable(&mut self, value: Option<Arc<str>>) -> Result<Option<Arc<str>>> { Ok(value) }
}
pub struct StrongStringInterner {
    max_string_length: i32,
    strings: HashSet<Arc<str>>,
}
impl Default for StrongStringInterner {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_STRING_LENGTH, DEFAULT_INITIAL_SIZE).expect("positive source default capacity")
    }
}
impl StrongStringInterner {
    pub fn new(max_string_length: i32, initial_size: i32) -> Result<Self> {
        if initial_size < 0 { return Err(Error::Invalid("negative interner initial size".into())); }
        Ok(Self { max_string_length, strings: HashSet::with_capacity(initial_size as usize) })
    }
    pub fn cached_strings(&self) -> usize { self.strings.len() }
}
impl StringInterner for StrongStringInterner {
    fn intern(&mut self, value: Arc<str>) -> Arc<str> {
        // java.lang.String.length counts UTF-16 code units, with a strict less-than threshold.
        if self.max_string_length > 0 && value.encode_utf16().count() < self.max_string_length as usize {
            if let Some(cached) = self.strings.get(value.as_ref()) { return cached.clone(); }
            self.strings.insert(value.clone());
        }
        value
    }
}

pub const UTF8_BOM_LENGTH: usize = 3;
pub fn starts_with_utf8_bom(bytes: &[u8]) -> bool { bytes.starts_with(&[0xef, 0xbb, 0xbf]) }

/// Java's UTF-8 decoder consumes a complete encoded surrogate as one malformed group.
/// Rust's from_utf8_lossy consumes its three bytes separately. Retain the source decoder's
/// replacement grouping, including valid prefixes of truncated/malformed sequences.
pub fn decode_utf8_replacing(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let first = bytes[index];
        if first < 0x80 { result.push(first as char); index += 1; continue; }
        let width = match first { 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => 1 };
        let mut consumed = 1;
        let mut scalar = (first & (0x7f >> width)) as u32;
        if width > 1 {
            while consumed < width && index + consumed < bytes.len() {
                let byte = bytes[index + consumed];
                if byte & 0xc0 != 0x80 || consumed == 1 && (
                    first == 0xe0 && byte < 0xa0 || first == 0xf0 && byte < 0x90 || first == 0xf4 && byte > 0x8f
                ) { break; }
                scalar = (scalar << 6) | u32::from(byte & 0x3f);
                consumed += 1;
            }
        }
        if width > 1 && consumed == width {
            result.push(char::from_u32(scalar).unwrap_or('\u{fffd}'));
        } else { result.push('\u{fffd}'); }
        index += consumed;
    }
    result
}
pub fn skip_bom_if_present(value: &str) -> &str { value.strip_prefix('\u{feff}').unwrap_or(value) }
pub fn content_from_buffered_source(bytes: &[u8]) -> Vec<String> {
    let decoded = decode_utf8_replacing(bytes);
    let text = skip_bom_if_present(&decoded);
    let mut lines = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices().peekable();
    while let Some((index, character)) = chars.next() {
        if character == '\r' || character == '\n' {
            lines.push(text[start..index].to_owned());
            start = index + character.len_utf8();
            if character == '\r' && chars.peek().is_some_and(|(_, next)| *next == '\n') {
                start = chars.next().expect("peeked newline").0 + 1;
            }
        }
    }
    if start < text.len() { lines.push(text[start..].to_owned()); }
    lines
}
pub fn read_lines_in_file(path: &Path) -> Result<Vec<String>> {
    Ok(content_from_buffered_source(&std::fs::read(path)?))
}
pub fn read_entire_file(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)?;
    let bytes = if starts_with_utf8_bom(&bytes) { &bytes[UTF8_BOM_LENGTH..] } else { &bytes };
    Ok(decode_utf8_replacing(bytes))
}

/// The temp-file crate owns the OS resource; explicit close retains deleteIfExists semantics.
pub struct TempFileCopy { path: tempfile::TempPath }
impl TempFileCopy {
    pub fn new(source: &Path) -> Result<Self> { Self::with_options(source, "resource", ".tmp", None) }
    pub fn with_options(source: &Path, prefix: &str, suffix: &str, directory: Option<&Path>) -> Result<Self> {
        if prefix.encode_utf16().count() < 3 { return Err(Error::Invalid("temporary file prefix shorter than three characters".into())); }
        let mut builder = tempfile::Builder::new();
        builder.prefix(prefix).suffix(suffix);
        let file = match directory { Some(directory) => builder.tempfile_in(directory)?, None => builder.tempfile()? };
        let path = file.into_temp_path();
        std::fs::copy(source, &path)?;
        Ok(Self { path })
    }
    pub fn path(&self) -> &Path { &self.path }
    pub fn close(&mut self) -> Result<bool> {
        match std::fs::remove_file(&self.path) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error.into()),
        }
    }
}

pub const FILE_NAME_TO_SEARCH_FOR_ENV_VAR: &str = "FILE_IN_PROJECT_ROOT";
pub const SEARCH_DEPTH: usize = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchDepthExceededError;
impl std::fmt::Display for SearchDepthExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("project root search depth exceeded") }
}
impl std::error::Error for SearchDepthExceededError {}
pub struct ProjectRoot { filename_to_search_for: OsString }
impl ProjectRoot {
    pub fn new(filename_to_search_for: OsString) -> Self { Self { filename_to_search_for } }
    pub fn from_environment() -> Self {
        static FILENAME: OnceLock<OsString> = OnceLock::new();
        Self::new(FILENAME.get_or_init(|| std::env::var_os(FILE_NAME_TO_SEARCH_FOR_ENV_VAR)
            .unwrap_or_else(|| ".git".into())).clone())
    }
    pub fn find_relative_from(&self, directory: &Path) -> std::result::Result<String, SearchDepthExceededError> {
        for depth in 0..=SEARCH_DEPTH {
            let prefix = format!("./{}", "../".repeat(depth));
            if directory.join(&prefix).join(&self.filename_to_search_for).exists() { return Ok(prefix); }
        }
        Err(SearchDepthExceededError)
    }
    pub fn find_relative_path(&self) -> std::result::Result<String, SearchDepthExceededError> {
        self.find_relative_from(Path::new("."))
    }
    pub fn find(&self) -> std::result::Result<PathBuf, SearchDepthExceededError> { self.find_relative_path().map(PathBuf::from) }
    pub fn relativise(&self, path: &str) -> std::result::Result<String, SearchDepthExceededError> {
        Ok(format!("{}{path}", self.find_relative_path()?))
    }
}

/// Calling iterator on a collection creates a fresh cursor; on an Iterator it returns itself.
/// The deprecated upstream method obtains an iterator separately for each of its three probes.
pub trait IterableOnceSource {
    type Item;
    fn iterator_has_next(&mut self) -> bool;
    fn iterator_next(&mut self) -> Option<Self::Item>;
}
pub struct OnceIterator<I: Iterator>(pub Peekable<I>);
impl<I: Iterator> OnceIterator<I> { pub fn new(iterator: I) -> Self { Self(iterator.peekable()) } }
impl<I: Iterator> IterableOnceSource for OnceIterator<I> {
    type Item = I::Item;
    fn iterator_has_next(&mut self) -> bool { self.0.peek().is_some() }
    fn iterator_next(&mut self) -> Option<Self::Item> { self.0.next() }
}
pub struct ReusableIterable<I>(pub I);
impl<I: Iterator + Clone> IterableOnceSource for ReusableIterable<I> {
    type Item = I::Item;
    fn iterator_has_next(&mut self) -> bool { self.0.clone().next().is_some() }
    fn iterator_next(&mut self) -> Option<Self::Item> { self.0.clone().next() }
}
pub fn only_checked_with_warning<S: IterableOnceSource>(source: &mut S, mut warn: impl FnMut(&str)) -> Result<S::Item> {
    if !source.iterator_has_next() { return Err(Error::Invalid("NoSuchElementException: empty iterable".into())); }
    let result = source.iterator_next().ok_or_else(|| Error::Invalid("NoSuchElementException: empty iterator".into()))?;
    if source.iterator_has_next() { warn("iterator was expected to have exactly one element, but it actually has more"); }
    Ok(result)
}
pub fn only_checked<S: IterableOnceSource>(source: &mut S) -> Result<S::Item> {
    only_checked_with_warning(source, |message| tracing::warn!("{message}"))
}

/// Java Integer.parseInt semantics, including signed limits and BMP decimal digits.
pub fn parse_java_integer_units(units:&[u16])->Option<i32> {
    static ZEROES:std::sync::OnceLock<Vec<u16>>=std::sync::OnceLock::new();
    let zeroes=ZEROES.get_or_init(||serde_json::from_str(include_str!("../templates/java-decimal-zeroes.json")).expect("pinned decimal character table"));
    let first=*units.first()?;let negative=first==u16::from(b'-');
    let start=usize::from(negative || first==u16::from(b'+'));
    if start==units.len() {return None;}
    let limit=if negative {i32::MIN} else {-i32::MAX};let minimum=limit/10;
    let mut result=0i32;
    for unit in &units[start..] {
        let digit=zeroes.iter().find_map(|zero|unit.checked_sub(*zero).filter(|digit|*digit<10).map(i32::from))?;
        if result<minimum {return None;}
        result*=10;
        if result<limit+digit {return None;}
        result-=digit;
    }
    Some(if negative {result} else {-result})
}
