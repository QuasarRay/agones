#[path = "../examples/sdk_version_replay.rs"]
mod replay;

use agones_core::{AGONES_BUILD_VERSION, VERSION_ANNOTATION};
use agones_rusternetes::{
    NoopApiHooks, PodBuildFeatures, build_pod, mutate_fleet_creation, mutate_game_server_creation,
};

#[test]
fn all_version_witnesses_reach_the_actual_rusternetes_paths() {
    assert_eq!(replay::replay().as_array().unwrap().len(), 10);
}

#[test]
fn default_pod_and_admission_entry_points_use_the_compiled_identity() {
    let mut parent = replay::game_server_fixture();
    parent.metadata.annotations.insert(
        VERSION_ANNOTATION.into(),
        format!("{AGONES_BUILD_VERSION}-before"),
    );
    let pod = build_pod(&parent, vec![], PodBuildFeatures::default(), &NoopApiHooks).unwrap();
    assert_eq!(
        pod.metadata.annotations.as_ref().unwrap()[VERSION_ANNOTATION],
        AGONES_BUILD_VERSION
    );
    replay::assert_version_patch(
        mutate_game_server_creation(&serde_json::to_value(parent).unwrap()).unwrap(),
        AGONES_BUILD_VERSION,
    );
    let mut fleet = replay::fleet_fixture();
    fleet.metadata.annotations.insert(
        VERSION_ANNOTATION.into(),
        format!("{AGONES_BUILD_VERSION}-before"),
    );
    replay::assert_version_patch(
        mutate_fleet_creation(&serde_json::to_value(fleet).unwrap()).unwrap(),
        AGONES_BUILD_VERSION,
    );
}
