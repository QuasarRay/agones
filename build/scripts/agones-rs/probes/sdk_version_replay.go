// Ordinary implementation witnesses; no source-test semantic extraction.
package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"

	"agones.dev/agones/pkg"
	"agones.dev/agones/pkg/apis"
	v1 "agones.dev/agones/pkg/apis/agones/v1"
	corev1 "k8s.io/api/core/v1"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/util/validation/field"
)

// Local deterministic product-hook effect, not a cluster/orchestrator adapter.
type hooks struct{}

type observingHooks struct {
	hooks
	expected string
}

func (h observingHooks) MutateGameServerPod(_ *v1.GameServerSpec, pod *corev1.Pod) error {
	if annotation(pod) != h.expected {
		panic("version not stamped before product hook")
	}
	pod.Annotations[v1.VersionAnnotation] = "product-hook-version"
	return nil
}

func (hooks) ValidateGameServerSpec(*v1.GameServerSpec, *field.Path) field.ErrorList  { return nil }
func (hooks) ValidateScheduling(apis.SchedulingStrategy, *field.Path) field.ErrorList { return nil }
func (hooks) MutateGameServerPod(*v1.GameServerSpec, *corev1.Pod) error               { return nil }
func (hooks) SetEviction(*v1.Eviction, *corev1.Pod) error                             { return nil }

func fixture() *v1.GameServer {
	return &v1.GameServer{
		ObjectMeta: metav1.ObjectMeta{Name: "version.server", Namespace: "default", Annotations: map[string]string{v1.VersionAnnotation: "parent-version", "keep": "gs"}},
		Spec: v1.GameServerSpec{Container: "game", Template: corev1.PodTemplateSpec{
			ObjectMeta: metav1.ObjectMeta{Annotations: map[string]string{v1.VersionAnnotation: "template-version", "keep": "template"}},
			Spec:       corev1.PodSpec{Containers: []corev1.Container{{Name: "game", Image: "game:v1"}}},
		}},
	}
}

func annotation(object interface{}) string {
	data, err := json.Marshal(object)
	if err != nil {
		panic(err)
	}
	var decoded struct {
		Metadata struct {
			Annotations map[string]string `json:"annotations"`
		} `json:"metadata"`
	}
	if err = json.Unmarshal(data, &decoded); err != nil {
		panic(err)
	}
	value, present := decoded.Metadata.Annotations[v1.VersionAnnotation]
	if !present {
		panic("missing source SDK annotation, including explicit empty values")
	}
	return value
}

func main() {
	if os.Args[1] == "--compiled" {
		gs := fixture()
		gs.ApplyDefaults()
		fleet := &v1.Fleet{}
		fleet.ApplyDefaults()
		pod, err := fixture().Pod(hooks{})
		if err != nil {
			panic(err)
		}
		output, err := json.Marshal(map[string]string{
			"raw_hex":  hex.EncodeToString([]byte(pkg.Version)),
			"constant": pkg.Version, "provider": pkg.Version,
			"game_server": annotation(gs), "fleet": annotation(fleet), "pod": annotation(pod),
		})
		if err != nil {
			panic(err)
		}
		fmt.Println(string(output))
		return
	}
	data, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	var family struct {
		SchemaVersion int `json:"schema_version"`
		Cases         []struct {
			ID         string `json:"id"`
			VersionHex string `json:"version_hex"`
			Expected   string `json:"expected_annotation"`
		} `json:"cases"`
	}
	if err = json.Unmarshal(data, &family); err != nil {
		panic(err)
	}
	if family.SchemaVersion != 1 || len(family.Cases) != 10 {
		panic("unexpected ordinary witness selection")
	}
	observations := []map[string]string{}
	for _, c := range family.Cases {
		bytes, err := hex.DecodeString(c.VersionHex)
		if err != nil {
			panic(err)
		}
		pkg.Version = string(bytes)
		gs := fixture()
		gs.ApplyDefaults()
		fleet := &v1.Fleet{ObjectMeta: metav1.ObjectMeta{Annotations: map[string]string{v1.VersionAnnotation: "old", "keep": "fleet"}}}
		fleet.ApplyDefaults()
		parent := fixture()
		pod, err := parent.Pod(hooks{})
		if err != nil {
			panic(err)
		}
		hooked, err := parent.Pod(observingHooks{expected: c.Expected})
		if err != nil {
			panic(err)
		}
		observed := map[string]string{"id": c.ID, "game_server": annotation(gs), "fleet": annotation(fleet), "pod": annotation(pod), "hooked_pod": annotation(hooked)}
		if observed["hooked_pod"] != "product-hook-version" {
			panic("source product hook override lost")
		}
		for _, kind := range []string{"game_server", "fleet", "pod"} {
			if observed[kind] != c.Expected {
				panic(fmt.Sprintf("%s %s: expected %q, got %q", c.ID, kind, c.Expected, observed[kind]))
			}
		}
		if gs.Annotations["keep"] != "gs" || fleet.Annotations["keep"] != "fleet" || pod.Annotations["keep"] != "template" || parent.Annotations[v1.VersionAnnotation] != "parent-version" || parent.Spec.Template.Annotations[v1.VersionAnnotation] != "template-version" {
			panic("unrelated or parent metadata changed")
		}
		observations = append(observations, observed)
	}
	output, err := json.MarshalIndent(observations, "", "  ")
	if err != nil {
		panic(err)
	}
	fmt.Println(string(output))
}
