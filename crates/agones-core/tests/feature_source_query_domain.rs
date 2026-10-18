//! Generated fixed-case harnesses preserve every original action/assertion.
//! Each uses unit under both engines. The aggregate u8 sampler below only tests
//! dispatch; it receives no symbolic-proof credit for the generated cases.
#![allow(unused_assignments, unused_mut)]
include!("generated/feature_gate_cases.rs");

#[test]
fn all_source_query_cases_are_mandatory_witnesses() {
    for case in 0..5 {
        check_source_query_case(case);
    }
}

#[test]
fn source_query_case_selector_sampling() {
    bolero::check!()
        .with_type::<u8>()
        .with_iterations(512)
        .for_each(|tag| check_source_query_case(tag % 5));
}
