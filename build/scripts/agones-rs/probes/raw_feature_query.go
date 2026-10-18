// Run the immutable original implementation against independent raw-byte
// invariants, and project observations for comparison with production Rust.
package main

import (
	"encoding/json"
	"os"
	"strings"

	agruntime "agones.dev/agones/pkg/util/runtime"
)

type observation struct {
	ID         string `json:"id"`
	ErrorClass string `json:"error_class"`
	Example    bool   `json:"example"`
	Encoded    string `json:"encoded"`
}

func main() {
	if err := agruntime.ParseFeatures(""); err != nil {
		panic(err)
	}
	cases := []struct{ id, query, want string }{
		{"ignored-raw-duplicate", "Example=true&Example=\xff", "NONE"},
		{"invalid-raw-bool", "Example=\xff", "INVALID_BOOL"},
		{"unknown-raw-key", "\xff=true", "UNKNOWN_FEATURE"},
		{"invalid-escape-in-duplicate", "Example=true&Example=\xff%QQ", "INVALID_QUERY"},
	}
	rows := []observation{}
	for _, c := range cases {
		err := agruntime.ParseFeatures(c.query)
		class := "NONE"
		if err != nil {
			switch {
			case strings.Contains(err.Error(), "error parsing query string for feature gates"):
				class = "INVALID_QUERY"
			case strings.Contains(err.Error(), "error parsing bool value from flag"):
				class = "INVALID_BOOL"
			case strings.Contains(err.Error(), "is not a valid Feature Gate"):
				class = "UNKNOWN_FEATURE"
			default:
				panic(err)
			}
		}
		if class != c.want || !agruntime.FeatureEnabled(agruntime.FeatureExample) {
			panic("independent raw-query invariant failed: " + c.id)
		}
		rows = append(rows, observation{c.id, class, agruntime.FeatureEnabled(agruntime.FeatureExample), agruntime.EncodeFeatures()})
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(rows); err != nil {
		panic(err)
	}
}
