//! Hand-written Rust composition of the CPG schema.
//!
//! This is the Rust equivalent of the Scala `CpgSchema` class/object: module
//! constructors execute in the same dependency order, while the companion
//! object's builder configuration is validated with `jsonschema`.

use crate::{
    cpg_schema as generated,
    schema::{ObjectSchemaBuilder, Schema},
    Error, Result,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::LazyLock;

/// Canonical configuration used by `CpgSchema.instance`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CpgSchemaConfig {
    pub domain_short_name: String,
    pub base_package: String,
    pub additional_traversals_packages: Vec<String>,
}

impl Default for CpgSchemaConfig {
    fn default() -> Self {
        Self {
            domain_short_name: "Cpg".into(),
            base_package: "io.shiftleft.codepropertygraph.generated".into(),
            additional_traversals_packages: vec!["io.shiftleft".into()],
        }
    }
}

/// JSON Schema for the exact companion-object configuration in the Scala source.
#[jsonschema::validator(
    schema = r#"{
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {
            "domainShortName": { "const": "Cpg" },
            "basePackage": {
                "const": "io.shiftleft.codepropertygraph.generated"
            },
            "additionalTraversalsPackages": {
                "const": ["io.shiftleft"]
            }
        },
        "required": [
            "domainShortName",
            "basePackage",
            "additionalTraversalsPackages"
        ],
        "additionalProperties": false
    }"#,
    draft = Draft202012
)]
pub struct CpgSchemaConfigValidator;

#[inline]
pub fn validate_config(value: &Value) -> Result<()> {
    CpgSchemaConfigValidator::validate(value)
        .map_err(|error| Error::Invalid(format!("invalid CpgSchema configuration: {error}")))
}

/// Typed composition corresponding to Scala's `class CpgSchema(builder)`.
///
/// Fields intentionally follow the source-level names rather than the generated
/// module names. Construction order is semantically significant because each
/// schema layer mutates the shared builder and later layers depend on earlier
/// ones.
#[derive(Clone, Debug)]
pub struct CpgSchema {
    pub base: generated::BaseSchema,
    pub fs: generated::FileSystemSchema,
    pub namespaces: generated::NamespaceSchema,

    pub operators: generated::OperatorsSchema,
    pub configuration: generated::ConfigurationSchema,
    pub meta_data: generated::MetaDataSchema,

    pub type_schema: generated::TypeSchema,
    pub method: generated::MethodSchema,
    pub ast: generated::AstSchema,
    pub call_graph: generated::CallGraphSchema,

    pub cfg: generated::CfgSchema,
    pub dominators: generated::DominatorsSchema,
    pub pdg: generated::PdgSchema,

    pub shortcuts: generated::ShortcutsSchema,

    pub source_specific: generated::CommentSchema,
    pub tags: generated::TagsSchema,
    pub binding: generated::BindingSchema,
    pub annotation: generated::AnnotationSchema,
    pub finding: generated::FindingSchema,
    pub hidden: generated::HiddenSchema,
    pub proto_serialize: generated::ProtoSerializeSchema,
}

