//! Source url.Values, query parsing and the fixed gateway's ParseForm path.
// Adapted from Go1.26.5/1.27.1 net/url and net/http/request.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{Headers, HeaderValues, HeaderAllocationProfile, HttpRequest, SourceReadError};
use crate::errors::{BoxError, Errors};
use bytes::Bytes;
use cgp::prelude::*;
use serde::Serialize;
use std::{error::Error, fmt, sync::{Arc, RwLock, atomic::{AtomicU64, Ordering}}};

const QUERY_ERRORS: Errors = Errors::from_package("net/url");
const FORM_ERRORS: Errors = Errors::from_package("net/http");
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(transparent)]
pub struct SourceValues(Headers);
impl SourceValues {
    pub fn new(profile: HeaderAllocationProfile) -> Self { Self(Headers::with_profile(profile)) }
    pub fn is_nil(&self) -> bool { self.0.is_nil() }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn contains_key(&self, key: impl AsRef<[u8]>) -> bool { self.0.contains_key(key) }
    pub fn get(&self, key: impl AsRef<[u8]>) -> Bytes { self.0.get(key).and_then(|values| values.first()).unwrap_or_default() }
    pub fn get_values(&self, key: impl AsRef<[u8]>) -> Option<HeaderValues> { self.0.get(key) }
    pub fn insert(&self, key: impl AsRef<[u8]>, values: HeaderValues) { self.0.insert(key, values); }
    pub fn remove(&self, key: impl AsRef<[u8]>) -> Option<HeaderValues> { self.0.remove(key) }
    pub fn iter(&self) -> impl Iterator<Item = (super::HeaderName, HeaderValues)> { self.0.iter() }
    pub fn add(&self, key: impl AsRef<[u8]>, value: impl Into<Bytes>) { self.0.append_exact_values(key, &[value.into()]); }
    pub fn copy_values(&self, source: &Self) {
        for (key, values) in &source.0 { self.0.append_exact_slice(key, &values); }
    }
}

/// The selected source word size and the value of urlmaxqueryparams, after
/// GODEBUG/default selection. Application/tool binding belongs to M0027;
/// this data profile never infers source behavior from the Rust host.
#[derive(Clone, Debug)]
pub struct SourceQueryProfile { pub allocation: HeaderAllocationProfile, pub urlmaxqueryparams: Bytes }
impl Default for SourceQueryProfile {
    fn default() -> Self { Self { allocation: HeaderAllocationProfile::GO126_64, urlmaxqueryparams: Bytes::new() } }
}
pub struct SourceQuerySettings { profile: RwLock<SourceQueryProfile>, nondefault: AtomicU64 }
impl Default for SourceQuerySettings {
    fn default() -> Self { Self { profile: RwLock::new(SourceQueryProfile::default()), nondefault: AtomicU64::new(0) } }
}
impl SourceQuerySettings {
    pub fn profile(&self) -> SourceQueryProfile { self.profile.read().unwrap_or_else(|error| error.into_inner()).clone() }
    pub fn set_profile(&self, profile: SourceQueryProfile) { *self.profile.write().unwrap_or_else(|error| error.into_inner()) = profile; }
    pub fn nondefault_count(&self) -> u64 { self.nondefault.load(Ordering::SeqCst) }
    fn allows(&self, query: &[u8], profile: &SourceQueryProfile) -> bool {
        let count = (query.iter().filter(|byte| **byte == b'&').count() as u64).wrapping_add(1);
        let params = if profile.allocation.word_bits() == 32 { count as u32 as i32 as i64 } else { count as i64 };
        let within_default = params <= 10000;
        // The source checks Value() for empty, then calls Value() again for
        // Atoi. Retain that second read when settings change between calls.
        if profile.urlmaxqueryparams.is_empty() { return within_default; }
        let selected = self.profile();
        let bytes = selected.urlmaxqueryparams.as_ref();
        let digits = match bytes.first() { Some(b'+' | b'-') => &bytes[1..], _ => bytes };
        if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) { return within_default; }
        let custom = std::str::from_utf8(bytes).ok().and_then(|value| value.parse::<i64>().ok())
            .filter(|value| profile.allocation.word_bits() != 32 || i32::try_from(*value).is_ok());
        let Some(custom) = custom else { return within_default; };
        let within_custom = custom == 0 || params < custom;
        if within_default != within_custom { self.nondefault.fetch_add(1, Ordering::SeqCst); }
        within_custom
    }
}
#[cgp_getter]
pub trait HasSourceQuerySettings { fn source_query_settings(&self) -> &Arc<SourceQuerySettings>; }

