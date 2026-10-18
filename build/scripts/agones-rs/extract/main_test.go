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

func TestUnknownSemanticCallsRemainUnsupported(t *testing.T) {
	for _, name := range []string{"github.com/stretchr/testify/assert.Greater", "os.Setenv", "unknown.example/helper.Compute", "testing.Helper"} {
		if supportedObservation(name) {
			t.Fatalf("unsupported semantic construct accepted: %s", name)
		}
	}
}
