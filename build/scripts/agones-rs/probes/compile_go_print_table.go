// Production data compiler for Go-compatible feature error rendering.
// No source test obligations are extracted here.
package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"unicode"
)

func main() {
	const count = 0x110000
	bitmap := make([]byte, count/8)
	ranges := [][2]uint32{}
	start := -1
	for value := 0; value < count; value++ {
		r := rune(value)
		printable := strconv.IsPrint(r)
		// An independent category relation from the documented IsPrint law.
		expected := r == ' ' || unicode.IsLetter(r) || unicode.IsMark(r) || unicode.IsNumber(r) || unicode.IsPunct(r) || unicode.IsSymbol(r)
		if printable != expected {
			panic("Go IsPrint violates the category relation")
		}
		if expected {
			bitmap[value/8] |= 1 << uint(value%8)
		}
		if printable && start < 0 {
			start = value
		}
		if !printable && start >= 0 {
			ranges = append(ranges, [2]uint32{uint32(start), uint32(value - 1)})
			start = -1
		}
	}
	if start >= 0 {
		ranges = append(ranges, [2]uint32{uint32(start), count - 1})
	}
	sources := map[string]string{}
	for _, path := range []string{"VERSION", "src/strconv/quote.go", "src/strconv/isprint.go", "src/strconv/number.go", "src/unicode/tables.go", "src/unicode/letter.go", "src/unicode/utf8/utf8.go", "src/net/url/url.go"} {
		b, err := os.ReadFile(filepath.Join(runtime.GOROOT(), path))
		if err != nil {
			panic(err)
		}
		h := sha256.Sum256(b)
		sources[path] = hex.EncodeToString(h[:])
	}
	if err := os.WriteFile(os.Args[1], bitmap, 0644); err != nil {
		panic(err)
	}
	h := sha256.Sum256(bitmap)
	result := map[string]interface{}{"schema_version": 1, "purpose": "PRODUCTION_PRINTABLE_RUNE_DATA", "go_version": runtime.Version(), "unicode_version": unicode.Version, "source_hashes": sources, "ranges": ranges, "codepoint_domain_count": count, "independent_category_bitmap_sha256": hex.EncodeToString(h[:]), "independent_category_bitmap": "verification/agones/go-print-expected.bin", "validation": "Every source strconv.IsPrint result equals the independent letter/mark/number/punctuation/symbol/ASCII-space relation; ordinary production data check, not a formal or source-test proof."}
	e := json.NewEncoder(os.Stdout)
	e.SetIndent("", "  ")
	if err := e.Encode(result); err != nil {
		panic(err)
	}
}
