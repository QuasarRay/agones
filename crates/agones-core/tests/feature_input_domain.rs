//! Source-contract input precedence property. Each optional value is an arbitrary
//! byte string of length 0..2. Both engines share the fixed tuple and projection.
use agones_core::select_feature_gate_input;

fn reference_input<'a>(
    override_value: Option<&'a [u8]>,
    flag: Option<&'a [u8]>,
    environment: Option<&'a [u8]>,
) -> &'a [u8] {
    match (override_value, flag, environment) {
        (Some(value), _, _) | (None, Some(value), _) => value,
        (None, None, Some(value)) if !value.is_empty() => value,
        _ => &[],
    }
}

// Independent guarded obligations from Viper find/getEnv, including explicit
// empty override/flag values and ignored empty environment values.
fn source_relation(
    override_value: Option<&[u8]>,
    flag: Option<&[u8]>,
    environment: Option<&[u8]>,
    output: &[u8],
) -> bool {
    let override_law = override_value.is_none_or(|value| output == value);
    let flag_law = override_value.is_some() || flag.is_none_or(|value| output == value);
    let environment_law = override_value.is_some()
        || flag.is_some()
        || environment.is_none_or(|value| value.is_empty() || output == value);
    let default_law = override_value.is_some()
        || flag.is_some()
        || environment.is_some_and(|value| !value.is_empty())
        || output.is_empty();
    override_law && flag_law && environment_law && default_law
}

#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(8))]
fn feature_input_common_domain() {
    #[cfg(not(kani))]
    let visits = std::sync::atomic::AtomicUsize::new(0);
    bolero::check!()
        .with_type::<([u8; 2], [u8; 2], [u8; 2], u8, u8, u8, u8)>()
        .with_iterations(4096)
        .for_each(|(a, b, c, flags, la, lb, lc)| {
            #[cfg(not(kani))]
            visits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let override_value = (flags & 1 != 0).then_some(&a[..usize::from(la % 3)]);
            let flag = (flags & 2 != 0).then_some(&b[..usize::from(lb % 3)]);
            let environment = (flags & 4 != 0).then_some(&c[..usize::from(lc % 3)]);
            let model = reference_input(override_value, flag, environment);
            let production = select_feature_gate_input(override_value, flag, environment);
            assert!(source_relation(override_value, flag, environment, model));
            assert!(source_relation(
                override_value,
                flag,
                environment,
                production
            ));
            assert_eq!(model, production);
            #[cfg(kani)]
            {
                kani::cover!(override_value.is_some(), "Set override selected");
                kani::cover!(
                    override_value.is_none() && flag.is_some(),
                    "changed flag selected"
                );
                kani::cover!(
                    override_value.is_none()
                        && flag.is_none()
                        && environment.is_some_and(|v| !v.is_empty()),
                    "nonempty environment selected"
                );
                kani::cover!(
                    override_value.is_none() && flag.is_none() && environment.is_none(),
                    "missing environment default"
                );
                kani::cover!(
                    override_value.is_none() && flag.is_none() && environment == Some(&[][..]),
                    "empty environment default"
                );
                kani::cover!(
                    override_value == Some(&[][..]),
                    "explicit empty Set override"
                );
                kani::cover!(
                    override_value.is_none() && flag == Some(&[][..]),
                    "explicit empty changed flag"
                );
                kani::cover!(production.len() == 2, "maximum modeled input length");
                kani::cover!(production.contains(&0xff), "raw non-UTF-8 byte selected");
            }
        });
    #[cfg(not(kani))]
    eprintln!(
        "ACCEPTED_DOMAIN_INPUTS={} REJECTED_BY_PRECONDITION=0",
        visits.load(std::sync::atomic::Ordering::Relaxed)
    );
}
