#!/usr/bin/env python3

import tempfile
from pathlib import Path
import unittest

from graphml_to_grafana import (
    collect_edges,
    collect_nodes,
    discover_graphml,
)


GRAPHML = """<?xml version="1.0" encoding="UTF-8"?>
<graphml xmlns="http://graphml.graphdrawing.org/xmlns">
  <key id="labelV" for="node" attr.name="labelV" attr.type="string"/>
  <key id="labelE" for="edge" attr.name="labelE" attr.type="string"/>
  <key id="name" for="node" attr.name="NAME" attr.type="string"/>
  <key id="line" for="node" attr.name="LINE_NUMBER" attr.type="int"/>
  <graph id="G" edgedefault="directed">
    <node id="1">
      <data key="labelV">METHOD</data>
      <data key="name">Allocate</data>
      <data key="line">42</data>
    </node>
    <node id="2">
      <data key="labelV">CALL</data>
      <data key="name">Create</data>
    </node>
    <edge id="e1" source="1" target="2">
      <data key="labelE">AST</data>
    </edge>
  </graph>
</graphml>
"""


class GraphmlConversionTest(unittest.TestCase):
    def test_tinkerpop_labels_and_edges_are_preserved(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "cpg.graphml"
            path.write_text(GRAPHML, encoding="utf-8")
            graph_files = discover_graphml(path)

            nodes, ids, node_counts = collect_nodes(graph_files, 200, None)
            edges, edge_counts = collect_edges(graph_files, ids, 1000, None)

            self.assertEqual(node_counts["METHOD"], 1)
            self.assertEqual(node_counts["CALL"], 1)
            self.assertEqual(edge_counts["AST"], 1)
            self.assertEqual(nodes[0]["title"], "Allocate")
            self.assertEqual(nodes[0]["subtitle"], "METHOD")
            self.assertEqual(nodes[0]["mainstat"], "42")
            self.assertEqual(edges[0]["mainstat"], "AST")
            self.assertEqual(edges[0]["source"], nodes[0]["id"])
            self.assertEqual(edges[0]["target"], nodes[1]["id"])


if __name__ == "__main__":
    unittest.main()
