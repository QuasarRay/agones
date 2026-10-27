//! Source HTTP/1 header decisions, inside the existing response owner.
// Adapted from Go net/http/server.go's chunkWriter.writeHeader/extraHeader.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::{ResponseWriter, Headers, header, body_allowed, sniff_content_type, CanReadSourceHttpClock, CanEmitSourceHttpDiagnostic};
use super::super::{header_has_token, trailers, request_body::Discard};
use hyper::ext::{SourceBody, SourceResponse};

#[derive(Default)]
struct ExtraHeaders {
    date: Option<Vec<u8>>,
    length: Option<Vec<u8>>,
    content_type: Option<Vec<u8>>,
    connection: Option<Vec<u8>>,
    transfer_encoding: Option<Vec<u8>>,
}
impl ExtraHeaders {
    fn append(self, fields: &mut Vec<u8>) {
        // extraHeader.Write has this fixed order, after the handler map's
        // independently sorted fields. It is not one merged sorted map.
        for (name, value) in [("Date", self.date), ("Content-Length", self.length),
            ("Content-Type", self.content_type), ("Connection", self.connection), ("Transfer-Encoding", self.transfer_encoding)] {
            if let Some(value) = value {
                fields.extend_from_slice(name.as_bytes()); fields.extend_from_slice(b": ");
                fields.extend_from_slice(&value); fields.extend_from_slice(b"\r\n");
            }
        }
    }
}

impl ResponseWriter {
    pub(super) async fn prepare_http1(&mut self, headers: &mut Headers, status: u16, first_data: &[u8]) -> SourceResponse {
        let mut extra = ExtraHeaders::default();
        let trailers_present = headers.keys().any(|key| key.starts_with("Trailer:"))
            || headers.get("Trailer").is_some_and(|values| !values.is_empty());
        let transfer_encoding = header(headers, "Transfer-Encoding").unwrap_or_default().to_vec();
        let has_transfer_encoding = !transfer_encoding.is_empty();
        if self.handler_done && !trailers_present && !has_transfer_encoding && body_allowed(status)
            && !headers.contains_key("Content-Length") && (!self.head_request || !first_data.is_empty()) {
            self.content_length = Some(first_data.len() as i64);
            extra.length = Some(first_data.len().to_string().into_bytes());
        }

        let keep_alives = !self.server_stops.iter().any(|stop| stop.is_cancelled());
        let mut close_after = false;
        let mut has_length = self.content_length.is_some();
        if self.wants10_keep_alive && (self.head_request || has_length || !body_allowed(status)) {
            if !headers.contains_key("Connection") { extra.connection = Some(b"keep-alive".to_vec()); }
        } else if !self.protocol_at_least_11 || self.wants_close { close_after = true; }
        if header(headers, "Connection") == Some(b"close") || !keep_alives { close_after = true; }
        if self.expect_continue && !self.request_body.expect_saw_eof() { close_after = true; }

        if self.request_content_length != 0 && !close_after && !self.full_duplex {
            match self.request_body.discard_before_response().await {
                Discard::Reusable => (),
                Discard::Close => close_after = true,
                Discard::TooLarge => {
                    close_after = true; headers.remove("Connection"); extra.connection = Some(b"close".to_vec());
                }
            }
        }
        if body_allowed(status) {
            let encoded = header(headers, "Content-Encoding").is_some_and(|value| !value.is_empty());
            if !encoded && !headers.contains_key("Content-Type") && !has_transfer_encoding && !first_data.is_empty() {
                extra.content_type = Some(sniff_content_type(first_data).as_bytes().to_vec());
            }
        } else {
            headers.remove("Content-Length"); headers.remove("Transfer-Encoding");
            if status == 304 { headers.remove("Content-Type"); }
        }
        if !headers.contains_key("Date") { extra.date = Some(self.runtime.source_http_date().into_bytes()); }
        if has_length && has_transfer_encoding && transfer_encoding != b"identity" {
            self.runtime.emit_source_http_diagnostic(&format!(
                "http: WriteHeader called with both Transfer-Encoding of {:?} and a Content-Length of {}",
                String::from_utf8_lossy(&transfer_encoding), self.content_length.unwrap_or(0)));
            headers.remove("Content-Length"); has_length = false;
        }
        let framing = if self.head_request || !body_allowed(status) {
            headers.remove("Transfer-Encoding"); SourceBody::Empty
        } else if has_length {
            headers.remove("Transfer-Encoding"); SourceBody::Length(self.content_length.unwrap_or(0) as u64)
        } else if self.protocol_at_least_11 {
            if has_transfer_encoding && transfer_encoding == b"identity" {
                close_after = true; headers.remove("Transfer-Encoding"); SourceBody::CloseDelimited
            } else {
                extra.transfer_encoding = Some(b"chunked".to_vec());
                if has_transfer_encoding && transfer_encoding == b"chunked" { headers.remove("Transfer-Encoding"); }
                headers.remove("Content-Length"); SourceBody::Chunked
            }
        } else {
            close_after = true; headers.remove("Transfer-Encoding"); SourceBody::CloseDelimited
        };
        let already_close = header(headers, "Connection").is_some_and(|value| header_has_token(value, b"close"));
        let protocol_switch = status == 101 && header(headers, "Upgrade").is_some_and(|value| !value.is_empty())
            && headers.get("Connection").is_some_and(|values| values.iter().any(|value|
                trailers::elements(value).any(|token| token.eq_ignore_ascii_case(b"Upgrade"))));
        if close_after && (!keep_alives || !already_close) && !protocol_switch {
            headers.remove("Connection");
            if self.protocol_at_least_11 { extra.connection = Some(b"close".to_vec()); }
        }
        let mut fields = trailers::encode_http1(headers).to_vec(); extra.append(&mut fields);
        self.close_after_reply = close_after;
        SourceResponse::new(fields.into(), framing, close_after).with_end(self.source_end.clone())
    }
}
