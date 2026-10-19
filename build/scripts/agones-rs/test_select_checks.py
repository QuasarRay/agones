"""Controls for successful, available, unchanged-input model proof reuse."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

from select_checks import POLICY, select


class ReuseControls(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory(prefix="agones-ci-reuse-")
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.git("init", "-q", "-b", "cache-fixture")
        self.policy = {"rust": ["crates/**"],
                       "model": ["crates/agones-core/**", ".github/workflows/rust.yml", POLICY],
                       "metadata_only": ["verification/**", "AGONES_RS_STATE.json"]}
        for name in ["crates/agones-core/src/model.rs", ".github/workflows/rust.yml",
                     "verification/agones/gameserver-defaulting.ir.json",
                     "verification/agones/gameserver-defaulting-codegen.json"]:
            self.write(name, "initial")
        self.write(POLICY, self.policy)
        self.commit()
        self.base = self.git("rev-parse", "HEAD")
        log = "verification/agones/runs/control.log"
        self.write(log, "Synthetic availability control; not a proof artifact.")
        self.retention = "verification/agones/ci/control-retention.json"
        self.write(self.retention, {"artifact_bindings": {log: self.digest(log)}})
        self.receipt_path = "verification/agones/ci/control.json"
        self.receipt = {"status": "PASSED", "head": self.base,
                        "tree": self.git("rev-parse", "HEAD^{tree}"),
                        "selected": {"model": True},
                        "jobs": [{"name": "feature-bool-model-check", "status": "completed", "conclusion": "success"}],
                        "proof_summaries": [{"harness_count": 1, "property_count": 3,
                                             "status_counts": {"SUCCESS": 2, "SATISFIED": 1}}],
                        "artifact_bindings": {log: self.digest(log)},
                        "proof_retention": {"control": self.retention}}
        self.save_receipt()
        self.policy["model"] += ["verification/agones/gameserver-defaulting.ir.json",
                                 "verification/agones/gameserver-defaulting-codegen.json"]
        self.write(POLICY, self.policy)
        self.commit()

    def git(self, *args):
        return subprocess.check_output(["git", "-c", "user.name=Evidence Control",
                                        "-c", "user.email=evidence@example.invalid",
                                        "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null",
                                        *args], cwd=self.root, stderr=subprocess.DEVNULL).decode().strip()

    def write(self, name, value):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(value) if isinstance(value, dict) else value)

    def digest(self, name):
        return hashlib.sha256((self.root / name).read_bytes()).hexdigest()

    def commit(self):
        self.git("add", ".")
        self.git("commit", "-qm", "isolated selection control")

    def save_receipt(self):
        self.write(self.receipt_path, self.receipt)
        self.write("verification/agones/gameserver-defaulting-evidence.json", {
            "assurance": {"current_ci": {"head": self.base, "receipt": self.receipt_path}},
            "artifact_bindings": {self.receipt_path: self.digest(self.receipt_path),
                                  self.retention: self.digest(self.retention)}})

    def result(self):
        return select(self.root, self.base, "HEAD")

    def assert_requires_model(self):
        result = self.result()
        self.assertTrue(result["selected"]["model"])
        self.assertIsNone(result["model_evidence_reuse"])

    def test_available_unchanged_inputs_and_monotonic_extension_reuse(self):
        result = self.result()
        self.assertFalse(result["selected"]["model"])
        self.assertEqual(result["model_evidence_reuse"]["head"], self.base)

    def test_changed_model_inputs_require_execution(self):
        for name in ["crates/agones-core/src/model.rs", ".github/workflows/rust.yml",
                     "verification/agones/gameserver-defaulting.ir.json",
                     "verification/agones/gameserver-defaulting-codegen.json"]:
            with self.subTest(name=name):
                self.write(name, "changed")
                self.commit()
                self.assert_requires_model()
                self.write(name, "initial")
                self.commit()
                self.assertFalse(self.result()["selected"]["model"])

    def test_failed_skipped_or_empty_model_results_reject_reuse(self):
        for conclusion in ["failure", "skipped"]:
            with self.subTest(conclusion=conclusion):
                self.receipt["jobs"][0]["conclusion"] = conclusion
                self.save_receipt()
                self.commit()
                self.assert_requires_model()
        self.receipt["jobs"][0]["conclusion"] = "success"
        self.receipt["proof_summaries"] = []
        self.save_receipt()
        self.commit()
        self.assert_requires_model()

    def test_unsatisfied_nonvacuity_rejects_reuse(self):
        self.receipt["proof_summaries"][0]["status_counts"] = {"SUCCESS": 3, "SATISFIED": 0}
        self.save_receipt()
        self.commit()
        self.assert_requires_model()

    def test_unselected_or_zero_harness_results_reject_reuse(self):
        self.receipt["selected"]["model"] = False
        self.save_receipt()
        self.commit()
        self.assert_requires_model()
        self.receipt["selected"]["model"] = True
        self.receipt["proof_summaries"][0]["harness_count"] = 0
        self.save_receipt()
        self.commit()
        self.assert_requires_model()

    def test_missing_or_corrupt_artifact_rejects_reuse(self):
        log = self.root / "verification/agones/runs/control.log"
        log.write_text("corrupt")
        self.commit()
        self.assert_requires_model()
        log.unlink()
        self.commit()
        self.assert_requires_model()

    def test_unbound_receipt_rejects_reuse(self):
        self.receipt["note"] = "unbound edit"
        self.write(self.receipt_path, self.receipt)
        self.commit()
        self.assert_requires_model()

    def test_incomplete_result_counts_reject_reuse(self):
        self.receipt["proof_summaries"][0]["property_count"] = 4
        self.save_receipt()
        self.commit()
        self.assert_requires_model()

    def test_weakened_policy_rejects_reuse(self):
        self.policy["model"].remove("crates/agones-core/**")
        self.write(POLICY, self.policy)
        self.commit()
        self.assert_requires_model()

    def test_unknown_dependency_requires_execution(self):
        self.write("unclassified-input", "new dependency")
        self.commit()
        self.assert_requires_model()

    def test_unknown_path_with_leading_space_requires_execution(self):
        self.write(" verification/agones/uncatalogued-input", "new dependency")
        self.commit()
        self.assert_requires_model()

    def test_widening_metadata_classification_rejects_reuse(self):
        self.policy["metadata_only"].append("unclassified/**")
        self.write(POLICY, self.policy)
        self.write("unclassified/model-input", "new dependency")
        self.commit()
        self.assert_requires_model()

    def test_wrong_snapshot_tree_rejects_reuse(self):
        self.receipt["tree"] = "0" * 40
        self.save_receipt()
        self.commit()
        self.assert_requires_model()

    def test_nonancestor_snapshot_rejects_reuse(self):
        self.git("checkout", "--orphan", "unrelated-fixture")
        self.commit()
        self.assert_requires_model()


if __name__ == "__main__":
    unittest.main()