impl CpgSchema {
    pub fn new(builder: &mut ObjectSchemaBuilder) -> Result<Self> {
        let base = generated::BaseSchema::new(builder)?;
        let fs = generated::FileSystemSchema::new(builder, &base)?;
        let namespaces = generated::NamespaceSchema::new(builder, &base, &fs)?;

        let operators = generated::OperatorsSchema::new(builder)?;
        let configuration = generated::ConfigurationSchema::new(builder, &base)?;
        let meta_data = generated::MetaDataSchema::new(builder, &base)?;

        let type_schema = generated::TypeSchema::new(builder, &base, &fs)?;
        let method = generated::MethodSchema::new(builder, &base, &type_schema, &fs)?;
        let ast = generated::AstSchema::new(
            builder,
            &base,
            &namespaces,
            &method,
            &type_schema,
            &fs,
        )?;
        let call_graph = generated::CallGraphSchema::new(builder, &method, &ast)?;

        let cfg = generated::CfgSchema::new(builder, &method, &ast)?;
        let dominators = generated::DominatorsSchema::new(builder, &method, &ast)?;
        let pdg = generated::PdgSchema::new(builder, &method, &ast)?;

        let shortcuts =
            generated::ShortcutsSchema::new(builder, &base, &method, &ast, &type_schema, &fs)?;

        let source_specific = generated::CommentSchema::new(builder, &base, &ast, &fs)?;
        let tags = generated::TagsSchema::new(
            builder,
            &base,
            &type_schema,
            &method,
            &ast,
            &fs,
            &call_graph,
        )?;
        let binding =
            generated::BindingSchema::new(builder, &base, &type_schema, &method, &call_graph)?;
        let annotation = generated::AnnotationSchema::new(
            builder,
            &base,
            &method,
            &type_schema,
            &ast,
            &shortcuts,
        )?;
        let finding = generated::FindingSchema::new(builder, &base)?;
        let hidden = generated::HiddenSchema::new(
            builder,
            &base,
            &method,
            &type_schema,
            &ast,
            &cfg,
            &fs,
            &call_graph,
            &pdg,
            &tags,
            &shortcuts,
        )?;
        let proto_serialize = generated::ProtoSerializeSchema::new(builder, &ast)?;

        Ok(Self {
            base,
            fs,
            namespaces,
            operators,
            configuration,
            meta_data,
            type_schema,
            method,
            ast,
            call_graph,
            cfg,
            dominators,
            pdg,
            shortcuts,
            source_specific,
            tags,
            binding,
            annotation,
            finding,
            hidden,
            proto_serialize,
        })
    }
}

/// Build the schema represented by Scala's `CpgSchema.instance`.
pub fn build_instance() -> Result<Schema> {
    let config = CpgSchemaConfig::default();
    let json = serde_json::to_value(&config)?;
    validate_config(&json)?;

    let mut builder = ObjectSchemaBuilder::new(
        config.domain_short_name,
        config.base_package,
        config.additional_traversals_packages,
    );
    CpgSchema::new(&mut builder)?;
    builder.build()?.to_model()
}

/// Lazy singleton equivalent of Scala's `CpgSchema.instance`.
pub static INSTANCE: LazyLock<Schema> = LazyLock::new(|| {
    build_instance().expect("the built-in CPG schema must satisfy its static contract")
});

#[inline]
pub fn instance() -> &'static Schema {
    &INSTANCE
}

/// Rust equivalent of Scala's `CpgSchema.PropertyDefaults`.
pub mod property_defaults {
    pub const STRING: &str = "<empty>";
    pub const BOOLEAN: bool = false;
    pub const BYTE: i8 = -1;
    pub const SHORT: i16 = -1;
    pub const INT: i32 = -1;
    pub const LONG: i64 = -1;
    pub const FLOAT: f32 = 0.0;
    pub const DOUBLE: f64 = 0.0;
    pub const CHAR: char = '?';
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn canonical_builder_configuration_is_valid() {
        let config = serde_json::to_value(CpgSchemaConfig::default()).unwrap();
        validate_config(&config).unwrap();
    }

    #[test]
    fn configuration_schema_rejects_semantic_drift() {
        assert!(validate_config(&json!({
            "domainShortName": "NotCpg",
            "basePackage": "io.shiftleft.codepropertygraph.generated",
            "additionalTraversalsPackages": ["io.shiftleft"]
        }))
        .is_err());

        assert!(validate_config(&json!({
            "domainShortName": "Cpg",
            "basePackage": "io.shiftleft.codepropertygraph.generated",
            "additionalTraversalsPackages": []
        }))
        .is_err());
    }

    #[test]
    fn property_defaults_match_the_scala_companion_object() {
        assert_eq!(property_defaults::STRING, "<empty>");
        assert!(!property_defaults::BOOLEAN);
        assert_eq!(property_defaults::BYTE, -1);
        assert_eq!(property_defaults::SHORT, -1);
        assert_eq!(property_defaults::INT, -1);
        assert_eq!(property_defaults::LONG, -1);
        assert_eq!(property_defaults::FLOAT, 0.0);
        assert_eq!(property_defaults::DOUBLE, 0.0);
        assert_eq!(property_defaults::CHAR, '?');
    }

    #[test]
    fn instance_has_the_canonical_identity() {
        let schema = build_instance().unwrap();
        assert_eq!(schema.domain_short_name, "Cpg");
        assert_eq!(
            schema.base_package,
            "io.shiftleft.codepropertygraph.generated"
        );
        assert_eq!(schema.additional_traversals_packages, ["io.shiftleft"]);
    }
}
