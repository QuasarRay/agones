use cpg_rs::graph::*;
use serde_json::{json, Value as Json};

fn handle(node: &DetachedNode) -> NodeHandle { node.stored_ref().unwrap().unwrap() }
fn json_property(graph: &Graph, node: NodeHandle, name: &str) -> Json {
    Json::Array(graph.property_values(node, name).unwrap().iter().map(|value| match value {
        Value::String(value) => json!(value.as_ref()), Value::Int(value) => json!(value), Value::Boolean(value) => json!(value),
        Value::Null => Json::Null, other => panic!("unaccounted observation value {other:?}"),
    }).collect())
}
fn snapshot(graph: &Graph) -> Json {
    json!({"nodes": graph.all_nodes().map(|node| json!({"id": node.handle.id.0.to_string(), "label": node.label,
        "name": json_property(graph, node.handle, "NAME"), "order": json_property(graph, node.handle, "ORDER")})).collect::<Vec<_>>(),
        "edges": graph.all_edges().iter().map(|edge| json!({"source": edge.source.id.0.to_string(), "target": edge.target.id.0.to_string(),
            "label": edge.label, "property": match &edge.property { Value::Null => Json::Null, Value::String(value) => json!(value.as_ref()), other => panic!("unexpected edge property {other:?}") }})).collect::<Vec<_>>()})
}

#[test]
fn native_mutation_sequence_matches_the_original_pinned_cpg_observation() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../verification/agones/first-generation-graph-reference.json");
    let reference: Json = serde_json::from_slice(&std::fs::read(path).expect("original graph observation must exist")).unwrap();
    let mut cpg = Cpg::empty().unwrap();
    let graph = cpg.graph_mut();
    let first = DetachedNode::new("METHOD");
    let second = DetachedNode::new("METHOD");
    let identifier = DetachedNode::new("IDENTIFIER");
    let mut initial = DiffGraphBuilder::new();
    initial.add_node(first.clone()).add_node(second.clone()).add_edge(first.clone(), identifier.clone(), "AST", EdgeProperty::Default).add_node(first.clone());
    let first_changes = graph.apply_diff(initial).unwrap().applied_changes;
    let a = handle(&first); let b = handle(&second); let id = handle(&identifier);
    let before = snapshot(graph);
    let mut changes = DiffGraphBuilder::new();
    changes.set_node_property(a, "NAME", Value::string("alpha"));
    changes.set_node_property(a, "ORDER", Value::Int(1));
    changes.set_node_property(a, "ORDER", Value::Int(7));
    changes.add_edge(a, b, "REACHING_DEF", EdgeProperty::Default);
    changes.add_edge(a, b, "REACHING_DEF", EdgeProperty::Value(Value::string("x")));
    changes.add_edge(a, b, "AST", EdgeProperty::Default);
    let second_changes = graph.apply_diff(changes).unwrap().applied_changes;
    let after = snapshot(graph);
    let reaching: Vec<_> = graph.edges(a, Direction::Outgoing, Some("REACHING_DEF")).unwrap().iter().map(|edge| edge.handle).collect();
    let mut deletion = DiffGraphBuilder::new();
    deletion.remove_node(id).add_edge(a, id, "AST", EdgeProperty::Default);
    deletion.set_edge_property(reaching[0], Value::string("updated")).remove_edge(reaching[1]);
    let third_changes = graph.apply_diff(deletion).unwrap().applied_changes;
    let deleted = snapshot(graph);
    graph.close().unwrap();
    let closed_rejected = graph.apply_diff(DiffGraphBuilder::new()).is_err();
    let mut failing = Cpg::empty().unwrap();
    let removed = DetachedNode::new("METHOD");
    let mut allocate = DiffGraphBuilder::new(); allocate.add_node(removed.clone());
    failing.graph_mut().apply_diff(allocate).unwrap();
    let removed = handle(&removed);
    let mut failure = DiffGraphBuilder::new();
    failure.set_node_property(removed, "NAME", Value::string("deleted")).remove_node(removed);
    let deleted_property_failure = match failing.graph_mut().apply_diff(failure) {
        Err(cpg_rs::Error::SourceMatchFailure { operation: "SetNodeProperty" }) => "scala.MatchError",
        other => panic!("expected original MatchError, found {other:?}"),
    };
    let actual = json!({"firstChanges": first_changes, "secondChanges": second_changes, "thirdChanges": third_changes,
        "before": before, "after": after, "deleted": deleted, "closedRejected": closed_rejected,
        "deletedPropertyFailure": deleted_property_failure});
    assert_eq!(actual, reference);
}

