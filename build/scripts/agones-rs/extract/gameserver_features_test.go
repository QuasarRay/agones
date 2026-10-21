package main

import "testing"

func TestFeatureValidationBodyRejectsDroppedChangedAndNewEffects(t *testing.T) {
	body := []string{"err := runtime.ParseFeatures(tc.feature)", "assert.NoError(t, err)", "errs := tc.gs.Validate(fakeAPIHooks{})", `assert.ElementsMatch(t, tc.want, errs, "ErrorList check")`}
	if !exactFeatureValidationBody(body) {
		t.Fatal("exact source body rejected")
	}
	for index := range body {
		changed := append([]string{}, body...)
		changed[index] = "unknown.NewEffect()"
		if exactFeatureValidationBody(changed) {
			t.Fatal("unmodeled source effect accepted")
		}
	}
	if exactFeatureValidationBody(body[:3]) || exactFeatureValidationBody(append(body, "assert.True(t, true)")) {
		t.Fatal("missing/new original assertion accepted")
	}
}
