use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use genco::lang::rust;
use genco::prelude::*;
use genco::tokens::quoted;
use serde_yaml::Value as YamlValue;

fn repo_root() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("agones-rusternetes must live under crates/")
        .to_path_buf()
}

fn yaml_key(value: &YamlValue, path: &[&str]) -> YamlValue {
    let mut current = value;
    for segment in path {
        current = current
            .as_mapping()
            .and_then(|mapping| mapping.get(YamlValue::String((*segment).to_owned())))
            .unwrap_or_else(|| panic!("missing values.yaml key {}", path.join(".")));
    }
    current.clone()
}

fn value_bool(values: &YamlValue, path: &[&str]) -> bool {
    yaml_key(values, path)
        .as_bool()
        .unwrap_or_else(|| panic!("{} must be boolean", path.join(".")))
}

fn value_i64(values: &YamlValue, path: &[&str]) -> i64 {
    yaml_key(values, path)
        .as_i64()
        .unwrap_or_else(|| panic!("{} must be integer", path.join(".")))
}

fn condition_value(directive: &str, context: &HashMap<&str, bool>) -> bool {
    for (name, value) in context {
        if directive.contains(&format!(".{name}")) {
            return *value;
        }
    }
    panic!("unsupported Helm condition: {directive}");
}

fn render_template(
    source: &str,
    context: &HashMap<&str, bool>,
    max_items: i64,
    includes: &HashMap<&str, String>,
) -> String {
    let mut output = String::new();
    let mut stack = Vec::<bool>::new();

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("{{/*") && trimmed.ends_with("*/}}") {
            continue;
        }
        if trimmed.starts_with("{{- define ") {
            stack.push(true);
            continue;
        }
        if trimmed.starts_with("{{- if ") {
            stack.push(condition_value(trimmed, context));
            continue;
        }
        if trimmed.starts_with("{{- end") {
            stack.pop().expect("unbalanced Helm end directive");
            continue;
        }

        let active = stack.iter().all(|value| *value);
        if !active {
            continue;
        }

        if trimmed.starts_with("{{- include ") {
            let first_quote = trimmed.find('"').expect("include name opening quote");
            let rest = &trimmed[first_quote + 1..];
            let second_quote = rest.find('"').expect("include name closing quote");
            let name = &rest[..second_quote];
            let indent = trimmed
                .split("| indent ")
                .nth(1)
                .and_then(|value| value.split_whitespace().next())
                .and_then(|value| value.parse::<usize>().ok())
                .expect("include must specify numeric indent");
            let included = includes
                .get(name)
                .unwrap_or_else(|| panic!("unsupported Helm include: {name}"));
            for included_line in included.lines() {
                if !included_line.is_empty() {
                    output.push_str(&" ".repeat(indent));
                    output.push_str(included_line);
                }
                output.push('\n');
            }
            continue;
        }

        let rendered = line.replace("{{ .lists.maxItems }}", &max_items.to_string());
        if rendered.contains("{{") || rendered.contains("}}") {
            panic!("unhandled Helm directive after rendering: {rendered}");
        }
        output.push_str(&rendered);
        output.push('\n');
    }

    assert!(stack.is_empty(), "unbalanced Helm template blocks");
    output
}

fn read(path: &Path) -> String {
    println!("cargo:rerun-if-changed={}", path.display());
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn main() {
    let root = repo_root();
    let values_path = root.join("install/helm/agones/values.yaml");
    let spec_path = root.join("install/helm/agones/templates/crds/_gameserverspecschema.yaml");
    let status_path = root.join("install/helm/agones/templates/crds/_gameserverstatus.yaml");
    let pod_path = root.join(
        "install/helm/agones/templates/crds/k8s/_io.k8s.api.core.v1.PodTemplateSpec.yaml",
    );

    let values: YamlValue =
        serde_yaml::from_str(&read(&values_path)).expect("parse install/helm/agones/values.yaml");
    let include_patch_keys = value_bool(
        &values,
        &["agones", "crds", "includeKubernetesPatchKeys"],
    );
    let preserve_unknown = value_bool(
        &values,
        &["gameservers", "podPreserveUnknownFields"],
    );
    let max_items = value_i64(&values, &["gameservers", "lists", "maxItems"]);

    let context = HashMap::from([
        ("metadata", false),
        ("podPreserveUnknownFields", preserve_unknown),
        ("includeKubernetesPatchKeys", include_patch_keys),
    ]);

    let pod_schema = render_template(&read(&pod_path), &context, max_items, &HashMap::new());
    let includes = HashMap::from([("io.k8s.api.core.v1.PodTemplateSpec", pod_schema)]);

    let spec_yaml = render_template(&read(&spec_path), &context, max_items, &includes);
    let status_yaml = render_template(&read(&status_path), &context, max_items, &HashMap::new());

    let mut schema: YamlValue =
        serde_yaml::from_str(&spec_yaml).expect("rendered GameServer spec schema must be YAML");
    let status: YamlValue =
        serde_yaml::from_str(&status_yaml).expect("rendered GameServer status schema must be YAML");

    let properties = schema
        .as_mapping_mut()
        .and_then(|mapping| mapping.get_mut(YamlValue::String("properties".to_owned())))
        .and_then(YamlValue::as_mapping_mut)
        .expect("GameServer schema must contain object properties");
    let status_mapping = status
        .as_mapping()
        .expect("GameServer status template must render to a mapping");
    for (key, value) in status_mapping {
        properties.insert(key.clone(), value.clone());
    }

    let json_value =
        serde_json::to_value(schema).expect("rendered GameServer schema must convert to JSON");
    let schema_json =
        serde_json::to_string(&json_value).expect("serialize rendered GameServer schema");

    let literal = quoted(schema_json.as_str());
    let tokens: rust::Tokens = quote! {
        // Generated from Agones Helm/Kubernetes CRD schema templates. Do not edit.
        pub const GAME_SERVER_OPENAPI_SCHEMA_JSON: &str = $literal;
    };
    let generated = tokens.to_string().expect("format generated Rust source");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("game_server_schema.rs"), generated)
        .expect("write generated GameServer schema source");
}
