#[path = "../examples/sdk_version_source_replay.rs"]
mod source_replay;

#[path = "generated/sdk_version_cases.rs"]
mod generated;

#[test]
fn all_source_version_assertions_replay_from_exact_source_fixtures() {
    assert_eq!(source_replay::replay().as_array().unwrap().len(), 19);
}
