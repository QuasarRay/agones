// Metadata only: this does not load types or translate test semantics.
package main

import (
	"encoding/json"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
)

type Test struct {
	ID          string   `json:"id"`
	Path        string   `json:"path"`
	Package     string   `json:"package"`
	Name        string   `json:"name"`
	Line        int      `json:"line"`
	LiteralRuns []string `json:"literal_run_labels"`
	DynamicRuns int      `json:"dynamic_run_sites"`
}

func main() {
	paths, err := exec.Command("git", "ls-tree", "-r", "--name-only", os.Args[1]).Output()
	if err != nil {
		panic(err)
	}
	tests := []Test{}
	for _, path := range strings.Split(strings.TrimSpace(string(paths)), "\n") {
		if strings.HasPrefix(path, "vendor/") || !strings.HasSuffix(path, "_test.go") {
			continue
		}
		source, err := exec.Command("git", "show", os.Args[1]+":"+path).Output()
		if err != nil {
			panic(err)
		}
		fset := token.NewFileSet()
		file, err := parser.ParseFile(fset, path, source, 0)
		if err != nil {
			panic(err)
		}
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv != nil || !(strings.HasPrefix(fn.Name.Name, "Test") || strings.HasPrefix(fn.Name.Name, "Fuzz") || strings.HasPrefix(fn.Name.Name, "Example") || strings.HasPrefix(fn.Name.Name, "Benchmark")) {
				continue
			}
			t := Test{ID: "agones.dev/agones/" + filepath.ToSlash(filepath.Dir(path)) + "::" + path + "::" + fn.Name.Name, Path: path, Package: file.Name.Name, Name: fn.Name.Name, Line: fset.Position(fn.Pos()).Line, LiteralRuns: []string{}}
			ast.Inspect(fn.Body, func(n ast.Node) bool {
				call, ok := n.(*ast.CallExpr)
				if !ok {
					return true
				}
				sel, ok := call.Fun.(*ast.SelectorExpr)
				if !ok || sel.Sel.Name != "Run" || len(call.Args) == 0 {
					return true
				}
				lit, ok := call.Args[0].(*ast.BasicLit)
				if ok && lit.Kind == token.STRING {
					label, err := strconv.Unquote(lit.Value)
					if err != nil {
						panic(err)
					}
					t.LiteralRuns = append(t.LiteralRuns, label)
				} else {
					t.DynamicRuns++
				}
				return true
			})
			tests = append(tests, t)
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(tests); err != nil {
		panic(err)
	}
}
