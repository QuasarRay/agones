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

fn extract_helm_definition(source: &str, name: &str) -> String {
    let marker = format!(r#"{{{{- define "{name}" }}}}"#);
    let mut inside = false;
    let mut depth = 0usize;
    let mut output = String::new();

    for line in source.lines() {
        let trimmed = line.trim();

        if !inside {
            if trimmed == marker {
                inside = true;
                depth = 1;
            }
            continue;
        }

        if trimmed.starts_with("{{- if ")
            || trimmed.starts_with("{{- range ")
            || trimmed.starts_with("{{- with ")
            || trimmed.starts_with("{{- define ")
        {
            depth += 1;
            output.push_str(line);
            output.push('\n');
            continue;
        }

        if trimmed.starts_with("{{- end") {
            depth = depth.checked_sub(1).expect("unbalanced Helm definition");
            if depth == 0 {
                return output;
            }
            output.push_str(line);
            output.push('\n');
            continue;
        }

        output.push_str(line);
        output.push('\n');
    }

    panic!("missing or unterminated Helm definition {name}");
}

fn include_indent(directive: &str) -> usize {
    directive
        .split("| indent ")
        .nth(1)
        .and_then(|value| value.split_whitespace().next())
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or_else(|| panic!("include must specify numeric indent: {directive}"))
}

fn push_indented(output: &mut String, rendered: &str, indent: usize) {
    for line in rendered.lines() {
        if !line.is_empty() {
            output.push_str(&" ".repeat(indent));
            output.push_str(line);
        }
        output.push('\n');
    }
}

fn dict_bool(directive: &str, key: &str, scheduled_autoscaler: bool) -> bool {
    let marker = format!(r#""{key}""#);
    let start = directive
        .find(&marker)
        .unwrap_or_else(|| panic!("missing {key} in Helm dict: {directive}"));
    let rest = directive[start + marker.len()..].trim_start();
    let token = rest
        .split(|character: char| character.is_whitespace() || character == ')')
        .next()
        .unwrap_or_default();

    match token {
        "true" => true,
        "false" => false,
        "$featureGates.ScheduledAutoscaler" => scheduled_autoscaler,
        _ => panic!("unsupported Helm bool value {token:?} for {key}: {directive}"),
    }
}

fn feature_gate_value(defaults: &YamlValue, values: &YamlValue, name: &str) -> bool {
    let defaults = defaults
        .as_mapping()
        .expect("defaultfeaturegates.yaml must be a mapping");
    let key = YamlValue::String(name.to_owned());
    let mut value = defaults
        .get(&key)
        .and_then(YamlValue::as_bool)
        .unwrap_or_else(|| panic!("missing boolean feature gate {name}"));

    let overrides_value = yaml_key(values, &["agones", "featureGates"]);
    let overrides = overrides_value
        .as_str()
        .expect("agones.featureGates must be a string");
    if overrides.is_empty() {
        return value;
    }

    for entry in overrides.split('&') {
        let (override_name, raw_value) = entry
            .split_once('=')
            .unwrap_or_else(|| panic!("invalid feature gate override {entry:?}"));
        if !defaults.contains_key(&YamlValue::String(override_name.to_owned())) {
            panic!("unknown feature gate override {override_name}");
        }
        let parsed = raw_value
            .parse::<bool>()
            .unwrap_or_else(|_| panic!("feature gate {override_name} must be true or false"));
        if override_name == name {
            value = parsed;
        }
    }

    value
}

fn render_fleet_autoscaler_policy(
    policy_source: &str,
    url_configuration: &str,
    scheduled_autoscaler: bool,
    include_policy: bool,
    include_schedule_policy: bool,
    include_chain_policy: bool,
) -> String {
    let context = HashMap::from([
        ("includePolicy", include_policy),
        ("includeSchedulePolicy", include_schedule_policy),
        ("includeChainPolicy", include_chain_policy),
    ]);
    let mut output = String::new();
    let mut stack = Vec::<bool>::new();

    for line in policy_source.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("{{- if ") {
            stack.push(condition_value(trimmed, &context));
            continue;
        }
        if trimmed.starts_with("{{- end") {
            stack.pop().expect("unbalanced FleetAutoscaler policy end");
            continue;
        }

        if !stack.iter().all(|value| *value) {
            continue;
        }

        if trimmed.starts_with("{{- include ") {
            let indent = include_indent(trimmed);
            if trimmed.contains(r#""url.configuration""#) {
                if url_configuration.contains("{{") || url_configuration.contains("}}") {
                    panic!("url.configuration contains unhandled Helm directives");
                }
                push_indented(&mut output, url_configuration, indent);
                continue;
            }

            if trimmed.contains(r#""fleetautoscaler.policy""#) {
                let child = render_fleet_autoscaler_policy(
                    policy_source,
                    url_configuration,
                    scheduled_autoscaler,
                    dict_bool(trimmed, "includePolicy", scheduled_autoscaler),
                    dict_bool(trimmed, "includeSchedulePolicy", scheduled_autoscaler),
                    dict_bool(trimmed, "includeChainPolicy", scheduled_autoscaler),
                );
                push_indented(&mut output, &child, indent);
                continue;
            }

            panic!("unsupported FleetAutoscaler policy include: {trimmed}");
        }

        if line.contains("{{") || line.contains("}}") {
            panic!("unhandled FleetAutoscaler policy directive: {line}");
        }

        output.push_str(line);
        output.push('\n');
    }

    assert!(
        stack.is_empty(),
        "unbalanced FleetAutoscaler policy template blocks"
    );
    output
}

fn render_fleet_autoscaler_schema(
    source: &str,
    policy_source: &str,
    url_configuration: &str,
    scheduled_autoscaler: bool,
) -> String {
    let (openapi, openapi_indent) = extract_indented_block(source, "openAPIV3Schema:");
    let mut output = String::new();

    for line in openapi.lines() {
        let trimmed = line.trim();

        if trimmed
            == r#"{{- $featureGates := include "agones.featureGates" . | fromYaml }}"#
        {
            continue;
        }

        if trimmed.starts_with(r#"{{- include "fleetautoscaler.policy" "#) {
            let rendered = render_fleet_autoscaler_policy(
                policy_source,
                url_configuration,
                scheduled_autoscaler,
                dict_bool(trimmed, "includePolicy", scheduled_autoscaler),
                dict_bool(trimmed, "includeSchedulePolicy", scheduled_autoscaler),
                dict_bool(trimmed, "includeChainPolicy", scheduled_autoscaler),
            );
            let indent = include_indent(trimmed)
                .checked_sub(openapi_indent)
                .expect("FleetAutoscaler policy include must be inside OpenAPI subtree");
            push_indented(&mut output, &rendered, indent);
            continue;
        }

        if line.contains("{{") || line.contains("}}") {
            panic!("unhandled FleetAutoscaler schema directive: {line}");
        }

        output.push_str(line);
        output.push('\n');
    }

    output
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
    let fleet_path = root.join("install/helm/agones/templates/crds/fleet.yaml");
    let fleet_autoscaler_path =
        root.join("install/helm/agones/templates/crds/fleetautoscaler.yaml");
    let fleet_autoscaler_policy_path =
        root.join("install/helm/agones/templates/crds/_fleetautoscalerpolicy.yaml");
    let default_feature_gates_path = root.join("install/helm/agones/defaultfeaturegates.yaml");

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
    let default_feature_gates: YamlValue = serde_yaml::from_str(&read(&default_feature_gates_path))
        .expect("parse install/helm/agones/defaultfeaturegates.yaml");
    let scheduled_autoscaler =
        feature_gate_value(&default_feature_gates, &values, "ScheduledAutoscaler");

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
        HashMap::from([("gameserver.schema", game_server_template_schema.clone())]);
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

    let fleet_source = read(&fleet_path);
    let (fleet_openapi, fleet_openapi_indent) =
        extract_indented_block(&fleet_source, "openAPIV3Schema:");
    let fleet_context = HashMap::from([
        ("metadata", true),
        ("podPreserveUnknownFields", preserve_unknown),
        ("includeKubernetesPatchKeys", include_patch_keys),
    ]);
    let fleet_includes = HashMap::from([("gameserver.schema", game_server_template_schema)]);
    let fleet_yaml = render_template_with_indent_offset(
        &fleet_openapi,
        &fleet_context,
        max_items,
        &fleet_includes,
        fleet_openapi_indent,
    );
    let fleet_schema: YamlValue =
        serde_yaml::from_str(&fleet_yaml).expect("rendered Fleet OpenAPI schema must be YAML");
    let fleet_schema_json = schema_json(fleet_schema, "Fleet");

    let fleet_autoscaler_policy_source = read(&fleet_autoscaler_policy_path);
    let url_configuration = extract_helm_definition(
        &fleet_autoscaler_policy_source,
        "url.configuration",
    );
    let fleet_autoscaler_policy = extract_helm_definition(
        &fleet_autoscaler_policy_source,
        "fleetautoscaler.policy",
    );
    let fleet_autoscaler_source = read(&fleet_autoscaler_path);
    let fleet_autoscaler_yaml = render_fleet_autoscaler_schema(
        &fleet_autoscaler_source,
        &fleet_autoscaler_policy,
        &url_configuration,
        scheduled_autoscaler,
    );
    let fleet_autoscaler_schema: YamlValue = serde_yaml::from_str(&fleet_autoscaler_yaml)
        .expect("rendered FleetAutoscaler OpenAPI schema must be YAML");
    let fleet_autoscaler_schema_json =
        schema_json(fleet_autoscaler_schema, "FleetAutoscaler");

    let game_server_literal = quoted(game_server_schema_json.as_str());
    let game_server_set_literal = quoted(game_server_set_schema_json.as_str());
    let fleet_literal = quoted(fleet_schema_json.as_str());
    let fleet_autoscaler_literal = quoted(fleet_autoscaler_schema_json.as_str());
    let tokens: rust::Tokens = quote! {
        // Generated from Agones Helm/Kubernetes CRD schema templates. Do not edit.
        pub const GAME_SERVER_OPENAPI_SCHEMA_JSON: &str = $game_server_literal;
        pub const GAME_SERVER_SET_OPENAPI_SCHEMA_JSON: &str = $game_server_set_literal;
        pub const FLEET_OPENAPI_SCHEMA_JSON: &str = $fleet_literal;
        pub const FLEET_AUTOSCALER_OPENAPI_SCHEMA_JSON: &str = $fleet_autoscaler_literal;
    };
    let generated = tokens.to_string().expect("format generated Rust source");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("game_server_schema.rs"), generated)
        .expect("write generated Agones CRD schema source");
}
