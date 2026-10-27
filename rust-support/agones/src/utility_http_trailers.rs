//! Source trailer declarations and wire fields; no transport or task owner.
// Adapted from Go net/http/{header,server,h2_bundle}.go and httpguts/guts.go.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::Headers;
use bytes::Bytes;

pub(super) fn field_name(name: &[u8]) -> bool {
    !name.is_empty() && name.iter().all(|byte| byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(byte))
}
pub(super) fn canonical_name(name: &[u8]) -> Vec<u8> {
    if !field_name(name) { return name.to_vec(); }
    let mut upper = true;
    name.iter().map(|byte| {
        let next = if upper { byte.to_ascii_uppercase() } else { byte.to_ascii_lowercase() };
        upper = *byte == b'-'; next
    }).collect()
}
pub(super) fn valid_trailer(name: &[u8]) -> bool {
    // Like httpguts.ValidTrailerHeader, this is the forbidden-field rule,
    // not a header-token check. Invalid field names are dropped at encoding.
    !name.starts_with(b"If-") && !matches!(name,
        b"Authorization" | b"Cache-Control" | b"Connection" | b"Content-Encoding" |
        b"Content-Length" | b"Content-Range" | b"Content-Type" | b"Expect" | b"Host" |
        b"Keep-Alive" | b"Max-Forwards" | b"Pragma" | b"Proxy-Authenticate" |
        b"Proxy-Authorization" | b"Proxy-Connection" | b"Range" | b"Realm" | b"Te" |
        b"Trailer" | b"Transfer-Encoding" | b"Www-Authenticate")
}
fn trim_space(mut value: &[u8]) -> &[u8] {
    while value.first().is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n')) { value = &value[1..]; }
    while value.last().is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n')) { value = &value[..value.len() - 1]; }
    value
}
pub(super) fn elements(value: &[u8]) -> impl Iterator<Item = &[u8]> {
    value.split(|byte| *byte == b',').map(trim_space).filter(|value| !value.is_empty())
}
pub(super) fn encode_http1(headers: &Headers) -> Bytes {
    // Header.Write: map keys sorted in byte order, original key spelling,
    // each value independently newline-replaced and SP/HT-trimmed.
    let mut encoded = Vec::new();
    for (name, values) in headers {
        if !field_name(name.as_bytes()) { continue; }
        for value in values.iter() {
            let value: Vec<_> = value.iter().map(|byte| if matches!(byte, b'\r' | b'\n') { b' ' } else { *byte }).collect();
            encoded.extend_from_slice(name.as_bytes()); encoded.extend_from_slice(b": ");
            encoded.extend_from_slice(trim_space(&value)); encoded.extend_from_slice(b"\r\n");
        }
    }
    Bytes::from(encoded)
}
pub(super) fn encode_http2(headers: &Headers) -> hyper::HeaderMap {
    let mut encoded = hyper::HeaderMap::new();
    for (name, values) in headers {
        if !field_name(name.as_bytes()) { continue; }
        let Ok(name) = hyper::header::HeaderName::from_bytes(name.as_bytes()) else { continue; };
        for value in values.iter() {
            // h2_bundle's encodeHeaders omits invalid values rather than
            // applying the HTTP/1 newline/space normalization.
            if value.iter().any(|byte| (*byte < b' ' && *byte != b'\t') || *byte == 0x7f) { continue; }
            if name == hyper::header::TRANSFER_ENCODING && value != b"trailers" { continue; }
            if let Ok(value) = hyper::header::HeaderValue::from_bytes(value) { encoded.append(name.clone(), value); }
        }
    }
    encoded
}
