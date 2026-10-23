#!/usr/bin/env python3
"""Retain the exact static protocol literal and pinned category iteration evidence."""
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[3]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    source = ROOT / "third_party/first-generation/flatgraph/domain-classes-generator/src/main/scala/flatgraph/codegen/ProtoGen.scala"
    metadata_path = ROOT / "verification/agones/first-generation-protobuf-template.json"
    metadata = json.loads(metadata_path.read_text())
    assert digest(source) == metadata["source_sha256"]
    target = ROOT / metadata["template_file"]
    assert digest(target) == metadata["template_sha256"], "preserve edited template before refreshing"
    text = source.read_text()
    start = text.index('s"""syntax = "proto3";') + 4
    end = text.index('""".stripMargin', start)
    # split/join preserves the source literal's final empty margin line without adding a newline.
    lines = []
    for line in text[start:end].split("\n"):
        leading = line.lstrip(" \t\r")
        lines.append(leading[1:] if leading.startswith("|") else line)
    target.write_text("\n".join(lines))
    reference = ROOT / metadata["reference_proto_retained"]
    assert digest(reference) == metadata["reference_proto_sha256"]
    schema = json.loads((ROOT / "verification/agones/first-generation-complete-schema-bootstrap.json").read_text())["protobufSchema"]
    names = {schema["protoOptions"]["uncommonProtoEnumNameMappings"].get(category, category): category
             for category, constants in schema["constantsByCategory"].items()
             if any(constant["protoId"] is not None for constant in constants)}
    order = [names[name] for name in re.findall(r"^enum ([A-Za-z_][A-Za-z0-9_]*) \{", reference.read_text(), re.MULTILINE) if name in names]
    assert set(order) == set(names.values())
    (ROOT / "cpg-rs/templates/pinned-category-order.json").write_text(json.dumps(order, indent=2) + "\n")
    metadata.update(template_sha256=digest(target), extraction_script=str(Path(__file__).relative_to(ROOT)),
                    extraction_script_sha256=digest(Path(__file__)), observed_pinned_category_order=order)
    metadata_path.write_text(json.dumps(metadata, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"template_bytes": target.stat().st_size, "template_sha256": metadata["template_sha256"]}))


if __name__ == "__main__":
    main()
