"""Synthetic acceptance controls; these fixtures are never backend proof credit."""
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
SCRIPTS = Path("build/scripts/agones-rs")
COMPLETE = "Complete - 1 successfully verified harnesses, 0 failures, 1 total."


class SourceCaseControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.fixture("gameserver-general-validation-source", 0)

    def copy(self, path):
        destination = self.root / path
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / path, destination)

    def fixture(self, family, index):
        general = family == "gameserver-general-validation-source"
        metadata_family = family == "object-metadata-source"
        stem = "object-metadata" if metadata_family else "gameserver-validation" if general else "gameserver-feature-validation"
        self.metadata_path = f"verification/agones/{stem}-codegen.json"
        self.metadata = json.loads((ROOT / self.metadata_path).read_text())
        self.family, self.index = family, index
        self.selected = self.metadata["harnesses"][index]
        ir = json.loads((ROOT / self.metadata["ir"]).read_text())
        self.gate_path = ir["implementation_gate"]
        gate = json.loads((ROOT / self.gate_path).read_text())
        paths = set(gate["production_files"]) | set(self.metadata["generator_source_files"])
        paths |= {self.metadata_path, self.metadata["output"], self.metadata["ir"],
                  self.gate_path, self.metadata["validation_error_mapping"]["source"]}
        if metadata_family:
            evidence_path = self.metadata["source_capture_evidence"]
            evidence = json.loads((ROOT / evidence_path).read_text())
            paths |= set(ir["source_files"]) | set(ir["loader_source_files"]) | {
                evidence_path, evidence["go_runtime_restored"]["manifest"],
                evidence["go_runtime_restored"]["unicode_reference"]}
        else:
            paths.add(self.metadata["multiset_mapping"]["manifest"])
        for path in paths:
            self.copy(path)
        for name in ("summarize_feature_proof.py", "kani_results.py", "protect_model.py"):
            self.copy(str(SCRIPTS / name))
        self.run_name = "synthetic-control"
        self.run_path = self.root / f"verification/agones/runs/{self.run_name}.json"
        self.log_path = self.run_path.with_suffix(".log")
        self.run_path.parent.mkdir(parents=True, exist_ok=True)
        self.record = {"status": "PASSED", "exit_code": 0,
                       "command": ["cargo", "bolero", "test", self.selected["harness"], "--engine", "kani"],
                       "source_files": {p: hashlib.sha256((self.root / p).read_bytes()).hexdigest()
                                        for p in paths},
                       "log": str(self.log_path.relative_to(self.root))}
        self.log = self.valid_log()
        self.save()

    def valid_log(self):
        rows = []
        for function in self.selected["assertion_functions"]:
            rows.append((f"source_module::{function}.assertion.1", "SUCCESS",
                         "original source assertion", self.metadata["output"] + ":1"))
        for index, description in enumerate(self.selected["required_covers"]):
            rows.append((f'{self.selected["harness"]}::witness.cover.{index}', "SATISFIED",
                         description, self.metadata["output"] + ":2"))
        return "".join(f'\nCheck {i}: {symbol}\n - Status: {status}\n'
                       f' - Description: "{description}"\n - Location: {location}\n'
                       for i, (symbol, status, description, location) in enumerate(rows, 1)) + COMPLETE

    def save(self):
        self.log_path.write_text(self.log)
        self.record["log_sha256"] = hashlib.sha256(self.log_path.read_bytes()).hexdigest()
        self.run_path.write_text(json.dumps(self.record))
        (self.root / self.metadata_path).write_text(json.dumps(self.metadata))

    def invoke(self, accepted):
        self.save()
        result = subprocess.run(
            [sys.executable, str(self.root / SCRIPTS / "summarize_feature_proof.py"),
             self.family, self.run_name, "--case-index", str(self.index),
             "--output", "synthetic-summary.json"],
            capture_output=True, text=True)
        if accepted:
            self.assertEqual(result.returncode, 0, result.stderr)
            summary = json.loads((self.root / "synthetic-summary.json").read_text())
            self.assertEqual(summary["selected_harnesses"], [self.selected["harness"]])
            return summary
        self.assertNotEqual(result.returncode, 0, result.stdout)

    def test_every_general_and_feature_case_is_accounted(self):
        origins = []
        for index in range(25):
            with self.subTest(general_case=index):
                self.fixture("gameserver-general-validation-source", index)
                summary = self.invoke(True)
                self.assertEqual(len(summary["assertions"]), 1)
                self.assertEqual(len(summary["covers"]), 2)
                origins.extend(row["origin"] for row in summary["assertions"])
        self.assertEqual(len(set(origins)), 25)
        origins = []
        for index in range(5):
            with self.subTest(feature_case=index):
                self.fixture("gameserver-validation-source", index)
                summary = self.invoke(True)
                self.assertEqual(len(summary["assertions"]), 2)
                self.assertEqual(len(summary["covers"]), 2)
                origins.extend(row["origin"] for row in summary["assertions"])
        self.assertEqual(len(set(origins)), 10)

    def test_missing_failed_unreachable_and_wrong_assertions_reject(self):
        baseline = self.log
        cases = [baseline.replace(" - Status: SUCCESS", " - Status: FAILURE", 1),
                 baseline.replace(" - Status: SUCCESS", " - Status: UNREACHABLE", 1),
                 baseline.replace(self.selected["assertion_functions"][0], "wrong_source_assertion"),
                 baseline.replace(self.metadata["output"] + ":1", "wrong_source.rs:1"),
                 baseline.replace(COMPLETE, "")]
        for log in cases:
            with self.subTest(log=log[:80]):
                self.log = log
                self.invoke(False)
        self.log = baseline
        self.selected["assertion_functions"] = []
        self.invoke(False)

    def test_wrong_source_origin_and_extra_source_assertion_reject(self):
        assertion = self.metadata["assertions"][0]
        original = assertion["origin"]
        assertion["origin"] = "unrelated::source::assertion"
        self.invoke(False)
        assertion["origin"] = original
        baseline = self.log
        extra = self.metadata["assertions"][1]["function"]
        next_check = len(re.findall(r"(?m)^Check [0-9]+:", baseline)) + 1
        self.log = baseline.replace(
            COMPLETE, f'\nCheck {next_check}: source_module::{extra}.assertion.1\n'
            ' - Status: SUCCESS\n - Description: "unexpected original assertion"\n'
            f' - Location: {self.metadata["output"]}:1\n' + COMPLETE)
        # Keep result numbering complete so the exact source-selection check
        # must reject the additional, otherwise successful original assertion.
        self.invoke(False)

    def test_cover_failure_missing_and_duplicate_metadata_reject(self):
        baseline = self.log
        self.log = baseline.replace("SATISFIED", "UNSATISFIABLE", 1)
        self.invoke(False)
        self.log = baseline.replace(self.selected["required_covers"][0], "wrong cover", 1)
        self.invoke(False)
        self.log = baseline
        self.selected["required_covers"][1] = self.selected["required_covers"][0]
        self.invoke(False)

    def test_wrong_selection_route_bounds_and_duplicate_identity_reject(self):
        original = self.record["command"]
        self.record["command"] = original[:-3] + ["source_gs_validate_case_024", "--engine", "kani"]
        self.invoke(False)
        self.record["command"] = original
        self.selected["unwind"] = 64
        self.invoke(False)
        self.selected["unwind"] = 128
        self.selected["preconditions"] = ["false"]
        self.invoke(False)
        self.selected["preconditions"] = []
        self.metadata["harnesses"][1]["case_id"] = self.selected["case_id"]
        self.invoke(False)

    def test_stale_source_gate_generator_and_full_field_mapping_reject(self):
        for path in [self.metadata["output"], self.metadata["ir"],
                     *self.metadata["generator_source_files"], self.gate_path,
                     "crates/agones-core/src/resource_quantity.rs"]:
            with self.subTest(path=path):
                target = self.root / path
                before = target.read_bytes()
                target.write_bytes(before + b"\n")
                self.invoke(False)
                target.write_bytes(before)
        self.metadata["validation_error_mapping"]["fields"].pop()
        self.invoke(False)
        self.metadata["validation_error_mapping"]["fields"].append("CoveredByDeclarative")
        self.metadata["multiset_mapping"]["manifest_sha256"] = "0" * 64
        self.invoke(False)

    def test_missing_actual_production_binding_rejects(self):
        del self.record["source_files"]["crates/agones-core/src/resource_quantity.rs"]
        self.invoke(False)


