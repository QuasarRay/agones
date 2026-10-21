package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/constant"
	"go/printer"
	"go/scanner"
	"go/token"
	"go/types"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"

	"golang.org/x/tools/go/packages"
)

type metadataSyntax struct {
	pkg  *packages.Package
	test *ast.FuncDecl
}

func (s metadataSyntax) render(node ast.Node) string {
	var out bytes.Buffer
	must(printer.Fprint(&out, s.pkg.Fset, node))
	return out.String()
}

func metadataTest(pkg *packages.Package, name string) metadataSyntax {
	var found *ast.FuncDecl
	for _, file := range pkg.Syntax {
		for _, decl := range file.Decls {
			if fn, ok := decl.(*ast.FuncDecl); ok && fn.Name.Name == name {
				if found != nil {
					panic("duplicate source metadata test")
				}
				found = fn
			}
		}
	}
	if found == nil {
		panic("missing source metadata test: " + name)
	}
	return metadataSyntax{pkg, found}
}

func metadataCallOwner(s metadataSyntax, call *ast.CallExpr) (string, string) {
	if id, ok := call.Fun.(*ast.Ident); ok {
		if builtin, ok := s.pkg.TypesInfo.Uses[id].(*types.Builtin); ok && builtin.Name() == "len" {
			return "builtin", "len"
		}
	}
	selector, ok := call.Fun.(*ast.SelectorExpr)
	if !ok {
		panic("unresolved source metadata call")
	}
	object := s.pkg.TypesInfo.Uses[selector.Sel]
	if object == nil || object.Pkg() == nil {
		panic("unresolved source metadata method")
	}
	return object.Pkg().Path(), object.Name()
}

func metadataAssertion(s metadataSyntax, call *ast.CallExpr) map[string]interface{} {
	owner, name := metadataCallOwner(s, call)
	if owner != "github.com/stretchr/testify/assert" && owner != "github.com/stretchr/testify/require" {
		return nil
	}
	if len(call.Args) < 2 || s.render(call.Args[0]) != "t" {
		panic("unsupported metadata assertion context")
	}
	site := map[string]interface{}{"package": owner, "source_kind": name,
		"line": s.pkg.Fset.Position(call.Pos()).Line, "expression": s.render(call)}
	switch name {
	case "Empty":
		if len(call.Args) != 2 || s.render(call.Args[1]) != "allErrs" {
			panic("unsupported metadata Empty assertion")
		}
		site["kind"], site["expected"] = "length", 0
	case "Len":
		if len(call.Args) != 3 || s.render(call.Args[1]) != "allErrs" {
			panic("unsupported metadata Len assertion")
		}
		value := s.pkg.TypesInfo.Types[call.Args[2]].Value
		if value == nil || value.Kind() != constant.Int {
			panic("metadata expected length is not a source integer constant")
		}
		n, exact := constant.Int64Val(value)
		if !exact || n < 0 {
			panic("unsupported metadata expected length")
		}
		site["kind"], site["expected"] = "length", n
	case "Equal":
		if len(call.Args) != 3 {
			panic("unsupported metadata Equal arguments")
		}
		value := s.pkg.TypesInfo.Types[call.Args[1]].Value
		field, ok := call.Args[2].(*ast.SelectorExpr)
		if value == nil || value.Kind() != constant.String || !ok || field.Sel.Name != "Field" {
			panic("unsupported metadata Equal observation")
		}
		index, ok := field.X.(*ast.IndexExpr)
		if !ok || s.render(index.X) != "allErrs" {
			panic("unsupported metadata error indexing")
		}
		literal := s.pkg.TypesInfo.Types[index.Index].Value
		if literal == nil || literal.Kind() != constant.Int {
			panic("metadata error index is not a source integer constant")
		}
		n, exact := constant.Int64Val(literal)
		if !exact || n < 0 {
			panic("unsupported metadata error index")
		}
		site["kind"], site["index"], site["expected"] = "field", n, constant.StringVal(value)
	default:
		panic("unsupported source metadata assertion: " + owner + "." + name)
	}
	return site
}

func metadataCapture(root, program string) []map[string]interface{} {
	dir, err := os.MkdirTemp("", "agones-metadata-source-")
	must(err)
	defer os.RemoveAll(dir)
	path := filepath.Join(dir, "main.go")
	must(os.WriteFile(path, []byte(program), 0600))
	cmd := exec.Command("go", "run", path)
	cmd.Dir = root
	cmd.Env = append(os.Environ(), "GOFLAGS=-mod=vendor", "GOTOOLCHAIN=local")
	output, err := cmd.CombinedOutput()
	if err != nil {
		panic(fmt.Sprintf("original metadata replay: %v: %s", err, output))
	}
	var rows []map[string]interface{}
	must(json.Unmarshal(output, &rows))
	return rows
}

