use agones_core::{
    AGONES_BUILD_VERSION, AGONES_BUILD_VERSION_BYTES, AgonesBuildVersionContext,
    CanGetAgonesVersionAnnotation, Fleet, GameServer, VERSION_ANNOTATION,
};

fn main() {
    let mut game_server = GameServer::default();
    game_server.apply_defaults();
    let mut fleet = Fleet::default();
    fleet.apply_defaults();
    println!(
        "{}",
        serde_json::json!({
            "raw_bytes": AGONES_BUILD_VERSION_BYTES,
            "constant": AGONES_BUILD_VERSION,
            "provider": AgonesBuildVersionContext::default().agones_version_annotation(),
            "game_server": game_server.metadata.annotations[VERSION_ANNOTATION],
            "fleet": fleet.metadata.annotations[VERSION_ANNOTATION],
        })
    );
}
