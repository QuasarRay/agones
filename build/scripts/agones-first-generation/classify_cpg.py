#!/usr/bin/env python3
"""Inventory every pinned CPG file; require regeneration evidence for derived code."""
import argparse
from collections import Counter
import hashlib
import json
import os
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[3]
VENDOR = ROOT / "third_party/first-generation/cpg"
VENDOR_RECEIPT = ROOT / "verification/agones/first-generation-vendor-cpg.json"
FLATGRAPH_RECEIPT = ROOT / "verification/agones/first-generation-vendor-flatgraph.json"
OUTPUT = ROOT / "verification/agones/first-generation-cpg-classification.json"


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def content(path):
    return os.readlink(path).encode() if path.is_symlink() else path.read_bytes()


def evidence(path, pattern, explanation):
    data = (VENDOR / path).read_bytes()
    text = data.decode()
    match = re.search(pattern, text, re.MULTILINE)
    assert match, f"source build relationship changed: {path}: {pattern}"
    return {"path": path, "sha256": sha256(data), "line": text[:match.start()].count("\n") + 1,
            "text": match.group(), "meaning": explanation}


def classify(regeneration_path=None):
    vendor = json.loads(VENDOR_RECEIPT.read_text())
    generator = json.loads(FLATGRAPH_RECEIPT.read_text())
    assert vendor["status"] == generator["status"] == "VENDORED"
    expected = set(vendor["upstream_files"])
    overlays = {row["path"] for row in vendor["guidance_overlays"]}
    actual = {str(path.relative_to(VENDOR)) for path in VENDOR.rglob("*")
              if path.is_file() or path.is_symlink()}
    assert actual == expected | overlays, "unaccounted added/deleted vendor files"
    for path, record in vendor["upstream_files"].items():
        assert sha256(content(VENDOR / path)) == record["sha256"], f"changed upstream file: {path}"
    for overlay in vendor["guidance_overlays"]:
        assert sha256(content(VENDOR / overlay["path"])) == overlay["sha256"]

    projects = (VENDOR / "project/Projects.scala").read_text()
    modules = re.findall(r'lazy val (\w+)\s*=\s*project\.in\(file\("([^"]+)"\)\)', projects)
    assert modules, "cannot discover production modules from the pinned build"
    module_paths = {path for _, path in modules}
    build_facts = [
        evidence("project/Projects.scala", r'lazy val schema\s*=.*', "Schema is a first-class production build module."),
        evidence("domainClasses/build.sbt", r'generatedSrcDir := .* / "generated"', "Checked-in generated source directory."),
        evidence("domainClasses/build.sbt", r'Compile/unmanagedSourceDirectories \+= generatedSrcDir.value', "Generated files participate in the production build."),
        evidence("domainClasses/build.sbt", r'Compile/compile := .*generateDomainClasses.*', "Domain compilation invokes schema generation."),
        evidence("schema/build.sbt", r'generateDomainClasses/classWithSchema := .*', "The generator evaluates CpgSchema.instance."),
        evidence("schema/build.sbt", r'generateDomainClasses/fieldName\s*:= .*', "The complete instance is the generator input."),
        evidence("schema/build.sbt", r'generateDomainClasses/outputDir\s*:= .*', "The schema task owns the domain output directory."),
        evidence("schema/build.sbt", r'generateDomainClasses/scala3\s*:= true', "Generated domain target is Scala 3."),
        evidence("schema/build.sbt", r'val generateProtobuf = .*', "Protobuf is generated from handwritten schema behavior."),
        evidence("proto-bindings/build.sbt", r'Compile / sourceGenerators \+= .*', "The IntelliJ marker is a managed build product, not handwritten behavior."),
    ]
    generated_prefix = "domainClasses/src/main/generated/"
    candidates = {p for p in expected if p.startswith(generated_prefix)}
    regeneration = None
    if regeneration_path:
        regeneration_file = ROOT / regeneration_path
        regeneration = json.loads(regeneration_file.read_text())
        assert regeneration["status"] == "COMPARED"
        assert regeneration["source_commit"] == vendor["commit"]
        assert regeneration["vendor_manifest_sha256"] == sha256(VENDOR_RECEIPT.read_bytes())
        assert regeneration["generator_manifest_sha256"] == sha256(FLATGRAPH_RECEIPT.read_bytes())
        assert set(regeneration["files"]) == candidates
        for path, row in regeneration["files"].items():
            assert row["committed_sha256"] == vendor["upstream_files"][path]["sha256"]
            assert sha256(Path(row["regenerated_path"]).read_bytes()) == row["regenerated_sha256"]

    rows = []
    for path, identity in sorted(vendor["upstream_files"].items()):
        suffix = Path(path).suffix
        code = suffix in (".scala", ".java", ".sc", ".sbt", ".bzl", ".sh")
        root = path.split("/", 1)[0]
        required = False
        if path.startswith("samples/"):
            role = "EXAMPLE_REFERENCE"
            reason = "Standalone sample; not a module in project/Projects.scala."
        elif "/src/test/" in path:
            role = "TEST_REFERENCE"
            reason = "Test source set; retained independently of production coverage."
        elif "/src/main/" in path and root in module_paths:
            role = "PRODUCTION_SOURCE" if code else "PRODUCTION_RESOURCE"
            required = code
            reason = "Main source/resource set of a module discovered in project/Projects.scala."
        elif path.startswith("project/") or suffix in (".sbt", ".bzl") or Path(path).name in ("BUILD", "BUILD.bazel"):
            role = "BUILD_SCHEMA_TOOLING"
            required = code
            reason = "Build definition/helper; schema and domain generation behavior stays in port accounting."
        elif identity["mode"] == "120000" and "/target/universal/stage/bin/" in content(VENDOR / path).decode():
            role = "PRODUCTION_TOOL_LAUNCHER"
            required = True
            reason = "Tracked symlink launches a staged production tool; map to its Rust executable."
        elif path.startswith(".github/") or suffix == ".sh":
            role = "REPOSITORY_MAINTENANCE"
            reason = "Repository automation, retained with source and separate from core graph behavior."
        elif suffix in (".scala", ".java", ".sc"):
            role = "UNRESOLVED_CODE_ROLE"
            required = True
            reason = "Code outside known build roots must not silently disappear."
        else:
            role = "DOCUMENTATION_OR_BUILD_METADATA"
            reason = "Non-code tracked input; retained with pinned identity."

        disposition = "HANDWRITTEN"
        supporting = [{"kind": "BUILD_SOURCE_SET", "detail": reason}]
        comparison = {"status": "NOT_A_GENERATOR_OUTPUT"}
        if path in candidates:
            supporting.append({"kind": "DECLARED_GENERATOR_OUTPUT", "build_fact_indices": list(range(1, 8)),
                               "generator_revision": generator["commit"]})
            if regeneration is None:
                disposition = "UNRESOLVED"
                comparison = {"status": "CLEAN_REGENERATION_PENDING", "candidate": "GENERATED"}
            else:
                compared = regeneration["files"][path]
                identical = compared["committed_sha256"] == compared["regenerated_sha256"]
                comparison = {"status": "IDENTICAL" if identical else "DIFFERS", **compared}
                disposition = "GENERATED" if identical else "UNRESOLVED"
                if not identical:
                    supporting.append({"kind": "POSSIBLE_MIXED_OR_STALE_OUTPUT", "detail": "Resolve changed symbols/spans; do not discard differences as generated."})
        elif code and identity["mode"] != "120000":
            leading = content(VENDOR / path).decode().splitlines()[:12]
            if any(re.match(r"\s*(?://|/\*|\*)\s*(?:.*automatically generated|generated by|do not edit)", line, re.I) for line in leading):
                disposition = "UNRESOLVED"
                supporting.append({"kind": "UNRECONCILED_GENERATED_HEADER", "detail": "No matching build generator relationship has been established."})
        elif suffix.lower() in (".png", ".jpg", ".jpeg", ".svg"):
            disposition = "UNRESOLVED"
            supporting.append({"kind": "NONPRODUCTION_ASSET_ORIGIN_UNKNOWN", "detail": "Asset provenance does not remove a production behavior obligation."})
        row = {"path": path, **identity, "role": role, "classification": disposition,
               "required_handwritten_port_accounting": required and disposition != "GENERATED",
               "required_provenance_resolution": required and disposition == "UNRESOLVED",
               "evidence": supporting, "regeneration_comparison": comparison,
               "destination": "SCHEMA_DERIVED_RUST_DOMAIN" if disposition == "GENERATED" else
                              "RUST_PORT_MAPPING_PENDING" if required else "RETAINED_REFERENCE_OR_BUILD_INPUT"}
        rows.append(row)
    unresolved = [row["path"] for row in rows if row["required_provenance_resolution"]]
    result = {"schema_version": 1, "source_commit": vendor["commit"], "source_tree": vendor["tree"],
              "vendor_manifest_sha256": sha256(VENDOR_RECEIPT.read_bytes()),
              "generator_manifest_sha256": sha256(FLATGRAPH_RECEIPT.read_bytes()),
              "classifier_sha256": sha256(Path(__file__).read_bytes()),
              "discovered_modules": dict(modules), "build_facts": build_facts,
              "regeneration_receipt": str(regeneration_path) if regeneration_path else None,
              "status": "CLASSIFIED_REQUIRED_PROVENANCE" if not unresolved else "PROVISIONAL_REQUIRED_PROVENANCE_UNRESOLVED",
              "files": rows, "counts": {"files": len(rows), "classifications": dict(sorted(Counter(row["classification"] for row in rows).items())),
                                       "roles": dict(sorted(Counter(row["role"] for row in rows).items()))},
              "required_unresolved_paths": unresolved,
              "untracked_guidance_overlays": vendor["guidance_overlays"],
              "virtual_generated_products": ["schema/target/protos/cpg.proto", "proto-bindings managed protobuf bindings", "proto-bindings managed IntelliJ marker"],
              "port_completion": "UNIMPLEMENTED; source classification is not Rust behavior coverage"}
    encoded = (json.dumps(result, indent=2, sort_keys=True) + "\n").encode()
    temporary = OUTPUT.with_suffix(".tmp")
    temporary.write_bytes(encoded)
    temporary.replace(OUTPUT)
    print(json.dumps({"status": result["status"], "counts": result["counts"], "required_unresolved": len(unresolved),
                      "sha256": sha256(encoded)}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--regeneration-receipt")
    classify(parser.parse_args().regeneration_receipt)