const metadataCapturePrelude = `package main
import("encoding/json";"fmt";. "SOURCE_PACKAGE";"github.com/stretchr/testify/assert";"github.com/stretchr/testify/require";"k8s.io/apimachinery/pkg/util/validation/field")
type captureTestingT struct{}
func(captureTestingT) Errorf(f string,args ...interface{}){panic(fmt.Sprintf(f,args...))}
func(captureTestingT) FailNow(){panic("require failed")}
func(captureTestingT) Parallel(){}
var _ = require.Len
func main(){t:=captureTestingT{};rows:=[]map[string]interface{}{}
`

const metadataOverflowExpectedBody = `errs := v.ao.Validate(field.NewPath("spec", "allocationOverflow"))
assert.Len(t, errs, len(v.fields))
for i, err := range errs { assert.Equal(t, field.ErrorTypeInvalid, err.Type); assert.Equal(t, v.expected.fields[i], err.Field) }`

func metadataBodyTokens(source string) string {
	files := token.NewFileSet()
	var lexer scanner.Scanner
	lexer.Init(files.AddFile("body.go", -1, len(source)), []byte(source), nil, 0)
	var result strings.Builder
	for {
		_, kind, literal := lexer.Scan()
		if kind == token.EOF {
			return result.String()
		}
		// Go printer may retain an explicit same-line semicolon or print an
		// automatically inserted newline. Preserve every other token/literal.
		if kind != token.SEMICOLON {
			result.WriteString(kind.String())
			result.WriteByte(0)
			result.WriteString(literal)
			result.WriteByte(0)
		}
	}
}

