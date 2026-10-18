// Deliberately shadow the old crate-root target: the derive must resolve the
// actual runtime trait, including in same-package integration test targets.
#[allow(dead_code)]
trait AgonesApiResource {}

#[derive(agones_macros::AgonesApiResource)]
#[agones_resource(
    group = "agones.dev",
    version = "v1",
    kind = "GameServer",
    plural = "gameservers"
)]
struct Shape<T, const N: usize>
where
    T: Clone,
{
    marker: std::marker::PhantomData<[T; N]>,
}

#[test]
fn resource_derive_ignores_root_shadow_and_preserves_generic_parameters() {
    type Resource = Shape<String, 3>;
    assert_eq!(
        <Resource as agones_core::AgonesApiResource>::API_VERSION,
        "agones.dev/v1"
    );
    assert_eq!(
        <Resource as agones_core::AgonesApiResource>::KIND,
        "GameServer"
    );
    assert_eq!(
        <Resource as agones_core::AgonesApiResource>::PLURAL,
        "gameservers"
    );
}
