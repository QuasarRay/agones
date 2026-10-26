#[cfg(kani)]
mod common_harness {
    use std::collections::BTreeMap;

    use super::*;

    fn check_case(
        allocation_overflow: AllocationOverflow,
        expected_fields: &[&str],
    ) {
        let errors = allocation_overflow.validate("spec.allocationOverflow");

        // Equivalent to:
        // assert.Len(t, errs, len(v.fields))
        assert_eq!(errors.len(), expected_fields.len());

        for (error, expected_field) in
            errors.iter().zip(expected_fields.iter())
        {
            // Equivalent to:
            // assert.Equal(t, field.ErrorTypeInvalid, err.Type)
            assert_eq!(error.kind, ValidationErrorKind::Invalid);

            // Equivalent to:
            // assert.Equal(t, v.expected.fields[i], err.Field)
            assert_eq!(error.path, *expected_field);
        }
    }

    #[kani::proof]
    fn allocation_overflow_validate() {
        // "empty"
        check_case(
            AllocationOverflow::default(),
            &[],
        );

        // "bad label name"
        check_case(
            AllocationOverflow {
                labels: BTreeMap::from([
                    ("$$$foobar".into(), "stuff".into()),
                ]),
                annotations: BTreeMap::new(),
            },
            &[
                "spec.allocationOverflow.labels",
            ],
        );

        // "bad label value"
        check_case(
            AllocationOverflow {
                labels: BTreeMap::from([
                    ("valid".into(), "$$$NOPE".into()),
                ]),
                annotations: BTreeMap::new(),
            },
            &[
                "spec.allocationOverflow.labels",
            ],
        );

        // "bad annotation name"
        check_case(
            AllocationOverflow {
                labels: BTreeMap::new(),
                annotations: BTreeMap::from([
                    ("$$$foobar".into(), "stuff".into()),
                ]),
            },
            &[
                "spec.allocationOverflow.annotations",
            ],
        );

        // "valid full"
        check_case(
            AllocationOverflow {
                labels: BTreeMap::from([
                    ("valid".into(), "yes".into()),
                    (
                        "still.valid".into(),
                        "check-me-out".into(),
                    ),
                ]),
                annotations: BTreeMap::from([
                    (
                        "icando-this".into(),
                        "yes, I can do all kinds of things here $$$".into(),
                    ),
                ]),
            },
            &[],
        );
    }
}