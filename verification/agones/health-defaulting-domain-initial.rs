//! Source-contract proof of the actual shared spec defaulting health behavior.
//! Other spec fields are zero; disabled and all three i32 inputs are arbitrary.
use agones_core::{GameServerSpec, Health};

fn reference(input: (bool, [i32; 3])) -> (bool, [i32; 3]) {
    let (disabled, values) = input;
    let defaults = [5, 3, 5];
    (
        disabled,
        std::array::from_fn(|i| match (disabled, values[i].cmp(&1)) {
            (false, std::cmp::Ordering::Less) => defaults[i],
            _ => values[i],
        }),
    )
}

fn satisfies_relation(input: (bool, [i32; 3]), output: (bool, [i32; 3])) -> bool {
    if input.0 != output.0 {
        return false;
    }
    if input.0 {
        return input.1 == output.1;
    }
    let constants = [5, 3, 5];
    (0..3).all(|i| {
        output.1[i] > 0
            && (input.1[i] > 0 || output.1[i] == constants[i])
            && (input.1[i] <= 0 || output.1[i] == input.1[i])
    })
}

#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(8))]
fn health_defaulting_common_domain() {
    #[cfg(not(kani))]
    let visits = std::sync::atomic::AtomicUsize::new(0);
    bolero::check!()
        .with_type::<(bool, i32, i32, i32)>()
        .with_iterations(8192)
        .for_each(|&(disabled, period, failure, delay)| {
            #[cfg(not(kani))]
            visits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let input = (disabled, [period, failure, delay]);
            let model = reference(input);
            let mut spec = GameServerSpec {
                health: Health {
                    disabled,
                    period_seconds: period,
                    failure_threshold: failure,
                    initial_delay_seconds: delay,
                },
                ..GameServerSpec::default()
            };
            spec.apply_defaults();
            let actual = (
                spec.health.disabled,
                [
                    spec.health.period_seconds,
                    spec.health.failure_threshold,
                    spec.health.initial_delay_seconds,
                ],
            );
            assert!(satisfies_relation(input, model));
            assert!(satisfies_relation(input, actual));
            assert_eq!(model, actual);
            #[cfg(kani)]
            {
                kani::cover!(
                    disabled && period <= 0 && failure <= 0 && delay <= 0,
                    "disabled health keeps nonpositive values"
                );
                kani::cover!(
                    !disabled && period <= 0 && failure <= 0 && delay <= 0,
                    "enabled health sets all three defaults"
                );
                kani::cover!(
                    !disabled && period == 1 && failure == 1 && delay == 1,
                    "small positive health values stay unchanged"
                );
                kani::cover!(
                    !disabled && period == 0,
                    "zero health threshold receives source default"
                );
                kani::cover!(
                    !disabled && period == i32::MIN && failure == i32::MAX,
                    "signed health extremes remain exact"
                );
                kani::cover!(
                    disabled && delay == i32::MIN,
                    "disabled minimum delay remains exact"
                );
                kani::cover!(
                    !disabled && period > 0 && failure <= 0 && delay > 0,
                    "mixed health thresholds default independently"
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
fn health_relation_rejects_overwriting_positive_or_disabled_values_and_wrong_defaults() {
    assert!(!satisfies_relation((false, [1, 1, 1]), (false, [5, 3, 5])));
    assert!(!satisfies_relation((true, [-1, 0, 2]), (true, [5, 3, 5])));
    assert!(!satisfies_relation((false, [0, 0, 0]), (false, [0, 3, 5])));
    assert!(!satisfies_relation((false, [0, 0, 0]), (false, [5, 5, 5])));
    assert!(!satisfies_relation((true, [1, 2, 3]), (false, [1, 2, 3])));
    assert!(satisfies_relation(
        (false, [1, 0, i32::MAX]),
        (false, [1, 3, i32::MAX])
    ));
}
