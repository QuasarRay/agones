use std::{collections::BTreeMap, time::Duration};

use prometheus::{process_collector::ProcessCollector, Registry, TextEncoder};
use summer_web::axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::get, Router,
};

use super::parse_labels;

#[derive(Clone, Default)]
pub struct Config {
    pub gcp_project_id: String,
    pub stackdriver_labels: String,
    pub stackdriver: bool,
    pub prometheus_metrics: bool,
}

#[derive(Clone)]
pub struct CloudConfig {
    pub project_id: String,
    pub labels: BTreeMap<String, String>,
}

#[derive(Clone)]
pub struct Metrics {
    pub registry: Registry,
    pub cloud: Option<CloudConfig>,
    pub period: Option<Duration>,
}

impl Metrics {
    pub fn setup(c: Config) -> Result<(Self, Router), Box<dyn std::error::Error>> {
        let registry = Registry::new();
        if c.prometheus_metrics {
            registry.register(Box::new(ProcessCollector::for_self()))?;
        }
        let cloud = c.stackdriver.then(|| {
            Ok::<_, String>(CloudConfig {
                project_id: c.gcp_project_id,
                labels: parse_labels(&c.stackdriver_labels)?,
            })
        }).transpose()?;
        let period = if c.stackdriver { Some(Duration::from_secs(60)) }
            else if c.prometheus_metrics { Some(Duration::from_secs(15)) } else { None };
        let this = Self { registry, cloud, period };
        let router = if c.prometheus_metrics || c.stackdriver {
            Router::new().route("/metrics", get(scrape)).route("/healthz", get(health))
                .with_state(this.clone())
        } else {
            Router::new().route("/healthz", get(|| async { StatusCode::OK }))
        };
        Ok((this, router))
    }
}

async fn scrape(State(m): State<Metrics>) -> impl IntoResponse {
    match TextEncoder::new().encode_to_string(&m.registry.gather()) {
        Ok(body) => (StatusCode::OK, body),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

async fn health(State(_): State<Metrics>) -> StatusCode { StatusCode::OK }