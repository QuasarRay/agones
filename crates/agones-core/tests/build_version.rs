use agones_core::{
    AGONES_BUILD_VERSION, AGONES_BUILD_VERSION_BYTES, AgonesBuildVersion,
    AgonesBuildVersionContext, AgonesBuildVersionGetterComponent,
    AgonesVersionAnnotationGetterComponent, CanGetAgonesVersionAnnotation,
    EncodeAgonesVersionAnnotation, FeatureFlags, Fleet, GameServer, VERSION_ANNOTATION,
};
use cgp::prelude::*;

#[derive(HasField)]
struct CapturedBuild {
    agones_build_version: AgonesBuildVersion,
}

delegate_and_check_components! {
    CapturedBuild {
        AgonesBuildVersionGetterComponent: UseField<Symbol!("agones_build_version")>,
        AgonesVersionAnnotationGetterComponent: EncodeAgonesVersionAnnotation,
    }
}

#[test]
fn the_same_provider_stamps_source_values_in_a_minimal_context() {
    for (raw, expected) in [
        (&b"dev"[..], "dev"),
        (&b""[..], ""),
        (&b"1.54.0-dev+abc"[..], "1.54.0-dev+abc"),
        ("release-λ😀".as_bytes(), "release-λ😀"),
        (&b"a\0\"\\\n"[..], "a\0\"\\\n"),
        (&b"\xff\xfe"[..], "��"),
        (&b"\xe2\x82"[..], "��"),
        (&b"\xed\xa0\x80"[..], "���"),
        ("�".as_bytes(), "�"),
        (&b"A\xf0\x9fB"[..], "A��B"),
    ] {
        let context = CapturedBuild {
            agones_build_version: AgonesBuildVersion::new(raw),
        };
        assert_eq!(context.agones_build_version.as_bytes(), raw);
        assert_eq!(context.agones_version_annotation(), expected);
        let mut game_server = GameServer::default();
        game_server
            .metadata
            .annotations
            .insert(VERSION_ANNOTATION.into(), "old".into());
        game_server
            .metadata
            .annotations
            .insert("keep".into(), "gs".into());
        game_server.apply_defaults_with_version_context(&context, FeatureFlags::default());
        assert_eq!(
            game_server.metadata.annotations[VERSION_ANNOTATION],
            expected
        );
        assert_eq!(game_server.metadata.annotations["keep"], "gs");
        let mut fleet = Fleet::default();
        fleet
            .metadata
            .annotations
            .insert(VERSION_ANNOTATION.into(), "old".into());
        fleet
            .metadata
            .annotations
            .insert("keep".into(), "fleet".into());
        fleet.apply_defaults_with_version_context(&context);
        assert_eq!(fleet.metadata.annotations[VERSION_ANNOTATION], expected);
        assert_eq!(fleet.metadata.annotations["keep"], "fleet");
        assert_eq!(context.agones_build_version.as_bytes(), raw);
    }
}

#[test]
fn default_entry_points_use_the_compiled_source_identity() {
    let context = AgonesBuildVersionContext::default();
    assert_eq!(
        context.agones_build_version.as_bytes(),
        AGONES_BUILD_VERSION_BYTES
    );
    assert_eq!(context.agones_version_annotation(), AGONES_BUILD_VERSION);
    assert_eq!(agones_core::DEFAULT_SDK_VERSION, AGONES_BUILD_VERSION);
    let mut game_server = GameServer::default();
    game_server.apply_defaults();
    let mut fleet = Fleet::default();
    fleet.apply_defaults();
    assert_eq!(
        game_server.metadata.annotations[VERSION_ANNOTATION],
        AGONES_BUILD_VERSION
    );
    assert_eq!(
        fleet.metadata.annotations[VERSION_ANNOTATION],
        AGONES_BUILD_VERSION
    );
}
