//! Ordinary implementation witnesses for source version annotation behavior.
use agones_core::{
    AgonesBuildVersionContext, Container, FeatureFlags, Fleet, GameServer, VERSION_ANNOTATION,
};
use agones_rusternetes::{
    AdapterError, ApiHooks, NoopApiHooks, PodBuildFeatures, build_pod_with_version_context,
    mutate_fleet_creation_with_version_context, mutate_game_server_creation_with_version_context,
};
use rusternetes_common::admission::AdmissionResponse;
use rusternetes_common::resources::Pod;

struct ObserveVersionHook<'a>(&'a str);

impl ApiHooks for ObserveVersionHook<'_> {
    fn mutate_game_server_pod(
        &self,
        _spec: &agones_core::GameServerSpec,
        pod: &mut Pod,
    ) -> Result<(), AdapterError> {
        let annotations = pod.metadata.annotations.as_mut().unwrap();
        assert_eq!(annotations[VERSION_ANNOTATION], self.0);
        annotations.insert(VERSION_ANNOTATION.into(), "product-hook-version".into());
        Ok(())
    }
}

pub fn game_server_fixture() -> GameServer {
    let mut gs = GameServer::default();
    gs.metadata.name = "version.server".into();
    gs.metadata.namespace = "default".into();
    gs.metadata
        .annotations
        .insert(VERSION_ANNOTATION.into(), "parent-version".into());
    gs.metadata.annotations.insert("keep".into(), "gs".into());
    gs.spec.container = "game".into();
    gs.spec
        .template
        .metadata
        .annotations
        .insert(VERSION_ANNOTATION.into(), "template-version".into());
    gs.spec
        .template
        .metadata
        .annotations
        .insert("keep".into(), "template".into());
    gs.spec.template.spec.containers.push(Container {
        name: "game".into(),
        image: "game:v1".into(),
        ..Default::default()
    });
    gs
}

pub fn fleet_fixture() -> Fleet {
    let mut fleet = Fleet::default();
    fleet
        .metadata
        .annotations
        .insert(VERSION_ANNOTATION.into(), "old".into());
    fleet
        .metadata
        .annotations
        .insert("keep".into(), "fleet".into());
    fleet
}

pub fn assert_version_patch(response: AdmissionResponse, expected: &str) {
    let AdmissionResponse::AllowWithPatch(patches) = response else {
        panic!("version overwrite must produce a real admission patch");
    };
    let version = patches
        .iter()
        .find(|patch| patch.path == "/metadata/annotations/agones.dev~1sdk-version")
        .expect("source version annotation patch");
    assert_eq!(
        version.value,
        Some(serde_json::Value::String(expected.into()))
    );
    assert!(
        !patches
            .iter()
            .any(|patch| patch.path == "/metadata/annotations/keep")
    );
}

pub fn replay() -> serde_json::Value {
    let family: serde_json::Value = serde_json::from_str(include_str!(
        "../../../verification/agones/sdk-version-witnesses.json"
    ))
    .unwrap();
    assert_eq!(family["schema_version"], 1);
    let cases = family["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 10);
    let mut observations = Vec::new();
    for case in cases {
        let raw = case["version_hex"].as_str().unwrap();
        let bytes: Vec<u8> = raw
            .as_bytes()
            .chunks_exact(2)
            .map(|part| u8::from_str_radix(std::str::from_utf8(part).unwrap(), 16).unwrap())
            .collect();
        assert_eq!(raw.len(), bytes.len() * 2);
        let expected = case["expected_annotation"].as_str().unwrap();
        let context = AgonesBuildVersionContext::from_bytes(bytes.clone());
        let mut gs = game_server_fixture();
        gs.apply_defaults_with_version_context(&context, FeatureFlags::default());
        let mut fleet = fleet_fixture();
        fleet.apply_defaults_with_version_context(&context);
        let parent = game_server_fixture();
        let pod = build_pod_with_version_context(
            &context,
            &parent,
            vec![],
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let pod_annotations = pod.metadata.annotations.as_ref().unwrap();
        let hooked = build_pod_with_version_context(
            &context,
            &parent,
            vec![],
            PodBuildFeatures::default(),
            &ObserveVersionHook(expected),
        )
        .unwrap();
        let hooked_version = &hooked.metadata.annotations.as_ref().unwrap()[VERSION_ANNOTATION];
        assert_eq!(hooked_version, "product-hook-version");
        assert_eq!(gs.metadata.annotations[VERSION_ANNOTATION], expected);
        assert_eq!(fleet.metadata.annotations[VERSION_ANNOTATION], expected);
        assert_eq!(pod_annotations[VERSION_ANNOTATION], expected);
        assert_eq!(gs.metadata.annotations["keep"], "gs");
        assert_eq!(fleet.metadata.annotations["keep"], "fleet");
        assert_eq!(pod_annotations["keep"], "template");
        assert_eq!(
            parent.metadata.annotations[VERSION_ANNOTATION],
            "parent-version"
        );
        assert_eq!(
            parent.spec.template.metadata.annotations[VERSION_ANNOTATION],
            "template-version"
        );
        assert_eq!(context.agones_build_version.as_bytes(), bytes);
        assert_version_patch(
            mutate_game_server_creation_with_version_context(
                &context,
                &serde_json::to_value(game_server_fixture()).unwrap(),
            )
            .unwrap(),
            expected,
        );
        assert_version_patch(
            mutate_fleet_creation_with_version_context(
                &context,
                &serde_json::to_value(fleet_fixture()).unwrap(),
            )
            .unwrap(),
            expected,
        );
        observations.push(serde_json::json!({
            "id": case["id"],
            "game_server": gs.metadata.annotations[VERSION_ANNOTATION],
            "fleet": fleet.metadata.annotations[VERSION_ANNOTATION],
            "pod": pod_annotations[VERSION_ANNOTATION],
            "hooked_pod": hooked_version,
        }));
    }
    serde_json::Value::Array(observations)
}

#[allow(dead_code)]
fn main() {
    println!("{}", serde_json::to_string_pretty(&replay()).unwrap());
}
