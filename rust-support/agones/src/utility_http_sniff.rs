//! Ordered Go net/http content detection used by the native response writer.
// Adapted from Go 1.26.5 net/http/sniff.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. Signatures are immutable source data;
// the CGP HTTP effect remains the owner of response writing and I/O.

enum Signature {
    Html(&'static [u8]),
    Exact(&'static [u8], &'static str),
    Masked { pattern: &'static [u8], mask: &'static [u8], skip_space: bool, content_type: &'static str },
    Mp4, FontObject, Text,
}
use Signature::*;
const SIGNATURES: &[Signature] = &[
    Html(b"<!DOCTYPE HTML"), Html(b"<HTML"), Html(b"<HEAD"), Html(b"<SCRIPT"), Html(b"<IFRAME"),
    Html(b"<H1"), Html(b"<DIV"), Html(b"<FONT"), Html(b"<TABLE"), Html(b"<A"), Html(b"<STYLE"),
    Html(b"<TITLE"), Html(b"<B"), Html(b"<BODY"), Html(b"<BR"), Html(b"<P"), Html(b"<!--"),
    Masked { pattern: b"<?xml", mask: b"\xff\xff\xff\xff\xff", skip_space: true, content_type: "text/xml; charset=utf-8" },
    Exact(b"%PDF-", "application/pdf"), Exact(b"%!PS-Adobe-", "application/postscript"),
    Masked { pattern: b"\xfe\xff\0\0", mask: b"\xff\xff\0\0", skip_space: false, content_type: "text/plain; charset=utf-16be" },
    Masked { pattern: b"\xff\xfe\0\0", mask: b"\xff\xff\0\0", skip_space: false, content_type: "text/plain; charset=utf-16le" },
    Masked { pattern: b"\xef\xbb\xbf\0", mask: b"\xff\xff\xff\0", skip_space: false, content_type: "text/plain; charset=utf-8" },
    Exact(b"\0\0\x01\0", "image/x-icon"), Exact(b"\0\0\x02\0", "image/x-icon"), Exact(b"BM", "image/bmp"),
    Exact(b"GIF87a", "image/gif"), Exact(b"GIF89a", "image/gif"),
    Masked { pattern: b"RIFF\0\0\0\0WEBPVP", mask: b"\xff\xff\xff\xff\0\0\0\0\xff\xff\xff\xff\xff\xff", skip_space: false, content_type: "image/webp" },
    Exact(b"\x89PNG\r\n\x1a\n", "image/png"), Exact(b"\xff\xd8\xff", "image/jpeg"),
    Masked { pattern: b"FORM\0\0\0\0AIFF", mask: b"\xff\xff\xff\xff\0\0\0\0\xff\xff\xff\xff", skip_space: false, content_type: "audio/aiff" },
    Masked { pattern: b"ID3", mask: b"\xff\xff\xff", skip_space: false, content_type: "audio/mpeg" },
    Masked { pattern: b"OggS\0", mask: b"\xff\xff\xff\xff\xff", skip_space: false, content_type: "application/ogg" },
    Masked { pattern: b"MThd\0\0\0\x06", mask: b"\xff\xff\xff\xff\xff\xff\xff\xff", skip_space: false, content_type: "audio/midi" },
    Masked { pattern: b"RIFF\0\0\0\0AVI ", mask: b"\xff\xff\xff\xff\0\0\0\0\xff\xff\xff\xff", skip_space: false, content_type: "video/avi" },
    Masked { pattern: b"RIFF\0\0\0\0WAVE", mask: b"\xff\xff\xff\xff\0\0\0\0\xff\xff\xff\xff", skip_space: false, content_type: "audio/wave" },
    Mp4, Exact(b"\x1a\x45\xdf\xa3", "video/webm"),
    FontObject,
    Exact(b"\0\x01\0\0", "font/ttf"), Exact(b"OTTO", "font/otf"), Exact(b"ttcf", "font/collection"),
    Exact(b"wOFF", "font/woff"), Exact(b"wOF2", "font/woff2"),
    Exact(b"\x1f\x8b\x08", "application/x-gzip"), Exact(b"PK\x03\x04", "application/zip"),
    Exact(b"Rar!\x1a\x07\0", "application/x-rar-compressed"),
    Exact(b"Rar!\x1a\x07\x01\0", "application/x-rar-compressed"),
    Exact(b"\0asm", "application/wasm"), Text,
];

pub(super) fn detect_content_type(input: &[u8]) -> &'static str {
    let data = &input[..input.len().min(512)];
    let first_non_space = data.iter().position(|byte| !matches!(*byte, b'\t' | b'\n' | b'\x0c' | b'\r' | b' ')).unwrap_or(data.len());
    SIGNATURES.iter().find_map(|signature| signature.matches(data, first_non_space)).unwrap_or("application/octet-stream")
}
impl Signature {
    fn matches(&self, data: &[u8], first_non_space: usize) -> Option<&'static str> {
        match self {
            Html(pattern) => {
                let data = &data[first_non_space..];
                if data.len() < pattern.len()+1 { return None; }
                for (index, expected) in pattern.iter().enumerate() {
                    let byte = if expected.is_ascii_uppercase() { data[index] & 0xdf } else { data[index] };
                    if byte != *expected { return None; }
                }
                matches!(data[pattern.len()], b' ' | b'>').then_some("text/html; charset=utf-8")
            }
            Exact(pattern, content_type) => data.starts_with(pattern).then_some(*content_type),
            Masked { pattern, mask, skip_space, content_type } => {
                let data = if *skip_space { &data[first_non_space..] } else { data };
                if pattern.len() != mask.len() || data.len() < pattern.len() { return None; }
                pattern.iter().zip(*mask).enumerate().all(|(index, (expected, mask))| data[index] & *mask == *expected).then_some(*content_type)
            }
            Mp4 => {
                if data.len() < 12 || &data[4..8] != b"ftyp" { return None; }
                let size = u32::from_be_bytes(data[..4].try_into().expect("four box bytes")) as usize;
                if data.len() < size || size % 4 != 0 { return None; }
                (8..size).step_by(4).any(|offset| offset != 12 && &data[offset..offset+3] == b"mp4").then_some("video/mp4")
            }
            FontObject => (data.len() >= 36 && &data[34..36] == b"LP").then_some("application/vnd.ms-fontobject"),
            Text => (!data[first_non_space..].iter().any(|byte|
                *byte <= 0x08 || *byte == 0x0b || (0x0e..=0x1a).contains(byte) || (0x1c..=0x1f).contains(byte))).then_some("text/plain; charset=utf-8"),
        }
    }
}
