# cpg-rs

Native Rust CPG support and generation tooling for the Agones translation pipeline.
CGP providers own graph creation, mutation, database access, loading, and pass lifecycle.

Grafeo 0.5.44 is the graph database engine and persistence engine. Both an empty CPG
and a CPG loaded from protobuf or Flatgraph use Grafeo records. The CPG API's borrowed
node and edge views are disposable read projections; mutations go through Grafeo's
CRUD APIs and invalidate those projections. There is no parallel writable graph.

```rust
use cpg_rs::graph::{Cpg, CpgContext, CanAccessCpgDatabase, DetachedNode, Value};

let mut cpg = Cpg::with_storage("analysis.grafeo")?;
let method = DetachedNode::new("METHOD");
method.set("NAME", Value::string("ready"))?;
let mut diff = Cpg::new_diff_graph_builder();
diff.add_node(method);
cpg.graph_mut().apply_diff(diff)?;
let result = CpgContext.query_cpg_database(
    cpg.graph(), "MATCH (n:METHOD) RETURN count(n)",
)?;
cpg.close()?;
# Ok::<(), cpg_rs::Error>(())
```

`with_storage` opens a Grafeo checkpoint into Grafeo's memory engine. Changed graphs
are saved on explicit `close()` when persistence is enabled. Unchanged new graphs do
not create files. `with_storage_options(path, false)` permits memory-only mutations
without changing the checkpoint. `save_database` explicitly saves any graph, including
one imported from Flatgraph. Saving uses Grafeo's single-file format, writes a sibling
temporary file, syncs it, and atomically replaces the destination. It does not require
a particular filename extension. Callers must propagate errors from `close()` or save.

GQL queries use a Grafeo read-only session, so query text cannot mutate the database
behind CPG validation. CPG nodes carry their original label and `__CpgRsNode`.
`__cpg_id` retains the source kind/sequence id; `__cpg_deleted` marks retained deleted
slots. CPG properties use `cpg:<name>` keys. Their ordered values carry `[type, payload]`
envelopes so nulls, integer widths, UTF-16 code units, node references, and floating
point bits survive persistence. For example, the first explicit name is
``n.`cpg:NAME`[0][1]``. Query live nodes with `WHERE n.__cpg_deleted = false`.
Real Grafeo edges carry the CPG edge label, independent incoming/outgoing visibility
and order, and a tagged `cpg:value` property. A separate metadata node binds the schema
identity, format version, next edge id, and Flatgraph physical-column compatibility
metadata. Internal metadata labels are not returned by the CPG node API.

Flatgraph v0 is a compatibility format for the pinned Joern frontend. Use
`graph::storage::read_graph(schema, path)` to import it into Grafeo and
`graph::storage::write_graph(graph, path)` to export it. `Cpg::with_storage` expects
Grafeo storage. The `cpg-storage` command supports explicit import, export, and
Flatgraph rewrite operations; an optional schema bootstrap supports other schemas.

`Cpg::load(input)` detects Grafeo, protobuf ZIP, Flatgraph, and legacy OverflowDB
storage. A Grafeo input opens directly. Other formats are imported into Grafeo and
use a sibling `<input-name>.grafeo` checkpoint. `Cpg::load_to(input, destination)`
selects a separate checkpoint and overwrites an existing destination. Legacy and
Flatgraph conversion create a checkpoint during loading; protobuf changes follow
the explicit-close lifecycle. The input data remains unchanged when the destination
is separate. CGP composes the format loader, protobuf loader, legacy importer, and
Grafeo storage opener.

Legacy import reads the pinned H2 MVStore 1.4.200 format and OverflowDB v2 messages
in Rust, including committed B-tree roots, page checksums, LZF and Deflate pages,
string mappings, and MessagePack value tags. Original converter limitations remain
observable, including its primitive-array and character-value failures. The MVStore
format module retains the H2 source attribution and `MPL-2.0 OR EPL-1.0` notice;
the original published source and license declarations are retained with its pin.

The focused CPG tooling tests are distinct from generated Agones validation. The
first complete generated Agones translation has not yet been produced.