func metadataOverflow(root string, s metadataSyntax) (map[string]interface{}, string) {
	if len(s.test.Body.List) != 3 {
		panic("unsupported whole AllocationOverflow validation setup")
	}
	declaration, ok := s.test.Body.List[1].(*ast.AssignStmt)
	if !ok || len(declaration.Lhs) != 1 || s.render(declaration.Lhs[0]) != "fixtures" || len(declaration.Rhs) != 1 {
		panic("missing AllocationOverflow fixture table")
	}
	table, ok := declaration.Rhs[0].(*ast.CompositeLit)
	if !ok || len(table.Elts) != 5 {
		panic("unsupported AllocationOverflow fixture table")
	}
	loop, ok := s.test.Body.List[2].(*ast.RangeStmt)
	if !ok || s.render(loop.X) != "fixtures" || s.render(loop.Key) != "k" || s.render(loop.Value) != "v" || len(loop.Body.List) != 1 {
		panic("unsupported AllocationOverflow source iteration")
	}
	expression, ok := loop.Body.List[0].(*ast.ExprStmt)
	if !ok {
		panic("missing AllocationOverflow source callback")
	}
	run, ok := expression.X.(*ast.CallExpr)
	if !ok || s.render(run.Fun) != "t.Run" || len(run.Args) != 2 || s.render(run.Args[0]) != "k" {
		panic("unsupported AllocationOverflow callback arguments")
	}
	callback, ok := run.Args[1].(*ast.FuncLit)
	if !ok || len(callback.Body.List) != 3 {
		panic("unsupported AllocationOverflow callback body")
	}
	var actual []string
	for _, stmt := range callback.Body.List {
		actual = append(actual, s.render(stmt))
	}
	programBody := strings.Join(actual, "\n")
	if metadataBodyTokens(programBody) != metadataBodyTokens(metadataOverflowExpectedBody) {
		panic("unmodeled whole AllocationOverflow validation action/assertion")
	}
	sites := []map[string]interface{}{}
	ast.Inspect(callback.Body, func(node ast.Node) bool {
		call, ok := node.(*ast.CallExpr)
		if !ok {
			return true
		}
		owner, name := metadataCallOwner(s, call)
		if owner == "github.com/stretchr/testify/assert" && (name == "Len" || name == "Equal") {
			site := map[string]interface{}{"source_kind": name, "line": s.pkg.Fset.Position(call.Pos()).Line, "package": owner, "expression": s.render(call)}
			if name == "Equal" {
				if value := s.pkg.TypesInfo.Types[call.Args[1]].Value; value != nil && value.Kind() == constant.String {
					site["expected"] = constant.StringVal(value)
				}
			}
			sites = append(sites, site)
		} else if !((owner == "agones.dev/agones/pkg/apis/agones/v1" && name == "Validate") || (owner == "k8s.io/apimachinery/pkg/util/validation/field" && name == "NewPath") || (owner == "builtin" && name == "len")) {
			panic("unmodeled AllocationOverflow source call: " + owner + "." + name)
		}
		return true
	})
	if len(sites) != 3 || sites[1]["expected"] != "FieldValueInvalid" {
		panic("missing AllocationOverflow assertion sites")
	}
	program := strings.ReplaceAll(metadataCapturePrelude, "SOURCE_PACKAGE", s.pkg.PkgPath)
	program += s.render(s.test.Body.List[0]) + "\n" + s.render(declaration) + "\nfor k,v:=range fixtures{\n" + programBody
	program += `
rows=append(rows,map[string]interface{}{"name":k,"labels":v.ao.Labels,"annotations":v.ao.Annotations,"labels_nil":v.ao.Labels==nil,"annotations_nil":v.ao.Annotations==nil,"expected_fields":v.fields,"observed_errors":errs,"source_assertions_passed":true})}
data,err:=json.Marshal(rows);if err!=nil{panic(err)};fmt.Println(string(data))}`
	rows := metadataCapture(root, program)
	byName := map[string]map[string]interface{}{}
	for _, row := range rows {
		name := row["name"].(string)
		if byName[name] != nil {
			panic("duplicate captured AllocationOverflow case")
		}
		byName[name] = row
	}
	qualified := "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::"
	cases := []map[string]interface{}{}
	occurrences := 0
	for _, entry := range table.Elts {
		key, ok := entry.(*ast.KeyValueExpr)
		if !ok {
			panic("unsupported AllocationOverflow case key")
		}
		name := s.pkg.TypesInfo.Types[key.Key].Value
		if name == nil || name.Kind() != constant.String {
			panic("nonconstant AllocationOverflow case identity")
		}
		row := byName[constant.StringVal(name)]
		if row == nil {
			panic("missing captured AllocationOverflow case")
		}
		row["id"] = qualified + constant.StringVal(name)
		fields, _ := row["expected_fields"].([]interface{})
		assertions := []map[string]interface{}{{"id": fmt.Sprintf("%s::line-%d", row["id"], sites[0]["line"]), "kind": "length", "expected": len(fields), "line": sites[0]["line"]}}
		for index, path := range fields {
			assertions = append(assertions,
				map[string]interface{}{"id": fmt.Sprintf("%s::line-%d::error-index-%d", row["id"], sites[1]["line"], index), "kind": "type", "expected": sites[1]["expected"], "index": index, "line": sites[1]["line"]},
				map[string]interface{}{"id": fmt.Sprintf("%s::line-%d::error-index-%d", row["id"], sites[2]["line"], index), "kind": "field", "expected": path, "index": index, "line": sites[2]["line"]})
		}
		row["assertions"] = assertions
		row["unexecuted_error_loop_sites"] = len(fields) == 0
		occurrences += len(assertions)
		cases = append(cases, row)
	}
	if len(rows) != len(cases) || occurrences != 11 {
		panic("AllocationOverflow assertion/case accounting changed")
	}
	return map[string]interface{}{"test": "TestAllocationOverflowValidate", "package_id": s.pkg.ID, "source_assertion_sites": sites, "source_body": s.render(s.test.Body), "cases": cases, "case_count": len(cases), "source_assertion_occurrences": occurrences}, program
}

func metadataString(s metadataSyntax, expression ast.Expr) string {
	value := s.pkg.TypesInfo.Types[expression].Value
	if value == nil || value.Kind() != constant.String {
		panic("unsupported nonconstant source metadata string")
	}
	return constant.StringVal(value)
}

func metadataMapField(s metadataSyntax, selector *ast.SelectorExpr) (string, string) {
	receiver := s.render(selector.X)
	if receiver != "mp" && receiver != "invalid" {
		panic("unsupported source metadata map receiver")
	}
	field := selector.Sel.Name
	if field != "Labels" && field != "Annotations" {
		panic("unsupported source metadata map field")
	}
	return receiver, strings.ToLower(field)
}

