// Translate only the gated, package-resolved feature-gate test family.
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/constant"
	"go/printer"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strings"
)

type Step struct {
	Kind    string `json:"kind"`
	Key     string `json:"key,omitempty"`
	Value   string `json:"value,omitempty"`
	Enabled bool   `json:"enabled"`
	Origin  string `json:"origin"`
}

type Case struct {
	ID             string `json:"id"`
	Test           string `json:"test"`
	Name           string `json:"name"`
	Disposition    string `json:"disposition"`
	Reason         string `json:"reason,omitempty"`
	Source         string `json:"source"`
	Steps          []Step `json:"steps"`
	AssertionCount int    `json:"assertion_count"`
}

type Default struct {
	Name    string `json:"name"`
	Enabled bool   `json:"enabled"`
}
type Gate struct {
	Status              string                        `json:"status"`
	Baseline            string                        `json:"baseline"`
	FeatureID           string                        `json:"feature_id"`
	ProductionFiles     map[string]string             `json:"production_files"`
	EnvironmentBindings map[string]EnvironmentBinding `json:"environment_bindings"`
}

type EnvironmentBinding struct {
	Present bool    `json:"present"`
	SHA256  *string `json:"sha256"`
}

func hash(source []byte) string { sum := sha256.Sum256(source); return hex.EncodeToString(sum[:]) }
func must(err error) {
	if err != nil {
		panic(err)
	}
}

func validateGate(root string, gate Gate) error {
	if gate.Status != "IMPLEMENTED" || len(gate.ProductionFiles) == 0 {
		return fmt.Errorf("extraction requires a nonempty implementation gate")
	}
	for name, binding := range gate.EnvironmentBindings {
		value, present := os.LookupEnv(name)
		if present != binding.Present || (present && (binding.SHA256 == nil || hash([]byte(value)) != *binding.SHA256)) || (!present && binding.SHA256 != nil) {
			return fmt.Errorf("stale gate environment: %s", name)
		}
	}
	for path, expected := range gate.ProductionFiles {
		source, err := os.ReadFile(filepath.Join(root, path))
		if err != nil {
			return err
		}
		if hash(source) != expected {
			return fmt.Errorf("stale gate: %s", path)
		}
	}
	return nil
}

func validateInputGate(root string, gate Gate) error {
	if gate.FeatureID != "feature-gates-environment-cli-binding" {
		return fmt.Errorf("environment extraction requires the input feature's gate")
	}
	return validateGate(root, gate)
}

func sourceCaseIdentity(base string, line int, seen map[string]bool) string {
	id := base
	if seen[id] {
		id += fmt.Sprintf("::source-line-%d", line)
	}
	if seen[id] {
		panic("unresolvable duplicate source identity")
	}
	seen[id] = true
	return id
}

func supportedObservation(name string) bool {
	switch name {
	case "agones.dev/agones/pkg/util/runtime.FeatureEnabled", "agones.dev/agones/pkg/util/runtime.EncodeFeatures", "agones.dev/agones/pkg/util/runtime.Feature", "testing.Parallel", "sync.Lock", "sync.Unlock":
		return true
	}
	return false
}

