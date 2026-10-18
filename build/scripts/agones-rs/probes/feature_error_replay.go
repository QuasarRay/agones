// Ordinary implementation witnesses against the original public error boundary.
package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"

	agonesruntime "agones.dev/agones/pkg/util/runtime"
)

func main() {
	var family struct {
		SchemaVersion int `json:"schema_version"`
		Cases         []struct {
			ID       string `json:"id"`
			Query    string `json:"query_hex"`
			Expected string `json:"expected_error"`
		} `json:"cases"`
	}
	data, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	if err = json.Unmarshal(data, &family); err != nil {
		panic(err)
	}
	if family.SchemaVersion != 1 || len(family.Cases) != 19 {
		panic("unexpected ordinary witness selection")
	}
	type observation struct {
		ID    string `json:"id"`
		Error string `json:"error"`
	}
	observations := []observation{}
	for _, c := range family.Cases {
		query, err := hex.DecodeString(c.Query)
		if err != nil {
			panic(err)
		}
		err = agonesruntime.ParseFeatures(string(query))
		if err == nil || err.Error() != c.Expected {
			panic(fmt.Sprintf("%s: expected %q, observed %v", c.ID, c.Expected, err))
		}
		observations = append(observations, observation{c.ID, err.Error()})
	}
	e := json.NewEncoder(os.Stdout)
	e.SetIndent("", "  ")
	if err := e.Encode(observations); err != nil {
		panic(err)
	}
}
