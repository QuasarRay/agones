#!/usr/bin/env python3
"""Create a separate, provenance-bound instrumentation overlay from pinned Flatgraph."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PINNED = ROOT / "third_party/first-generation/flatgraph/domain-classes-generator/src/main/scala/flatgraph/schema"
TARGET = ROOT / ".git/agones-first-generation-sources/cpg/schema/src/main/scala"
OWNED = Path(__file__).parent
RECEIPT = ROOT / "verification/agones/first-generation-schema-trace-patch.json"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def replace_once(text, before, after):
    assert text.count(before) == 1, "pinned schema mutation site changed: " + before[:90]
    return text.replace(before, after)


def main():
    assert not RECEIPT.exists(), "recover existing instrumentation overlay"
    builder = (PINNED / "SchemaBuilder.scala").read_text()
    original_builder = builder
    builder = replace_once(builder,
        'class SchemaBuilder(domainShortName: String, basePackage: String, additionalTraversalsPackages: Seq[String] = Seq.empty) {',
        'class SchemaBuilder(domainShortName: String, basePackage: String, additionalTraversalsPackages: Seq[String] = Seq.empty) {\n'
        '  SchemaConstructionTrace.builder(this, domainShortName, basePackage, additionalTraversalsPackages)')
    builder = replace_once(builder, "lazy val anyNode: AnyNodeType = new AnyNodeType",
        "lazy val anyNode: AnyNodeType = { val node = new AnyNodeType; SchemaConstructionTrace.node(node); node }")
    builder = replace_once(builder, "    properties.append(property)\n", "    properties.append(property)\n    SchemaConstructionTrace.property(this, property)\n")
    builder = replace_once(builder, "    val previousEntries = constantsByCategory.getOrElse(category, Seq.empty)",
        "    SchemaConstructionTrace.constants(this, category, constants)\n    val previousEntries = constantsByCategory.getOrElse(category, Seq.empty)")
    builder = replace_once(builder, "    this.protoOptions = Option(value)",
        "    SchemaConstructionTrace.protoOptions(this, value)\n    this.protoOptions = Option(value)")
    builder = replace_once(builder, "    noWarnList.add((nodeType, property))",
        "    SchemaConstructionTrace.noWarn(this, nodeType, property)\n    noWarnList.add((nodeType, property))")
    builder = replace_once(builder, "    verifyProtoIdsUnique(schema)\n    schema",
        "    verifyProtoIdsUnique(schema)\n    SchemaConstructionTrace.built(this, schema)\n    schema")
    builder = replace_once(builder, "    buffer.append(a)\n    a", "    buffer.append(a)\n    SchemaConstructionTrace.element(a.asInstanceOf[AnyRef])\n    a")
    schema = (PINNED / "Schema.scala").read_text()
    original_schema = schema
    replacements = [
        ("def starterName(name: String): this.type = { this._starterName = Option(name); this }",
         "def starterName(name: String): this.type = { SchemaConstructionTrace.starter(this, name); this._starterName = Option(name); this }"),
        ("    _properties.add(additional)", "    SchemaConstructionTrace.addProperty(this, additional)\n    _properties.add(additional)"),
        ("    additional.foreach(_extendz.add)", "    SchemaConstructionTrace.extend(this, additional)\n    additional.foreach(_extendz.add)"),
        ("    _outEdges.add(AdjacentNode(edge, inNode, cardinalityOut, stringToOption(stepNameOut), stringToOption(stepNameOutDoc)))",
         "    SchemaConstructionTrace.connect(this, edge, inNode, cardinalityOut, cardinalityIn, stepNameOut, stepNameOutDoc, stepNameIn, stepNameInDoc)\n"
         "    _outEdges.add(AdjacentNode(edge, inNode, cardinalityOut, stringToOption(stepNameOut), stringToOption(stepNameOutDoc)))"),
        ("    _inEdges.add(AdjacentNode(edge, outNode, cardinalityIn, stringToOption(stepNameIn), stringToOption(stepNameInDoc)))",
         "    SchemaConstructionTrace.connect(outNode, edge, this, cardinalityOut, cardinalityIn, stepNameOut, stepNameOutDoc, stepNameIn, stepNameInDoc)\n"
         "    _inEdges.add(AdjacentNode(edge, outNode, cardinalityIn, stringToOption(stepNameIn), stringToOption(stepNameInDoc)))"),
        ("    _markerTraits.add(MarkerTrait(name))", "    SchemaConstructionTrace.marker(this, name)\n    _markerTraits.add(MarkerTrait(name))"),
        ("    this._primaryKey = Option(p)", "    SchemaConstructionTrace.primaryKey(this, p)\n    this._primaryKey = Option(p)"),
        ("    _containedNodes.add(ContainedNode(node, localName, cardinality, stringToOption(comment)))",
         "    SchemaConstructionTrace.contained(this, node, localName, cardinality, comment)\n    _containedNodes.add(ContainedNode(node, localName, cardinality, stringToOption(comment)))"),
        ("    _property = Option(property)", "    SchemaConstructionTrace.edgeProperty(this, property)\n    _property = Option(property)"),
        ("    _defaultAccessorName = Option(value)", "    SchemaConstructionTrace.edgeAccessor(this, value)\n    _defaultAccessorName = Option(value)"),
        ("    _cardinality = Cardinality.One(Property.Default(default))",
         "    SchemaConstructionTrace.mandatory(this, default)\n    _cardinality = Cardinality.One(Property.Default(default))"),
        ("    _cardinality = Cardinality.List", "    SchemaConstructionTrace.asList(this)\n    _cardinality = Cardinality.List"),
        ("    _protoId = Some(id)", "    SchemaConstructionTrace.protoId(this.asInstanceOf[AnyRef], id)\n    _protoId = Some(id)"),
    ]
    for before, after in replacements:
        schema = replace_once(schema, before, after)
    outputs = {
        "flatgraph/schema/Schema.scala": (schema.encode(), original_schema.encode()),
        "flatgraph/schema/SchemaBuilder.scala": (builder.encode(), original_builder.encode()),
        "flatgraph/schema/SchemaConstructionTrace.scala": ((OWNED / "SchemaConstructionTrace.scala").read_bytes(), None),
        "CaptureSchemaConstruction.scala": ((OWNED / "CaptureSchemaConstruction.scala").read_bytes(), None),
    }
    patches = []
    for relative, (data, original) in outputs.items():
        target = TARGET / relative
        assert not target.exists(), "do not overwrite an existing bootstrap source: " + str(target)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
        patches.append({"added_cpg_build_source": str(target.relative_to(ROOT)), "result_sha256": digest(data),
                        "flatgraph_pristine_sha256": digest(original) if original else None,
                        "license": "Apache-2.0; original Flatgraph license retained in its vendor tree."})
    # Additions are exact complete source overlays; all pristine upstream trees remain unchanged.
    patch = OWNED / "schema-construction-trace.patch"
    text = ""
    for relative, (data, _) in outputs.items():
        lines = data.decode().splitlines()
        text += "--- /dev/null\n+++ b/schema/src/main/scala/" + relative + "\n@@ -0,0 +1," + str(len(lines)) + " @@\n"
        text += "".join("+" + line + "\n" for line in lines)
    patch.write_text(text)
    receipt = {"schema_version": 1, "status": "PREPARED", "source_patches": patches,
               "patch_file": str(patch.relative_to(ROOT)), "patch_sha256": digest(patch.read_bytes()),
               "flatgraph_revision": "61aa9de52231f95f9730b3a5aa7a3fb0d23fd128",
               "flatgraph_vendor_manifest_sha256": digest((ROOT / "verification/agones/first-generation-vendor-flatgraph.json").read_bytes()),
               "reason": "Observe all fixed CPG schema constructor mutations in order with original source frames, then compile them to native Rust calls.",
               "pristine_upstream_modified": False, "native_runtime_scala_dependency": False}
    RECEIPT.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": "PREPARED", "source_files": len(outputs), "patch_sha256": receipt["patch_sha256"]}))


if __name__ == "__main__":
    main()
