use agones_core::{GameServerStatus, GoSlice, PodSpec};
use serde_json::{Value, json};

#[test]
fn source_status_slice_wire_shapes_round_trip_without_normalizing_null() {
    for wire in [
        json!({"ports":null,"addresses":null}),
        json!({"ports":[],"addresses":[]}),
        json!({"ports":[{"name":"game","port":7777}],"addresses":[{"type":"InternalIP","address":"10.0.0.1"}]}),
    ] {
        let status: GameServerStatus = serde_json::from_value(wire.clone()).unwrap();
        let observed = serde_json::to_value(&status).unwrap();
        for key in ["ports", "addresses"] {
            assert_eq!(observed[key], wire[key]);
        }
    }
    let zero = serde_json::to_value(GameServerStatus::default()).unwrap();
    assert_eq!(zero["ports"], Value::Null);
    assert_eq!(zero["addresses"], Value::Null);
    assert!(zero.as_object().unwrap().contains_key("addresses"));
}

#[test]
fn nil_and_allocated_slices_keep_distinct_value_and_append_semantics() {
    let mut nil = GoSlice::<u8>::default();
    let empty: GoSlice<u8> = Vec::new().into();
    assert!(nil.is_nil());
    assert!(!empty.is_nil());
    assert_ne!(nil, empty);
    assert_eq!(serde_json::to_value(&nil).unwrap(), Value::Null);
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!([]));
    nil.extend(std::iter::empty());
    assert!(nil.is_nil());
    nil.push(7);
    assert!(!nil.is_nil());
    assert_eq!(serde_json::to_value(&nil).unwrap(), json!([7]));
    assert_eq!(nil.iter().copied().sum::<u8>(), 7);
}

#[test]
fn source_required_pod_containers_keep_nil_allocated_and_populated_wire_values() {
    for containers in [Value::Null, json!([]), json!([{"name":"game"}])] {
        let spec: PodSpec =
            serde_json::from_value(json!({"containers":containers.clone()})).unwrap();
        let observed = serde_json::to_value(spec).unwrap();
        assert_eq!(observed["containers"], containers);
        assert!(observed.as_object().unwrap().contains_key("containers"));
    }
    assert_eq!(
        serde_json::to_value(PodSpec::default()).unwrap()["containers"],
        Value::Null
    );
}