class MetadataSourceCaseControls(SourceCaseControls):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.fixture("object-metadata-source", 5)

    def test_every_general_and_feature_case_is_accounted(self):
        origins = []
        for index, count in enumerate([1, 3, 3, 3, 1, 10]):
            with self.subTest(metadata_case=index):
                self.fixture("object-metadata-source", index)
                summary = self.invoke(True)
                self.assertEqual(len(summary["assertions"]), count)
                self.assertEqual(len(summary["covers"]), 2)
                origins.extend(row["origin"] for row in summary["assertions"])
        self.assertEqual(len(origins), len(set(origins)))
        self.assertEqual(len(origins), 21)

    def test_stale_source_gate_generator_and_full_field_mapping_reject(self):
        ir = json.loads((ROOT / self.metadata["ir"]).read_text())
        for path in [self.metadata["output"], self.metadata["ir"],
                     *self.metadata["generator_source_files"], self.gate_path,
                     "crates/agones-core/src/object_metadata_validation.rs",
                     "pkg/apis/agones/v1/common_test.go",
                     next(iter(ir["loader_source_files"]))]:
            with self.subTest(path=path):
                target = self.root / path
                before = target.read_bytes()
                target.write_bytes(before + b"\n")
                self.invoke(False)
                target.write_bytes(before)
        self.metadata["validation_error_mapping"]["fields"].pop()
        self.invoke(False)
        self.metadata["validation_error_mapping"]["fields"].append("CoveredByDeclarative")
        self.metadata["source_capture_evidence_sha256"] = "0" * 64
        self.invoke(False)

    def test_missing_actual_production_binding_rejects(self):
        del self.record["source_files"]["crates/agones-core/src/object_metadata_validation.rs"]
        self.invoke(False)


