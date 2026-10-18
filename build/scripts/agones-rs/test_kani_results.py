"""Negative controls for result completeness and non-vacuous claim acceptance."""
import unittest

from kani_results import parse_complete_log, require_claims_and_covers

COMPLETE = "Complete - 1 successfully verified harnesses, 0 failures, 1 total."


def row(number=1, status="SUCCESS"):
    return f'\nCheck {number}: claim.assertion.1\n - Status: {status}\n - Description: "claim"\n - Location: source.rs:1\n'


class ResultControls(unittest.TestCase):
    def test_backend_global_pointer_location_is_optional_but_claim_location_is_not(self):
        pointer = '\nCheck 1: pointer_dereference.1\n - Status: SUCCESS\n - Description: "pointer NULL"\n'
        self.assertIsNone(parse_complete_log(pointer+COMPLETE)[0]["location"])
        for altered in [pointer.replace("pointer_dereference.1", "claim.assertion.1"),
                        pointer.replace("SUCCESS", "FAILURE"),
                        pointer.replace(' - Description: "pointer NULL"\n', '')]:
            with self.assertRaises(AssertionError):
                parse_complete_log(altered+COMPLETE)

    def test_empty_missing_duplicate_truncated_and_failing_results_reject(self):
        self.assertEqual(len(parse_complete_log(row()+COMPLETE)), 1)
        for log in [COMPLETE, row(), row(2)+COMPLETE, row()+row()+COMPLETE,
                    row(status="FAILURE")+COMPLETE, '\nCheck 1: x\n - Status: SUCCESS\n'+COMPLETE]:
            with self.assertRaises(AssertionError):
                parse_complete_log(log)

    def test_unreachable_claims_and_unsatisfied_missing_or_extra_covers_reject(self):
        claim = {"status":"SUCCESS"}
        cover = {"symbol":"branch.cover.1", "status":"SATISFIED", "description":"reachable"}
        self.assertEqual(require_claims_and_covers([cover],[claim],["reachable"]),[cover])
        for rows, claims, required in [([cover],[{"status":"UNREACHABLE"}],["reachable"]),
                                       ([cover],[],["reachable"]), ([],[claim],["reachable"]),
                                       ([{**cover,"status":"UNSATISFIABLE"}],[claim],["reachable"]),
                                       ([cover,cover],[claim],["reachable"])]:
            with self.assertRaises(AssertionError):
                require_claims_and_covers(rows, claims, required)


if __name__ == "__main__":
    unittest.main()
