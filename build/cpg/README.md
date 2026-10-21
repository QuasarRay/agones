# Agones Joern CPG + Grafana

This directory provides a reproducible local pipeline for:

1. parsing the Agones Go source tree with Joern's Go frontend,
2. persisting the authoritative Joern CPG as `out/cpg.bin`,
3. exporting the CPG to GraphML,
4. deriving bounded Grafana Node Graph JSON from the GraphML export, and
5. serving the derived data to a provisioned Grafana dashboard.

Grafana is intentionally a visualization layer, not the source of truth. The Joern CPG remains `out/cpg.bin`. The JSON files under `out/grafana/` are bounded, regenerable views so Grafana does not attempt to render an entire repository-scale CPG in one panel.

## Prerequisites

- Docker with Compose v2.
- Enough memory for Joern to parse Agones. The exact requirement depends on the Joern build and source scope.
- Port 3000 available, or set `GRAFANA_PORT`.

The Joern container defaults to the official `ghcr.io/joernio/joern:nightly` image because Joern publishes `nightly` and `master` container tags rather than a reliable `latest` tag. Override it with `JOERN_IMAGE` when pinning a known-good release or digest.

Grafana is pinned to 12.2.5 and the Grafana Labs Infinity data source to 4.1.1.

## Quick start

```bash
cd build/cpg
make validate
make generate
make up
```

Open <http://localhost:3000/d/agones-joern-cpg/agones-joern-cpg>.

The local Grafana instance has anonymous Viewer access enabled and the login form disabled. It is intended only for local analysis.

The `make validate` target runs a small GraphML conversion regression test and validates the Compose file before the expensive Joern parse.

Stop the stack with:

```bash
make down
```

## Generated artifacts

`make generate` creates:

- `out/cpg.bin` — Joern's CPG database/file; authoritative analysis artifact.
- `out/graphml/` — Joern `--repr=all --format=graphml` export.
- `out/grafana/nodes.json` — bounded nodes for Grafana Node Graph.
- `out/grafana/edges.json` — bounded edges whose endpoints are in the node sample.
- `out/grafana/node-types.json` — node-label inventory.
- `out/grafana/edge-types.json` — edge-label inventory.
- `out/grafana/manifest.json` — generation metadata and sample sizes.

Generated files are ignored by Git.

## Configuration

Environment variables accepted by `generate.sh`:

| Variable | Default | Purpose |
| --- | --- | --- |
| `JOERN_IMAGE` | `ghcr.io/joernio/joern:nightly` | Joern container image. |
| `CPG_EXCLUDE_REGEX` | `(^|/)(vendor|\\.git)(/|$)` | Joern frontend exclusion regex. Vendor code is excluded by default so the graph describes Agones rather than vendored dependencies. |
| `CPG_ENABLE_FILE_CONTENT` | `0` | Set to `1` to ask the frontend to attach source file contents to FILE nodes. |
| `CPG_GRAFANA_MAX_NODES` | `200` | Maximum nodes emitted to the Node Graph sample. Grafana itself also defaults to 200 visible nodes. |
| `CPG_GRAFANA_MAX_EDGES` | `1000` | Maximum edges emitted to the Node Graph sample. |
| `CPG_GRAFANA_NODE_LABELS` | empty | Optional comma-separated node labels to keep, for example `METHOD,CALL,TYPE_DECL`. |
| `CPG_GRAFANA_EDGE_LABELS` | empty | Optional comma-separated edge labels to keep, for example `AST,CFG,CALL,REF`. |
| `GRAFANA_PORT` | `3000` | Host port for Grafana. |

Examples:

```bash
# Include source text on FILE nodes.
CPG_ENABLE_FILE_CONTENT=1 make generate

# Produce a method/call/type-oriented bounded view.
CPG_GRAFANA_NODE_LABELS=METHOD,CALL,TYPE_DECL \
CPG_GRAFANA_EDGE_LABELS=AST,CFG,CALL,REF \
make generate

# Pin Joern to a known digest.
JOERN_IMAGE='ghcr.io/joernio/joern@sha256:...' make generate
```

## Direct Joern access

Start a shell in the Joern container with the repository mounted read-only and the generated artifacts mounted read/write:

```bash
make joern-shell
```

From there, use `joern`, `joern-parse`, `joern-export`, or `joern-slice` against `/workspace` and `/out/cpg.bin`.

## Design notes

Joern's Go frontend is selected explicitly with `--language GOLANG`. The export step uses Joern's whole-graph GraphML export rather than a hand-maintained schema so new CPG node/edge kinds remain visible in the inventory automatically.

Grafana's Node Graph requires separate node and edge frames. The converter therefore translates GraphML into the field names Grafana expects (`id`, `source`, `target`, `title`, `subtitle`, and `mainstat`). The dashboard uses Grafana Labs' Infinity data source in backend mode to fetch the generated JSON through the internal `cpg-data` service.

The Node Graph is deliberately bounded. Repository-scale CPGs can contain far more nodes than a browser graph should display at once. Use Joern queries for exhaustive analysis and regenerate filtered Grafana samples for visual exploration.