func metadataPatchAction(s metadataSyntax, statement ast.Stmt) map[string]interface{} {
	action := map[string]interface{}{"line": s.pkg.Fset.Position(statement.Pos()).Line, "source": s.render(statement)}
	if expression, ok := statement.(*ast.ExprStmt); ok {
		call, ok := expression.X.(*ast.CallExpr)
		if !ok {
			panic("unsupported MetaPatch expression effect")
		}
		owner, name := metadataCallOwner(s, call)
		if owner == "testing" && name == "Parallel" && len(call.Args) == 0 && s.render(call.Fun) == "t.Parallel" {
			action["kind"] = "parallel"
			return action
		}
		if site := metadataAssertion(s, call); site != nil {
			action["kind"], action["assertion"] = "assertion", site
			return action
		}
		panic("unsupported MetaPatch expression call")
	}
	assignment, ok := statement.(*ast.AssignStmt)
	if !ok || len(assignment.Lhs) != 1 || len(assignment.Rhs) != 1 {
		panic("unsupported MetaPatch assignment shape")
	}
	action["declare"] = assignment.Tok == token.DEFINE
	left, right := assignment.Lhs[0], assignment.Rhs[0]
	if call, ok := right.(*ast.CallExpr); ok {
		owner, name := metadataCallOwner(s, call)
		switch {
		case owner == s.pkg.PkgPath && name == "Validate":
			if s.render(left) != "allErrs" || len(call.Args) != 1 || s.render(call.Args[0]) != "path" {
				panic("unsupported MetaPatch validation destination/path")
			}
			receiver := s.render(call.Fun.(*ast.SelectorExpr).X)
			if receiver != "mp" && receiver != "invalid" {
				panic("unsupported MetaPatch validation receiver")
			}
			action["kind"], action["receiver"] = "validate", receiver
		case owner == s.pkg.PkgPath && name == "DeepCopy":
			if s.render(left) != "invalid" || s.render(call.Fun.(*ast.SelectorExpr).X) != "mp" || len(call.Args) != 0 {
				panic("unsupported MetaPatch copy/alias action")
			}
			action["kind"], action["target"], action["receiver"] = "clone_patch", "invalid", "mp"
		case owner == "k8s.io/apimachinery/pkg/util/validation/field" && name == "NewPath":
			if s.render(left) != "path" || assignment.Tok != token.DEFINE {
				panic("unsupported metadata field path action")
			}
			segments := []string{}
			for _, arg := range call.Args {
				segment := metadataString(s, arg)
				if segment == "" {
					panic("empty source field.Path segment requires explicit lowering")
				}
				segments = append(segments, segment)
			}
			if len(segments) == 0 {
				panic("missing source field.Path root")
			}
			action["kind"], action["segments"] = "field_path", segments
		default:
			panic("unsupported MetaPatch assignment call")
		}
		return action
	}
	if pointer, ok := right.(*ast.UnaryExpr); ok && pointer.Op == token.AND {
		literal, ok := pointer.X.(*ast.CompositeLit)
		if !ok || s.render(left) != "mp" || s.render(literal.Type) != "MetaPatch" || assignment.Tok != token.DEFINE || len(literal.Elts) != 2 {
			panic("unsupported MetaPatch construction")
		}
		fields := map[string]bool{}
		for _, entry := range literal.Elts {
			pair, ok := entry.(*ast.KeyValueExpr)
			if !ok || s.render(pair.Value) != "nil" || (s.render(pair.Key) != "Labels" && s.render(pair.Key) != "Annotations") {
				panic("unsupported MetaPatch initial map construction")
			}
			fields[strings.ToLower(s.render(pair.Key))] = true
		}
		if len(fields) != 2 {
			panic("missing source MetaPatch map initialization")
		}
		action["kind"], action["labels_nil"], action["annotations_nil"] = "new_patch", true, true
		return action
	}
	if selector, ok := left.(*ast.SelectorExpr); ok {
		receiver, field := metadataMapField(s, selector)
		literal, ok := right.(*ast.CompositeLit)
		if !ok || s.render(literal.Type) != "map[string]string" || assignment.Tok != token.ASSIGN {
			panic("unsupported source metadata map assignment")
		}
		entries := [][2]string{}
		for _, entry := range literal.Elts {
			pair, ok := entry.(*ast.KeyValueExpr)
			if !ok {
				panic("unsupported metadata map initializer entry")
			}
			entries = append(entries, [2]string{metadataString(s, pair.Key), metadataString(s, pair.Value)})
		}
		action["kind"], action["receiver"], action["field"], action["entries"] = "set_map", receiver, field, entries
		return action
	}
	if index, ok := left.(*ast.IndexExpr); ok {
		selector, ok := index.X.(*ast.SelectorExpr)
		if !ok || assignment.Tok != token.ASSIGN {
			panic("unsupported source metadata indexed mutation")
		}
		receiver, field := metadataMapField(s, selector)
		action["kind"], action["receiver"], action["field"] = "insert_map", receiver, field
		action["key"], action["value"] = metadataString(s, index.Index), metadataString(s, right)
		return action
	}
	panic("unsupported source MetaPatch state transition")
}

