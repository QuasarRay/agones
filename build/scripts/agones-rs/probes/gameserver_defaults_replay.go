// Ordinary implementation witnesses, before source-test semantic extraction.
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"reflect"

	v1 "agones.dev/agones/pkg/apis/agones/v1"
	"agones.dev/agones/pkg/util/runtime"
)

func main() {
	data, err := os.ReadFile("verification/agones/gameserver-defaulting-witnesses.json")
	if err != nil {
		panic(err)
	}
	var fixture struct {
		Cases []struct {
			Name         string                 `json:"name"`
			Input        json.RawMessage        `json:"input"`
			FeatureFlags string                 `json:"feature_flags"`
			Expected     map[string]interface{} `json:"expected"`
			Mutate       bool                   `json:"mutate_spec_after_defaults"`
			Iterations   int                    `json:"iterations"`
		}
	}
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.UseNumber()
	if err := decoder.Decode(&fixture); err != nil {
		panic(err)
	}
	runtime.FeatureTestMutex.Lock()
	defer runtime.FeatureTestMutex.Unlock()
	output := []map[string]interface{}{}
	for _, test := range fixture.Cases {
		var gs v1.GameServer
		if err := json.Unmarshal(test.Input, &gs); err != nil {
			panic(err)
		}
		if err := runtime.ParseFeatures(test.FeatureFlags); err != nil {
			panic(err)
		}
		for i := 0; i < test.Iterations; i++ {
			gs.ApplyDefaults()
		}
		if test.Mutate {
			counter := gs.Spec.Counters["n"]
			counter.Count = 99
			gs.Spec.Counters["n"] = counter
			list := gs.Spec.Lists["l"]
			list.Values[0] = "changed"
			gs.Spec.Lists["l"] = list
		}
		policy, protocol := "", ""
		if len(gs.Spec.Ports) > 0 {
			policy = string(gs.Spec.Ports[0].PortPolicy)
			protocol = string(gs.Spec.Ports[0].Protocol)
		}
		specSafe, statusSafe := "", ""
		if gs.Spec.Eviction != nil {
			specSafe = string(gs.Spec.Eviction.Safe)
		}
		if gs.Status.Eviction != nil {
			statusSafe = string(gs.Status.Eviction.Safe)
		}
		observed := map[string]interface{}{
			"state": gs.Status.State, "container": gs.Spec.Container, "scheduling": gs.Spec.Scheduling,
			"port_policy": policy, "protocol": protocol, "health": gs.Spec.Health, "sdk": gs.Spec.SdkServer,
			"eviction_spec": specSafe, "eviction_status": statusSafe, "players": gs.Status.Players,
			"counters": gs.Status.Counters, "lists": gs.Status.Lists,
			"counters_nil": gs.Status.Counters == nil, "lists_nil": gs.Status.Lists == nil,
			"finalizers": gs.Finalizers, "version": gs.Annotations[v1.VersionAnnotation],
		}
		encoded, err := json.Marshal(observed)
		if err != nil {
			panic(err)
		}
		var normalized map[string]interface{}
		d := json.NewDecoder(bytes.NewReader(encoded))
		d.UseNumber()
		if err := d.Decode(&normalized); err != nil {
			panic(err)
		}
		if !reflect.DeepEqual(normalized, test.Expected) {
			panic(fmt.Sprintf("%s: source violates independent literal expectations: %s", test.Name, encoded))
		}
		output = append(output, map[string]interface{}{"name": test.Name, "observed": normalized, "object": gs})
	}
	if len(output) != 14 {
		panic("empty/incomplete implementation witness selection")
	}
	encoded, err := json.MarshalIndent(output, "", "  ")
	if err != nil {
		panic(err)
	}
	fmt.Println(string(encoded))
}
