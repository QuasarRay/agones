//! Post-gate source-contract proof of the real CGP PlayerTracking decision.
//! This gives no qualified original table-assertion or unbounded-path credit.
use agones_core::{
    CanValidateGameServerFeatures, FeatureFlags, GameServerDefaultingContext, GameServerSpec,
    PlayersSpec, ValidationError, ValidationErrorKind,
};

fn reference(enabled: bool, present: bool) -> Vec<ValidationError> {
    match (enabled, present) {
        (false, true) => vec![ValidationError {
            kind: ValidationErrorKind::Forbidden,
            path: "spec.players".into(),
            bad_value: None,
            detail: "Value cannot be set unless feature flag PlayerTracking is enabled".into(),
        }],
        (false, false) | (true, false) | (true, true) => vec![],
    }
}

fn satisfies_source_relation(enabled: bool, present: bool, errors: &[ValidationError]) -> bool {
    if enabled || !present {
        return errors.is_empty();
    }
    errors.len() == 1
        && errors[0].kind == ValidationErrorKind::Forbidden
        && errors[0].path == "spec.players"
        && errors[0].bad_value.is_none()
        && errors[0].detail == "Value cannot be set unless feature flag PlayerTracking is enabled"
}

#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(16))]
fn gameserver_feature_validation_common_domain() {
    #[cfg(not(kani))]
    let visits = std::sync::atomic::AtomicUsize::new(0);
    bolero::check!()
        .with_type::<(bool, bool, i64)>()
        .with_iterations(8192)
        .for_each(|&(enabled, present, initial_capacity)| {
            #[cfg(not(kani))]
            visits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let context = GameServerDefaultingContext {
                game_server_feature_flags: FeatureFlags {
                    player_tracking: enabled,
                },
                ..Default::default()
            };
            let spec = GameServerSpec {
                players: present.then_some(PlayersSpec { initial_capacity }),
                ..Default::default()
            };
            let model = reference(enabled, present);
            let actual = context.validate_game_server_features(&spec, "spec");
            assert!(satisfies_source_relation(enabled, present, &model));
            assert!(satisfies_source_relation(enabled, present, &actual));
            assert_eq!(model, actual);
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
    let expected = reference(false, true);
    assert!(satisfies_source_relation(false, true, &expected));
    assert!(!satisfies_source_relation(true, true, &expected));
    assert!(!satisfies_source_relation(false, false, &expected));
    assert!(!satisfies_source_relation(false, true, &[]));
    assert!(!satisfies_source_relation(
        false,
        true,
        &[expected[0].clone(), expected[0].clone()]
    ));
    for field in 0..4 {
        let mut wrong = expected.clone();
        match field {
            0 => wrong[0].kind = ValidationErrorKind::Invalid,
            1 => wrong[0].path = "players".into(),
            2 => wrong[0].bad_value = Some("0".into()),
            _ => wrong[0].detail = "PlayerTracking".into(),
        }
        assert!(!satisfies_source_relation(false, true, &wrong));
    }
}
