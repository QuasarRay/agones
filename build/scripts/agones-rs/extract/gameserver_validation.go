package main

// The complete ordinary validation table has its own exact action contract;
// share loading/capture with the feature table without dropping defaulting.
func exactGeneralValidationBody(statements []string) bool {
	want := []string{
		"if tc.applyDefaults {\n\ttc.gs.ApplyDefaults()\n}",
		"errs := tc.gs.Validate(fakeAPIHooks{})",
		`assert.ElementsMatch(t, tc.want, errs, "ErrorList check")`,
	}
	if len(statements) != len(want) {
		return false
	}
	for index := range want {
		if statements[index] != want[index] {
			return false
		}
	}
	return true
}
