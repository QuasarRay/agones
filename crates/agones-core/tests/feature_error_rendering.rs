#[path = "../examples/feature_error_replay.rs"]
mod replay;

#[test]
fn all_error_witnesses_reach_the_public_cgp_parser_and_renderer() {
    assert_eq!(replay::replay_feature_error_witnesses().len(), 19);
}
