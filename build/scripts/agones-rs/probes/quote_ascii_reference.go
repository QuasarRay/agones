// Post-gate concrete reference observations for the declared ASCII quote domain.
package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
)

func main() {
	quotes := []string{strconv.Quote("")}
	for first := 0; first < 128; first++ {
		quotes = append(quotes, strconv.Quote(string([]byte{byte(first)})))
	}
	for first := 0; first < 128; first++ {
		for second := 0; second < 128; second++ {
			quotes = append(quotes, strconv.Quote(string([]byte{byte(first), byte(second)})))
		}
	}
	data, err := os.ReadFile(filepath.Join(runtime.GOROOT(), "src/strconv/quote.go"))
	if err != nil {
		panic(err)
	}
	h := sha256.Sum256(data)
	result := map[string]interface{}{"schema_version": 1, "classification": "SOURCE_CONTRACT_PROPERTY_CONCRETE_REFERENCE", "go_version": runtime.Version(), "source_quote_sha256": hex.EncodeToString(h[:]), "alphabet": "ASCII bytes 0..127", "lengths": "0..2", "enumeration": "empty; one-byte ascending; two-byte lexicographic", "quotes": quotes}
	if err := json.NewEncoder(os.Stdout).Encode(result); err != nil {
		panic(err)
	}
}