#[test]
fn contained_nodes_are_materialized_once_and_cycles_keep_reference_identity() {
    let finding = DetachedNode::new("FINDING");
    let method = DetachedNode::new("METHOD");
    method.set("NAME", Value::string("child")).unwrap();
    finding.set("evidence", PropertyInput::List(vec![finding.clone().into(), method.clone().into(), method.clone().into()])).unwrap();
    let mut diff = DiffGraphBuilder::new();
    diff.add_node(finding.clone());
    let mut cpg = Cpg::empty().unwrap();
    let report = cpg.graph_mut().apply_diff(diff).unwrap();
    assert_eq!(report.submitted, 1);
    assert_eq!(report.created_nodes, 2);
    assert_eq!(report.applied_changes, 2);
    assert_eq!(cpg.graph().property(handle(&finding), "evidence").unwrap(), Some(Value::List(vec![Value::Node(handle(&finding)), Value::Node(handle(&method)), Value::Node(handle(&method))])));
    assert_eq!(cpg.graph().property(handle(&method), "NAME").unwrap(), Some(Value::string("child")));
    let mut other = Cpg::empty().unwrap();
    let mut diff = DiffGraphBuilder::new(); diff.add_node(method);
    assert!(other.graph_mut().apply_diff(diff).is_err());
}

#[test]
fn schema_defaults_presence_queries_multiedges_and_half_edges_remain_distinct() {
    let a = DetachedNode::new("METHOD");
    let b = DetachedNode::new("IDENTIFIER");
    let mut cpg = Cpg::from_initial(|diff| {
        diff.add_node(a.clone()).add_node(b.clone()); Ok(())
    }).unwrap();
    let a = handle(&a); let b = handle(&b);
    let graph = cpg.graph_mut();
    assert_eq!(graph.property(a, "NAME").unwrap(), Some(Value::string("<empty>")));
    assert!(graph.property_values(a, "NAME").unwrap().is_empty());
    assert_eq!(graph.property(a, "LINE_NUMBER").unwrap(), None);
    assert_eq!(graph.nodes_with_property(None, "NAME", "<empty>").count(), 0);
    let mut diff = DiffGraphBuilder::new();
    diff.set_node_property(a, "NAME", Value::string("alpha"));
    diff.set_node_property(b, "DYNAMIC_TYPE_HINT_FULL_NAME", Value::List(vec![Value::string("T"), Value::string("T")]));
    diff.set_node_property(a, "deprecated-unrecognized-field", Value::Int(1));
    diff.add_edge(a, b, "CALL", EdgeProperty::Value(Value::string("ignored by a propertyless edge")));
    diff.add_edge(a, b, "CALL", EdgeProperty::Default);
    diff.add_half_edge(a, b, "AST", EdgeProperty::Default, Direction::Outgoing);
    let report = graph.apply_diff(diff).unwrap();
    assert_eq!(report.applied_changes, 5);
    assert_eq!(graph.property(a, "NAME").unwrap(), Some(Value::string("alpha")));
    assert_eq!(graph.nodes_with_property(Some("METHOD"), "NAME", "alpha").count(), 1);
    assert_eq!(graph.nodes_with_property(None, "DYNAMIC_TYPE_HINT_FULL_NAME", "T").count(), 2);
    assert_eq!(graph.neighbors(a, Direction::Outgoing, Some("CALL")).unwrap(), [b, b]);
    assert_eq!(graph.neighbors(b, Direction::Incoming, Some("CALL")).unwrap(), [a, a]);
    assert_eq!(graph.neighbors(b, Direction::Incoming, Some("AST")).unwrap(), []);
    assert_eq!(graph.all_edges().len(), 3);
    assert_eq!(graph.nodes_of_base("AST_NODE").count(), 2);
    assert_eq!(graph.nodes("missing").count(), 0);
    assert_eq!(graph.node_count_by_label().get("METHOD"), Some(&1));
    assert_eq!(graph.node_count_by_label().get("IDENTIFIER"), Some(&1));
    let mut diff = DiffGraphBuilder::new();
    diff.set_node_property(a, "NAME", Value::Null);
    graph.apply_diff(diff).unwrap();
    assert_eq!(graph.property(a, "NAME").unwrap(), Some(Value::string("<empty>")));
}

#[test]
fn deletion_wins_over_earlier_changes_and_ids_are_not_reused() {
    let a = DetachedNode::new("METHOD");
    let b = DetachedNode::new("METHOD");
    let mut cpg = Cpg::from_initial(|diff| { diff.add_node(a.clone()).add_node(b.clone()); Ok(()) }).unwrap();
    let a = handle(&a); let b = handle(&b);
    let mut diff = DiffGraphBuilder::new();
    diff.add_edge(a, b, "CALL", EdgeProperty::Default);
    diff.remove_node(a).remove_node(a);
    let report = cpg.graph_mut().apply_diff(diff).unwrap();
    assert_eq!(report.applied_changes, 1);
    assert!(cpg.graph().node(a.id).is_none());
    assert_eq!(cpg.graph().edge_count(), 0);
    let third = DetachedNode::new("METHOD");
    let mut diff = DiffGraphBuilder::new(); diff.add_node(third.clone());
    cpg.graph_mut().apply_diff(diff).unwrap();
    assert_eq!(handle(&third).id.sequence(), 2);
    assert_eq!(cpg.graph().all_nodes().map(|node| node.handle.id.sequence()).collect::<Vec<_>>(), [1, 2]);
}
