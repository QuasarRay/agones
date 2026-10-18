#[path = "../examples/gameserver_defaults_replay.rs"]
mod witnesses;

#[path = "../examples/gameserver_source_defaulting_replay.rs"]
mod original_source;

#[test]
fn every_exact_source_defaulting_occurrence_replays_through_cgp() {
    assert_eq!(original_source::replay().as_array().unwrap().len(), 304);
}

#[test]
fn ordinary_source_defaulting_contract_and_wire_witnesses() {
    assert_eq!(witnesses::replay().as_array().unwrap().len(), 14);
}

#[test]
fn the_same_defaulting_provider_runs_in_a_minimal_composed_context() {
    use agones_core::*;
    use cgp::prelude::*;

    #[derive(HasField)]
    struct Captured {
        agones_build_version: AgonesBuildVersion,
        game_server_feature_flags: FeatureFlags,
    }
    delegate_and_check_components! {
        Captured {
            AgonesBuildVersionGetterComponent: UseField<Symbol!("agones_build_version")>,
            AgonesVersionAnnotationGetterComponent: EncodeAgonesVersionAnnotation,
            GameServerFeatureFlagsGetterComponent: UseField<Symbol!("game_server_feature_flags")>,
            GameServerDefaulterComponent: ApplyAgonesGameServerDefaults,
        }
    }
    let context = Captured {
        agones_build_version: AgonesBuildVersion::new(b"composed-version".to_vec()),
        game_server_feature_flags: FeatureFlags {
            player_tracking: true,
        },
    };
    let mut gs = GameServer::default();
    gs.spec.players = Some(PlayersSpec {
        initial_capacity: 9,
    });
    gs.apply_defaults_with_context(&context);
    assert_eq!(
        gs.metadata.annotations[VERSION_ANNOTATION],
        "composed-version"
    );
    assert_eq!(gs.status.players.as_ref().unwrap().capacity, 9);
    assert!(gs.status.players.as_ref().unwrap().ids.is_nil());
    assert_eq!(gs.status.state, Some(GameServerState::Creating));
}
