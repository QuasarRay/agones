package main

import (
	"go/ast"
	"go/constant"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"path/filepath"
	"testing"

	"golang.org/x/tools/go/packages"
)

func sdkAssertionFixture(t *testing.T) (*packages.Package, *ast.CallExpr) {
	t.Helper()
	expression, err := parser.ParseExpr("assert.Equal(t, pkg.Version, gs.Annotations[VersionAnnotation])")
	if err != nil {
		t.Fatal(err)
	}
	call := expression.(*ast.CallExpr)
	info := &types.Info{Uses: map[*ast.Ident]types.Object{}, Types: map[ast.Expr]types.TypeAndValue{}}
	assertion := call.Fun.(*ast.SelectorExpr).Sel
	info.Uses[assertion] = types.NewFunc(token.NoPos, types.NewPackage("github.com/stretchr/testify/assert", "assert"), "Equal", nil)
	version := call.Args[1].(*ast.SelectorExpr).Sel
	info.Uses[version] = types.NewVar(token.NoPos, types.NewPackage("agones.dev/agones/pkg", "pkg"), "Version", types.Typ[types.String])
	index := call.Args[2].(*ast.IndexExpr).Index
	info.Types[index] = types.TypeAndValue{Type: types.Typ[types.String], Value: constant.MakeString("agones.dev/sdk-version")}
	return &packages.Package{TypesInfo: info}, call
}

func TestSDKVersionAssertionUsesResolvedSymbolsAndExactObservation(t *testing.T) {
	pkg, call := sdkAssertionFixture(t)
	if !isSDKVersionAssertion(pkg, call) {
		t.Fatal("valid source assertion rejected")
	}
	name := call.Fun.(*ast.SelectorExpr).Sel
	pkg.TypesInfo.Uses[name] = types.NewFunc(token.NoPos, types.NewPackage("different/assert", "assert"), "Equal", nil)
	if isSDKVersionAssertion(pkg, call) {
		t.Fatal("shadowed assertion accepted")
	}
	pkg, call = sdkAssertionFixture(t)
	version := call.Args[1].(*ast.SelectorExpr).Sel
	pkg.TypesInfo.Uses[version] = types.NewVar(token.NoPos, types.NewPackage("different/pkg", "pkg"), "Version", types.Typ[types.String])
	if isSDKVersionAssertion(pkg, call) {
		t.Fatal("wrong version source accepted")
	}
	pkg, call = sdkAssertionFixture(t)
	index := call.Args[2].(*ast.IndexExpr).Index
	pkg.TypesInfo.Types[index] = types.TypeAndValue{Value: constant.MakeString("different-annotation")}
	if isSDKVersionAssertion(pkg, call) {
		t.Fatal("wrong annotation accepted")
	}
}

func TestSDKGateRejectsChangedBuildEnvironment(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "owner.rs"), []byte("owner"), 0600); err != nil {
		t.Fatal(err)
	}
	t.Setenv("AGONES_BUILD_VERSION", "source-dev")
	expected := hash([]byte("source-dev"))
	gate := Gate{Status: "IMPLEMENTED", ProductionFiles: map[string]string{"owner.rs": hash([]byte("owner"))}, EnvironmentBindings: map[string]EnvironmentBinding{"AGONES_BUILD_VERSION": {Present: true, SHA256: &expected}}}
	if err := validateGate(root, gate); err != nil {
		t.Fatal(err)
	}
	t.Setenv("AGONES_BUILD_VERSION", "changed")
	if err := validateGate(root, gate); err == nil {
		t.Fatal("stale build environment accepted")
	}
}
