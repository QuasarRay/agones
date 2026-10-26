//! Source rest/clientcmd configuration data and concrete read effects.
//! The resulting wire configuration belongs to the Rusternetes transport.
//! Auth-provider and exec records remain data here; no plugin registry or
//! credential command is instantiated by configuration loading.
use std::{collections::BTreeMap, error::Error, fmt, path::{Path, PathBuf, Component}, sync::LazyLock};
use base64::{Engine, engine::general_purpose::STANDARD};
use cgp::prelude::*;
use futures::future::BoxFuture;
use serde::Deserialize;
use serde_json::Value;
use tokio::io::AsyncReadExt;
use crate::{errors::{BoxError, Errors}, utility_admission::null_default};

pub const TOKEN_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";
pub const ROOT_CA_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt";
#[derive(Clone, Default, PartialEq, Eq)]
pub struct SecretBytes(pub Vec<u8>);
impl fmt::Debug for SecretBytes { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("SecretBytes([REDACTED])") } }
#[derive(Clone, Default, PartialEq)]
pub struct SecretRecord(pub Value);
impl fmt::Debug for SecretRecord { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("SecretRecord([REDACTED])") } }
#[derive(Clone, Debug, Default)]
pub struct RusternetesClientConfiguration {
    pub host: String, pub proxy_url: String, pub disable_compression: bool,
    pub insecure_skip_tls_verify: bool, pub tls_server_name: String,
    pub ca_file: PathBuf, pub ca_data: Option<SecretBytes>,
    pub cert_file: PathBuf, pub cert_data: Option<SecretBytes>,
    pub key_file: PathBuf, pub key_data: Option<SecretBytes>,
    pub bearer_token: SecretBytes, pub bearer_token_file: PathBuf,
    pub username: String, pub password: SecretBytes,
    pub impersonate: String, pub impersonate_uid: String,
    pub impersonate_groups: Option<Vec<String>>, pub impersonate_extra: Option<BTreeMap<String, Option<Vec<String>>>>,
    pub auth_provider: Option<SecretRecord>, pub exec_provider: Option<SecretRecord>, pub exec_cluster_config: Option<SecretRecord>,
    pub auth_persistence: Option<AuthPersistenceOrigin>,
}
#[derive(Clone, Debug)]
pub struct AuthPersistenceOrigin { pub file: PathBuf, pub user: String }
#[derive(Debug)]
pub struct NotInCluster;
impl fmt::Display for NotInCluster {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("unable to load in-cluster configuration, KUBERNETES_SERVICE_HOST and KUBERNETES_SERVICE_PORT must be defined") }
}
impl Error for NotInCluster {}
#[derive(Debug)]
pub enum ClientConfigurationError { Empty, Invalid(Vec<String>) }
impl fmt::Display for ClientConfigurationError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str("invalid configuration: ")?;
        match self {
            Self::Empty => out.write_str("no configuration has been provided, try setting KUBERNETES_MASTER environment variable"),
            Self::Invalid(errors) => {
                // apimachinery Aggregate removes duplicate messages while
                // retaining the visit order, with brackets only for >1.
                let mut unique = Vec::new(); for error in errors { if !unique.contains(&error) { unique.push(error); } }
                if unique.len() == 1 { out.write_str(unique[0]) } else { write!(out, "[{}]", unique.into_iter().map(String::as_str).collect::<Vec<_>>().join(", ")) }
            }
        }
    }
}
impl Error for ClientConfigurationError {}
#[derive(Debug)]
struct ConfigPathError { operation: &'static str, path: PathBuf, error: std::io::Error }
impl fmt::Display for ConfigPathError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { write!(out, "{} {}: {}", self.operation, self.path.display(), self.error) }
}
impl Error for ConfigPathError { fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.error) } }
async fn read_file(path: &Path) -> Result<Vec<u8>, BoxError> {
    let mut file = tokio::fs::File::open(path).await.map_err(|error| ConfigPathError { operation: "open", path: path.into(), error })?;
    let mut data = Vec::new(); file.read_to_end(&mut data).await.map_err(|error| ConfigPathError { operation: "read", path: path.into(), error })?; Ok(data)
}
fn clean_path(path: &Path) -> PathBuf {
    let mut clean = PathBuf::new();
    for part in path.components() {
        match part {
            Component::CurDir => (),
            Component::ParentDir if clean.file_name().is_some_and(|name| name != "..") => { clean.pop(); }
            Component::ParentDir if path.has_root() => (),
            part => clean.push(part.as_os_str()),
        }
    }
    if clean.as_os_str().is_empty() { clean.push("."); } clean
}
fn local_path(base: &Path, value: &str) -> PathBuf {
    if value.is_empty() { PathBuf::new() } else { let path = Path::new(value); clean_path(&if path.is_absolute() { path.into() } else { base.join(path) }) }
}
fn env(name: &str) -> String { std::env::var_os(name).map(|value| value.to_string_lossy().into_owned()).unwrap_or_default() }

