package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"go/types"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

func gameServerDefaultingMain(args []string) {
	if len(args) != 2 {
		panic("GameServer defaulting extraction requires root and implementation gate")
	}
	root, err := filepath.Abs(args[0])
	must(err)
	gateBytes, err := os.ReadFile(filepath.Join(root, args[1]))
	must(err)
	var gate Gate
	must(json.Unmarshal(gateBytes, &gate))
	if gate.FeatureID != "GS-DEFAULTS-001" {
		panic("wrong GameServer defaulting gate")
	}
	must(validateGate(root, gate))
	pkg := loadTypedTests(root, "./pkg/apis/agones/v1", "pkg/apis/agones/v1 [")
	render := func(node ast.Node) string {
		var b bytes.Buffer
		must(printer.Fprint(&b, pkg.Fset, node))
		return b.String()
	}
	canonical := func(node ast.Node) string {
		var b bytes.Buffer
		must(printer.Fprint(&b, token.NewFileSet(), node))
		return b.String()
	}
	var test *ast.FuncDecl
	for _, file := range pkg.Syntax {
		for _, decl := range file.Decls {
			if fn, ok := decl.(*ast.FuncDecl); ok && fn.Name.Name == "TestGameServerApplyDefaults" {
				test = fn
			}
		}
	}
	if test == nil {
		panic("missing original GameServer defaulting test")
	}
	// Reuse the existing source fixture packet only after validating both its
	// original-source bindings and every captured setup statement against the
	// currently type-resolved source. Its old production gate is not reused.
	sdkBytes, err := os.ReadFile(filepath.Join(root, "verification/agones/sdk-version.ir.json"))
	must(err)
	var sdk struct {
		SourceFiles map[string]string `json:"source_files"`
		Program     string            `json:"fixture_program"`
		ProgramHash string            `json:"fixture_program_sha256"`
	}
	must(json.Unmarshal(sdkBytes, &sdk))
	if hash([]byte(sdk.Program)) != sdk.ProgramHash {
		panic("corrupt reusable fixture program")
	}
	for path, want := range sdk.SourceFiles {
		current, err := os.ReadFile(filepath.Join(root, path))
		must(err)
		original, err := exec.Command("git", "-C", root, "show", gate.Baseline+":"+path).Output()
		must(err)
		if hash(current) != want || hash(original) != want {
			panic("stale original fixture source: " + path)
		}
	}
	captured, err := parseFixtureProgram(sdk.Program)
	must(err)
	prefix := []ast.Stmt{}
	var sourceLoop *ast.RangeStmt
	for _, statement := range test.Body.List {
		if expr, ok := statement.(*ast.ExprStmt); ok {
			if call, ok := expr.X.(*ast.CallExpr); ok {
				if sel, ok := call.Fun.(*ast.SelectorExpr); ok && sel.Sel.Name == "Parallel" {
					continue
				}
			}
		}
		if assignment, ok := statement.(*ast.AssignStmt); ok && len(assignment.Lhs) == 1 && render(assignment.Lhs[0]) == "data" {
			prefix = append(prefix, statement)
			break
		}
		prefix = append(prefix, statement)
	}
	if len(captured) < len(prefix) {
		panic("incomplete reusable fixture prefix")
	}
	for i, statement := range prefix {
		if canonical(statement) != canonical(captured[i]) {
			panic("reusable source fixture setup differs from typed source")
		}
	}
	for _, statement := range test.Body.List {
		if loop, ok := statement.(*ast.RangeStmt); ok {
			sourceLoop = loop
		}
	}
	if sourceLoop == nil || render(sourceLoop.X) != "data" {
		panic("unsupported source table loop")
	}
	if len(sourceLoop.Body.List) != 1 {
		panic("unsupported additional table effects")
	}
	run, ok := sourceLoop.Body.List[0].(*ast.ExprStmt)
	if !ok {
		panic("missing source subtest")
	}
	runCall, ok := run.X.(*ast.CallExpr)
	if !ok || len(runCall.Args) != 2 {
		panic("unsupported source subtest call")
	}
	runSelector, ok := runCall.Fun.(*ast.SelectorExpr)
	if !ok {
		panic("unresolved source subtest")
	}
	runObject := pkg.TypesInfo.Uses[runSelector.Sel]
	if runObject == nil || runObject.Pkg() == nil || runObject.Pkg().Path() != "testing" || runObject.Name() != "Run" || render(runCall.Args[0]) != "name" {
		panic("unsupported source subtest identity")
	}
	callback, ok := runCall.Args[1].(*ast.FuncLit)
	if !ok {
		panic("missing source subtest body")
	}
	imports := map[string]string{".": "agones.dev/agones/pkg/apis/agones/v1"}
	collect := func(node ast.Node) {
		ast.Inspect(node, func(n ast.Node) bool {
			if id, ok := n.(*ast.Ident); ok {
				if obj, ok := pkg.TypesInfo.Uses[id].(*types.PkgName); ok {
					imports[obj.Name()] = obj.Imported().Path()
				}
			}
			return true
		})
	}
	for _, statement := range prefix {
		collect(statement)
	}
	collect(callback.Body)
	qualified := "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::"
	sites := []map[string]interface{}{}
	actions := []string{}
	var translate func(*ast.BlockStmt, []string) string
	translate = func(block *ast.BlockStmt, guards []string) string {
		var body strings.Builder
		for _, statement := range block.List {
			switch statement := statement.(type) {
			case *ast.AssignStmt:
				text := render(statement)
				if text != "err := runtime.ParseFeatures(test.featureFlags)" && text != "spec := test.gameServer.Spec" {
					panic("unsupported source assignment: " + text)
				}
				actions = append(actions, text)
				body.WriteString(text + "\n")
			case *ast.IfStmt:
				if statement.Init != nil {
					panic("unsupported conditional initializer")
				}
				condition := render(statement.Cond)
				if !supportedDefaultingGuard(condition) {
					panic("unsupported defaulting guard: " + condition)
				}
				if strings.HasPrefix(condition, "len(") {
					binary := statement.Cond.(*ast.BinaryExpr)
					call := binary.X.(*ast.CallExpr)
					name := call.Fun.(*ast.Ident)
					if builtin, ok := pkg.TypesInfo.Uses[name].(*types.Builtin); !ok || builtin.Name() != "len" {
						panic("shadowed conditional length operation")
					}
				}
				body.WriteString("if " + condition + " {\n" + translate(statement.Body, append(append([]string{}, guards...), condition)) + "}")
				if statement.Else != nil {
					other, ok := statement.Else.(*ast.BlockStmt)
					if !ok {
						panic("unsupported conditional alternative")
					}
					body.WriteString(" else {\n" + translate(other, append(append([]string{}, guards...), "!("+condition+")")) + "}")
				}
				body.WriteByte('\n')
			case *ast.ExprStmt:
				call, ok := statement.X.(*ast.CallExpr)
				if !ok {
					panic("unsupported defaulting expression")
				}
				selector, ok := call.Fun.(*ast.SelectorExpr)
				if !ok {
					panic("unsupported source call")
				}
				obj := pkg.TypesInfo.Uses[selector.Sel]
				if obj == nil || obj.Pkg() == nil {
					panic("unresolved source call")
				}
				if obj.Pkg().Path() == "agones.dev/agones/pkg/apis/agones/v1" && selector.Sel.Name == "ApplyDefaults" && render(selector.X) == "test.gameServer" && len(call.Args) == 0 {
					actions = append(actions, render(call))
					body.WriteString(render(call) + "\n")
					continue
				}
				if obj.Pkg().Path() != "github.com/stretchr/testify/assert" {
					panic("unsupported source effect: " + render(call))
				}
				if len(call.Args) == 0 || render(call.Args[0]) != "t" {
					panic("unsupported assertion context")
				}
				kind := obj.Name()
				expected, actual := "nil", ""
				switch kind {
				case "Equal":
					if len(call.Args) != 3 {
						panic("unsupported equality shape")
					}
					expected, actual = render(call.Args[1]), render(call.Args[2])
				case "Contains":
					if len(call.Args) != 3 {
						panic("unsupported membership shape")
					}
					expected, actual = render(call.Args[2]), render(call.Args[1])
				case "Nil", "NoError":
					if len(call.Args) != 2 {
						panic("unsupported nil/error shape")
					}
					actual = render(call.Args[1])
				default:
					panic("unsupported source assertion: " + kind)
				}
				for _, argument := range call.Args[1:] {
					ast.Inspect(argument, func(n ast.Node) bool {
						if _, ok := n.(*ast.CallExpr); ok {
							panic("effectful observation requires an explicit translation")
						}
						return true
					})
				}
				line := pkg.Fset.Position(call.Pos()).Line
				original := render(call)
				argumentTypes := []string{}
				for _, argument := range call.Args[1:] {
					value := pkg.TypesInfo.Types[argument]
					if value.Type == nil {
						panic("unresolved assertion argument type")
					}
					argumentTypes = append(argumentTypes, value.Type.String())
				}
				sites = append(sites, map[string]interface{}{"line": line, "kind": kind, "resolved_function": obj.Pkg().Path() + "." + obj.Name(), "argument_types": argumentTypes, "expression": original, "expected_expression": expected, "actual_expression": actual, "guards": append([]string{}, guards...)})
				call.Args[0] = ast.NewIdent("captureT")
				body.WriteString(fmt.Sprintf("record(%d,%s,%s,%s,%s)\n", line, strconv.Quote(kind), expected, actual, render(call)))
			default:
				panic("unsupported source statement: " + render(statement))
			}
		}
		return body.String()
	}
	body := translate(callback.Body, nil)
	sourceSiteCount := 0
	ast.Inspect(test.Body, func(n ast.Node) bool {
		if call, ok := n.(*ast.CallExpr); ok {
			if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
				if obj := pkg.TypesInfo.Uses[sel.Sel]; obj != nil && obj.Pkg() != nil && obj.Pkg().Path() == "github.com/stretchr/testify/assert" {
					sourceSiteCount++
				}
			}
		}
		return true
	})
	if sourceSiteCount != len(sites) {
		panic("original assertion site omitted from translation")
	}
	ensure := func(path, name string) string {
		for alias, p := range imports {
			if p == path {
				return alias
			}
		}
		imports[name] = path
		return name
	}
	jsonAlias := ensure("encoding/json", "captureJSON")
	fmtAlias := ensure("fmt", "captureFMT")
	sortAlias := ensure("sort", "captureSORT")
	runtimeAlias := ensure("agones.dev/agones/pkg/util/runtime", "captureRuntime")
	var program strings.Builder
	program.WriteString("package main\nimport (\n")
	names := []string{}
	for name := range imports {
		names = append(names, name)
	}
	sort.Strings(names)
	for _, name := range names {
		program.WriteString(name + " " + strconv.Quote(imports[name]) + "\n")
	}
	program.WriteString(")\n")
	program.WriteString("type captureTestingT struct{}\nfunc (captureTestingT) Errorf(f string,args ...interface{}) {panic(" + fmtAlias + ".Sprintf(f,args...))}\nfunc main(){captureT:=captureTestingT{}\n")
	for _, statement := range prefix {
		program.WriteString(render(statement) + "\n")
	}
	program.WriteString(runtimeAlias + ".FeatureTestMutex.Lock();defer " + runtimeAlias + ".FeatureTestMutex.Unlock()\nkeys:=make([]string,0,len(data));for name:=range data {keys=append(keys,name)};" + sortAlias + ".Strings(keys)\nrows:=[]map[string]interface{}{}\nfor _,name:=range keys {test:=data[name]\nbefore,captureErr:=" + jsonAlias + ".Marshal(test.gameServer);if captureErr!=nil {panic(captureErr)}\nassertions:=[]map[string]interface{}{}\nrecord:=func(line int,kind string,expected,actual interface{},passed bool){if !passed {panic(\"source assertion failed\")};assertions=append(assertions,map[string]interface{}{\"line\":line,\"kind\":kind,\"expected\":expected,\"observed\":actual,\"source_passed\":passed})}\n")
	program.WriteString(body)
	program.WriteString("rows=append(rows,map[string]interface{}{\"name\":name,\"before\":" + jsonAlias + ".RawMessage(before),\"after\":test.gameServer,\"feature_flags\":test.featureFlags,\"assertions\":assertions,\"expected_context\":map[string]interface{}{\"alpha_capacity\":test.expected.alphaPlayerCapacity,\"eviction_spec\":test.expected.evictionSafeSpec,\"eviction_status\":test.expected.evictionSafeStatus,\"counter_spec\":test.expected.counterSpec,\"list_spec\":test.expected.listSpec}})}\nencoded,err:=" + jsonAlias + ".Marshal(rows);if err!=nil {panic(err)};" + fmtAlias + ".Println(string(encoded))}\n")
	directory, err := os.MkdirTemp("", "agones-defaulting-source-")
	must(err)
	defer os.RemoveAll(directory)
	filename := filepath.Join(directory, "main.go")
	must(os.WriteFile(filename, []byte(program.String()), 0600))
	command := exec.Command("go", "run", filename)
	command.Dir = root
	command.Env = append(os.Environ(), "GOFLAGS=-mod=vendor", "GOTOOLCHAIN=local")
	output, err := command.CombinedOutput()
	if err != nil {
		panic(fmt.Sprintf("defaulting source capture: %v: %s", err, output))
	}
	decoder := json.NewDecoder(bytes.NewReader(output))
	decoder.UseNumber()
	var cases []map[string]interface{}
	must(decoder.Decode(&cases))
	if len(cases) != 17 {
		panic("original GameServer table count changed")
	}
	total := 0
	occurrences := map[string]int{}
	for _, row := range cases {
		id := qualified + row["name"].(string)
		row["id"] = id
		for _, raw := range row["assertions"].([]interface{}) {
			assertion := raw.(map[string]interface{})
			assertion["id"] = id + "::line-" + assertion["line"].(json.Number).String()
			occurrences[assertion["line"].(json.Number).String()]++
			total++
		}
	}
	for _, site := range sites {
		count := occurrences[strconv.Itoa(site["line"].(int))]
		site["fixture_occurrences"] = count
		if count == 0 {
			site["status"] = "NO_CURRENT_FIXTURE_OCCURRENCE_NOT_CREDITED"
		} else {
			site["status"] = "EXACT_DYNAMIC_OCCURRENCES_CAPTURED_FORMAL_REQUIRED"
		}
	}
	result := map[string]interface{}{"schema_version": 1, "feature_id": "GS-DEFAULTS-001", "classification": "EXACT_CASE_ASSERTIONS", "implementation_gate": args[1], "implementation_gate_sha256": hash(gateBytes), "package_id": pkg.ID, "source_files": sdk.SourceFiles, "reused_sdk_fixture_packet_sha256": hash(sdkBytes), "source_assertion_sites": sites, "source_actions": actions, "cases": cases, "case_count": len(cases), "source_assertion_occurrences": total, "capture_program": program.String(), "capture_program_sha256": hash([]byte(program.String())), "loader_source_files": loaderSourceBindings(root), "environment": loaderEnvironment(root), "scope": "Every assertion and conditional occurrence in the selected original TestGameServerApplyDefaults table. This is exact source capture/runtime evidence, not formal credit or complete baseline coverage.", "scheduling_mapping": "Capture uses every exact fixture in sorted order under the source feature mutex. Original t.Parallel/t.Run remain in the original test replay; no concurrent executor proof is claimed."}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	must(encoder.Encode(result))
}

func supportedDefaultingGuard(condition string) bool {
	switch condition {
	case "test.expected.alphaPlayerCapacity != nil", "len(test.expected.evictionSafeSpec) > 0", "len(test.expected.evictionSafeStatus) > 0", "test.expected.counterSpec != nil", "test.expected.listSpec != nil":
		return true
	default:
		return false
	}
}

func parseFixtureProgram(source string) ([]ast.Stmt, error) {
	file, err := parseSourceProgram(source)
	if err != nil {
		return nil, err
	}
	for _, decl := range file.Decls {
		if fn, ok := decl.(*ast.FuncDecl); ok && fn.Name.Name == "main" {
			return fn.Body.List, nil
		}
	}
	return nil, fmt.Errorf("missing reusable fixture main")
}

func parseSourceProgram(source string) (*ast.File, error) {
	return parser.ParseFile(token.NewFileSet(), "fixture.go", source, 0)
}
