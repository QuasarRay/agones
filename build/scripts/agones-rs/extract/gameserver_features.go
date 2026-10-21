package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/printer"
	"go/types"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

func exactFeatureValidationBody(statements []string) bool {
	want := []string{
		"err := runtime.ParseFeatures(tc.feature)",
		"assert.NoError(t, err)",
		"errs := tc.gs.Validate(fakeAPIHooks{})",
		`assert.ElementsMatch(t, tc.want, errs, "ErrorList check")`,
	}
	if len(statements) != len(want) {
		return false
	}
	for i := range want {
		if statements[i] != want[i] {
			return false
		}
	}
	return true
}

func gameServerFeaturesMain(args []string) {
	gameServerValidationMain(args, true)
}

func gameServerValidationMain(args []string, features bool) {
	testName, featureID, prefixCount, siteCount := "TestGameServerValidateFeatures", "GS-VALIDATE-FEATURES-001", 4, 2
	if !features {
		testName, featureID, prefixCount, siteCount = "TestGameServerValidate", "RESOURCE-QUANTITY-001", 2, 1
	}
	if len(args) != 2 {
		panic("GameServer feature validation extraction requires root and its implementation gate")
	}
	root, err := filepath.Abs(args[0])
	must(err)
	gateBytes, err := os.ReadFile(filepath.Join(root, args[1]))
	must(err)
	var gate Gate
	must(json.Unmarshal(gateBytes, &gate))
	if gate.FeatureID != featureID {
		panic("wrong GameServer feature validation gate")
	}
	must(validateGate(root, gate))
	// Reuse the package-wide original-source inventory, not its old Rust gate.
	packet, err := os.ReadFile(filepath.Join(root, "verification/agones/sdk-version.ir.json"))
	must(err)
	var reusable struct {
		SourceFiles map[string]string `json:"source_files"`
	}
	must(json.Unmarshal(packet, &reusable))
	if len(reusable.SourceFiles) == 0 {
		panic("missing original package source bindings")
	}
	for path, expected := range reusable.SourceFiles {
		current, err := os.ReadFile(filepath.Join(root, path))
		must(err)
		original, err := exec.Command("git", "-C", root, "show", gate.Baseline+":"+path).Output()
		must(err)
		if hash(current) != expected || hash(original) != expected {
			panic("changed original feature validation dependency: " + path)
		}
	}
	pkg := loadTypedTests(root, "./pkg/apis/agones/v1", "pkg/apis/agones/v1 [")
	render := func(node ast.Node) string {
		var b bytes.Buffer
		must(printer.Fprint(&b, pkg.Fset, node))
		return b.String()
	}
	var test *ast.FuncDecl
	var hookDecls []ast.Decl
	for _, file := range pkg.Syntax {
		for _, decl := range file.Decls {
			if fn, ok := decl.(*ast.FuncDecl); ok && fn.Name.Name == testName {
				test = fn
			}
			if filepath.Base(pkg.Fset.Position(file.Pos()).Filename) == "apihooksfake.go" {
				if gen, ok := decl.(*ast.GenDecl); ok && gen.Tok.String() == "import" {
					continue
				}
				hookDecls = append(hookDecls, decl)
			}
		}
	}
	if test == nil || len(hookDecls) != 6 {
		panic("missing original test or source fakeAPIHooks declarations")
	}
	var prefix []ast.Stmt
	var loop *ast.RangeStmt
	var table *ast.CompositeLit
	for _, statement := range test.Body.List {
		if render(statement) == "t.Parallel()" {
			continue
		}
		if candidate, ok := statement.(*ast.RangeStmt); ok {
			if loop != nil || render(candidate.X) != "testCases" || render(candidate.Value) != "tc" {
				panic("unsupported source validation loop")
			}
			loop = candidate
			continue
		}
		if loop != nil {
			panic("unsupported source effect after validation loop")
		}
		prefix = append(prefix, statement)
		if assignment, ok := statement.(*ast.AssignStmt); ok && len(assignment.Lhs) == 1 && render(assignment.Lhs[0]) == "testCases" && len(assignment.Rhs) == 1 {
			table, _ = assignment.Rhs[0].(*ast.CompositeLit)
		}
	}
	if loop == nil || table == nil || len(table.Elts) == 0 || (features && len(table.Elts) != 5) || len(prefix) != prefixCount || len(loop.Body.List) != 1 {
		panic("unsupported whole original validation table/setup")
	}
	expression, ok := loop.Body.List[0].(*ast.ExprStmt)
	if !ok {
		panic("unsupported source test callback")
	}
	run, ok := expression.X.(*ast.CallExpr)
	if !ok || render(run.Fun) != "t.Run" || len(run.Args) != 2 || render(run.Args[0]) != "tc.description" {
		panic("unsupported source test callback")
	}
	callback, ok := run.Args[1].(*ast.FuncLit)
	if !ok {
		panic("missing source callback body")
	}
	body := []string{}
	for _, statement := range callback.Body.List {
		body = append(body, render(statement))
	}
	if (features && !exactFeatureValidationBody(body)) || (!features && !exactGeneralValidationBody(body)) {
		panic("unmodeled validation action/assertion: " + strings.Join(body, "; "))
	}
	imports := map[string]string{".": "agones.dev/agones/pkg/apis/agones/v1", "captureJSON": "encoding/json", "captureFMT": "fmt"}
	privateConstants := map[string]*types.Const{}
	if !features {
		imports["captureReflect"] = "reflect"
	}
	collect := func(node ast.Node) {
		ast.Inspect(node, func(node ast.Node) bool {
			if id, ok := node.(*ast.Ident); ok {
				if name, ok := pkg.TypesInfo.Uses[id].(*types.PkgName); ok {
					imports[name.Name()] = name.Imported().Path()
				}
				if constant, ok := pkg.TypesInfo.Uses[id].(*types.Const); ok && constant.Pkg() == pkg.Types && constant.Parent() == pkg.Types.Scope() && !constant.Exported() {
					privateConstants[constant.Name()] = constant
				}
			}
			return true
		})
	}
	for _, statement := range prefix {
		collect(statement)
	}
	collect(callback.Body)
	for _, decl := range hookDecls {
		collect(decl)
	}
	// Require the actual package-resolved source parser, validator and assertions.
	sites := []map[string]interface{}{}
	ast.Inspect(callback.Body, func(node ast.Node) bool {
		call, ok := node.(*ast.CallExpr)
		if !ok {
			return true
		}
		selector, ok := call.Fun.(*ast.SelectorExpr)
		if !ok {
			panic("unresolved validation effect")
		}
		object := pkg.TypesInfo.Uses[selector.Sel]
		if object == nil || object.Pkg() == nil {
			panic("unresolved validation effect")
		}
		owner := object.Pkg().Path()
		if owner == "github.com/stretchr/testify/assert" {
			position := pkg.Fset.Position(call.Pos())
			sites = append(sites, map[string]interface{}{"line": position.Line, "kind": selector.Sel.Name, "expression": render(call), "package": owner})
		} else if !((owner == "agones.dev/agones/pkg/util/runtime" && selector.Sel.Name == "ParseFeatures") || (owner == "agones.dev/agones/pkg/apis/agones/v1" && (selector.Sel.Name == "Validate" || (!features && selector.Sel.Name == "ApplyDefaults")))) {
			panic("unmodeled source call: " + owner + "." + selector.Sel.Name)
		}
		return true
	})
	if len(sites) != siteCount {
		panic("missing source validation assertion site")
	}
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
	program.WriteString(")\ntype captureTestingT struct{}\nfunc(captureTestingT) Errorf(f string,args ...interface{}){panic(captureFMT.Sprintf(f,args...))}\n")
	for _, decl := range hookDecls {
		program.WriteString(render(decl) + "\n")
	}
	constantBindings := []map[string]interface{}{}
	constantNames := []string{}
	for name := range privateConstants {
		constantNames = append(constantNames, name)
	}
	sort.Strings(constantNames)
	for _, name := range constantNames {
		constant := privateConstants[name]
		program.WriteString("const " + name + " = " + constant.Val().ExactString() + "\n")
		position := pkg.Fset.Position(constant.Pos())
		source, err := filepath.Rel(root, position.Filename)
		must(err)
		constantBindings = append(constantBindings, map[string]interface{}{"name": name, "value": constant.Val().ExactString(), "source": source, "line": position.Line})
	}
	if !features {
		program.WriteString(`func captureQuantityState(q resource.Quantity) {
    canonical := q.String()
    restored, err := resource.ParseQuantity(canonical)
    if err != nil { panic(err) }
    _ = restored.String()
    if !captureReflect.DeepEqual(q, restored) { panic("unsupported lossy quantity construction: " + canonical) }
}
`)
	}
	program.WriteString("func main(){t:=captureTestingT{}\n")
	for _, statement := range prefix {
		program.WriteString(render(statement) + "\n")
	}
	program.WriteString("rows:=[]map[string]interface{}{}\nfor _,tc:=range testCases {before,captureErr:=captureJSON.Marshal(tc.gs);if captureErr!=nil{panic(captureErr)}\n")
	if !features {
		program.WriteString(`capturePresence:=map[string]bool{"ports_nil":tc.gs.Spec.Ports==nil,"players_present":tc.gs.Spec.Players!=nil,"counters_present":tc.gs.Spec.Counters!=nil,"lists_present":tc.gs.Spec.Lists!=nil}
for _, containers := range [][]corev1.Container{tc.gs.Spec.Template.Spec.InitContainers,tc.gs.Spec.Template.Spec.Containers} {
    for _, container := range containers {
        for _, quantities := range []corev1.ResourceList{container.Resources.Limits,container.Resources.Requests} {
            for _, quantity := range quantities { captureQuantityState(quantity) }
        }
    }
}
`)
	}
	program.WriteString(strings.Join(body, "\n") + "\n")
	if features {
		program.WriteString(`rows=append(rows,map[string]interface{}{"name":tc.description,"feature_flags":tc.feature,"before":captureJSON.RawMessage(before),"fixture_presence":map[string]bool{"ports_nil":tc.gs.Spec.Ports==nil,"players_present":tc.gs.Spec.Players!=nil,"counters_present":tc.gs.Spec.Counters!=nil,"lists_present":tc.gs.Spec.Lists!=nil},"expected_errors":tc.want,"observed_errors":errs,"source_assertions_passed":true})}
encoded,err:=captureJSON.Marshal(rows);if err!=nil{panic(err)};captureFMT.Println(string(encoded))}`)
	} else {
		program.WriteString(`after,captureErr:=captureJSON.Marshal(tc.gs);if captureErr!=nil{panic(captureErr)}
rows=append(rows,map[string]interface{}{"name":tc.description,"apply_defaults":tc.applyDefaults,"before":captureJSON.RawMessage(before),"after":captureJSON.RawMessage(after),"fixture_presence":capturePresence,"expected_errors":tc.want,"observed_errors":errs,"source_assertions_passed":true,"quantity_construction_roundtrip_checked":true})}
encoded,err:=captureJSON.Marshal(rows);if err!=nil{panic(err)};captureFMT.Println(string(encoded))}`)
	}
	directory, err := os.MkdirTemp("", "agones-feature-validation-source-")
	must(err)
	defer os.RemoveAll(directory)
	filename := filepath.Join(directory, "main.go")
	must(os.WriteFile(filename, []byte(program.String()), 0600))
	command := exec.Command("go", "run", filename)
	command.Dir = root
	command.Env = append(os.Environ(), "GOFLAGS=-mod=vendor", "GOTOOLCHAIN=local")
	output, err := command.CombinedOutput()
	if err != nil {
		panic(fmt.Sprintf("original validation source replay: %v: %s", err, output))
	}
	decoder := json.NewDecoder(bytes.NewReader(output))
	decoder.UseNumber()
	var cases []map[string]interface{}
	must(decoder.Decode(&cases))
	if len(cases) != len(table.Elts) {
		panic("missing original validation case")
	}
	qualified := "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::" + testName + "::"
	seen := map[string]bool{}
	for index, row := range cases {
		id := sourceCaseIdentity(qualified+row["name"].(string), pkg.Fset.Position(table.Elts[index].Pos()).Line, seen)
		row["id"] = id
		assertions := []map[string]interface{}{}
		for _, site := range sites {
			assertions = append(assertions, map[string]interface{}{"id": fmt.Sprintf("%s::line-%d", id, site["line"]), "kind": site["kind"], "line": site["line"]})
		}
		row["assertions"] = assertions
	}
	result := map[string]interface{}{"schema_version": 1, "feature_id": featureID, "classification": "EXACT_CASE_ASSERTIONS", "implementation_gate": args[1], "implementation_gate_sha256": hash(gateBytes), "package_id": pkg.ID, "source_files": reusable.SourceFiles, "source_assertion_sites": sites, "source_actions": body, "cases": cases, "case_count": len(cases), "source_assertion_occurrences": len(cases) * len(sites), "capture_program": program.String(), "capture_program_sha256": hash([]byte(program.String())), "loader_source_files": loaderSourceBindings(root), "environment": loaderEnvironment(root), "scope": "All five original TestGameServerValidateFeatures fixtures and both assertion sites, ten qualified occurrences. Whole source fixtures and original private fakeAPIHooks declarations are captured without changing the original tree. No case sets Players, so this table gives no present-Players branch credit; the separate source-contract model remains required.", "scheduling_mapping": "Source feature mutex and serial table order retained; t.Parallel and t.Run mapped to serial source replay without executor/concurrency credit.", "assertion_semantics": "Original testify NoError and ElementsMatch execute on the Go reference. ElementsMatch compares complete error elements as an unordered multiset, with nil and empty lists both empty; it does not compare error-message formatting."}
	if !features {
		result["captured_private_constants"] = constantBindings
		result["scope"] = "The entire pinned original TestGameServerValidate table, every conditional ApplyDefaults action, validation call and qualified ElementsMatch occurrence. Independent expected error elements and actual source fake hooks are retained. No formal credit or complete migration claim."
		result["scheduling_mapping"] = "Original t.Parallel/t.Run mapped to serial exact-case replay; no executor/concurrency credit. The original test has no feature-mutex setup."
		result["assertion_semantics"] = "Original testify ElementsMatch executes on the reference, preserving all complete error elements and multiplicities. Nil and empty error lists are both empty; no error-message-only replacement. Quantity construction state must survive lowering; unsupported lossy serialization aborts capture."
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	must(encoder.Encode(result))
}
