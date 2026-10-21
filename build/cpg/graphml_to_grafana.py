#!/usr/bin/env python3
"""Convert Joern GraphML exports into bounded Grafana Node Graph JSON.

The Joern CPG remains authoritative. This utility emits a browser-sized sample
plus complete node/edge label inventories for visualization.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
import sys
import xml.etree.ElementTree as ET


def local_name(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


def discover_graphml(path: Path) -> list[Path]:
    if path.is_file():
        return [path]
    if not path.is_dir():
        raise FileNotFoundError(path)

    files = sorted(
        p
        for p in path.rglob("*")
        if p.is_file() and p.suffix.lower() in {".graphml", ".xml"}
    )
    if not files:
        raise FileNotFoundError(f"no GraphML/XML files under {path}")
    return files


def read_key_map(path: Path) -> dict[str, str]:
    """Read GraphML key declarations without walking the whole graph."""
    keys: dict[str, str] = {}
    for event, elem in ET.iterparse(path, events=("start", "end")):
        tag = local_name(elem.tag)
        if event == "start" and tag == "graph":
            break
        if event == "end" and tag == "key":
            key_id = elem.attrib.get("id")
            if key_id:
                keys[key_id] = elem.attrib.get("attr.name", key_id)
            elem.clear()
    return keys


def data_properties(elem: ET.Element, keys: dict[str, str]) -> dict[str, str]:
    props: dict[str, str] = {}
    for child in elem:
        if local_name(child.tag) != "data":
            continue
        key_id = child.attrib.get("key", "")
        name = keys.get(key_id, key_id)
        props[name] = child.text or ""
    return props


def pick(props: dict[str, str], *names: str) -> str:
    if not props:
        return ""
    normalized = {k.upper(): v for k, v in props.items()}
    for name in names:
        value = normalized.get(name.upper(), "")
        if value:
            return value
    return ""


def shorten(value: str, limit: int = 120) -> str:
    value = " ".join(value.split())
    if len(value) <= limit:
        return value
    return value[: limit - 1] + "…"


def parse_filter(raw: str | None) -> set[str] | None:
    if not raw:
        return None
    values = {item.strip().upper() for item in raw.split(",") if item.strip()}
    return values or None


def node_type(props: dict[str, str]) -> str:
    return (pick(props, "label", "_label", "labelV", "LABEL") or "NODE").upper()


def edge_type(props: dict[str, str]) -> str:
    return (pick(props, "label", "_label", "labelE", "LABEL") or "EDGE").upper()


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def collect_nodes(
    graph_files: list[Path],
    max_nodes: int,
    allowed_labels: set[str] | None,
) -> tuple[list[dict[str, object]], set[str], Counter[str]]:
    selected: list[dict[str, object]] = []
    selected_ids: set[str] = set()
    counts: Counter[str] = Counter()

    for graph_index, path in enumerate(graph_files):
        keys = read_key_map(path)
        prefix = f"g{graph_index}:"

        for _, elem in ET.iterparse(path, events=("end",)):
            tag = local_name(elem.tag)
            if tag == "node":
                props = data_properties(elem, keys)
                kind = node_type(props)
                counts[kind] += 1

                if len(selected) < max_nodes and (
                    allowed_labels is None or kind in allowed_labels
                ):
                    raw_id = elem.attrib.get("id", str(len(selected)))
                    graph_id = prefix + raw_id
                    title = pick(props, "NAME", "FULL_NAME", "CODE", "FILENAME") or kind
                    filename = pick(props, "FILENAME")
                    line = pick(props, "LINE_NUMBER", "LINE")
                    full_name = pick(props, "FULL_NAME")
                    code = pick(props, "CODE")

                    row: dict[str, object] = {
                        "id": graph_id,
                        "title": shorten(title),
                        "subtitle": kind,
                    }
                    if line:
                        row["mainstat"] = line
                    if filename:
                        row["detail__file"] = shorten(filename, 240)
                    if full_name:
                        row["detail__full_name"] = shorten(full_name, 240)
                    if code:
                        row["detail__code"] = shorten(code, 300)

                    selected.append(row)
                    selected_ids.add(graph_id)

            if tag in {"node", "edge"}:
                elem.clear()

    return selected, selected_ids, counts


def collect_edges(
    graph_files: list[Path],
    selected_ids: set[str],
    max_edges: int,
    allowed_labels: set[str] | None,
) -> tuple[list[dict[str, object]], Counter[str]]:
    selected: list[dict[str, object]] = []
    counts: Counter[str] = Counter()

    for graph_index, path in enumerate(graph_files):
        keys = read_key_map(path)
        prefix = f"g{graph_index}:"
        edge_ordinal = 0

        for _, elem in ET.iterparse(path, events=("end",)):
            tag = local_name(elem.tag)
            if tag == "edge":
                props = data_properties(elem, keys)
                kind = edge_type(props)
                counts[kind] += 1

                source = prefix + elem.attrib.get("source", "")
                target = prefix + elem.attrib.get("target", "")
                if (
                    len(selected) < max_edges
                    and source in selected_ids
                    and target in selected_ids
                    and (allowed_labels is None or kind in allowed_labels)
                ):
                    raw_id = elem.attrib.get("id")
                    graph_id = prefix + (raw_id if raw_id else f"edge-{edge_ordinal}")
                    selected.append(
                        {
                            "id": graph_id,
                            "source": source,
                            "target": target,
                            "mainstat": kind,
                        }
                    )

                edge_ordinal += 1

            if tag in {"node", "edge"}:
                elem.clear()

    return selected, counts


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path, help="GraphML file or export directory")
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--max-nodes", type=int, default=200)
    parser.add_argument("--max-edges", type=int, default=1000)
    parser.add_argument("--node-labels", help="comma-separated node labels")
    parser.add_argument("--edge-labels", help="comma-separated edge labels")
    args = parser.parse_args()

    if args.max_nodes < 1:
        parser.error("--max-nodes must be >= 1")
    if args.max_edges < 0:
        parser.error("--max-edges must be >= 0")

    graph_files = discover_graphml(args.input)
    node_labels = parse_filter(args.node_labels)
    edge_labels = parse_filter(args.edge_labels)

    nodes, selected_ids, node_counts = collect_nodes(
        graph_files, args.max_nodes, node_labels
    )
    edges, edge_counts = collect_edges(
        graph_files, selected_ids, args.max_edges, edge_labels
    )

    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_json(args.out_dir / "nodes.json", nodes)
    write_json(args.out_dir / "edges.json", edges)
    write_json(
        args.out_dir / "node-types.json",
        [{"type": key, "count": count} for key, count in node_counts.most_common()],
    )
    write_json(
        args.out_dir / "edge-types.json",
        [{"type": key, "count": count} for key, count in edge_counts.most_common()],
    )
    write_json(
        args.out_dir / "manifest.json",
        {
            "graphml_files": [str(path) for path in graph_files],
            "total_nodes": sum(node_counts.values()),
            "total_edges": sum(edge_counts.values()),
            "sample_nodes": len(nodes),
            "sample_edges": len(edges),
            "max_nodes": args.max_nodes,
            "max_edges": args.max_edges,
            "node_labels": sorted(node_labels) if node_labels else [],
            "edge_labels": sorted(edge_labels) if edge_labels else [],
        },
    )

    print(
        f"converted {sum(node_counts.values())} nodes / {sum(edge_counts.values())} edges; "
        f"Grafana sample has {len(nodes)} nodes / {len(edges)} edges",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
