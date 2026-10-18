package main

import "testing"

func TestDefaultingGuardSelectionRejectsUnmodeledConditions(t *testing.T) {
	for _, guard := range []string{"test.expected.alphaPlayerCapacity != nil", "len(test.expected.evictionSafeSpec) > 0", "len(test.expected.evictionSafeStatus) > 0", "test.expected.counterSpec != nil", "test.expected.listSpec != nil"} {
		if !supportedDefaultingGuard(guard) {
			t.Fatalf("source guard rejected: %s", guard)
		}
	}
	for _, guard := range []string{"test.expected.alphaPlayerCapacity == nil", "len(test.expected.evictionSafeSpec) >= 0", "len(effect()) > 0", "test.expected.otherMap != nil", "true"} {
		if supportedDefaultingGuard(guard) {
			t.Fatalf("unmodeled guard accepted: %s", guard)
		}
	}
}
