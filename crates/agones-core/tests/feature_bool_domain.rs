//! A bounded source-contract property for the ParseBool helper reached by
//! Agones ParseFeatures. Domain: all byte strings of length 0..=5. Both engines
//! generate ([u8; 5], u8) and apply the same total projection, len = tag % 6.
use agones_core::parse_feature_bool;

// An independent grammar, rather than calling the production parser.
fn reference_bool(value: &[u8]) -> Option<bool> {
    match value.len() {
        1 => match value[0] {
            b'0' | b'f' | b'F' => Some(false),
            b'1' | b't' | b'T' => Some(true),
            _ => None,
        },
        4 | 5 => {
            let lower = if value.len() == 4 {
                b"true".as_slice()
            } else {
                b"false".as_slice()
            };
            let all_lower = value.iter().zip(lower).all(|(a, b)| a == b);
            let all_upper = value
                .iter()
                .zip(lower)
                .all(|(a, b)| *a == b.to_ascii_uppercase());
            let capitalized = value[0] == lower[0].to_ascii_uppercase() && value[1..] == lower[1..];
            (all_lower || all_upper || capitalized).then_some(value.len() == 4)
        }
        _ => None,
    }
}

// The independent invariant is the complete allowed-token relation in Go's
// strconv.ParseBool contract. It checks both the model and production output.
fn satisfies_source_relation(input: &[u8], observation: Option<bool>) -> bool {
    const TRUE: &[&[u8]] = &[b"1", b"t", b"T", b"TRUE", b"true", b"True"];
    const FALSE: &[&[u8]] = &[b"0", b"f", b"F", b"FALSE", b"false", b"False"];
    match observation {
        Some(true) => TRUE.contains(&input),
        Some(false) => FALSE.contains(&input),
        None => !TRUE.contains(&input) && !FALSE.contains(&input),
    }
}

#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(8))]
fn feature_bool_common_domain() {
    #[cfg(not(kani))]
    let visits = std::sync::atomic::AtomicUsize::new(0);
    bolero::check!()
        .with_type::<([u8; 5], u8)>()
        .with_iterations(20_000)
        .for_each(|(bytes, tag)| {
            #[cfg(not(kani))]
            visits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let len = usize::from(tag % 6);
            let input = &bytes[..len];
            let model = reference_bool(input);
            let production = parse_feature_bool(input);
            assert!(satisfies_source_relation(input, model));
            assert!(satisfies_source_relation(input, production));
            assert_eq!(model, production);
            #[cfg(kani)]
            {
                kani::cover!(production == Some(true), "accepted true spelling");
                kani::cover!(production == Some(false), "accepted false spelling");
                kani::cover!(production.is_none(), "rejected spelling");
                kani::cover!(len == 0, "empty input");
                kani::cover!(len == 5, "maximum modeled length");
            }
        });
    #[cfg(not(kani))]
    {
        let visits = visits.load(std::sync::atomic::Ordering::Relaxed);
        assert!(visits > 0);
        eprintln!(
            "ACCEPTED_DOMAIN_INPUTS={} REJECTED_BY_PRECONDITION=0",
            visits
        );
    }
}

#[test]
fn every_accepted_source_spelling_is_a_mandatory_witness() {
    for token in [
        "1", "t", "T", "TRUE", "true", "True", "0", "f", "F", "FALSE", "false", "False",
    ] {
        let model = reference_bool(token.as_bytes());
        let production = parse_feature_bool(token.as_bytes());
        assert!(production.is_some());
        assert!(satisfies_source_relation(token.as_bytes(), model));
        assert!(satisfies_source_relation(token.as_bytes(), production));
        assert_eq!(model, production);
    }
}
