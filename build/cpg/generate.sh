#!/usr/bin/env bash
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

export JOERN_IMAGE="${JOERN_IMAGE:-ghcr.io/joernio/joern:nightly}"
export CPG_EXCLUDE_REGEX="${CPG_EXCLUDE_REGEX:-(^|/)(vendor|\\.git)(/|$)}"
export CPG_ENABLE_FILE_CONTENT="${CPG_ENABLE_FILE_CONTENT:-0}"
export CPG_GRAFANA_MAX_NODES="${CPG_GRAFANA_MAX_NODES:-200}"
export CPG_GRAFANA_MAX_EDGES="${CPG_GRAFANA_MAX_EDGES:-1000}"
export CPG_GRAFANA_NODE_LABELS="${CPG_GRAFANA_NODE_LABELS:-}"
export CPG_GRAFANA_EDGE_LABELS="${CPG_GRAFANA_EDGE_LABELS:-}"

mkdir -p out
rm -f out/cpg.bin
rm -rf out/graphml out/grafana
mkdir -p out/graphml out/grafana

echo "==> Generating Joern CPG with GOLANG frontend"
docker compose run --rm   -e CPG_EXCLUDE_REGEX   -e CPG_ENABLE_FILE_CONTENT   joern '
    set -euo pipefail
    frontend_args=(--output /out/cpg.bin --exclude-regex "$CPG_EXCLUDE_REGEX")
    if [[ "$CPG_ENABLE_FILE_CONTENT" == "1" ]]; then
      frontend_args+=(--enable-file-content)
    fi
    joern-parse /workspace --language GOLANG --frontend-args "${frontend_args[@]}"
  '

test -s out/cpg.bin || {
  echo "Joern did not create out/cpg.bin" >&2
  exit 1
}

echo "==> Exporting full CPG to GraphML"
docker compose run --rm joern '
  set -euo pipefail
  rm -rf /out/graphml/*
  joern-export /out/cpg.bin --repr=all --format=graphml --out /out/graphml
'

echo "==> Converting GraphML to bounded Grafana Node Graph JSON"
converter_args=(
  /out/graphml
  --out-dir /out/grafana
  --max-nodes "$CPG_GRAFANA_MAX_NODES"
  --max-edges "$CPG_GRAFANA_MAX_EDGES"
)
if [[ -n "$CPG_GRAFANA_NODE_LABELS" ]]; then
  converter_args+=(--node-labels "$CPG_GRAFANA_NODE_LABELS")
fi
if [[ -n "$CPG_GRAFANA_EDGE_LABELS" ]]; then
  converter_args+=(--edge-labels "$CPG_GRAFANA_EDGE_LABELS")
fi

docker compose run --rm converter "${converter_args[@]}"

echo
echo "CPG ready:"
echo "  $here/out/cpg.bin"
echo "Grafana data ready:"
echo "  $here/out/grafana"
echo
echo "Run 'make up' to start Grafana."
