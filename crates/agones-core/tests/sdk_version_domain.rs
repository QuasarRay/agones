//! One declared byte domain for Bolero and Kani, without rejected inputs.
use agones_core::{AgonesBuildVersionContext, CanGetAgonesVersionAnnotation};

// Independent byte grammar: preserve a shortest-form scalar's original bytes;
// consume one byte and emit EF BF BD for every invalid prefix, as Go JSON does.
fn reference(input: &[u8]) -> ([u8; 12], usize) {
    let mut out = [0; 12];
    let (mut source, mut target) = (0, 0);
    while source < input.len() {
        let tail = &input[source..];
        let continuation =
            |index: usize| tail.get(index).is_some_and(|b| (0x80..=0xbf).contains(b));
        let width = match tail[0] {
            0x00..=0x7f => 1,
            0xc2..=0xdf if continuation(1) => 2,
            0xe0 if tail.get(1).is_some_and(|b| (0xa0..=0xbf).contains(b)) && continuation(2) => 3,
            0xe1..=0xec | 0xee..=0xef if continuation(1) && continuation(2) => 3,
            0xed if tail.get(1).is_some_and(|b| (0x80..=0x9f).contains(b)) && continuation(2) => 3,
            0xf0 if tail.get(1).is_some_and(|b| (0x90..=0xbf).contains(b))
                && continuation(2)
                && continuation(3) =>
            {
                4
            }
            0xf1..=0xf3 if continuation(1) && continuation(2) && continuation(3) => 4,
            0xf4 if tail.get(1).is_some_and(|b| (0x80..=0x8f).contains(b))
                && continuation(2)
                && continuation(3) =>
            {
                4
            }
            _ => 0,
        };
        if width == 0 {
            out[target..target + 3].copy_from_slice(b"\xef\xbf\xbd");
            target += 3;
            source += 1;
        } else {
            for index in 0..width {
                out[target] = tail[index];
                target += 1;
            }
            source += width;
        }
    }
    (out, target)
}

// A separate observation relation uses the standard validator's valid-prefix
// result, without either implementation's byte grammar or scalar construction.
fn satisfies_relation(input: &[u8], output: &[u8]) -> bool {
    let (mut source, mut target) = (0, 0);
    while source < input.len() {
        match std::str::from_utf8(&input[source..]) {
            Ok(valid) => return output.get(target..) == Some(valid.as_bytes()),
            Err(error) => {
                let valid = error.valid_up_to();
                if output.get(target..target + valid) != Some(&input[source..source + valid]) {
                    return false;
                }
                source += valid;
                target += valid;
                if output.get(target..target + 3) != Some(b"\xef\xbf\xbd") {
                    return false;
                }
                source += 1;
                target += 3;
            }
        }
    }
    target == output.len()
}

#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(16))]
fn sdk_version_common_domain() {
    #[cfg(not(kani))]
    let visits = std::sync::atomic::AtomicUsize::new(0);
    bolero::check!()
        .with_type::<([u8; 4], u8)>()
        .with_iterations(8192)
        .for_each(|(bytes, tag)| {
            #[cfg(not(kani))]
            visits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let input = &bytes[..usize::from(tag % 5)];
            let (model, length) = reference(input);
            let model = &model[..length];
            let production =
                AgonesBuildVersionContext::from_bytes(input).agones_version_annotation();
            assert!(satisfies_relation(input, model));
            assert!(satisfies_relation(input, production.as_bytes()));
            assert_eq!(model, production.as_bytes());
            #[cfg(kani)]
            {
                kani::cover!(input.is_empty(), "empty SDK version");
                kani::cover!(input.len() == 4, "maximum modeled SDK byte length");
                kani::cover!(input.contains(&0), "SDK version NUL preserved");
                kani::cover!(input == b"\xc2\xa2", "valid two-byte SDK scalar");
                kani::cover!(input == b"\xe2\x82\xac", "valid three-byte SDK scalar");
                kani::cover!(input == b"\xf0\x9f\x8c\x8d", "valid four-byte SDK scalar");
                kani::cover!(input == b"\xff", "invalid leading SDK byte");
                kani::cover!(input == b"\xc0\x80", "overlong SDK byte sequence");
                kani::cover!(input == b"\xed\xa0\x80", "surrogate SDK byte sequence");
                kani::cover!(input == b"\xe2\x82", "truncated SDK byte prefix");
                kani::cover!(input == b"\xef\xbf\xbd", "literal replacement SDK scalar");
                kani::cover!(input == b"\xe2A", "invalid SDK continuation byte");
                kani::cover!(
                    input == b"\xf4\x90\x80\x80",
                    "SDK scalar above Unicode maximum"
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
fn sdk_relation_rejects_lossy_grouping_and_changed_valid_bytes() {
    assert!(!satisfies_relation(b"\xe2\x82", "\u{fffd}".as_bytes()));
    assert!(!satisfies_relation(b"\xc0\x80", b"\0"));
    assert!(!satisfies_relation(b"\xef\xbf\xbd", b"?"));
    assert!(!satisfies_relation(b"A\0B", b"AB"));
    assert!(!satisfies_relation(b"", b"x"));
    assert!(!satisfies_relation(b"\xf0\x9f\x8c\x8d", b"earth"));
    for input in [
        b"\xe2\x82".as_slice(),
        b"\xc0\x80",
        b"\xef\xbf\xbd",
        b"A\0B",
        b"",
        b"\xf0\x9f\x8c\x8d",
    ] {
        let (model, length) = reference(input);
        assert!(satisfies_relation(input, &model[..length]));
    }
}
