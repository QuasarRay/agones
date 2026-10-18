package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/constant"
	"go/printer"
	"go/types"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strconv"
	"strings"

	"golang.org/x/tools/go/packages"
)

func loadTypedTests(root, pattern, fragment string) *packages.Package {
	config := &packages.Config{Dir: root, Tests: true, Mode: packages.NeedName | packages.NeedFiles | packages.NeedCompiledGoFiles | packages.NeedImports | packages.NeedDeps | packages.NeedSyntax | packages.NeedTypes | packages.NeedTypesInfo | packages.NeedModule, Env: append(os.Environ(), "GOFLAGS=-mod=vendor", "GOTOOLCHAIN=local")}
	loaded, err := packages.Load(config, pattern)
	must(err)
	if packages.PrintErrors(loaded) != 0 {
		panic("package/type loading failed")
	}
	for _, candidate := range loaded {
		if strings.Contains(candidate.ID, fragment) {
			return candidate
		}
	}
	panic("missing typed test variant: " + fragment)
}

func loaderSourceBindings(root string) map[string]string {
	result := map[string]string{}
	files, err := filepath.Glob(filepath.Join(root, "build/scripts/agones-rs/extract/*.go"))
	must(err)
	files = append(files, filepath.Join(root, "build/scripts/agones-rs/extract/go.mod"), filepath.Join(root, "build/scripts/agones-rs/extract/go.sum"))
	for _, name := range files {
		data, err := os.ReadFile(name)
		must(err)
		relative, err := filepath.Rel(root, name)
		must(err)
		result[filepath.ToSlash(relative)] = hash(data)
	}
	return result
}

func loaderEnvironment(root string) map[string]string {
	command := exec.Command("go", "env", "-json", "GOVERSION", "GOOS", "GOARCH", "CGO_ENABLED", "GOEXPERIMENT", "GOFLAGS", "GOTOOLCHAIN", "GOMOD", "GOENV")
	command.Dir = root
	command.Env = append(os.Environ(), "GOFLAGS=-mod=vendor", "GOTOOLCHAIN=local")
	data, err := command.Output()
	must(err)
	var result map[string]string
	must(json.Unmarshal(data, &result))
	return result
}

func isSDKVersionAssertion(pkg *packages.Package, call *ast.CallExpr) bool {
	selector, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || len(call.Args) != 3 {
		return false
	}
	function := pkg.TypesInfo.Uses[selector.Sel]
	if function == nil || function.Pkg() == nil || function.Pkg().Path() != "github.com/stretchr/testify/assert" || function.Name() != "Equal" {
		return false
	}
	expected, ok := call.Args[1].(*ast.SelectorExpr)
	if !ok {
		return false
	}
	variable := pkg.TypesInfo.Uses[expected.Sel]
	if variable == nil || variable.Pkg() == nil || variable.Pkg().Path() != "agones.dev/agones/pkg" || variable.Name() != "Version" {
		return false
	}
	index, ok := call.Args[2].(*ast.IndexExpr)
	if !ok {
		return false
	}
	value := pkg.TypesInfo.Types[index.Index].Value
	return value != nil && value.Kind() == constant.String && constant.StringVal(value) == "agones.dev/sdk-version"
}

