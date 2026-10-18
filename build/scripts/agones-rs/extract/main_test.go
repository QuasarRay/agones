package main

import (
	"os"
	"path/filepath"
	"testing"
)

func TestGateRejectsMissingUnfinishedAndStaleProduction(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "production.rs"), []byte("original"), 0600); err != nil {
		t.Fatal(err)
	}
	valid := Gate{Status: "IMPLEMENTED", ProductionFiles: map[string]string{"production.rs": hash([]byte("original"))}}
	if err := validateGate(root, valid); err != nil {
		t.Fatal(err)
	}
	for _, invalid := range []Gate{{Status: "IMPLEMENTING", ProductionFiles: valid.ProductionFiles}, {Status: "IMPLEMENTED"}, {Status: "IMPLEMENTED", ProductionFiles: map[string]string{"production.rs": hash([]byte("mutated"))}}} {
		if validateGate(root, invalid) == nil {
			t.Fatal("missing/unfinished/stale gate was accepted")
		}
	}
}

func TestRepeatedNamesPreserveBothSourceIdentities(t *testing.T) {
	seen := map[string]bool{}
	first := sourceCaseIdentity("package::source::test::same name", 40, seen)
	second := sourceCaseIdentity("package::source::test::same name", 50, seen)
	if first == second || second != "package::source::test::same name::source-line-50" {
		t.Fatal(first, second)
	}
}

func TestInputExtractionRequiresItsOwnFreshFeatureGate(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "input.rs"), []byte("production input"), 0600); err != nil {
		t.Fatal(err)
	}
	valid := Gate{Status: "IMPLEMENTED", FeatureID: "feature-gates-environment-cli-binding", ProductionFiles: map[string]string{"input.rs": hash([]byte("production input"))}}
	if err := validateInputGate(root, valid); err != nil {
		t.Fatal(err)
	}
	for _, feature := range []string{"", "feature-gate-query-policy-and-controller-configuration"} {
		invalid := valid
		invalid.FeatureID = feature
		if validateInputGate(root, invalid) == nil {
			t.Fatal("query-only gate admitted environment extraction")
		}
	}
	if err := os.WriteFile(filepath.Join(root, "input.rs"), []byte("mutated"), 0600); err != nil {
		t.Fatal(err)
	}
	if validateInputGate(root, valid) == nil {
		t.Fatal("stale input feature gate admitted environment extraction")
	}
}

func TestUnknownSemanticCallsRemainUnsupported(t *testing.T) {
	for _, name := range []string{"github.com/stretchr/testify/assert.Greater", "os.Setenv", "unknown.example/helper.Compute", "testing.Helper"} {
		if supportedObservation(name) {
			t.Fatalf("unsupported semantic construct accepted: %s", name)
		}
	}
}
