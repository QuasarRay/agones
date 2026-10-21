//! Shared label and annotation policy used by Agones object metadata and patches.
//!
//! The pinned policy is common.go::validateObjectMeta / AllocationOverflow.Validate
//! and their vendored apimachinery helpers. Map traversal is deterministic here;
//! callers comparing Go observations must retain the permitted map-order multiset.
use std::{borrow::Cow, collections::BTreeMap};

use cgp::prelude::*;

use crate::ValidationError;

// The pinned Go toolchain and both configured Rust compilers use Unicode 17.
// A toolchain update must recheck simple casing before changing this binding.
const _: () = assert!(
    char::UNICODE_VERSION.0 == 17 && char::UNICODE_VERSION.1 == 0 && char::UNICODE_VERSION.2 == 0
);

pub const TOTAL_ANNOTATION_SIZE_LIMIT_BYTES: i64 = 256 * (1 << 10);

#[cgp_component(ObjectMetadataValidator)]
pub trait CanValidateObjectMetadata {
    fn validate_object_metadata(
        &self,
        labels: &BTreeMap<String, String>,
        annotations: &BTreeMap<String, String>,
        path: &str,
    ) -> Vec<ValidationError>;
}

#[cgp_new_provider]
impl<Context> ObjectMetadataValidator<Context> for ValidateAgonesObjectMetadata {
    fn validate_object_metadata(
        _context: &Context,
        labels: &BTreeMap<String, String>,
        annotations: &BTreeMap<String, String>,
        path: &str,
    ) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        for (key, value) in labels {
            for detail in qualified_name_errors(key) {
                errors.push(
                    ValidationError::invalid(format!("{path}.labels"), key.as_str(), detail)
                        .with_origin("format=k8s-label-key"),
                );
            }
            if value.len() > 63 {
                errors.push(
                    ValidationError::invalid(
                        format!("{path}.labels"),
                        value.as_str(),
                        "must be no more than 63 bytes",
                    )
                    .with_origin("format=k8s-label-value"),
                );
            }
            if !value.is_empty() && !valid_name(value) {
                errors.push(
                    ValidationError::invalid(
                        format!("{path}.labels"),
                        value.as_str(),
                        LABEL_VALUE_FORMAT_ERROR,
                    )
                    .with_origin("format=k8s-label-value"),
                );
            }
        }

        let mut total_size = 0i64;
        for (key, value) in annotations {
            // Go sums original UTF-8 byte lengths in int64, including values
            // with arbitrary contents. Key validation uses its lowercased copy.
            total_size = add_annotation_size(total_size, key.len(), value.len());
            for detail in qualified_name_errors(&go_lowercase(key)) {
                errors.push(
                    ValidationError::invalid(format!("{path}.annotations"), key.as_str(), detail)
                        .with_origin("format=k8s-label-key"),
                );
            }
        }
        if total_size > TOTAL_ANNOTATION_SIZE_LIMIT_BYTES {
            errors.push(ValidationError::too_long(
                format!("{path}.annotations"),
                "",
                TOTAL_ANNOTATION_SIZE_LIMIT_BYTES as isize,
            ));
        }
        errors
    }
}

const QUALIFIED_NAME_FORMAT_ERROR: &str = "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')";
const QUALIFIED_KEY_FORMAT_ERROR: &str = "a valid label key must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]') with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')";
const DNS_PREFIX_FORMAT_ERROR: &str = "prefix part a lowercase RFC 1123 subdomain must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character (e.g. 'example.com', regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?(\\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*')";
const LABEL_VALUE_FORMAT_ERROR: &str = "a valid label must be an empty string or consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyValue',  or 'my_value',  or '12345', regex used for validation is '(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9])?')";

fn valid_name(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        && bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        && bytes
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}

fn valid_dns_subdomain(value: &str) -> bool {
    // IsDNS1123Subdomain checks total length separately. In this source it
    // deliberately does not impose the DNS-label 63-byte limit per segment.
    let mut label_start = true;
    let mut last_alphanumeric = false;
    for byte in value.bytes() {
        if byte == b'.' {
            if !last_alphanumeric {
                return false;
            }
            label_start = true;
            last_alphanumeric = false;
        } else {
            last_alphanumeric = byte.is_ascii_lowercase() || byte.is_ascii_digit();
            if !last_alphanumeric && (label_start || byte != b'-') {
                return false;
            }
            label_start = false;
        }
    }
    last_alphanumeric
}

fn qualified_name_errors(value: &str) -> Vec<&'static str> {
    let mut errors = Vec::new();
    let mut slash = None;
    for (index, byte) in value.bytes().enumerate() {
        if byte == b'/' {
            if slash.is_some() {
                return vec![QUALIFIED_KEY_FORMAT_ERROR];
            }
            slash = Some(index);
        }
    }
    let name = if let Some(index) = slash {
        // An ASCII slash is always a UTF-8 boundary, including in malformed
        // qualified names whose non-ASCII bytes are rejected below.
        let first = &value[..index];
        if first.is_empty() {
            errors.push("prefix part must be non-empty");
        } else {
            if first.len() > 253 {
                errors.push("prefix part must be no more than 253 bytes");
            }
            if !valid_dns_subdomain(first) {
                errors.push(DNS_PREFIX_FORMAT_ERROR);
            }
        }
        &value[index + 1..]
    } else {
        value
    };
    if name.is_empty() {
        errors.push("name part must be non-empty");
    } else if name.len() > 63 {
        errors.push("name part must be no more than 63 bytes");
    }
    if !valid_name(name) {
        errors.push(QUALIFIED_NAME_FORMAT_ERROR);
    }
    errors
}

fn go_lowercase(value: &str) -> Cow<'_, str> {
    if value.is_ascii() && !value.bytes().any(|b| b.is_ascii_uppercase()) {
        return Cow::Borrowed(value);
    }
    // strings.ToLower maps each rune using unicode.ToLower. Rust's full-string
    // casing expands U+0130 and applies final sigma; taking the first scalar of
    // each character's mapping preserves simple lowercase instead.
    Cow::Owned(
        value
            .chars()
            .map(|ch| ch.to_lowercase().next().unwrap())
            .collect(),
    )
}

fn add_annotation_size(total: i64, key_bytes: usize, value_bytes: usize) -> i64 {
    total
        .wrapping_add(key_bytes as i64)
        .wrapping_add(value_bytes as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annotation_size_preserves_go_int64_wrapping() {
        assert_eq!(add_annotation_size(i64::MAX, 1, 0), i64::MIN);
        assert_eq!(add_annotation_size(i64::MIN, 1, 1), i64::MIN + 2);
    }

    #[test]
    fn annotation_lowercase_is_simple_rune_mapping() {
        assert_eq!(go_lowercase("İ.K/ΣΟΣẞ"), "i.k/σοσß");
    }
}
