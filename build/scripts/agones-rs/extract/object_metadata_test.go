package main

import (
	"encoding/json"
	"go/ast"
	"go/constant"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"golang.org/x/tools/go/packages"
)

func metadataAssertionFixture(t *testing.T) (metadataSyntax, *ast.CallExpr) {
	t.Helper()
	files := token.NewFileSet()
	expression, err := parser.ParseExprFrom(files, "source.go", `assert.Equal(t, "spec.metadata.labels", allErrs[0].Field)`, 0)
	if err != nil {
		t.Fatal(err)
	}
	call := expression.(*ast.CallExpr)
	info := &types.Info{Uses: map[*ast.Ident]types.Object{}, Types: map[ast.Expr]types.TypeAndValue{}}
	info.Uses[call.Fun.(*ast.SelectorExpr).Sel] = types.NewFunc(token.NoPos, types.NewPackage("github.com/stretchr/testify/assert", "assert"), "Equal", nil)
	info.Types[call.Args[1]] = types.TypeAndValue{Value: constant.MakeString("spec.metadata.labels")}
	index := call.Args[2].(*ast.SelectorExpr).X.(*ast.IndexExpr).Index
	info.Types[index] = types.TypeAndValue{Value: constant.MakeInt64(0)}
	return metadataSyntax{pkg: &packages.Package{Fset: files, TypesInfo: info}}, call
}

func metadataMustReject(t *testing.T, f func()) {
	t.Helper()
	defer func() {
		if recover() == nil {
			t.Fatal("unsupported metadata extraction was accepted")
		}
	}()
	f()
}

func TestMetadataAssertionsKeepResolvedOwnerConstantFieldAndIndex(t *testing.T) {
	syntax, call := metadataAssertionFixture(t)
	site := metadataAssertion(syntax, call)
	if site["kind"] != "field" || site["index"] != int64(0) || site["expected"] != "spec.metadata.labels" {
		t.Fatal(site)
	}
	selector := call.Fun.(*ast.SelectorExpr).Sel
	syntax.pkg.TypesInfo.Uses[selector] = types.NewFunc(token.NoPos, types.NewPackage("shadowed/assert", "assert"), "Equal", nil)
	if metadataAssertion(syntax, call) != nil {
		t.Fatal("shadowed assertion owner accepted")
	}
	syntax, call = metadataAssertionFixture(t)
	call.Args[2].(*ast.SelectorExpr).Sel.Name = "Detail"
	metadataMustReject(t, func() { metadataAssertion(syntax, call) })
	syntax, call = metadataAssertionFixture(t)
	delete(syntax.pkg.TypesInfo.Types, call.Args[2].(*ast.SelectorExpr).X.(*ast.IndexExpr).Index)
	metadataMustReject(t, func() { metadataAssertion(syntax, call) })
	syntax, call = metadataAssertionFixture(t)
	delete(syntax.pkg.TypesInfo.Types, call.Args[1])
	metadataMustReject(t, func() { metadataAssertion(syntax, call) })
}

func TestMetadataExtractionRequiresItsOwnCompletedGateBeforeLoading(t *testing.T) {
	root := t.TempDir()
	gate, err := json.Marshal(Gate{Status: "IMPLEMENTED", FeatureID: "GS-VALIDATE-FEATURES-001", ProductionFiles: map[string]string{"owner.rs": "unused"}})
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "gate.json"), gate, 0600); err != nil {
		t.Fatal(err)
	}
	metadataMustReject(t, func() { objectMetadataMain([]string{root, "gate.json"}) })
}

func TestMetadataOverflowShapeRetainsLiteralAndObservationTokens(t *testing.T) {
	withNewline := strings.Replace(metadataOverflowExpectedBody, "; assert.Equal", "\nassert.Equal", 1)
	if metadataBodyTokens(withNewline) != metadataBodyTokens(metadataOverflowExpectedBody) {
		t.Fatal("equivalent explicit/inserted Go semicolon rejected")
	}
	for _, mutation := range []string{
		strings.Replace(metadataOverflowExpectedBody, "err.Type", "err.Detail", 1),
		strings.Replace(metadataOverflowExpectedBody, "spec", "spec ", 1),
		strings.Replace(metadataOverflowExpectedBody, "assert.Len", "assert.Empty", 1),
	} {
		if metadataBodyTokens(mutation) == metadataBodyTokens(metadataOverflowExpectedBody) {
			t.Fatal("changed literal, observation or assertion accepted")
		}
	}
}
