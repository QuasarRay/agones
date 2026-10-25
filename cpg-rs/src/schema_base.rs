//! Rust equivalent of `io.shiftleft.codepropertygraph.schema.SchemaBase`.
//!
//! The trait models the typed Rust API. The validator models the equivalent
//! JSON object contract when schema metadata crosses a dynamic boundary.

use serde_json::Value;

/// Common metadata exposed by CPG schema definitions.
pub trait SchemaBase {
    /// Ordering key used by generated documentation.
    ///
    /// Smaller values are shown first.
    fn doc_index(&self) -> i32;

    /// Human-readable description of this schema.
    fn description(&self) -> &str;

    /// Whether this schema is supplied by a frontend.
    ///
    /// Matches the Scala default: `false`.
    fn provided_by_frontend(&self) -> bool {
        false
    }
}

/// Compile-time JSON Schema validator for the dynamic representation of
/// `SchemaBase`.
///
/// Additional properties are intentionally allowed: concrete schema types may
/// expose metadata beyond the three members defined by `SchemaBase`.
#[jsonschema::validator(
    schema = r#"{
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {
            "docIndex": {
                "type": "integer",
                "minimum": -2147483648,
                "maximum": 2147483647
            },
            "description": {
                "type": "string"
            },
            "providedByFrontend": {
                "type": "boolean",
                "default": false
            }
        },
        "required": ["docIndex", "description"]
    }"#,
    draft = Draft202012
)]
pub struct SchemaBaseValidator;

/// Validate a dynamic JSON representation of `SchemaBase`.
#[inline]
pub fn validate_schema_base_json(
    value: &Value,
) -> Result<(), jsonschema::ValidationError<'_>> {
    SchemaBaseValidator::validate(value)
}

/// Fast validity check for a dynamic JSON representation of `SchemaBase`.
#[inline]
pub fn is_valid_schema_base_json(value: &Value) -> bool {
    SchemaBaseValidator::is_valid(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct Example;

    impl SchemaBase for Example {
        fn doc_index(&self) -> i32 {
            7
        }

        fn description(&self) -> &str {
            "example schema"
        }
    }

    #[test]
    fn trait_preserves_scala_semantics() {
        let schema = Example;

        assert_eq!(schema.doc_index(), 7);
        assert_eq!(schema.description(), "example schema");
        assert!(!schema.provided_by_frontend());
    }

    #[test]
    fn json_schema_accepts_the_contract_and_optional_default() {
        assert!(is_valid_schema_base_json(&json!({
            "docIndex": 0,
            "description": "base"
        })));

        assert!(is_valid_schema_base_json(&json!({
            "docIndex": -3,
            "description": "frontend schema",
            "providedByFrontend": true,
            "concreteTypeMetadata": "allowed"
        })));
    }

    #[test]
    fn json_schema_rejects_values_outside_the_scala_contract() {
        assert!(!is_valid_schema_base_json(&json!({
            "docIndex": 2_147_483_648_i64,
            "description": "outside Int"
        })));

        assert!(!is_valid_schema_base_json(&json!({
            "docIndex": 1
        })));

        assert!(!is_valid_schema_base_json(&json!({
            "docIndex": 1,
            "description": "bad boolean",
            "providedByFrontend": "false"
        })));
    }

    #[test]
    fn validation_exposes_structured_jsonschema_errors() {
        let value = json!({
            "docIndex": 1,
            "description": false
        });

        assert!(validate_schema_base_json(&value).is_err());
    }
}
