//! Rusternetes is the only concrete CRD transport. Portable waiting policy is
//! in the literal source mirror and consumes only its read/clock/log abilities.
use std::{error::Error, fmt};
use cgp::prelude::*;
use derive_deftly::Deftly;
use futures::future::BoxFuture;
use serde_json::Value;
use crate::{crd::{self, CrdView, CanWaitForEstablishedCrd}, errors::BoxError, logfields::LogEntry,
    utility_logging::{CanEmitSourceLog, LogLevel}, utility_runtime::UtilityRuntimeContext,
    utility_declarations::derive_deftly_template_UtilityCrdProviders};

pub struct RusternetesCrdConnection { pub client: reqwest::Client, pub api_root: url::Url }
#[cgp_getter]
pub trait HasRusternetesCrdConnection { fn rusternetes_crd_connection(&self) -> &RusternetesCrdConnection; }

#[derive(Debug)]
pub struct CrdApiStatusError { pub status_code: u16, pub reason: String, pub message: String, pub body: Vec<u8> }
impl fmt::Display for CrdApiStatusError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.message.is_empty() { out.write_str(&self.message) }
        else if !self.reason.is_empty() { out.write_str(&self.reason) }
        else { write!(out, "Rusternetes CRD request returned HTTP {}", self.status_code) }
    }
}
impl Error for CrdApiStatusError {}

#[cgp_new_provider]
impl<Context> crd::CrdReader<Context> for ReadCrdFromRusternetes
where Context: HasRusternetesCrdConnection + Sync,
{
    fn read_crd<'a>(context: &'a Context, name: &'a str) -> BoxFuture<'a, Result<CrdView, BoxError>> {
        Box::pin(async move {
            validate_resource_name(name)?;
            let connection = context.rusternetes_crd_connection();
            let mut endpoint = connection.api_root.clone();
            endpoint.path_segments_mut().map_err(|_| crate::errors::Errors::default().New("Rusternetes API root is not a base URL"))?
                .pop_if_empty().extend(["apis", "apiextensions.k8s.io", "v1", "customresourcedefinitions", name]);
            let response = connection.client.get(endpoint).send().await?;
            let status = response.status();
            let bytes = response.bytes().await?;
            if !status.is_success() {
                let payload: Option<Value> = serde_json::from_slice(&bytes).ok();
                return Err(Box::new(CrdApiStatusError {
                    status_code: status.as_u16(),
                    reason: payload.as_ref().and_then(|value| value["reason"].as_str()).unwrap_or_default().to_owned(),
                    message: payload.as_ref().and_then(|value| value["message"].as_str()).unwrap_or_default().to_owned(),
                    body: bytes.to_vec(),
                }) as BoxError);
            }
            Ok(serde_json::from_slice(&bytes)?)
        })
    }
}

fn validate_resource_name(name: &str) -> Result<(), BoxError> {
    let errors = crate::errors::Errors::default();
    if name.is_empty() { return Err(errors.New("resource name may not be empty")); }
    let messages = if name == "." { vec!["may not be '.'"] }
        else if name == ".." { vec!["may not be '..'"] }
        else { [('/', "may not contain '/'"), ('%', "may not contain '%'")].into_iter()
            .filter_map(|(part, message)| name.contains(part).then_some(message)).collect() };
    if !messages.is_empty() {
        return Err(errors.New(format!("invalid resource name {}: [{}]",
            agones_core::quote_feature_error_bytes(name.as_bytes()), messages.join(" "))));
    }
    Ok(())
}

#[cgp_new_provider]
impl<Context: HasCrdRuntime> crd::CrdEstablishmentLogger<Context> for LogEstablishedCrd {
    fn log_crd_established(context: &Context, logger: &LogEntry, name: &str) {
        context.crd_runtime().emit_source_log(&logger.with_field("crd", name), LogLevel::Debug, "custom resource definition established");
    }
}

#[derive(HasField, Deftly)]
#[derive_deftly(UtilityCrdProviders)]
pub struct RusternetesCrdContext {
    pub rusternetes_crd_connection: RusternetesCrdConnection,
    pub crd_runtime: UtilityRuntimeContext,
}
#[cgp_getter]
pub trait HasCrdRuntime { fn crd_runtime(&self) -> &UtilityRuntimeContext; }
impl RusternetesCrdContext {
    pub fn new(client: reqwest::Client, api_root: url::Url, runtime: UtilityRuntimeContext) -> Self {
        Self { rusternetes_crd_connection: RusternetesCrdConnection { client, api_root }, crd_runtime: runtime }
    }
    pub async fn wait(&self, name: &str, logger: &LogEntry) -> Result<(), BoxError> {
        self.wait_for_established_crd(name, logger).await
    }
}
