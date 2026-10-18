//! Source-contract property for the production Go quote helper.
//! Both engines use ([u8;2],u8), mask bytes to ASCII and take tag%3 length.
use agones_core::quote_feature_error_bytes;

fn reference_quote(input: &[u8]) -> ([u8; 10], usize) {
    // Fixed Go control-escape observations are independently checked by the
    // token relation and all 16,513 Go witnesses below. This reference never
    // calls production quoting or its UTF-8/Unicode machinery.
    const CONTROLS: [&str; 32] = [
        "\\x00", "\\x01", "\\x02", "\\x03", "\\x04", "\\x05", "\\x06", "\\a", "\\b", "\\t", "\\n",
        "\\v", "\\f", "\\r", "\\x0e", "\\x0f", "\\x10", "\\x11", "\\x12", "\\x13", "\\x14",
        "\\x15", "\\x16", "\\x17", "\\x18", "\\x19", "\\x1a", "\\x1b", "\\x1c", "\\x1d", "\\x1e",
        "\\x1f",
    ];
    let mut result = [0_u8; 10];
    result[0] = b'"';
    let mut cursor = 1;
    for &byte in input {
        let literal = [byte];
        let spelling = match byte {
            0..=31 => CONTROLS[usize::from(byte)].as_bytes(),
            127 => b"\\x7f",
            b'"' => b"\\\"",
            b'\\' => b"\\\\",
            _ => &literal,
        };
        for &token in spelling {
            result[cursor] = token;
            cursor += 1;
        }
    }
    result[cursor] = b'"';
    (result, cursor + 1)
}

// A separate token relation checks canonical source spelling, not agreement.
fn satisfies_quote_relation(input: &[u8], output: &[u8]) -> bool {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    if output.len() < 2 || output.first() != Some(&b'"') || output.last() != Some(&b'"') {
        return false;
    }
    let mut cursor = 1;
    for &byte in input {
        if byte >= 128 {
            return false;
        }
        if (0x20..=0x7e).contains(&byte) && byte != b'"' && byte != b'\\' {
            if output.get(cursor) != Some(&byte) {
                return false;
            }
            cursor += 1;
        } else {
            if output.get(cursor) != Some(&b'\\') {
                return false;
            }
            cursor += 1;
            let short = match byte {
                7 => Some(b'a'),
                8 => Some(b'b'),
                9 => Some(b't'),
                10 => Some(b'n'),
                11 => Some(b'v'),
                12 => Some(b'f'),
                13 => Some(b'r'),
                b'"' => Some(b'"'),
                b'\\' => Some(b'\\'),
                _ => None,
            };
            if let Some(token) = short {
                if output.get(cursor) != Some(&token) {
                    return false;
                }
                cursor += 1;
            } else {
                if output.get(cursor) != Some(&b'x')
                    || output.get(cursor + 1) != Some(&HEX[usize::from(byte >> 4)])
                    || output.get(cursor + 2) != Some(&HEX[usize::from(byte & 15)])
                {
                    return false;
                }
                cursor += 3;
            }
        }
    }
    cursor == output.len() - 1
}

#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(16))]
fn feature_error_ascii_common_domain() {
    #[cfg(not(kani))]
    let visits = std::sync::atomic::AtomicUsize::new(0);
    bolero::check!()
        .with_type::<([u8; 2], u8)>()
        .with_iterations(8192)
        .for_each(|(raw, tag)| {
            #[cfg(not(kani))]
            visits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let ascii = [raw[0] & 0x7f, raw[1] & 0x7f];
            let input = &ascii[..usize::from(tag % 3)];
            let (model, length) = reference_quote(input);
            let model = &model[..length];
            let production = quote_feature_error_bytes(input);
            assert!(satisfies_quote_relation(input, model));
            assert!(satisfies_quote_relation(input, production.as_bytes()));
            assert_eq!(model, production.as_bytes());
            #[cfg(kani)]
            {
                kani::cover!(input.is_empty(), "empty ASCII quote input");
                kani::cover!(input.len() == 2, "maximum modeled ASCII length");
                kani::cover!(input.contains(&b'"'), "double quote escape");
                kani::cover!(input.contains(&b'\\'), "backslash escape");
                kani::cover!(input.contains(&b'\''), "apostrophe remains literal");
                kani::cover!(
                    input.iter().any(|b| matches!(b, 7 | 8 | 11 | 12)),
                    "Go-specific named control escape"
                );
                kani::cover!(
                    input
                        .iter()
                        .any(|b| (*b < 32 && !matches!(b, 7 | 8 | 9 | 10 | 11 | 12 | 13))
                            || *b == 127),
                    "canonical lowercase hex control escape"
                );
                kani::cover!(
                    input
                        .iter()
                        .any(|b| (0x20..=0x7e).contains(b) && *b != b'"' && *b != b'\\'),
                    "printable ASCII remains literal"
                );
            }
        });
    #[cfg(not(kani))]
    eprintln!(
        "ACCEPTED_DOMAIN_INPUTS={} REJECTED_BY_PRECONDITION=0",
        visits.load(std::sync::atomic::Ordering::Relaxed)
    );
}

#[cfg(not(kani))]
#[test]
fn every_go_ascii_reference_observation_is_a_mandatory_witness() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../verification/agones/go-quote-ascii-reference.json"
    ))
    .unwrap();
    assert_eq!(data["schema_version"], 1);
    let quotes = data["quotes"].as_array().unwrap();
    assert_eq!(quotes.len(), 1 + 128 + 128 * 128);
    let mut inputs = vec![Vec::new()];
    for first in 0..128_u8 {
        inputs.push(vec![first]);
    }
    for first in 0..128_u8 {
        for second in 0..128_u8 {
            inputs.push(vec![first, second]);
        }
    }
    for (input, source) in inputs.iter().zip(quotes) {
        let source = source.as_str().unwrap();
        let (reference, length) = reference_quote(input);
        let reference = &reference[..length];
        let production = quote_feature_error_bytes(input);
        assert!(satisfies_quote_relation(input, source.as_bytes()));
        assert!(satisfies_quote_relation(input, reference));
        assert!(satisfies_quote_relation(input, production.as_bytes()));
        assert_eq!(source.as_bytes(), reference);
        assert_eq!(source, production);
    }
}

#[test]
fn independent_relation_rejects_wrong_escape_and_missing_output_mutations() {
    for (input, wrong) in [
        (&b"\x07"[..], r#""\x07""#),
        (&b"'"[..], r#""\'""#),
        (&b"a"[..], r#""""#),
        (&b"\0"[..], r#""\x0A""#),
        (&b"\""[..], r#"""""#),
    ] {
        assert!(!satisfies_quote_relation(input, wrong.as_bytes()));
    }
}
