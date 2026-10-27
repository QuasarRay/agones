//! Go ServeMux policy, called by the existing CGP-owned HTTP request phase.
// Source algorithms adapted from Go net/http server.go, pattern.go,
// routing_tree.go, servemux121.go and net/url. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. Routes are source handler data, not an
// effect-provider or application-lifecycle registry.
use super::{HttpHandler, HttpRequest, ResponseWriter, set_header};
use crate::utility_runtime::UtilityRuntimeContext;
use std::{collections::{BTreeMap, BTreeSet}, panic::Location, sync::{Arc, LazyLock, RwLock}};

/// The production parent selects the source tool/GODEBUG profile explicitly.
/// The compatibility constructor uses the repository's Go 1.26.5 baseline;
/// it does not infer source behavior from the Rust compiler or host version.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxProfile { Go121, Go126, Go127 }
#[derive(Clone)]
pub struct SourceMux { profile: MuxProfile, routes: Arc<RwLock<Vec<Route>>> }
impl Default for SourceMux { fn default() -> Self { Self::with_profile(MuxProfile::Go126) } }
struct Route { pattern: Pattern, handler: HttpHandler, location: &'static Location<'static> }
#[derive(Clone)]
struct Pattern { text: String, method: String, host: String, segments: Vec<Segment> }
#[derive(Clone)]
enum Segment { Literal(Vec<u8>), Single(String), Multi(String) }
impl Segment { fn multi(&self) -> bool { matches!(self, Self::Multi(_)) } }
#[derive(Clone, Copy, PartialEq, Eq)]
enum Relationship { Equivalent, General, Specific, Disjoint, Overlap }
use Relationship::*;
fn combine(left: Relationship, right: Relationship) -> Relationship {
    match (left, right) {
        (Disjoint, _) | (_, Disjoint) => Disjoint,
        (Equivalent, value) | (value, Equivalent) => value,
        (General, Specific) | (Specific, General) => Overlap,
        (Overlap, _) | (_, Overlap) => Overlap,
        (value, _) => value,
    }
}
impl Pattern {
    fn parse(text: &str) -> Result<Self, String> {
        let mut offset = 0;
        let mut parse = || -> Result<Self, String> {
            let (method, mut rest, separated) = match text.find([' ', '\t']) {
                Some(index) => (&text[..index], text[index+1..].trim_start_matches([' ', '\t']), true),
                None => ("", text, false),
            };
            if !method.is_empty() && !method.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte)) {
                return Err(format!("invalid method {:?}", method));
            }
            if separated { offset = method.len() + 1; }
            let slash = rest.find('/').ok_or("host/path missing /")?;
            let host = &rest[..slash]; rest = &rest[slash..];
            if let Some(index) = host.find('{') { offset += index; return Err("host contains '{' (missing initial '/'?)".into()); }
            offset += slash;
            if !method.is_empty() && method != "CONNECT" && rest.as_bytes() != clean_path(rest.as_bytes()) {
                return Err("non-CONNECT pattern with unclean path can never match".into());
            }
            let mut segments = Vec::new(); let mut names = BTreeSet::new();
            while !rest.is_empty() {
                rest = &rest[1..]; offset = text.len() - rest.len();
                if rest.is_empty() { segments.push(Segment::Multi(String::new())); break; }
                let end = rest.find('/').unwrap_or(rest.len());
                let segment = &rest[..end]; rest = &rest[end..];
                let Some(brace) = segment.find('{') else {
                    segments.push(Segment::Literal(unescape_or_original(segment.as_bytes()))); continue;
                };
                if brace != 0 { return Err("bad wildcard segment (must start with '{')".into()); }
                if !segment.ends_with('}') { return Err("bad wildcard segment (must end with '}')".into()); }
                let name = &segment[1..segment.len()-1];
                if name == "$" {
                    if !rest.is_empty() { return Err("{$} not at end".into()); }
                    segments.push(Segment::Literal(vec![b'/'])); break;
                }
                let (name, multi) = name.strip_suffix("...").map_or((name, false), |name| (name, true));
                if multi && !rest.is_empty() { return Err("{...} wildcard not at end".into()); }
                if name.is_empty() { return Err("empty wildcard".into()); }
                if !valid_name(name) { return Err(format!("bad wildcard name {:?}", name)); }
                if !names.insert(name.to_owned()) { return Err(format!("duplicate wildcard name {:?}", name)); }
                segments.push(if multi { Segment::Multi(name.to_owned()) } else { Segment::Single(name.to_owned()) });
            }
            Ok(Self { text: text.to_owned(), method: method.to_owned(), host: host.to_owned(), segments })
        };
        let result = parse();
        result.map_err(|error| format!("at offset {offset}: {error}"))
    }
    fn compare_methods(&self, other: &Self) -> Relationship {
        if self.method == other.method { Equivalent }
        else if self.method.is_empty() || self.method == "GET" && other.method == "HEAD" { General }
        else if other.method.is_empty() || other.method == "GET" && self.method == "HEAD" { Specific }
        else { Disjoint }
    }
    fn compare_paths(&self, other: &Self) -> Relationship {
        let left_multi = self.segments.last().is_some_and(Segment::multi);
        let right_multi = other.segments.last().is_some_and(Segment::multi);
        if self.segments.len() != other.segments.len() && !left_multi && !right_multi { return Disjoint; }
        let mut result = Equivalent;
        for (left, right) in self.segments.iter().zip(&other.segments) {
            result = combine(result, compare_segments(left, right));
            if result == Disjoint { return result; }
        }
        match self.segments.len().cmp(&other.segments.len()) {
            std::cmp::Ordering::Equal => result,
            std::cmp::Ordering::Less if left_multi => combine(result, General),
            std::cmp::Ordering::Greater if right_multi => combine(result, Specific),
            _ => Disjoint,
        }
    }
    fn relationship(&self, other: &Self) -> Relationship { combine(self.compare_methods(other), self.compare_paths(other)) }
    fn matches_method(&self, method: &[u8]) -> bool { self.method.is_empty() || self.method.as_bytes() == method || self.method == "GET" && method == b"HEAD" }
    fn match_path(&self, mut path: &[u8]) -> Option<BTreeMap<String, Vec<u8>>> {
        let mut captures = BTreeMap::new();
        for segment in &self.segments {
            if path.is_empty() { return None; }
            let tail = path.strip_prefix(b"/")?;
            if let Segment::Multi(name) = segment {
                if !name.is_empty() { captures.insert(name.clone(), unescape_or_original(tail)); }
                return Some(captures);
            }
            let (value, rest) = if path == b"/" { (vec![b'/'], &b""[..]) } else {
                let end = tail.iter().position(|byte| *byte == b'/').unwrap_or(tail.len());
                (unescape_or_original(&tail[..end]), &tail[end..])
            };
            match segment {
                Segment::Literal(literal) if *literal == value => (),
                Segment::Single(name) if value != b"/" => { captures.insert(name.clone(), value); }
                _ => return None,
            }
            path = rest;
        }
        path.is_empty().then_some(captures)
    }
    fn exact(&self, path: &[u8]) -> bool {
        !self.segments.last().is_some_and(Segment::multi) || path.ends_with(b"/")
            && self.segments.len() == path.iter().filter(|byte| **byte == b'/').count()
    }
}
fn valid_name(name: &str) -> bool {
    static NAME: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"^[_\p{L}][_\p{L}\p{Nd}]*$").expect("Go wildcard identifier categories"));
    NAME.is_match(name)
}
fn compare_segments(left: &Segment, right: &Segment) -> Relationship {
    match (left, right) {
        (Segment::Multi(_), Segment::Multi(_)) | (Segment::Single(_), Segment::Single(_)) => Equivalent,
        (Segment::Multi(_), _) => General,
        (_, Segment::Multi(_)) => Specific,
        (Segment::Single(_), Segment::Literal(value)) => if value == b"/" { Disjoint } else { General },
        (Segment::Literal(value), Segment::Single(_)) => if value == b"/" { Disjoint } else { Specific },
        (Segment::Literal(left), Segment::Literal(right)) => if left == right { Equivalent } else { Disjoint },
    }
}
struct Matched<'a> { route: &'a Route, captures: BTreeMap<String, Vec<u8>> }
fn match_modern<'a>(routes: &'a [Route], host: &str, method: &[u8], path: &[u8]) -> Option<Matched<'a>> {
    let mut selected: Option<Matched<'a>> = None;
    for route in routes {
        let pattern = &route.pattern;
        if !pattern.host.is_empty() && pattern.host != host || !pattern.matches_method(method) { continue; }
        let Some(captures) = pattern.match_path(path) else { continue; };
        let prefer = selected.as_ref().is_none_or(|old| {
            if pattern.host.is_empty() != old.route.pattern.host.is_empty() { !pattern.host.is_empty() }
            else { pattern.relationship(&old.route.pattern) == Specific }
        });
        if prefer { selected = Some(Matched { route, captures }); }
    }
    selected
}
enum Selection {
    Handler { handler: HttpHandler, pattern: Vec<u8>, captures: BTreeMap<String, Vec<u8>> },
    Redirect { url: Vec<u8>, pattern: Vec<u8>, status: u16 },
    MethodNotAllowed(String), NotFound,
}
fn selected(matched: Option<Matched<'_>>) -> Selection {
    matched.map_or(Selection::NotFound, |matched| Selection::Handler { handler: Arc::clone(&matched.route.handler),
        pattern: matched.route.pattern.text.as_bytes().to_vec(), captures: matched.captures })
}
impl SourceMux {
    pub fn with_profile(profile: MuxProfile) -> Self { Self { profile, routes: Arc::new(RwLock::new(Vec::new())) } }
    pub fn profile(&self) -> MuxProfile { self.profile }
    #[track_caller]
    pub fn handle(&self, text: &str, handler: HttpHandler) {
        if text.is_empty() { panic!("http: invalid pattern"); }
        let pattern = if self.profile == MuxProfile::Go121 {
            Pattern { text: text.to_owned(), method: String::new(), host: String::new(), segments: Vec::new() }
        } else { Pattern::parse(text).unwrap_or_else(|error| panic!("parsing {:?}: {}", text, error)) };
        let mut routes = self.routes.write().unwrap_or_else(|error| error.into_inner());
        for old in routes.iter() {
            if self.profile == MuxProfile::Go121 {
                if old.pattern.text == text { panic!("http: multiple registrations for {text}"); }
            } else if pattern.host == old.pattern.host && matches!(pattern.relationship(&old.pattern), Equivalent | Overlap) {
                panic!("pattern {:?} (registered at {}) conflicts with pattern {:?} (registered at {})",
                    text, Location::caller(), old.pattern.text, old.location);
            }
        }
        routes.push(Route { pattern, handler, location: Location::caller() });
    }
    fn resolve(&self, request: &HttpRequest) -> Selection {
        let routes = self.routes.read().unwrap_or_else(|error| error.into_inner());
        if self.profile == MuxProfile::Go121 { return resolve_legacy(&routes, request); }
        let escaped = request.escaped_path();
        let method = request.method();
        let connect = method.as_ref() == b"CONNECT";
        let host = if connect { request.url_host.clone() } else { strip_host_port(&request.host).to_owned() };
        let path = if connect { escaped.clone() } else { clean_path(&escaped) };
        let found = match_modern(&routes, &host, &method, &path);
        if !found.as_ref().is_some_and(|found| found.route.pattern.exact(&path)) && !path.ends_with(b"/") && !path.is_empty() {
            let mut slash = path.clone(); slash.push(b'/');
            if let Some(found) = match_modern(&routes, &host, &method, &slash).filter(|found| found.route.pattern.exact(&slash)) {
                let target = if self.profile == MuxProfile::Go127 { slash } else {
                    let mut decoded = clean_path(&request.url_path); decoded.push(b'/'); escape_url_path(&decoded)
                };
                let pattern = if connect { unescape_or_original(&target) } else { found.route.pattern.text.as_bytes().to_vec() };
                return Selection::Redirect { url: url_with_query(target, &request.url_raw_query), pattern, status: 307 };
            }
        }
        if !connect && path != escaped {
            let pattern = found.as_ref().map_or_else(Vec::new, |found| found.route.pattern.text.as_bytes().to_vec());
            let target = if self.profile == MuxProfile::Go127 { path.clone() } else { escape_url_path(&path) };
            return Selection::Redirect { url: url_with_query(target, &request.url_raw_query), pattern, status: 307 };
        }
        let found = if connect { match_modern(&routes, &request.host, &method, &path) } else { found };
        if found.is_some() { return selected(found); }
        let mut allowed = BTreeSet::new();
        for route in routes.iter().filter(|route| !route.pattern.method.is_empty() && (route.pattern.host.is_empty() || route.pattern.host == host)) {
            let mut slash = path.clone(); slash.push(b'/');
            if route.pattern.match_path(&path).is_some() || !path.ends_with(b"/") && route.pattern.match_path(&slash).is_some() {
                allowed.insert(route.pattern.method.clone());
                if route.pattern.method == "GET" { allowed.insert("HEAD".to_owned()); }
            }
        }
        if allowed.is_empty() { Selection::NotFound } else { Selection::MethodNotAllowed(allowed.into_iter().collect::<Vec<_>>().join(", ")) }
    }
    pub(super) async fn dispatch(&self, context: &UtilityRuntimeContext, writer: &mut ResponseWriter, request: &mut HttpRequest) {
        if request.request_uri == "*" {
            if request.protocol_at_least_11 { set_header(&mut writer.headers, "Connection", "close"); }
            let _ = writer.write_header(400).await; return;
        }
        let selection = self.resolve(request); // release the route lock before callbacks/awaits
        if self.profile != MuxProfile::Go121 {
            request.path_values.clear();
            request.matched_pattern = match &selection {
                Selection::Handler { pattern, .. } | Selection::Redirect { pattern, .. } => pattern.clone(),
                _ => Vec::new(),
            };
        }
        match selection {
            Selection::Handler { handler, captures, .. } => {
                if self.profile != MuxProfile::Go121 { request.path_values = captures; }
                handler(context, writer, request).await;
            }
            Selection::Redirect { url, status, .. } => redirect(writer, request, url, status).await,
            Selection::MethodNotAllowed(allowed) => { set_header(&mut writer.headers, "Allow", allowed); writer.http_error("Method Not Allowed", 405).await; }
            Selection::NotFound => writer.http_error("404 page not found", 404).await,
        }
    }
}
fn legacy_match<'a>(routes: &'a [Route], host: &str, path: &[u8]) -> Option<&'a Route> {
    let matches = |key: &[u8]| routes.iter().filter(|route| {
        let pattern = route.pattern.text.as_bytes(); pattern == key || pattern.ends_with(b"/") && key.starts_with(pattern)
    }).max_by_key(|route| route.pattern.text.len());
    let mut key = host.as_bytes().to_vec(); key.extend_from_slice(path);
    matches(&key).or_else(|| matches(path))
}
fn resolve_legacy(routes: &[Route], request: &HttpRequest) -> Selection {
    let connect = request.method().as_ref() == b"CONNECT";
    let host = if connect { request.url_host.as_str() } else { strip_host_port(&request.host) };
    let path = if connect { request.url_path.clone() } else { clean_path(&request.url_path) };
    let mut key = host.as_bytes().to_vec(); key.extend_from_slice(&path);
    let exact = |key: &[u8]| routes.iter().any(|route| route.pattern.text.as_bytes() == key);
    if !path.is_empty() && !path.ends_with(b"/") && !exact(&path) && !exact(&key) {
        let mut slash = path.clone(); slash.push(b'/'); key.push(b'/');
        if exact(&slash) || exact(&key) {
            return Selection::Redirect { url: url_with_query(escape_url_path(&slash), &request.url_raw_query), pattern: slash, status: 301 };
        }
    }
    let found = legacy_match(routes, if connect { &request.host } else { host }, &path);
    if !connect && path != request.url_path {
        return Selection::Redirect { url: url_with_query(escape_url_path(&path), &request.url_raw_query),
            pattern: found.map_or_else(Vec::new, |route| route.pattern.text.as_bytes().to_vec()), status: 301 };
    }
    found.map_or(Selection::NotFound, |route| Selection::Handler { handler: Arc::clone(&route.handler),
        pattern: route.pattern.text.as_bytes().to_vec(), captures: BTreeMap::new() })
}
fn strip_host_port(host: &str) -> &str {
    if let Some(rest) = host.strip_prefix('[') {
        if let Some(end) = rest.find(']') {
            if rest[end+1..].starts_with(':') && !rest[end+2..].contains([':', '[', ']']) && !rest[..end].contains(['[', ']']) { return &rest[..end]; }
        }
    } else if let Some((name, port)) = host.rsplit_once(':') {
        if !name.contains([':', '[', ']']) && !port.contains(['[', ']']) { return name; }
    }
    host
}
fn clean_path(path: &[u8]) -> Vec<u8> {
    let mut segments: Vec<&[u8]> = Vec::new();
    for segment in path.split(|byte| *byte == b'/') {
        match segment { b"" | b"." => (), b".." => { segments.pop(); }, _ => segments.push(segment) }
    }
    let mut clean = vec![b'/'];
    for (index, segment) in segments.iter().enumerate() { if index > 0 { clean.push(b'/'); } clean.extend_from_slice(segment); }
    if path.ends_with(b"/") && clean != b"/" { clean.push(b'/'); }
    clean
}
pub(super) fn path_unescape(raw: &[u8]) -> Option<Vec<u8>> {
    let mut decoded = Vec::with_capacity(raw.len()); let mut index = 0;
    while index < raw.len() {
        if raw[index] == b'%' {
            let high = (*raw.get(index+1)? as char).to_digit(16)?; let low = (*raw.get(index+2)? as char).to_digit(16)?;
            decoded.push((high*16+low) as u8); index += 3;
        } else { decoded.push(raw[index]); index += 1; }
    }
    Some(decoded)
}
fn unescape_or_original(raw: &[u8]) -> Vec<u8> { path_unescape(raw).unwrap_or_else(|| raw.to_vec()) }
pub(super) fn escape_url_path(path: &[u8]) -> Vec<u8> {
    let mut escaped = Vec::with_capacity(path.len());
    for byte in path {
        if byte.is_ascii_alphanumeric() || b"-_.~$&+,/:;=@".contains(byte) { escaped.push(*byte); }
        else { escaped.extend_from_slice(&[b'%', b"0123456789ABCDEF"[(*byte >> 4) as usize], b"0123456789ABCDEF"[(*byte & 15) as usize]]); }
    }
    escaped
}
pub(super) fn valid_encoded_path(path: &[u8]) -> bool {
    path.iter().all(|byte| byte.is_ascii_alphanumeric() || b"-_.~/$&+,:;=@!'()*[]%".contains(byte))
}
fn url_with_query(mut path: Vec<u8>, query: &[u8]) -> Vec<u8> {
    if !query.is_empty() { path.push(b'?'); path.extend_from_slice(query); } path
}
async fn redirect(writer: &mut ResponseWriter, request: &HttpRequest, mut url: Vec<u8>, status: u16) {
    // Mux-generated redirects are absolute local paths. As in http.Redirect,
    // keep an actual //authority URL; otherwise clean the escaped string once
    // more and preserve the raw query and a trailing slash.
    let authority = url.strip_prefix(b"//").is_some_and(|rest| !rest.is_empty() && !matches!(rest[0], b'/' | b'?'));
    if !authority {
        let query = url.iter().position(|byte| *byte == b'?').unwrap_or(url.len());
        let mut clean = clean_path(&url[..query]); clean.extend_from_slice(&url[query..]); url = clean;
    }
    let content_type = writer.headers.contains_key("Content-Type");
    let mut location = Vec::new();
    for byte in &url {
        if *byte < 128 { location.push(*byte); }
        else { location.extend_from_slice(&[b'%', b"0123456789ABCDEF"[(*byte >> 4) as usize], b"0123456789ABCDEF"[(*byte & 15) as usize]]); }
    }
    set_header(&mut writer.headers, "Location", location);
    if !content_type && matches!(request.method().as_ref(), b"GET" | b"HEAD") { set_header(&mut writer.headers, "Content-Type", "text/html; charset=utf-8"); }
    let _ = writer.write_header(status).await;
    if !content_type && request.method().as_ref() == b"GET" {
        let mut body = b"<a href=\"".to_vec();
        for byte in url {
            let escaped: Option<&[u8]> = match byte { b'&' => Some(b"&amp;"), b'<' => Some(b"&lt;"), b'>' => Some(b"&gt;"),
                b'\"' => Some(b"&#34;"), b'\'' => Some(b"&#39;"), _ => None };
            match escaped { Some(escaped) => body.extend_from_slice(escaped), None => body.push(byte) }
        }
        body.extend_from_slice(if status == 301 { b"\">Moved Permanently</a>.\n\n" } else { b"\">Temporary Redirect</a>.\n\n" });
        let _ = writer.write(&body).await;
    }
}
