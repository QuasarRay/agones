"""Controls for source provenance: added files and changed generated bodies stay open."""
from contextlib import redirect_stdout
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch

SCRIPT = Path(__file__).with_name("classify_cpg.py")
SPEC = importlib.util.spec_from_file_location("first_generation_classification", SCRIPT)
CLASSIFIER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CLASSIFIER)


class ClassificationControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.vendor = self.root / "cpg"
        shutil.copytree(CLASSIFIER.VENDOR, self.vendor, symlinks=True)
        self.vendor_receipt = self.root / "vendor.json"
        self.generator_receipt = self.root / "generator.json"
        shutil.copyfile(CLASSIFIER.VENDOR_RECEIPT, self.vendor_receipt)
        shutil.copyfile(CLASSIFIER.FLATGRAPH_RECEIPT, self.generator_receipt)
        self.output = self.root / "classification.json"
        self.replaced = patch.multiple(CLASSIFIER, ROOT=self.root, VENDOR=self.vendor,
                                      VENDOR_RECEIPT=self.vendor_receipt,
                                      FLATGRAPH_RECEIPT=self.generator_receipt, OUTPUT=self.output)
        self.replaced.start()
        self.addCleanup(self.replaced.stop)

    def classify(self, receipt=None):
        with redirect_stdout(io.StringIO()):
            CLASSIFIER.classify(receipt)
        return json.loads(self.output.read_text())

    def synthetic_comparison(self, changed_path=None):
        """A controlled fixture receipt; this never supplies production regeneration credit."""
        upstream = json.loads(self.vendor_receipt.read_text())
        files = {}
        for path, original in upstream["upstream_files"].items():
            if not path.startswith("domainClasses/src/main/generated/"):
                continue
            target = self.root / "synthetic-generator" / path
            target.parent.mkdir(parents=True, exist_ok=True)
            data = (self.vendor / path).read_bytes()
            if path == changed_path:
                data += b"\n// Synthetic handwritten extension must remain unresolved.\n"
            target.write_bytes(data)
            files[path] = {"committed_sha256": original["sha256"],
                           "regenerated_sha256": hashlib.sha256(data).hexdigest(),
                           "regenerated_path": str(target)}
        record = {"status": "COMPARED", "source_commit": upstream["commit"], "synthetic_test_only": True,
                  "vendor_manifest_sha256": hashlib.sha256(self.vendor_receipt.read_bytes()).hexdigest(),
                  "generator_manifest_sha256": hashlib.sha256(self.generator_receipt.read_bytes()).hexdigest(),
                  "files": files}
        receipt = self.root / "synthetic-regeneration.json"
        receipt.write_text(json.dumps(record))
        return receipt

    def test_directory_name_does_not_grant_generated_credit(self):
        result = self.classify()
        self.assertEqual(result["status"], "PROVISIONAL_REQUIRED_PROVENANCE_UNRESOLVED")
        self.assertEqual(len(result["required_unresolved_paths"]), 250)
        self.assertEqual(result["counts"]["classifications"].get("GENERATED", 0), 0)

    def test_added_source_cannot_escape_the_pinned_inventory(self):
        (self.vendor / "schema/src/main/scala/Unrecorded.scala").write_text("object Unrecorded {}\n")
        with self.assertRaisesRegex(AssertionError, "unaccounted added/deleted"):
            self.classify()

    def test_changed_generated_body_requires_symbol_resolution(self):
        changed = "domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Cpg.scala"
        result = self.classify(self.synthetic_comparison(changed))
        self.assertEqual(result["required_unresolved_paths"], [changed])
        row = next(row for row in result["files"] if row["path"] == changed)
        self.assertEqual(row["regeneration_comparison"]["status"], "DIFFERS")
        self.assertEqual(row["classification"], "UNRESOLVED")
        self.assertTrue(row["required_handwritten_port_accounting"])

    def test_omitted_comparison_identity_rejects_equal_looking_totals(self):
        receipt = self.synthetic_comparison()
        record = json.loads(receipt.read_text())
        path, value = record["files"].popitem()
        record["files"][path + ".unrelated"] = value
        receipt.write_text(json.dumps(record))
        with self.assertRaises(AssertionError):
            self.classify(receipt)

    def test_changed_handwritten_source_is_stale(self):
        path = self.vendor / "schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Hidden.scala"
        path.write_bytes(path.read_bytes() + b"\n// Synthetic stale input\n")
        with self.assertRaisesRegex(AssertionError, "changed upstream file"):
            self.classify()


if __name__ == "__main__":
    unittest.main()