/// Parsing always returns a non-nil map of valid pairs, including on error.
pub struct SourceQueryParse { pub values: SourceValues, pub error: Option<BoxError> }
pub fn parse_source_query(query: &[u8], settings: &SourceQuerySettings) -> SourceQueryParse {
    let profile = settings.profile();
    let values = SourceValues::new(profile.allocation);
    if !settings.allows(query, &profile) {
        return SourceQueryParse { values, error: Some(QUERY_ERRORS.New("number of URL query parameters exceeded limit")) };
    }
    let mut error = None;
    for pair in query.split(|byte| *byte == b'&') {
        if pair.contains(&b';') { error = Some(QUERY_ERRORS.New("invalid semicolon separator in query")); continue; }
        if pair.is_empty() { continue; }
        let equal = pair.iter().position(|byte| *byte == b'=');
        let (key, value) = equal.map_or((pair, &[][..]), |index| (&pair[..index], &pair[index+1..]));
        let key = match unescape_query(key) { Ok(value) => value, Err(next) => { if error.is_none() { error = Some(next); } continue; } };
        let value = match unescape_query(value) { Ok(value) => value, Err(next) => { if error.is_none() { error = Some(next); } continue; } };
        values.add(key, value);
    }
    SourceQueryParse { values, error }
}
#[derive(Debug)]
pub struct SourceUrlEscapeError { pub bytes: Bytes }
impl fmt::Display for SourceUrlEscapeError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Retain the typed original escape. Full strconv.Quote Unicode/
        // arbitrary-byte diagnostics stay in the shared formatting gap.
        write!(out, "invalid URL escape {:?}", String::from_utf8_lossy(&self.bytes))
    }
}
impl Error for SourceUrlEscapeError {}
fn unescape_query(input: &[u8]) -> Result<Bytes, BoxError> {
    let mut result = Vec::with_capacity(input.len()); let mut index = 0;
    let digit = |byte| match byte { b'0'..=b'9' => Some(byte-b'0'), b'a'..=b'f' => Some(byte-b'a'+10), b'A'..=b'F' => Some(byte-b'A'+10), _ => None };
    while index < input.len() {
        match input[index] {
            b'+' => result.push(b' '),
            b'%' => {
                let pair = input.get(index+1..index+3).and_then(|pair| Some((digit(pair[0])?, digit(pair[1])?)));
                let Some((high, low)) = pair else { return Err(Box::new(SourceUrlEscapeError { bytes: Bytes::copy_from_slice(&input[index..input.len().min(index+3)]) })); };
                result.push(high*16+low); index += 2;
            }
            byte => result.push(byte),
        }
        index += 1;
    }
    Ok(Bytes::from(result))
}

impl HttpRequest {
    /// Source ParseForm reached only through the fixed gateway's exact POST +
    /// application/x-www-form-urlencoded fallback predicate. General MIME/
    /// multipart/MaxBytesReader APIs remain their separate source boundaries.
    pub(crate) async fn parse_gateway_form(&mut self) -> Result<SourceValues, BoxError> {
        let profile = self.runtime.source_query_settings().profile();
        let mut error = None;
        if self.fields.post_form().is_nil() {
            let method = self.method();
            if matches!(method.as_ref(), b"POST" | b"PUT" | b"PATCH") {
                if self.body().is_nil() { error = Some(FORM_ERRORS.New("missing form body")); }
                else {
                    let read = self.read_body_all_limited(Some((10 << 20) + 1)).await;
                    if let Some(SourceReadError::Failure(next)) = read.error { error = Some(next); }
                    else if read.data.len() > (10 << 20) { error = Some(FORM_ERRORS.New("http: POST too large")); }
                    else {
                        let parsed = parse_source_query(&read.data, self.runtime.source_query_settings());
                        self.fields.set_post_form(parsed.values); error = parsed.error;
                    }
                }
            }
            if self.fields.post_form().is_nil() { self.fields.set_post_form(SourceValues::new(profile.allocation)); }
        }
        if self.fields.form().is_nil() {
            let post = self.fields.post_form();
            if !post.is_empty() {
                let form = SourceValues::new(profile.allocation); form.copy_values(&post); self.fields.set_form(form);
            }
            let parsed = self.url().map(|url| parse_source_query(&url.raw_query(), self.runtime.source_query_settings()));
            let query = if let Some(parsed) = parsed { if error.is_none() { error = parsed.error; } parsed.values }
                else { SourceValues::new(profile.allocation) };
            if self.fields.form().is_nil() { self.fields.set_form(query); }
            else { self.fields.form().copy_values(&query); }
        }
        match error { Some(error) => Err(error), None => Ok(self.fields.form()) }
    }
}
