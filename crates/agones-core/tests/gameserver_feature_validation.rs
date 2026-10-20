use agones_core::*;
use cgp::prelude::*;

fn context(enabled: bool) -> GameServerDefaultingContext {
    GameServerDefaultingContext {
        game_server_feature_flags: FeatureFlags {
            player_tracking: enabled,
        },
        ..Default::default()
    }
}

fn valid_spec() -> GameServerSpec {
    let mut spec = GameServerSpec::default();
    spec.template.spec.containers = vec![Container {
        name: "game".into(),
        ..Default::default()
    }]
    .into();
    spec.apply_defaults();
    spec
}

fn forbidden(path: &str) -> ValidationError {
    ValidationError {
        kind: ValidationErrorKind::Forbidden,
        path: path.into(),
        bad_value: None,
        detail: "Value cannot be set unless feature flag PlayerTracking is enabled".into(),
    }
}

#[test]
fn feature_policy_preserves_presence_flag_and_exact_error_contract() {
    for enabled in [false, true] {
        for capacity in [None, Some(0), Some(i64::MIN), Some(i64::MAX)] {
            let mut spec = valid_spec();
            spec.players = capacity.map(|initial_capacity| PlayersSpec { initial_capacity });
            let original = spec.clone();
            for path in ["", "spec", "spec.template.spec"] {
                let expected = if !enabled && capacity.is_some() {
                    vec![forbidden(&if path.is_empty() {
                        "players".into()
                    } else {
                        format!("{path}.players")
                    })]
                } else {
                    vec![]
                };
                assert_eq!(
                    context(enabled).validate_game_server_features(&spec, path),
                    expected
                );
            }
            assert_eq!(spec, original);
        }
    }
}

#[test]
fn feature_policy_runs_before_both_development_server_paths() {
    let spec = GameServerSpec {
        players: Some(PlayersSpec::default()),
        ..Default::default()
    };
    assert_eq!(spec.validate("127.0.0.1"), vec![forbidden("spec.players")]);
    assert!(
        spec.validate_with_context(&context(true), "127.0.0.1")
            .is_empty()
    );
    let errors = spec.validate("invalid-address");
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0], forbidden("spec.players"));
    assert_eq!(errors[1].kind, ValidationErrorKind::Invalid);
    assert_eq!(
        errors[1].path,
        format!("metadata.annotations.{DEV_ADDRESS_ANNOTATION}")
    );
    let ordinary = spec.validate("");
    assert_eq!(ordinary[0], forbidden("spec.players"));
    assert!(ordinary.iter().any(|e| e.path == "spec.container"));
}

#[test]
fn direct_resource_consumers_use_shared_policy_and_nested_paths() {
    let mut spec = valid_spec();
    spec.players = Some(PlayersSpec {
        initial_capacity: 9,
    });
    let gs = GameServer {
        spec: spec.clone(),
        ..Default::default()
    };
    assert_eq!(gs.validate(), vec![forbidden("spec.players")]);
    assert!(gs.validate_with_context(&context(true)).is_empty());
    let mut fleet = Fleet::default();
    fleet.spec.template.spec = spec.clone();
    fleet.apply_defaults();
    assert_eq!(
        fleet.validate(),
        vec![forbidden("spec.template.spec.players")]
    );
    assert!(fleet.validate_with_context(&context(true)).is_empty());
    let mut set = GameServerSet::default();
    set.spec.template.spec = spec;
    assert_eq!(
        set.validate(),
        vec![forbidden("spec.template.spec.players")]
    );
    assert!(set.validate_with_context(&context(true)).is_empty());
}

#[derive(HasField)]
struct FeatureOnlyContext {
    game_server_feature_flags: FeatureFlags,
}

delegate_and_check_components! {
    FeatureOnlyContext {
        GameServerFeatureFlagsGetterComponent: UseField<Symbol!("game_server_feature_flags")>,
        GameServerFeatureValidatorComponent: ValidateAgonesGameServerFeatures,
    }
}

#[test]
fn same_provider_accepts_a_context_with_only_its_feature_dependency() {
    let context = FeatureOnlyContext {
        game_server_feature_flags: FeatureFlags {
            player_tracking: true,
        },
    };
    let mut spec = valid_spec();
    spec.players = Some(PlayersSpec::default());
    assert!(spec.validate_with_context(&context, "").is_empty());
}