#[cgp_component(SourceClientConfigurationReader)]
pub trait CanReadSourceClientConfiguration: Sync {
    fn source_in_cluster_config(&self) -> BoxFuture<'_, Result<RusternetesClientConfiguration, BoxError>>;
    fn source_deferred_client_config<'a>(&'a self, explicit_path: &'a str) -> BoxFuture<'a, Result<RusternetesClientConfiguration, BoxError>>;
}
#[cgp_new_provider]
impl<Context: Sync> SourceClientConfigurationReader<Context> for ReadRusternetesSourceConfiguration {
    fn source_in_cluster_config(_context: &Context) -> BoxFuture<'_, Result<RusternetesClientConfiguration, BoxError>> { Box::pin(in_cluster_config()) }
    fn source_deferred_client_config<'a>(_context: &'a Context, explicit_path: &'a str) -> BoxFuture<'a, Result<RusternetesClientConfiguration, BoxError>> { Box::pin(deferred_client_config(explicit_path)) }
}
async fn in_cluster_config() -> Result<RusternetesClientConfiguration, BoxError> {
    let (host, port) = (env("KUBERNETES_SERVICE_HOST"), env("KUBERNETES_SERVICE_PORT"));
    if host.is_empty() || port.is_empty() { return Err(Box::new(NotInCluster)); }
    let token = read_file(Path::new(TOKEN_FILE)).await?;
    let mut config = RusternetesClientConfiguration { host: if host.contains(':') { format!("https://[{host}]:{port}") } else { format!("https://{host}:{port}") },
        bearer_token: SecretBytes(token), bearer_token_file: TOKEN_FILE.into(), ..Default::default() };
    let ca = async {
        let data = read_file(Path::new(ROOT_CA_FILE)).await?;
        let mut pool = rustls::RootCertStore::empty();
        for certificate in rustls_pemfile::certs(&mut std::io::Cursor::new(data)) { if let Ok(certificate) = certificate { let _ = pool.add(certificate); } }
        if pool.is_empty() { Err(Errors::default().New("data does not contain any valid RSA or ECDSA certificates")) } else { Ok(()) }
    }.await;
    match ca {
        Ok(()) => config.ca_file = ROOT_CA_FILE.into(),
        Err(error) => eprintln!("Expected to load root CA config from {ROOT_CA_FILE}, but got err: {error}"),
    }
    Ok(config)
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct WireConfig {
    #[serde(deserialize_with = "null_default")] kind: String,
    #[serde(rename = "apiVersion", deserialize_with = "null_default")] api_version: String,
    #[serde(deserialize_with = "null_default")] current_context: String,
    clusters: Option<Vec<NamedCluster>>, users: Option<Vec<NamedUser>>, contexts: Option<Vec<NamedContext>>,
    #[serde(rename = "preferences", deserialize_with = "null_default")] _preferences: WirePreferences,
    #[serde(rename = "extensions")] _extensions: Option<Vec<NamedExtension>>,
}
#[derive(Default, Deserialize)] #[serde(default)]
struct WirePreferences { #[serde(rename = "colors", deserialize_with = "null_default")] _colors: bool, #[serde(rename = "extensions")] _extensions: Option<Vec<NamedExtension>> }
#[derive(Default, Deserialize)] #[serde(default)]
struct NamedCluster { #[serde(deserialize_with = "null_default")] name: String, #[serde(deserialize_with = "null_default")] cluster: WireCluster }
#[derive(Default, Deserialize)] #[serde(default)]
struct NamedUser { #[serde(deserialize_with = "null_default")] name: String, #[serde(deserialize_with = "null_default")] user: WireUser }
#[derive(Default, Deserialize)] #[serde(default)]
struct NamedContext { #[serde(deserialize_with = "null_default")] name: String, #[serde(deserialize_with = "null_default")] context: WireContext }
#[derive(Default, Deserialize)] #[serde(default)]
struct WireContext {
    #[serde(deserialize_with = "null_default")] cluster: String, #[serde(deserialize_with = "null_default")] user: String,
    #[serde(rename = "namespace", deserialize_with = "null_default")] _namespace: String,
    #[serde(rename = "extensions")] _extensions: Option<Vec<NamedExtension>>,
}
#[derive(Default, Deserialize)] #[serde(default, rename_all = "kebab-case")]
struct WireCluster {
    #[serde(deserialize_with = "null_default")] server: String,
    #[serde(deserialize_with = "null_default")] tls_server_name: String,
    #[serde(deserialize_with = "null_default")] insecure_skip_tls_verify: bool,
    #[serde(deserialize_with = "null_default")] certificate_authority: String,
    #[serde(deserialize_with = "deserialize_data")] certificate_authority_data: Option<SecretBytes>,
    #[serde(deserialize_with = "null_default")] proxy_url: String,
    #[serde(deserialize_with = "null_default")] disable_compression: bool,
    extensions: Option<Vec<NamedExtension>>,
}
#[derive(Default, Deserialize)] #[serde(default)]
struct NamedExtension { #[serde(deserialize_with = "null_default")] name: String, extension: Value }
#[derive(Default, Deserialize)] #[serde(default, rename_all = "kebab-case")]
struct WireUser {
    #[serde(deserialize_with = "null_default")] client_certificate: String, #[serde(deserialize_with = "deserialize_data")] client_certificate_data: Option<SecretBytes>,
    #[serde(deserialize_with = "null_default")] client_key: String, #[serde(deserialize_with = "deserialize_data")] client_key_data: Option<SecretBytes>,
    #[serde(deserialize_with = "null_default")] token: String,
    #[serde(rename = "tokenFile", deserialize_with = "null_default")] token_file: String,
    #[serde(deserialize_with = "null_default")] username: String, #[serde(deserialize_with = "null_default")] password: String,
    #[serde(rename = "as", deserialize_with = "null_default")] impersonate: String,
    #[serde(rename = "as-uid", deserialize_with = "null_default")] impersonate_uid: String,
    #[serde(rename = "as-groups")] impersonate_groups: Option<Vec<String>>,
    #[serde(rename = "as-user-extra")] impersonate_extra: Option<BTreeMap<String, Option<Vec<String>>>>,
    auth_provider: Option<WireAuthProvider>, exec: Option<WireExec>,
    #[serde(rename = "extensions")] _extensions: Option<Vec<NamedExtension>>,
}
#[derive(Default, Deserialize, serde::Serialize)] #[serde(default)]
struct WireAuthProvider { #[serde(deserialize_with = "null_default")] name: String, config: Option<BTreeMap<String, String>> }
#[derive(Default, Deserialize, serde::Serialize)] #[serde(default, rename_all = "camelCase")]
struct WireExec {
    #[serde(deserialize_with = "null_default")] command: String, args: Option<Vec<String>>, env: Option<Vec<WireExecEnv>>,
    #[serde(deserialize_with = "null_default")] api_version: String, #[serde(deserialize_with = "null_default")] install_hint: String,
    #[serde(deserialize_with = "null_default")] provide_cluster_info: bool, #[serde(deserialize_with = "null_default")] interactive_mode: String,
}
#[derive(Default, Deserialize, serde::Serialize)] #[serde(default)]
struct WireExecEnv { #[serde(deserialize_with = "null_default")] name: String, #[serde(deserialize_with = "null_default")] value: String }

fn deserialize_data<'de, D: serde::Deserializer<'de>>(input: D) -> Result<Option<SecretBytes>, D::Error> {
    #[derive(Deserialize)] #[serde(untagged)] enum Input { Encoded(String), Bytes(Vec<u8>) }
    match Option::<Input>::deserialize(input)? {
        None => Ok(None), Some(Input::Bytes(bytes)) => Ok(Some(SecretBytes(bytes))),
        Some(Input::Encoded(value)) => STANDARD.decode(value.bytes().filter(|byte| *byte != b'\n' && *byte != b'\r').collect::<Vec<_>>())
            .map(|data| Some(SecretBytes(data))).map_err(serde::de::Error::custom),
    }
}
async fn validate_file(path: &Path, what: &str, name: &str, errors: &mut Vec<String>) {
    if path.as_os_str().is_empty() { return; }
    if let Err(error) = tokio::fs::File::open(path).await { errors.push(format!("unable to read {what} {} for {name} due to {error}", path.display())); }
}
fn clean_install_hint(value: &str) -> String {
    static ALLOWED: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"^[\p{L}\p{M}\p{N}\p{P}\p{S}\p{Z}\x09-\x0D]$").expect("source Unicode category expression"));
    let mut output = String::new();
    for rune in value.chars() {
        if ALLOWED.is_match(rune.encode_utf8(&mut [0; 4])) { output.push(rune); }
        else { output.push_str(&format!("U+{:04X}", rune as u32)); }
    }
    output
}
fn normalize_host(host: &str) -> String {
    // DirectClientConfig strips query/fragment only for a parsed request URI
    // with a non-opaque path longer than one character.
    if let Ok(mut parsed) = url::Url::parse(host) {
        if parsed.path().len() > 1 && !parsed.cannot_be_a_base() { parsed.set_query(None); parsed.set_fragment(None); return parsed.into(); }
    }
    host.into()
}
fn parse_proxy(value: &str) -> Result<(), String> {
    let url = url::Url::parse(value).map_err(|_| format!("could not parse: {value}"))?;
    if !["http", "https", "socks5"].contains(&url.scheme()) { return Err(format!("unsupported scheme {:?}, must be http, https, or socks5", url.scheme())); } Ok(())
}

async fn explicit_config(path: &Path) -> Result<RusternetesClientConfiguration, BoxError> {
    if let Err(error) = tokio::fs::metadata(path).await {
        if error.kind() == std::io::ErrorKind::NotFound { return Err(Box::new(ConfigPathError { operation: "stat", path: path.into(), error })); }
    }
    let bytes = match read_file(path).await {
        Ok(bytes) => bytes,
        Err(error) if error.downcast_ref::<ConfigPathError>().is_some_and(|error| error.error.kind() == std::io::ErrorKind::NotFound) => return Err(Box::new(ClientConfigurationError::Empty)),
        Err(error) => return Err(Errors::default().New(format!("error loading config file {:?}: {error}", path))),
    };
    let load = || -> Result<WireConfig, BoxError> {
        if bytes.is_empty() { return Ok(WireConfig::default()); }
        let value: Value = serde_yaml::from_slice(&bytes)?;
        let config: WireConfig = if value.is_null() { WireConfig::default() } else { serde_json::from_value(value)? };
        if !config.kind.is_empty() && config.kind != "Config" || !config.api_version.is_empty() && config.api_version != "v1" {
            return Err(Errors::default().New(format!("unsupported kubeconfig kind/version {}/{}", config.api_version, config.kind)));
        } Ok(config)
    };
    let config = load().map_err(|error| Errors::default().New(format!("error loading config file {:?}: {error}", path)))?;
    let base = if path.is_absolute() { clean_path(path) } else { clean_path(&std::env::current_dir()?.join(path)) };
    let base = base.parent().unwrap_or(Path::new("."));
    let mut clusters = BTreeMap::new(); let mut users = BTreeMap::new(); let mut contexts = BTreeMap::new();
    for named in config.clusters.unwrap_or_default() {
        if clusters.insert(named.name.clone(), named.cluster).is_some() { return Err(Errors::default().New(format!("error converting named clusters: duplicate name {:?}", named.name))); }
    }
    for named in config.users.unwrap_or_default() {
        if users.insert(named.name.clone(), named.user).is_some() { return Err(Errors::default().New(format!("error converting named users: duplicate name {:?}", named.name))); }
    }
    for named in config.contexts.unwrap_or_default() {
        if contexts.insert(named.name.clone(), named.context).is_some() { return Err(Errors::default().New(format!("error converting named contexts: duplicate name {:?}", named.name))); }
    }
    let context_exists = contexts.contains_key(&config.current_context);
    let context = contexts.remove(&config.current_context).unwrap_or_default();
    let cluster = clusters.remove(&context.cluster).unwrap_or_default(); let mut user = users.remove(&context.user).unwrap_or_default();
    let (ca, cert, key) = (cluster.certificate_authority_data.clone(), user.client_certificate_data.clone(), user.client_key_data.clone());
    let (ca_path, cert_path, key_path) = (local_path(base, &cluster.certificate_authority), local_path(base, &user.client_certificate), local_path(base, &user.client_key));
    let has_data = |data: &Option<SecretBytes>| data.as_ref().is_some_and(|data| !data.0.is_empty());
    let mut errors = Vec::new();
    if !config.current_context.is_empty() && !context_exists { errors.push(format!("context was not found for specified context: {}", config.current_context)); }
    let mut methods = Vec::new(); if !user.token.is_empty() { methods.push("token"); } if !user.username.is_empty() || !user.password.is_empty() { methods.push("basicAuth"); }
    if !cert_path.as_os_str().is_empty() || has_data(&cert) {
        if !cert_path.as_os_str().is_empty() && has_data(&cert) { errors.push(format!("client-cert-data and client-cert are both specified for {}. client-cert-data will override.", context.user)); }
        if !key_path.as_os_str().is_empty() && has_data(&key) { errors.push(format!("client-key-data and client-key are both specified for {}; client-key-data will override", context.user)); }
        if key_path.as_os_str().is_empty() && !has_data(&key) { errors.push(format!("client-key-data or client-key must be specified for {} to use the clientCert authentication method.", context.user)); }
        validate_file(&cert_path, "client-cert", &context.user, &mut errors).await; validate_file(&key_path, "client-key", &context.user, &mut errors).await;
    }
    if let Some(exec) = &mut user.exec {
        if user.auth_provider.is_some() { errors.push(format!("authProvider cannot be provided in combination with an exec plugin for {}", context.user)); }
        if exec.command.is_empty() { errors.push(format!("command must be specified for {} to use exec authentication plugin", context.user)); }
        if exec.api_version.is_empty() { errors.push(format!("apiVersion must be specified for {} to use exec authentication plugin", context.user)); }
        for entry in exec.env.as_deref().unwrap_or_default() { if entry.name.is_empty() { errors.push(format!("env variable name must be specified for {} to use exec authentication plugin", context.user)); } }
        if exec.interactive_mode.is_empty() && ["client.authentication.k8s.io/v1beta1", "client.authentication.k8s.io/v1alpha1"].contains(&exec.api_version.as_str()) { exec.interactive_mode = "IfAvailable".into(); }
        match exec.interactive_mode.as_str() {
            "" => errors.push(format!("interactiveMode must be specified for {} to use exec authentication plugin", context.user)),
            "Never" | "IfAvailable" | "Always" => (),
            other => errors.push(format!("invalid interactiveMode for {}: {other:?}", context.user)),
        }
        // PluginPolicy is json:"-" and has no override in this call path;
        // its source validation is the successful zero/AllowAll + nil case.
    }
    if methods.len() > 1 { errors.push(format!("more than one authentication method found for {}; found [{}], only one is allowed", context.user, methods.join(" "))); }
    if user.impersonate.is_empty() && (!user.impersonate_uid.is_empty() || user.impersonate_groups.as_ref().is_some_and(|groups| !groups.is_empty()) || user.impersonate_extra.as_ref().is_some_and(|extra| !extra.is_empty())) {
        errors.push(format!("requesting uid, groups or user-extra for {} without impersonating a user", context.user));
    }
    let empty_cluster = cluster.server.is_empty() && cluster.tls_server_name.is_empty() && !cluster.insecure_skip_tls_verify && ca_path.as_os_str().is_empty() && !has_data(&ca)
        && cluster.proxy_url.is_empty() && !cluster.disable_compression && cluster.extensions.as_ref().is_none_or(Vec::is_empty);
    if empty_cluster {
        if errors.is_empty() { return Err(Box::new(ClientConfigurationError::Empty)); }
        errors.push("cluster has no server defined".into());
    } else {
        if cluster.server.is_empty() { errors.push(if context.cluster.is_empty() { "default cluster has no server defined".into() } else { format!("no server found for cluster {:?}", context.cluster) }); }
        if !cluster.proxy_url.is_empty() { if let Err(error) = parse_proxy(&cluster.proxy_url) { errors.push(format!("invalid 'proxy-url' {:?} for cluster {:?}: {error}", cluster.proxy_url, context.cluster)); } }
        if !ca_path.as_os_str().is_empty() && has_data(&ca) { errors.push(format!("certificate-authority-data and certificate-authority are both specified for {}. certificate-authority-data will override.", context.cluster)); }
        validate_file(&ca_path, "certificate-authority", &context.cluster, &mut errors).await;
    }
    if !errors.is_empty() { return Err(Box::new(ClientConfigurationError::Invalid(errors))); }
    let mut result = RusternetesClientConfiguration { host: normalize_host(&cluster.server), proxy_url: cluster.proxy_url, disable_compression: cluster.disable_compression,
        impersonate: user.impersonate.clone(), impersonate_uid: user.impersonate_uid.clone(), impersonate_groups: user.impersonate_groups.clone(), impersonate_extra: user.impersonate_extra.clone(), ..Default::default() };
    let explicit_https = cluster.server.split_once("://").is_some_and(|(scheme, authority)| scheme.eq_ignore_ascii_case("https") && !authority.starts_with('/') && !authority.is_empty());
    let tls = explicit_https && url::Url::parse(&cluster.server).is_ok_and(|url| url.scheme() == "https" && url.host().is_some());
    if tls {
        result.ca_file = ca_path; result.ca_data = ca; result.insecure_skip_tls_verify = cluster.insecure_skip_tls_verify; result.tls_server_name = cluster.tls_server_name;
        if !user.token.is_empty() { result.bearer_token = SecretBytes(user.token.into_bytes()); result.bearer_token_file = local_path(base, &user.token_file); }
        else if !user.token_file.is_empty() { result.bearer_token_file = local_path(base, &user.token_file); result.bearer_token = SecretBytes(read_file(&result.bearer_token_file).await?); }
        if !cert_path.as_os_str().is_empty() || has_data(&cert) { result.cert_file = cert_path; result.cert_data = cert; result.key_file = key_path; result.key_data = key; }
        result.username = user.username; result.password = SecretBytes(user.password.into_bytes());
        result.auth_provider = user.auth_provider.map(serde_json::to_value).transpose()?.map(SecretRecord);
        if result.auth_provider.is_some() { result.auth_persistence = Some(AuthPersistenceOrigin { file: path.into(), user: context.user }); }
        if let Some(mut exec) = user.exec {
            exec.install_hint = clean_install_hint(&exec.install_hint);
            result.exec_provider = Some(SecretRecord(serde_json::to_value(exec)?));
            result.exec_cluster_config = cluster.extensions.unwrap_or_default().into_iter().find(|extension| extension.name == "client.authentication.k8s.io/exec").map(|extension| SecretRecord(extension.extension));
        }
    }
    Ok(result)
}
async fn deferred_client_config(explicit_path: &str) -> Result<RusternetesClientConfiguration, BoxError> {
    // The caller constructs a zero-valued LoadingRules with ExplicitPath only.
    // Precedence and DefaultClientConfig are nil: do not search env/home files
    // and do not replace an already valid explicit config with in-cluster data.
    let result = if explicit_path.is_empty() { Err(Box::new(ClientConfigurationError::Empty) as BoxError) } else { explicit_config(Path::new(explicit_path)).await };
    if result.as_ref().err().and_then(|error| error.downcast_ref::<ClientConfigurationError>()).is_some_and(|error| matches!(error, ClientConfigurationError::Empty)) {
        let metadata = tokio::fs::metadata(TOKEN_FILE).await;
        let (host, port) = (env("KUBERNETES_SERVICE_HOST"), env("KUBERNETES_SERVICE_PORT"));
        if !host.is_empty() && !port.is_empty() && metadata.is_ok_and(|metadata| !metadata.is_dir()) { return in_cluster_config().await; }
    }
    result
}