func main() {
	if len(os.Args) > 1 && os.Args[1] == "--gameserver-features" {
		gameServerFeaturesMain(os.Args[2:])
		return
	}
	if len(os.Args) > 1 && os.Args[1] == "--gameserver-defaulting" {
		gameServerDefaultingMain(os.Args[2:])
		return
	}
	if len(os.Args) > 1 && os.Args[1] == "--sdk-version" {
		sdkVersionMain(os.Args[2:])
		return
	}
	root, err := filepath.Abs(os.Args[1])
	must(err)
	gatePath := os.Args[2]
	gateBytes, err := os.ReadFile(filepath.Join(root, gatePath))
	must(err)
	var gate Gate
	must(json.Unmarshal(gateBytes, &gate))
	must(validateGate(root, gate))
	inputGatePath := ""
	var inputGateBytes []byte
	if len(os.Args) > 3 {
		inputGatePath = os.Args[3]
		inputGateBytes, err = os.ReadFile(filepath.Join(root, inputGatePath))
		must(err)
		var inputGate Gate
		must(json.Unmarshal(inputGateBytes, &inputGate))
		must(validateInputGate(root, inputGate))
		if inputGate.Baseline != gate.Baseline {
			panic("query/input gate baseline mismatch")
		}
	}
	sourcePaths := []string{"pkg/util/runtime/features_test.go", "pkg/util/runtime/features.go", "pkg/util/runtime/runtime.go", "pkg/util/errors/errors.go", "go.mod", "go.sum"}
	if inputGatePath != "" {
		sourcePaths = append(sourcePaths, "vendor/github.com/spf13/viper/viper.go", "vendor/github.com/spf13/pflag/flag.go", "cmd/controller/main.go", "cmd/sdk-server/main.go")
	}
	for _, path := range sourcePaths {
		original, err := exec.Command("git", "-C", root, "show", gate.Baseline+":"+path).Output()
		must(err)
		current, err := os.ReadFile(filepath.Join(root, path))
		must(err)
		if hash(original) != hash(current) {
			panic("original source/config changed without scope revision: " + path)
		}
	}
	// Loading is against the original root module, including its exact vendor,
	// replacements, external test package variants, imports and assertion types.
	pkg := loadTypedTests(root, "./pkg/util/runtime", "pkg/util/runtime [")
	path := "pkg/util/runtime/features_test.go"
	var file *ast.File
	for _, candidate := range pkg.Syntax {
		if filepath.Clean(pkg.Fset.Position(candidate.Pos()).Filename) == filepath.Join(root, path) {
			file = candidate
			break
		}
	}
	if file == nil {
		panic("test source absent from typed package")
	}
	text := func(node ast.Node) string {
		var b bytes.Buffer
		must(printer.Fprint(&b, pkg.Fset, node))
		return b.String()
	}
	value := func(expr ast.Expr) string {
		tv, ok := pkg.TypesInfo.Types[expr]
		if !ok || tv.Value == nil || tv.Value.Kind() != constant.String {
			panic("unsupported nonconstant string: " + text(expr))
		}
		return constant.StringVal(tv.Value)
	}
	callName := func(call *ast.CallExpr) string {
		var id *ast.Ident
		switch fn := call.Fun.(type) {
		case *ast.Ident:
			id = fn
		case *ast.SelectorExpr:
			id = fn.Sel
		default:
			return ""
		}
		obj := pkg.TypesInfo.Uses[id]
		if obj == nil || obj.Pkg() == nil {
			return ""
		}
		return obj.Pkg().Path() + "." + obj.Name()
	}
	inputAction := func(call *ast.CallExpr, origin string) (Step, bool) {
		step := Step{Origin: origin}
		switch callName(call) {
		case "os.Setenv":
			if len(call.Args) != 2 || value(call.Args[0]) != "FEATURE_GATES" {
				panic("unsupported environment mutation: " + text(call))
			}
			step.Kind, step.Key, step.Value = "SET_ENV", value(call.Args[0]), value(call.Args[1])
		case "agones.dev/agones/pkg/util/runtime.FeaturesBindFlags":
			step.Kind = "BIND_FLAGS"
		case "github.com/spf13/pflag.Parse":
			step.Kind = "PARSE_FLAGS"
		case "agones.dev/agones/pkg/util/runtime.FeaturesBindEnv":
			step.Kind, step.Key, step.Value = "BIND_ENV", "feature-gates", "FEATURE_GATES"
		case "agones.dev/agones/pkg/util/runtime.ParseFeaturesFromEnv":
			step.Kind = "PARSE_ENV"
		case "github.com/spf13/viper.SetEnvKeyReplacer":
			if len(call.Args) != 1 {
				panic("unsupported environment key binding")
			}
			replacer, ok := call.Args[0].(*ast.CallExpr)
			if !ok || callName(replacer) != "strings.NewReplacer" || len(replacer.Args) != 2 || value(replacer.Args[0]) != "-" || value(replacer.Args[1]) != "_" {
				panic("unsupported environment key replacer: " + text(call))
			}
			step.Kind, step.Key, step.Value = "SET_ENV_KEY_REPLACER", "-", "_"
		default:
			return step, false
		}
		if step.Kind != "SET_ENV" && step.Kind != "SET_ENV_KEY_REPLACER" && len(call.Args) != 0 {
			panic("unsupported input action arguments: " + text(call))
		}
		return step, true
	}
	defaultRows := []Default{}
	cases := []Case{}
	identities := map[string]bool{}
	rootCount := 0
	for _, decl := range file.Decls {
		fn, ok := decl.(*ast.FuncDecl)
		if !ok || (fn.Name.Name != "TestFeatures" && fn.Name.Name != "TestEnableAllFeatures") {
			continue
		}
		rootCount++
		var testDefaults []Default
		for _, statement := range fn.Body.List {
			assign, ok := statement.(*ast.AssignStmt)
			if !ok {
				continue
			}
			for i, left := range assign.Lhs {
				id, ok := left.(*ast.Ident)
				if !ok || id.Name != "featureDefaults" {
					continue
				}
				literal, ok := assign.Rhs[i].(*ast.CompositeLit)
				if !ok {
					continue
				}
				for _, element := range literal.Elts {
					pair := element.(*ast.KeyValueExpr)
					tv := pkg.TypesInfo.Types[pair.Value]
					testDefaults = append(testDefaults, Default{Name: value(pair.Key), Enabled: constant.BoolVal(tv.Value)})
				}
			}
		}
		sort.Slice(testDefaults, func(i, j int) bool { return testDefaults[i].Name < testDefaults[j].Name })
		if len(testDefaults) != 2 || testDefaults[0].Name != "Example" || !testDefaults[0].Enabled || testDefaults[1].Name != "Test" || testDefaults[1].Enabled {
			panic("source fixture changed; revisit mapping")
		}
		defaultRows = testDefaults
		translate := func(name string, body *ast.BlockStmt) {
			c := Case{ID: pkg.PkgPath + "::" + path + "::" + fn.Name.Name + "::" + name, Test: fn.Name.Name, Name: name, Disposition: "READY", Source: text(body), Steps: []Step{}}
			c.ID = sourceCaseIdentity(c.ID, pkg.Fset.Position(body.Pos()).Line, identities)
			if name == "parse env vars" && inputGatePath == "" {
				c.Disposition = "UNSUPPORTED"
				c.Reason = "Viper/pflag/environment effects and process-global input binding remain FG-INPUT-001; retain all source assertions without credit."
			}
			ast.Inspect(body, func(n ast.Node) bool {
				call, ok := n.(*ast.CallExpr)
				if !ok {
					return true
				}
				name := callName(call)
				line := pkg.Fset.Position(call.Pos()).Line
				origin := fmt.Sprintf("%s::line-%d", c.ID, line)
				step := Step{Origin: origin}
				if inputGatePath != "" {
					if action, ok := inputAction(call, origin); ok {
						c.Steps = append(c.Steps, action)
						return false
					}
				}
				switch name {
				case "agones.dev/agones/pkg/util/runtime.ParseFeatures":
					if nested, ok := call.Args[0].(*ast.CallExpr); ok && callName(nested) == "agones.dev/agones/pkg/util/runtime.EncodeFeatures" {
						step.Kind = "PARSE_ENCODED"
					} else {
						step.Kind = "PARSE"
						step.Value = value(call.Args[0])
					}
				case "agones.dev/agones/pkg/util/runtime.EnableAllFeatures":
					step.Kind = "ENABLE_ALL"
				case "github.com/stretchr/testify/assert.NoError":
					if inputGatePath != "" {
						if nested, ok := call.Args[1].(*ast.CallExpr); ok {
							action, supported := inputAction(nested, fmt.Sprintf("%s::line-%d", c.ID, pkg.Fset.Position(nested.Pos()).Line))
							if !supported {
								panic("unsupported effect inside NoError: " + text(nested))
							}
							// Go evaluates the effect argument before the assertion.
							c.Steps = append(c.Steps, action, Step{Kind: "ASSERT_NO_ERROR", Origin: origin})
							c.AssertionCount++
							return false
						}
					}
					step.Kind = "ASSERT_NO_ERROR"
					c.AssertionCount++
				case "github.com/stretchr/testify/assert.Error":
					step.Kind = "ASSERT_ERROR"
					c.AssertionCount++
				case "github.com/stretchr/testify/assert.ErrorContains":
					step.Kind = "ASSERT_ERROR_CONTAINS"
					step.Value = value(call.Args[2])
					c.AssertionCount++
				case "github.com/stretchr/testify/assert.True", "github.com/stretchr/testify/assert.False":
					inner, ok := call.Args[1].(*ast.CallExpr)
					if !ok || callName(inner) != "agones.dev/agones/pkg/util/runtime.FeatureEnabled" {
						panic("unsupported boolean assertion: " + text(call))
					}
					step.Kind = "ASSERT_ENABLED"
					step.Value = value(inner.Args[0])
					step.Enabled = strings.HasSuffix(name, ".True")
					c.AssertionCount++
				case "github.com/stretchr/testify/assert.Equal":
					inner, ok := call.Args[2].(*ast.CallExpr)
					if !ok || callName(inner) != "agones.dev/agones/pkg/util/runtime.EncodeFeatures" {
						panic("unsupported equality assertion: " + text(call))
					}
					step.Kind = "ASSERT_ENCODING"
					step.Value = value(call.Args[1])
					c.AssertionCount++
				}
				if step.Kind != "" {
					c.Steps = append(c.Steps, step)
				} else if c.Disposition == "READY" {
					if !supportedObservation(name) {
						panic("unsupported semantic call: " + text(call))
					}
				}
				return true
			})
			if c.AssertionCount == 0 {
				panic("unexpected empty assertion selection")
			}
			cases = append(cases, c)
		}
		if fn.Name.Name == "TestEnableAllFeatures" {
			translate("$root", fn.Body)
			continue
		}
		for _, statement := range fn.Body.List {
			expr, ok := statement.(*ast.ExprStmt)
			if !ok {
				continue
			}
			call, ok := expr.X.(*ast.CallExpr)
			if !ok {
				continue
			}
			sel, ok := call.Fun.(*ast.SelectorExpr)
			if !ok || sel.Sel.Name != "Run" {
				continue
			}
			translate(value(call.Args[0]), call.Args[1].(*ast.FuncLit).Body)
		}
	}
	if rootCount != 2 || len(cases) != 6 {
		panic("unexpected source test selection")
	}
	tests, err := os.ReadFile(filepath.Join(root, path))
	must(err)
	helperPath := "pkg/util/runtime/features.go"
	helper, err := os.ReadFile(filepath.Join(root, helperPath))
	must(err)
	result := map[string]interface{}{"schema_version": 1, "feature_id": "FG-CONFIG-001", "implementation_gate": gatePath, "implementation_gate_sha256": hash(gateBytes), "classification": "EXACT_CASE", "loader": "golang.org/x/tools/go/packages v0.48.0", "package_id": pkg.ID, "source_files": map[string]string{path: hash(tests), helperPath: hash(helper)}, "defaults": defaultRows, "cases": cases, "environment": map[string]string{"GOFLAGS": "-mod=vendor", "GOTOOLCHAIN": "local", "GOOS": runtime.GOOS, "GOARCH": runtime.GOARCH, "GOVERSION": runtime.Version()}, "support": "Package/type-resolved ParseFeatures, EncodeFeatures, FeatureEnabled, EnableAllFeatures and testify assertions; exact Example/Test fixture; source t.Parallel/global-map isolation maps to instance-local Rust contexts. Unknown test selection, fixture or assertion shapes fail closed. Environment case remains unsupported; no dynamic table normalization.", "source_mapping_note": "Same root package owns both tests. Exact assertion identities use source-bound line spans. No test/helper/fixture exclusion is implied by this focused selection."}
	result["loader_source_files"] = loaderSourceBindings(root)
	result["environment"] = loaderEnvironment(root)
	if inputGatePath != "" {
		result["input_implementation_gate"] = inputGatePath
		result["input_implementation_gate_sha256"] = hash(inputGateBytes)
		bindings := result["source_files"].(map[string]string)
		for _, sourcePath := range sourcePaths {
			source, err := os.ReadFile(filepath.Join(root, sourcePath))
			must(err)
			bindings[sourcePath] = hash(source)
		}
		result["support"] = "The gated query family plus the gated exact input case: environment setup, flag registration/parse, key replacement, binding and input parsing in source evaluation order. The Rust graph statically binds the constant feature key; command parser and OS effects are explicit trusted/model boundaries, validated separately with real process replay. Unknown calls, input shapes and argument replacements fail closed."
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	must(encoder.Encode(result))
}
