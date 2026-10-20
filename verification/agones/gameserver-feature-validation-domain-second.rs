//! Post-gate source-contract proof of the real CGP PlayerTracking decision.
//! This gives no qualified original table-assertion or unbounded-path credit.
use cgp::prelude::*;

use agones_core::{
    CanValidateGameServerFeatures, FeatureFlags, GameServerSpec, PlayersSpec, ValidationError,
    ValidationErrorKind,
};

#[derive(Clone, Copy)]
struct ErrorView<'a> {
    kind: ValidationErrorKind,
    path: &'a str,
    bad_value: Option<&'a str>,
    detail: &'a str,
}

#[derive(Clone, Copy)]
struct Observation<'a> {
    count: usize,
    error: Option<ErrorView<'a>>,
}

fn observe(errors: &[ValidationError]) -> Observation<'_> {
    Observation {
        count: errors.len(),
        error: errors.first().map(|error| ErrorView {
            kind: error.kind,
            path: &error.path,
            bad_value: error.bad_value.as_deref(),
            detail: &error.detail,
        }),
    }
}

fn reference(enabled: bool, present: bool) -> Observation<'static> {
    match (enabled, present) {
        (false, true) => Observation {
            count: 1,
            error: Some(ErrorView {
                kind: ValidationErrorKind::Forbidden,
                path: "spec.players",
                bad_value: None,
                detail: "Value cannot be set unless feature flag PlayerTracking is enabled",
            }),
        },
        (false, false) | (true, false) | (true, true) => Observation {
            count: 0,
            error: None,
        },
    }
}

fn satisfies_source_relation(enabled: bool, present: bool, output: Observation<'_>) -> bool {
    if enabled || !present {
        return output.count == 0 && output.error.is_none();
    }
    let Some(error) = output.error else {
        return false;
    };
    output.count == 1
        && error.kind == ValidationErrorKind::Forbidden
        && error.path == "spec.players"
        && error.bad_value.is_none()
        && error.detail == "Value cannot be set unless feature flag PlayerTracking is enabled"
}

#[derive(HasField)]
struct FeatureContext {
    game_server_feature_flags: FeatureFlags,
}

delegate_and_check_components! {
    FeatureContext {
        agones_core::GameServerFeatureFlagsGetterComponent: UseField<Symbol!("game_server_feature_flags")>,
        agones_core::GameServerFeatureValidatorComponent: agones_core::ValidateAgonesGameServerFeatures,
    }
}

#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn gameserver_feature_validation_common_domain() {
    #[cfg(not(kani))]
    let visits = std::sync::atomic::AtomicUsize::new(0);
    bolero::check!()
        .with_type::<(bool, bool, i64)>()
        .with_iterations(8192)
        .for_each(|&(enabled, present, initial_capacity)| {
            #[cfg(not(kani))]
            visits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let context = FeatureContext {
                game_server_feature_flags: FeatureFlags {
                    player_tracking: enabled,
                },
            };
            let spec = GameServerSpec {
                players: present.then_some(PlayersSpec { initial_capacity }),
                ..Default::default()
            };
            let model = reference(enabled, present);
            let errors = context.validate_game_server_features(&spec, "spec");
            let actual = observe(&errors);
            assert!(satisfies_source_relation(enabled, present, model));
            assert!(satisfies_source_relation(enabled, present, actual));
            // Each relation fixes all observable fields uniquely; equal counts also
            // check agreement without a duplicate heap/string comparison.
            assert_eq!(model.count, actual.count);
            #[cfg(kani)]
            {
                kani::cover!(
                    !enabled && !present,
                    "disabled PlayerTracking absent players"
                );
                kani::cover!(
                    !enabled && present,
                    "disabled PlayerTracking present players forbidden"
                );
                kani::cover!(enabled && !present, "enabled PlayerTracking absent players");
                kani::cover!(
                    enabled && present,
                    "enabled PlayerTracking present players accepted"
                );
                kani::cover!(
                    present && initial_capacity == 0,
                    "present zero player capacity remains present"
                );
                kani::cover!(
                    !enabled && present && initial_capacity == i64::MIN,
                    "disabled minimum player capacity remains forbidden"
                );
                kani::cover!(
                    enabled && present && initial_capacity == i64::MAX,
                    "enabled maximum player capacity remains accepted"
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
fn source_relation_rejects_flag_presence_and_error_field_faults() {
    let expected = vec![ValidationError {
        kind: ValidationErrorKind::Forbidden,
        path: "spec.players".into(),
        bad_value: None,
        detail: "Value cannot be set unless feature flag PlayerTracking is enabled".into(),
    }];
    assert!(satisfies_source_relation(false, true, observe(&expected)));
    assert!(!satisfies_source_relation(true, true, observe(&expected)));
    assert!(!satisfies_source_relation(false, false, observe(&expected)));
    assert!(!satisfies_source_relation(false, true, observe(&[])));
    assert!(!satisfies_source_relation(
        false,
        true,
        observe(&[expected[0].clone(), expected[0].clone()])
    ));
    for field in 0..4 {
        let mut wrong = expected.clone();
        match field {
            0 => wrong[0].kind = ValidationErrorKind::Invalid,
            1 => wrong[0].path = "players".into(),
            2 => wrong[0].bad_value = Some("0".into()),
            _ => wrong[0].detail = "PlayerTracking".into(),
        }
        assert!(!satisfies_source_relation(false, true, observe(&wrong)));
    }
}