class MatrixControls(unittest.TestCase):
    def test_matrix_contains_every_source_case_and_preserves_protected_aggregate(self):
        workflow = (ROOT / ".github/workflows/rust.yml").read_text()
        block = workflow.split("  feature-validation-source-model-check:\n")[1].split(
            "  feature-validation-contract-model-check:\n")[0]
        entries = [json.loads(row) for row in re.findall(r"^\s+- (\{.*\})$", block, re.M)]
        observed = [(row["family"], row["index"], row["case"]) for row in entries]
        expected = []
        for family, stem in [("feature", "gameserver-feature-validation"),
                             ("general", "gameserver-validation"),
                             ("metadata", "object-metadata")]:
            metadata = json.loads((ROOT / f"verification/agones/{stem}-codegen.json").read_text())
            for index, harness in enumerate(metadata["harnesses"]):
                case = f"{index:03}"
                self.assertTrue(harness["harness"].endswith("_case_" + case))
                self.assertEqual(harness["raw_domain"], "unit")
                self.assertEqual(harness["preconditions"], [])
                self.assertEqual(harness["unwind"], 128)
                self.assertEqual(len(set(harness["required_covers"])), 2)
                expected.append((family, index, case))
        self.assertEqual(observed, expected)
        self.assertEqual(len(observed), len(set(observed)))
        self.assertIn("fail-fast: false", block)
        self.assertIn("protect_model.py cargo bolero test", block)
        self.assertIn("if: always()", block)
        aggregate = workflow.split("  feature-bool-model-check:\n")[1]
        self.assertIn("feature-validation-source-model-check", aggregate)
        self.assertIn('test "$VALIDATION_RESULT" = success', aggregate)
        self.assertIn("--gameserver-validation --check", workflow)
        self.assertIn("--object-metadata --check", workflow)
        self.assertEqual(len(observed), 36)


class ContractControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.domain_path = "verification/agones/gameserver-feature-validation-domain.json"
        self.domain = json.loads((ROOT / self.domain_path).read_text())
        gate = json.loads((ROOT / self.domain["implementation_gate"]).read_text())
        paths = set(gate["production_files"]) | {
            self.domain_path, self.domain["implementation_gate"],
            "crates/agones-core/tests/gameserver_feature_validation_domain.rs",
            self.domain["validation_error_mapping"]["source"],
        } | {str(SCRIPTS / name) for name in (
            "summarize_feature_proof.py", "kani_results.py", "protect_model.py")}
        for path in paths:
            target = self.root / path
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / path, target)
        self.run_name = "synthetic-contract-control"
        self.run_path = self.root / f"verification/agones/runs/{self.run_name}.json"
        self.log_path = self.run_path.with_suffix(".log")
        self.run_path.parent.mkdir(parents=True, exist_ok=True)
        self.record = {
            "status": "PASSED", "exit_code": 0,
            "command": ["cargo", "bolero", "test", self.domain["harness"], "--engine", "kani"],
            "source_files": {path: hashlib.sha256((self.root / path).read_bytes()).hexdigest()
                             for path in paths},
            "log": str(self.log_path.relative_to(self.root)),
        }
        harness = self.domain["harness"]
        rows = [(f"{harness}::{{closure#0}}.assertion.{index}", "SUCCESS", "contract relation")
                for index in range(1, 4)]
        rows += [(f"{harness}::witness.cover.{index}", "SATISFIED", description)
                 for index, description in enumerate(self.domain["required_covers"])]
        self.log = "".join(
            f'\nCheck {index}: {symbol}\n - Status: {status}\n'
            f' - Description: "{description}"\n'
            ' - Location: crates/agones-core/tests/gameserver_feature_validation_domain.rs:1\n'
            for index, (symbol, status, description) in enumerate(rows, 1)) + COMPLETE

    def invoke(self, accepted):
        self.log_path.write_text(self.log)
        self.record["log_sha256"] = hashlib.sha256(self.log_path.read_bytes()).hexdigest()
        self.run_path.write_text(json.dumps(self.record))
        (self.root / self.domain_path).write_text(json.dumps(self.domain))
        result = subprocess.run(
            [sys.executable, str(self.root / SCRIPTS / "summarize_feature_proof.py"),
             "gameserver-features", self.run_name, "--output", "synthetic-summary.json"],
            capture_output=True, text=True)
        if accepted:
            self.assertEqual(result.returncode, 0, result.stderr)
            summary = json.loads((self.root / "synthetic-summary.json").read_text())
            self.assertEqual(len(summary["assertions"]), 3)
            self.assertEqual(len(summary["covers"]), 8)
        else:
            self.assertNotEqual(result.returncode, 0, result.stdout)

    def test_real_common_manifest_shape_and_six_field_mapping(self):
        self.invoke(True)
        fields = self.domain["validation_error_mapping"].pop("fields")
        self.invoke(False)
        self.domain["validation_error_mapping"]["fields"] = fields[:-1]
        self.invoke(False)

    def test_contract_assertion_and_cover_failures_reject(self):
        baseline = self.log
        self.log = baseline.replace(" - Status: SUCCESS", " - Status: UNREACHABLE", 1)
        self.invoke(False)
        self.log = baseline.replace(" - Status: SATISFIED", " - Status: UNSATISFIABLE", 1)
        self.invoke(False)

    def test_actual_provider_and_constructor_bindings_reject(self):
        del self.record["source_files"]["crates/agones-core/src/gameserver_validation.rs"]
        self.invoke(False)
        self.record["source_files"]["crates/agones-core/src/gameserver_validation.rs"] = hashlib.sha256(
            (self.root / "crates/agones-core/src/gameserver_validation.rs").read_bytes()).hexdigest()
        self.domain["validation_error_mapping"]["source_sha256"] = "0" * 64
        self.invoke(False)


if __name__ == "__main__":
    unittest.main()