func metadataPatch(root string, s metadataSyntax) (map[string]interface{}, string) {
	qualified := "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate"
	program := strings.ReplaceAll(metadataCapturePrelude, "SOURCE_PACKAGE", s.pkg.PkgPath)
	var actions []map[string]interface{}
	assertions, validations := 0, 0
	for _, statement := range s.test.Body.List {
		position := s.pkg.Fset.Position(statement.Pos())
		action := metadataPatchAction(s, statement)
		ast.Inspect(statement, func(node ast.Node) bool {
			call, ok := node.(*ast.CallExpr)
			if !ok {
				return true
			}
			owner, name := metadataCallOwner(s, call)
			if owner == "github.com/stretchr/testify/assert" || owner == "github.com/stretchr/testify/require" {
				_ = metadataAssertion(s, call)
			} else if !((owner == s.pkg.PkgPath && (name == "Validate" || name == "DeepCopy")) || (owner == "testing" && name == "Parallel") || (owner == "k8s.io/apimachinery/pkg/util/validation/field" && name == "NewPath")) {
				panic("unmodeled MetaPatch source effect: " + owner + "." + name)
			}
			return true
		})
		program += s.render(statement) + "\n"
		if assignment, ok := statement.(*ast.AssignStmt); ok && len(assignment.Rhs) == 1 {
			if call, ok := assignment.Rhs[0].(*ast.CallExpr); ok {
				owner, name := metadataCallOwner(s, call)
				if owner == s.pkg.PkgPath && name == "Validate" {
					if len(assignment.Lhs) != 1 || s.render(assignment.Lhs[0]) != "allErrs" || len(call.Args) != 1 || s.render(call.Args[0]) != "path" {
						panic("unsupported MetaPatch validation assignment")
					}
					receiver := s.render(call.Fun.(*ast.SelectorExpr).X)
					if receiver != "mp" && receiver != "invalid" {
						panic("unsupported MetaPatch validation receiver")
					}
					action["kind"], action["receiver"] = "validate", receiver
					program += fmt.Sprintf("rows=append(rows,map[string]interface{}{\"source_line\":%d,\"labels\":%s.Labels,\"annotations\":%s.Annotations,\"labels_nil\":%s.Labels==nil,\"annotations_nil\":%s.Annotations==nil,\"observed_errors\":allErrs,\"assertions\":[]map[string]interface{}{}})\n", position.Line, receiver, receiver, receiver, receiver)
					// Freeze maps now, before later source mutations. JSON round-trip
					// snapshots observations; it does not supply assertion expectations.
					program += "{snapshot,e:=json.Marshal(rows[len(rows)-1]);if e!=nil{panic(e)};var frozen map[string]interface{};if e=json.Unmarshal(snapshot,&frozen);e!=nil{panic(e)};frozen[\"assertions\"]=[]map[string]interface{}{};rows[len(rows)-1]=frozen}\n"
					validations++
				}
			}
		}
		if expression, ok := statement.(*ast.ExprStmt); ok {
			if call, ok := expression.X.(*ast.CallExpr); ok {
				if site := metadataAssertion(s, call); site != nil {
					site["id"] = fmt.Sprintf("%s::line-%d", qualified, site["line"])
					action["kind"], action["assertion"] = "assertion", site
					encoded, err := json.Marshal(site)
					must(err)
					program += "{var site map[string]interface{};if e:=json.Unmarshal([]byte(" + strconv.Quote(string(encoded)) + "),&site);e!=nil{panic(e)};row:=rows[len(rows)-1];row[\"assertions\"]=append(row[\"assertions\"].([]map[string]interface{}),site)}\n"
					assertions++
				}
			}
		}
		actions = append(actions, action)
	}
	if validations != 6 || assertions != 10 {
		panic("MetaPatch complete action/assertion accounting changed")
	}
	program += "data,err:=json.Marshal(rows);if err!=nil{panic(err)};fmt.Println(string(data))}"
	rows := metadataCapture(root, program)
	if len(rows) != validations {
		panic("missing MetaPatch validation observation")
	}
	return map[string]interface{}{"id": qualified, "test": "TestMetaPatchValidate", "package_id": s.pkg.ID, "source_body": s.render(s.test.Body), "source_actions": actions, "validation_steps": rows, "source_assertion_occurrences": assertions, "source_assertions_passed": true}, program
}

