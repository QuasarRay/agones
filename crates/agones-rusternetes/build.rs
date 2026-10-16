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
            .and_then(|mapping| mapping.get(&YamlValue::String((*segment).to_owned())))
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

fn render_template_with_indent_offset(
    source: &str,
    context: &HashMap<&str, bool>,
    max_items: i64,
    includes: &HashMap<&str, String>,
    include_indent_offset: usize,
) -> String {
    let mut output = String::new();
    let mut stack = Vec::<bool>::new();
    let has_definition = source
        .lines()
        .any(|line| line.trim().starts_with("{{- define "));
    let mut inside_definition = !has_definition;
    let mut in_dict_assignment = false;

    for line in source.lines() {
        let trimmed = line.trim();

        if in_dict_assignment {
            if trimmed.ends_with("}}") {
                in_dict_assignment = false;
            }
            continue;
        }

        if trimmed.starts_with("{{- $") && trimmed.contains(":= dict") {
            in_dict_assignment = !trimmed.ends_with("}}");
            continue;
        }
        if trimmed.starts_with("{{/*") && trimmed.ends_with("*/}}") {
            continue;
        }
        if trimmed.starts_with("{{- define ") {
            stack.push(true);
            inside_definition = true;
            continue;
        }
        if trimmed.starts_with("{{- if ") {
            stack.push(condition_value(trimmed, context));
            continue;
        }
        if trimmed.starts_with("{{- end") {
            stack.pop().expect("unbalanced Helm end directive");
            if has_definition && stack.is_empty() {
                inside_definition = false;
            }
            continue;
        }

        let active = inside_definition && stack.iter().all(|value| *value);
        if !active {
            continue;
        }

        if trimmed.starts_with("{{- include ") {
            let first_quote = trimmed.find('"').expect("include name opening quote");
            let rest = &trimmed[first_quote + 1..];
            let second_quote = rest.find('"').expect("include name closing quote");
            let name = &rest[..second_quote];
            let helm_indent = trimmed
                .split("| indent ")
                .nth(1)
                .and_then(|value| value.split_whitespace().next())
                .and_then(|value| value.parse::<usize>().ok())
                .expect("include must specify numeric indent");
            let indent = helm_indent
                .checked_sub(include_indent_offset)
                .expect("include indent must be inside extracted schema subtree");
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

    assert!(!in_dict_assignment, "unterminated Helm dict assignment");
    assert!(stack.is_empty(), "unbalanced Helm template blocks");
    output
}

fn render_template(
    source: &str,
    context: &HashMap<&str, bool>,
    max_items: i64,
    includes: &HashMap<&str, String>,
) -> String {
    render_template_with_indent_offset(source, context, max_items, includes, 0)
}

fn extract_indented_block(source: &str, marker: &str) -> (String, usize) {
    let lines: Vec<_> = source.lines().collect();
    let marker_index = lines
        .iter()
        .position(|line| line.trim() == marker)
        .unwrap_or_else(|| panic!("missing YAML marker {marker}"));
    let marker_indent = lines[marker_index]
        .chars()
        .take_while(|character| *character == ' ')
        .count();
    let content_indent = marker_indent + 2;
    let prefix = " ".repeat(content_indent);
    let mut output = String::new();

    for line in &lines[marker_index + 1..] {
        if line.trim().is_empty() {
            output.push('\n');
            continue;
        }

        let indent = line
            .chars()
            .take_while(|character| *character == ' ')
            .count();
        if indent <= marker_indent {
            break;
        }

        let rebased = line
            .strip_prefix(&prefix)
            .unwrap_or_else(|| panic!("line under {marker} is less indented than expected: {line}"));
        output.push_str(rebased);
        output.push('\n');
    }

    (output, content_indent)
}

fn read(path: &Path) -> String {
    println!("cargo:rerun-if-changed={}", path.display());
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn render_game_server_schema(
    spec_source: &str,
    pod_schema: &str,
    object_meta_schema: &str,
    preserve_unknown: bool,
    include_patch_keys: bool,
    max_items: i64,
    metadata: bool,
) -> String {
    let context = HashMap::from([
        ("metadata", metadata),
        ("podPreserveUnknownFields", preserve_unknown),
        ("includeKubernetesPatchKeys", include_patch_keys),
    ]);
    let includes = HashMap::from([
        ("io.k8s.api.core.v1.PodTemplateSpec", pod_schema.to_owned()),
        (
            "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
            object_meta_schema.to_owned(),
        ),
    ]);
    render_template(spec_source, &context, max_items, &includes)
}

fn schema_json(schema: YamlValue, name: &str) -> String {
    let json_value = serde_json::to_value(schema)
        .unwrap_or_else(|error| panic!("{name} schema must convert to JSON: {error}"));
    serde_json::to_string(&json_value)
        .unwrap_or_else(|error| panic!("serialize {name} schema: {error}"))
}

fn main() {
    let root = repo_root();
    let values_path = root.join("install/helm/agones/values.yaml");
    let spec_path = root.join("install/helm/agones/templates/crds/_gameserverspecschema.yaml");
    let status_path = root.join("install/helm/agones/templates/crds/_gameserverstatus.yaml");
    let pod_path = root.join(
        "install/helm/agones/templates/crds/k8s/_io.k8s.api.core.v1.PodTemplateSpec.yaml",
    );
    let object_meta_path = root.join(
        "install/helm/agones/templates/crds/k8s/_io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.yaml",
    );
    let game_server_set_path =
        root.join("install/helm/agones/templates/crds/gameserverset.yaml");

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

    let base_context = HashMap::from([
        ("metadata", false),
        ("podPreserveUnknownFields", preserve_unknown),
        ("includeKubernetesPatchKeys", include_patch_keys),
    ]);

    let pod_source = read(&pod_path);
    let object_meta_source = read(&object_meta_path);
    let spec_source = read(&spec_path);
    let pod_schema = render_template(&pod_source, &base_context, max_items, &HashMap::new());
    let object_meta_schema =
        render_template(&object_meta_source, &base_context, max_items, &HashMap::new());

    let spec_yaml = render_game_server_schema(
        &spec_source,
        &pod_schema,
        &object_meta_schema,
        preserve_unknown,
        include_patch_keys,
        max_items,
        false,
    );
    let status_yaml =
        render_template(&read(&status_path), &base_context, max_items, &HashMap::new());

    let mut game_server_schema: YamlValue =
        serde_yaml::from_str(&spec_yaml).expect("rendered GameServer spec schema must be YAML");
    let status: YamlValue =
        serde_yaml::from_str(&status_yaml).expect("rendered GameServer status schema must be YAML");

    let properties = game_server_schema
        .as_mapping_mut()
        .and_then(|mapping| mapping.get_mut(&YamlValue::String("properties".to_owned())))
        .and_then(YamlValue::as_mapping_mut)
        .expect("GameServer schema must contain object properties");
    let status_mapping = status
        .as_mapping()
        .expect("GameServer status template must render to a mapping");
    for (key, value) in status_mapping {
        properties.insert(key.clone(), value.clone());
    }

    let game_server_schema_json = schema_json(game_server_schema, "GameServer");

    let game_server_template_schema = render_game_server_schema(
        &spec_source,
        &pod_schema,
        &object_meta_schema,
        preserve_unknown,
        include_patch_keys,
        max_items,
        true,
    );
    let game_server_set_source = read(&game_server_set_path);
    let (game_server_set_openapi, openapi_indent) =
        extract_indented_block(&game_server_set_source, "openAPIV3Schema:");
    let game_server_set_context = HashMap::from([
        ("metadata", true),
        ("podPreserveUnknownFields", preserve_unknown),
        ("includeKubernetesPatchKeys", include_patch_keys),
    ]);
    let game_server_set_includes =
        HashMap::from([("gameserver.schema", game_server_template_schema)]);
    let game_server_set_yaml = render_template_with_indent_offset(
        &game_server_set_openapi,
        &game_server_set_context,
        max_items,
        &game_server_set_includes,
        openapi_indent,
    );
    let game_server_set_schema: YamlValue = serde_yaml::from_str(&game_server_set_yaml)
        .expect("rendered GameServerSet OpenAPI schema must be YAML");
    let game_server_set_schema_json = schema_json(game_server_set_schema, "GameServerSet");

    let game_server_literal = quoted(game_server_schema_json.as_str());
    let game_server_set_literal = quoted(game_server_set_schema_json.as_str());
    let tokens: rust::Tokens = quote! {
        // Generated from Agones Helm/Kubernetes CRD schema templates. Do not edit.
        pub const GAME_SERVER_OPENAPI_SCHEMA_JSON: &str = $game_server_literal;
        pub const GAME_SERVER_SET_OPENAPI_SCHEMA_JSON: &str = $game_server_set_literal;
    };
    let generated = tokens.to_string().expect("format generated Rust source");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("game_server_schema.rs"), generated)
        .expect("write generated Agones CRD schema source");
}
