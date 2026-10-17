use agones_core::{AgonesApiResource, GameServer};
use rusternetes_common::resources::{
    Event, EventSource, EventType, ObjectReference,
};
use rusternetes_storage::{build_key, Storage};

/// Record a Kubernetes Event for a GameServer without coupling reconciliation
/// success to event persistence.
///
/// client-go's EventRecorder sends events asynchronously. Controllers therefore
/// do not fail their reconciliation after a successful state write merely
/// because the event sink is temporarily unavailable. Rusternetes storage is
/// synchronous at this boundary, so this helper deliberately keeps event
/// recording best-effort while centralizing the wire shape shared by Agones
/// controllers.
pub(crate) async fn record_game_server_event<S: Storage>(
    storage: &S,
    game_server: &GameServer,
    component: &str,
    event_type: EventType,
    reason: &str,
    message: &str,
) {
    let involved_object = ObjectReference {
        api_version: Some(GameServer::API_VERSION.to_owned()),
        kind: Some(GameServer::KIND.to_owned()),
        namespace: Some(game_server.metadata.namespace.clone()),
        name: Some(game_server.metadata.name.clone()),
        uid: (!game_server.metadata.uid.is_empty())
            .then(|| game_server.metadata.uid.clone()),
        resource_version: game_server.metadata.resource_version.clone(),
        field_path: None,
    };

    let name = Event::generate_name(&involved_object, reason);
    let mut event = Event::new(
        name,
        game_server.metadata.namespace.clone(),
        involved_object,
        reason.to_owned(),
        message.to_owned(),
        event_type,
    );
    event.source = EventSource {
        component: component.to_owned(),
        host: None,
    };

    let key = build_key(
        "events",
        Some(game_server.metadata.namespace.as_str()),
        event.metadata.name.as_str(),
    );
    let _ = storage.create(&key, &event).await;
}
