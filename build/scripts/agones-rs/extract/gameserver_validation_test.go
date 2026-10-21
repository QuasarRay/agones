package main

import "testing"

func TestGeneralValidationBodyRetainsEveryConditionalAndAssertion(t *testing.T) {
	body := []string{"if tc.applyDefaults {\n\ttc.gs.ApplyDefaults()\n}", "errs := tc.gs.Validate(fakeAPIHooks{})", `assert.ElementsMatch(t, tc.want, errs, "ErrorList check")`}
	if !exactGeneralValidationBody(body) {
		t.Fatal("exact general validation body rejected")
	}
	for index := range body {
		changed := append([]string{}, body...)
		changed[index] = "unknown.NewEffect()"
		if exactGeneralValidationBody(changed) {
			t.Fatal("changed source effect accepted")
		}
	}
	if exactGeneralValidationBody(body[1:]) || exactGeneralValidationBody(append(body, "assert.True(t,true)")) {
		t.Fatal("dropped/new source effect accepted")
	}
}