func sdkVersionMain(args []string) {
	if len(args) != 2 {
		panic("SDK extraction requires root and its implementation gate")
	}
	root, err := filepath.Abs(args[0])
	must(err)
	gateBytes, err := os.ReadFile(filepath.Join(root, args[1]))
	must(err)
	var gate Gate
	must(json.Unmarshal(gateBytes, &gate))
	if gate.FeatureID != "SDK-VERSION-001" {
		panic("SDK extraction requires the version feature gate")
	}
	must(validateGate(root, gate))
	pkg := loadTypedTests(root, "./pkg/apis/agones/v1", "pkg/apis/agones/v1 [")
	sourceFiles := map[string]string{}
	for _, name := range pkg.CompiledGoFiles {
		relative, err := filepath.Rel(root, name)
		must(err)
		if strings.HasPrefix(relative, "..") {
			continue
		}
		current, err := os.ReadFile(name)
		must(err)
		original, err := exec.Command("git", "-C", root, "show", gate.Baseline+":"+filepath.ToSlash(relative)).Output()
		must(err)
		if hash(current) != hash(original) {
			panic("source package changed: " + relative)
		}
		sourceFiles[filepath.ToSlash(relative)] = hash(current)
	}
	for _, name := range []string{"pkg/version.go", "pkg/util/runtime/features.go", "pkg/util/runtime/runtime.go", "pkg/util/errors/errors.go", "go.mod", "go.sum", "vendor/github.com/stretchr/testify/assert/assertions.go"} {
		current, err := os.ReadFile(filepath.Join(root, name))
		must(err)
		original, err := exec.Command("git", "-C", root, "show", gate.Baseline+":"+name).Output()
		must(err)
		if hash(current) != hash(original) {
			panic("SDK helper/config changed: " + name)
		}
		sourceFiles[name] = hash(current)
	}
	var gameServer, fleet *ast.FuncDecl
	var gsFile, fleetFile *ast.File
	for _, file := range pkg.Syntax {
		for _, declaration := range file.Decls {
			function, ok := declaration.(*ast.FuncDecl)
			if !ok {
				continue
			}
			switch function.Name.Name {
			case "TestGameServerApplyDefaults":
				gameServer, gsFile = function, file
			case "TestFleetApplyDefaults":
				fleet, fleetFile = function, file
			}
		}
	}
	if gameServer == nil || fleet == nil {
		panic("missing source version tests")
	}
	render := func(node ast.Node) string {
		var b bytes.Buffer
		must(printer.Fprint(&b, pkg.Fset, node))
		return b.String()
	}
	sites := []map[string]interface{}{}
	unselected := []map[string]interface{}{}
	for _, pair := range []struct {
		function *ast.FuncDecl
		file     *ast.File
	}{{gameServer, gsFile}, {fleet, fleetFile}} {
		relative, err := filepath.Rel(root, pkg.Fset.Position(pair.file.Pos()).Filename)
		must(err)
		ast.Inspect(pair.function.Body, func(node ast.Node) bool {
			call, ok := node.(*ast.CallExpr)
			if !ok {
				return true
			}
			selector, ok := call.Fun.(*ast.SelectorExpr)
			if !ok {
				return true
			}
			function := pkg.TypesInfo.Uses[selector.Sel]
			if function == nil || function.Pkg() == nil || function.Pkg().Path() != "github.com/stretchr/testify/assert" {
				return true
			}
			position := pkg.Fset.Position(call.Pos())
			row := map[string]interface{}{"source": filepath.ToSlash(relative), "test": pair.function.Name.Name, "line": position.Line, "expression": render(call), "package": function.Pkg().Path()}
			if isSDKVersionAssertion(pkg, call) {
				index := call.Args[2].(*ast.IndexExpr)
				observation := render(index.X)
				if (pair.function == gameServer && observation != "test.gameServer.Annotations") || (pair.function == fleet && observation != "f.ObjectMeta.Annotations") {
					panic("unsupported SDK observation path: " + observation)
				}
				sites = append(sites, row)
			} else {
				row["status"] = "REQUIRED_UNSELECTED_ASSERTION_SITE"
				row["dynamic_occurrence_count"] = "UNRESOLVED"
				unselected = append(unselected, row)
			}
			return true
		})
	}
	if len(sites) != 3 {
		panic(fmt.Sprintf("SDK version assertion sites changed: %d", len(sites)))
	}
	// The selected Fleet source has exactly one empty fixture and two defaulting
	// mutations. Other assertions stay required; do not guess past new effects.
	fleetMutations := 0
	for index, statement := range fleet.Body.List {
		if index == 0 {
			assignment, ok := statement.(*ast.AssignStmt)
			if !ok || len(assignment.Lhs) != 1 || render(assignment.Lhs[0]) != "f" || len(assignment.Rhs) != 1 || render(assignment.Rhs[0]) != "&Fleet{}" {
				panic("unsupported Fleet fixture setup")
			}
			continue
		}
		expression, ok := statement.(*ast.ExprStmt)
		if !ok {
			panic("unsupported Fleet source effect")
		}
		call, ok := expression.X.(*ast.CallExpr)
		if !ok {
			panic("unsupported Fleet source effect")
		}
		selector, ok := call.Fun.(*ast.SelectorExpr)
		if !ok {
			panic("unsupported Fleet source effect")
		}
		if render(selector.X) == "f" && selector.Sel.Name == "ApplyDefaults" && len(call.Args) == 0 {
			fleetMutations++
			continue
		}
		function := pkg.TypesInfo.Uses[selector.Sel]
		if function == nil || function.Pkg() == nil || function.Pkg().Path() != "github.com/stretchr/testify/assert" {
			panic("unsupported Fleet source effect")
		}
	}
	if fleetMutations != 2 {
		panic("Fleet source mutation count changed")
	}
	var table *ast.CompositeLit
	prefix := []ast.Stmt{}
	for _, statement := range gameServer.Body.List {
		if expression, ok := statement.(*ast.ExprStmt); ok {
			if call, ok := expression.X.(*ast.CallExpr); ok {
				if selector, ok := call.Fun.(*ast.SelectorExpr); ok && selector.Sel.Name == "Parallel" {
					continue
				}
			}
		}
		prefix = append(prefix, statement)
		if assignment, ok := statement.(*ast.AssignStmt); ok && len(assignment.Lhs) == 1 {
			if id, ok := assignment.Lhs[0].(*ast.Ident); ok && id.Name == "data" {
				table, ok = assignment.Rhs[0].(*ast.CompositeLit)
				if !ok {
					panic("unsupported SDK table shape")
				}
				break
			}
		}
	}
	if table == nil {
		panic("missing exact source fixture table")
	}
	caseNames := []string{}
	for _, element := range table.Elts {
		entry, ok := element.(*ast.KeyValueExpr)
		if !ok {
			panic("unsupported SDK table entry")
		}
		value := pkg.TypesInfo.Types[entry.Key].Value
		if value == nil || value.Kind() != constant.String {
			panic("nonconstant SDK case name")
		}
		caseNames = append(caseNames, constant.StringVal(value))
	}
	sort.Strings(caseNames)
	imports := map[string]string{}
	var source bytes.Buffer
	for _, statement := range prefix {
		ast.Inspect(statement, func(node ast.Node) bool {
			id, ok := node.(*ast.Ident)
			if !ok {
				return true
			}
			object := pkg.TypesInfo.Uses[id]
			if object == nil {
				return true
			}
			if name, ok := object.(*types.PkgName); ok {
				imports[name.Name()] = name.Imported().Path()
				return true
			}
			if object.Pkg() == pkg.Types && object.Pos() < gameServer.Body.Pos() && !object.Exported() {
				panic("unsupported private source setup dependency: " + object.Name())
			}
			return true
		})
		source.WriteString(render(statement))
		source.WriteByte('\n')
	}
	imports["."] = "agones.dev/agones/pkg/apis/agones/v1"
	ensureImport := func(path, preferred string) string {
		for alias, existing := range imports {
			if existing == path {
				return alias
			}
		}
		imports[preferred] = path
		return preferred
	}
	aliases := []string{"$JSON", ensureImport("encoding/json", "captureJSON"), "$FMT", ensureImport("fmt", "captureFMT"), "$SORT", ensureImport("sort", "captureSORT"), "$RUNTIME", ensureImport("agones.dev/agones/pkg/util/runtime", "captureRuntime"), "$PKG", ensureImport("agones.dev/agones/pkg", "capturePkg")}
	footer := strings.NewReplacer(aliases...).Replace(`
    keys:=make([]string,0,len(data));for name:=range data {keys=append(keys,name)};$SORT.Strings(keys)
    rows:=[]map[string]interface{}{}
    $RUNTIME.FeatureTestMutex.Lock();defer $RUNTIME.FeatureTestMutex.Unlock()
    for _,name:=range keys {
        test:=data[name];before,err:=$JSON.Marshal(test.gameServer);if err!=nil {panic(err)}
        if err:=$RUNTIME.ParseFeatures(test.featureFlags);err!=nil {panic(err)}
        gs:=test.gameServer.DeepCopy();gs.ApplyDefaults();observed,present:=gs.Annotations[VersionAnnotation]
        if !present || observed!=$PKG.Version {panic("source SDK fixture version assertion failed")}
        rows=append(rows,map[string]interface{}{"name":name,"feature_flags":test.featureFlags,"before":$JSON.RawMessage(before),"observed":observed})
    }
    f:=&Fleet{};before,err:=$JSON.Marshal(f);if err!=nil {panic(err)};fleetVersions:=[]string{}
    for i:=0;i<2;i++ {f.ApplyDefaults();version,present:=f.Annotations[VersionAnnotation];if !present || version!=$PKG.Version {panic("source Fleet version assertion failed")};fleetVersions=append(fleetVersions,version)}
    result:=map[string]interface{}{"game_server_cases":rows,"fleet_before":$JSON.RawMessage(before),"fleet_versions":fleetVersions,"expected_version":$PKG.Version}
    encoded,err:=$JSON.Marshal(result);if err!=nil {panic(err)};$FMT.Println(string(encoded))
`)
	var program bytes.Buffer
	program.WriteString("package main\nimport (\n")
	names := []string{}
	for alias := range imports {
		names = append(names, alias)
	}
	sort.Strings(names)
	for _, alias := range names {
		program.WriteString(alias + " " + strconv.Quote(imports[alias]) + "\n")
	}
	program.WriteString(")\nfunc main(){\n")
	program.Write(source.Bytes())
	program.WriteString(footer)
	program.WriteString("}\n")
	directory, err := os.MkdirTemp("", "agones-sdk-source-fixtures-")
	must(err)
	defer os.RemoveAll(directory)
	filename := filepath.Join(directory, "main.go")
	must(os.WriteFile(filename, program.Bytes(), 0600))
	command := exec.Command("go", "run", filename)
	command.Dir = root
	command.Env = append(os.Environ(), "GOFLAGS=-mod=vendor", "GOTOOLCHAIN=local")
	output, err := command.CombinedOutput()
	if err != nil {
		panic(fmt.Sprintf("source fixture construction: %v: %s", err, output))
	}
	var fixtures map[string]interface{}
	must(json.Unmarshal(output, &fixtures))
	if rows, ok := fixtures["game_server_cases"].([]interface{}); !ok || len(rows) != len(caseNames) {
		panic("source fixture count mismatch")
	}
	cases := []map[string]interface{}{}
	for _, raw := range fixtures["game_server_cases"].([]interface{}) {
		row := raw.(map[string]interface{})
		name := row["name"].(string)
		base := pkg.PkgPath + "::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::" + name
		cases = append(cases, map[string]interface{}{"id": base, "kind": "GAME_SERVER_VERSION", "name": name, "input_json": row["before"], "feature_flags": row["feature_flags"], "expected_version": fixtures["expected_version"], "assertion_ids": []string{fmt.Sprintf("%s::line-%v", base, sites[0]["line"])}, "steps": []string{"CONSTRUCT_EXACT_SOURCE_FIXTURE", "PARSE_SOURCE_FEATURE_FLAGS", "ASSERT_PARSE_NO_ERROR", "APPLY_DEFAULTS", "ASSERT_SDK_VERSION"}, "disposition": "RUNTIME_MAPPED_FORMAL_REQUIRED"})
	}
	fleetBase := pkg.PkgPath + "::pkg/apis/agones/v1/fleet_test.go::TestFleetApplyDefaults::$root"
	cases = append(cases, map[string]interface{}{"id": fleetBase, "kind": "FLEET_VERSION", "name": "$root", "input_json": fixtures["fleet_before"], "expected_version": fixtures["expected_version"], "assertion_ids": []string{fmt.Sprintf("%s::line-%v", fleetBase, sites[1]["line"]), fmt.Sprintf("%s::line-%v", fleetBase, sites[2]["line"])}, "steps": []string{"CONSTRUCT_EXACT_SOURCE_FIXTURE", "APPLY_DEFAULTS", "ASSERT_SDK_VERSION", "APPLY_DEFAULTS", "ASSERT_SDK_VERSION"}, "disposition": "RUNTIME_MAPPED_FORMAL_REQUIRED"})
	for _, site := range unselected {
		site["id"] = fmt.Sprintf("%s::%v::%v::assertion-site-line-%v", pkg.PkgPath, site["source"], site["test"], site["line"])
	}
	result := map[string]interface{}{"schema_version": 1, "feature_id": "SDK-VERSION-001", "classification": "EXACT_CASE_ASSERTION_PROJECTIONS", "implementation_gate": args[1], "implementation_gate_sha256": hash(gateBytes), "package_id": pkg.ID, "source_files": sourceFiles, "assertion_sites": sites, "game_server_case_names": caseNames, "cases": cases, "source_mapped_version_assertions": len(caseNames) + 2, "fixtures": fixtures, "fixture_program": program.String(), "fixture_program_sha256": hash(program.Bytes()), "required_unselected_assertion_sites": unselected, "scope": "Only three source version equality assertion sites, expanded over every exact GameServer table case and both Fleet observations. All other source assertions remain required; their dynamic totals are unresolved.", "scheduling_mapping": "Original t.Parallel and FeatureTestMutex remain in the original source replay. Fixture capture runs exact source setup/helpers and source methods serially under the same feature mutex; it certifies no executor/concurrency behavior."}
	result["loader_source_files"] = loaderSourceBindings(root)
	result["environment"] = loaderEnvironment(root)
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	must(encoder.Encode(result))
}