func metadataSourceBindings(root, baseline string, roots ...*packages.Package) map[string]string {
	paths := map[string]bool{"go.mod": true, "go.sum": true, "vendor/modules.txt": true}
	seen := map[string]bool{}
	var visit func(*packages.Package)
	visit = func(pkg *packages.Package) {
		if seen[pkg.ID] {
			return
		}
		seen[pkg.ID] = true
		for _, path := range pkg.CompiledGoFiles {
			relative, err := filepath.Rel(root, path)
			must(err)
			if !strings.HasPrefix(relative, "..") {
				paths[filepath.ToSlash(relative)] = true
			}
		}
		for _, dependency := range pkg.Imports {
			visit(dependency)
		}
	}
	for _, pkg := range roots {
		visit(pkg)
	}
	ordered := []string{}
	for path := range paths {
		ordered = append(ordered, path)
	}
	sort.Strings(ordered)
	result := map[string]string{}
	for _, path := range ordered {
		current, err := os.ReadFile(filepath.Join(root, path))
		must(err)
		original, err := exec.Command("git", "-C", root, "show", baseline+":"+path).Output()
		must(err)
		if hash(current) != hash(original) {
			panic("changed original metadata dependency: " + path)
		}
		result[path] = hash(current)
	}
	return result
}

func objectMetadataMain(args []string) {
	if len(args) != 2 {
		panic("metadata extraction requires root and the completed metadata implementation gate")
	}
	root, err := filepath.Abs(args[0])
	must(err)
	gateBytes, err := os.ReadFile(filepath.Join(root, args[1]))
	must(err)
	var gate Gate
	must(json.Unmarshal(gateBytes, &gate))
	if gate.FeatureID != "META-VALIDATE-001" {
		panic("metadata extraction requires its own feature gate")
	}
	must(validateGate(root, gate))
	agones := loadTypedTests(root, "./pkg/apis/agones/v1", "pkg/apis/agones/v1 [")
	allocation := loadTypedTests(root, "./pkg/apis/allocation/v1", "pkg/apis/allocation/v1 [")
	// Close and check the original transitive repository source before executing
	// any replay. Standard-library identity is retained by the restored tool archive.
	bindings := metadataSourceBindings(root, gate.Baseline, agones, allocation)
	overflow, overflowProgram := metadataOverflow(root, metadataTest(agones, "TestAllocationOverflowValidate"))
	patch, patchProgram := metadataPatch(root, metadataTest(allocation, "TestMetaPatchValidate"))
	result := map[string]interface{}{"schema_version": 1, "feature_id": "META-VALIDATE-001", "classification": "EXACT_CASE_AND_WHOLE_TEST_ASSERTIONS", "implementation_gate": args[1], "implementation_gate_sha256": hash(gateBytes), "source_files": bindings, "loader_source_files": loaderSourceBindings(root), "environment": loaderEnvironment(root), "runtime_version": runtime.Version(), "overflow": overflow, "metadata_patch": patch, "case_count": 6, "validation_observations": 11, "source_assertion_occurrences": 21, "capture_programs": []map[string]interface{}{{"family": "AllocationOverflow", "source": overflowProgram, "sha256": hash([]byte(overflowProgram))}, {"family": "MetaPatch", "source": patchProgram, "sha256": hash([]byte(patchProgram))}}, "scope": "All five pinned AllocationOverflow validation fixtures and the complete MetaPatch test retain every source action and all21executed qualified assertion occurrences. Existing GameServer/Fleet/GameServerSet source families remain required independently. Typed IR/reference replay only; no Rust property translation or backend proof credit.", "scheduling_mapping": "The source map iteration order is permitted nondeterminism; captured overflow rows are ordered by source fixture declaration. MetaPatch t.Parallel maps to serial isolated-instance replay. No executor/concurrency credit.", "support": "Package/type-resolved original Validate, DeepCopy, field.NewPath and testify Empty/Len/Equal/require.Len; unsupported effects/assertion shapes fail closed. Original expected fields/lengths/types remain independent from observed complete error records. Map snapshots freeze the original nil/empty distinction and every validation state.", "tool_provenance": "verification/agones/go-runtime-restoration-table.json"}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	must(encoder.Encode(result))
}
