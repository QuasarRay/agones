// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/crd/crd.go. Copyright Contributors to Agones a Series of LF
// Projects, LLC. Apache-2.0.
#![allow(non_snake_case)]
use std::{error::Error, fmt, time::Duration};
use cgp::prelude::*;
use futures::future::BoxFuture;
use serde::Deserialize;
use tokio::time::{Instant, Interval};
use crate::{errors::BoxError, logfields::LogEntry};

/// Only fields read by this source helper cross its capability boundary.
/// Optional strings/lists accept source JSON null without conflating it with
/// a parse error; no unknown CRD field is rewritten or sent back to a server.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CrdMetadata { pub name: Option<String> }
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CrdCondition {
    #[serde(rename = "type")]
    pub condition_type: Option<String>,
    pub status: Option<String>,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CrdStatus { pub conditions: Option<Vec<CrdCondition>> }
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CrdView { pub metadata: Option<CrdMetadata>, pub status: Option<CrdStatus> }

#[derive(Debug)]
pub struct CrdDeadlineExceeded;
impl fmt::Display for CrdDeadlineExceeded {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("context deadline exceeded") }
}
impl Error for CrdDeadlineExceeded {}

pub struct CrdPollTimer { pub deadline: Instant, pub interval: Interval }
#[cgp_component(CrdPollClock)]
pub trait CanCreateCrdPollTimer {
    fn create_crd_poll_timer(&self) -> CrdPollTimer;
    fn crd_poll_now(&self) -> Instant;
}
#[cgp_new_provider]
impl<Context> CrdPollClock<Context> for UseTokioCrdPollClock {
    fn create_crd_poll_timer(_context: &Context) -> CrdPollTimer {
        let deadline = Instant::now() + Duration::from_secs(60);
        let first_tick = Instant::now() + Duration::from_secs(1);
        let mut interval = tokio::time::interval_at(first_tick, Duration::from_secs(1));
        // Like a Go Ticker, coalesce missed intervals instead of replaying
        // every missed tick after a slow GET.
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        CrdPollTimer { deadline, interval }
    }
    fn crd_poll_now(_context: &Context) -> Instant { Instant::now() }
}

#[cgp_component(CrdReader)]
pub trait CanReadCrd: Sync {
    fn read_crd<'a>(&'a self, name: &'a str) -> BoxFuture<'a, Result<CrdView, BoxError>>;
}
#[cgp_component(CrdEstablishmentLogger)]
pub trait CanLogCrdEstablished {
    fn log_crd_established(&self, logger: &LogEntry, name: &str);
}

pub async fn WaitForEstablishedCRD<Context>(context: &Context,
    _parent: Option<&tokio::sync::watch::Receiver<bool>>, name: &str, logger: &LogEntry) -> Result<(), BoxError>
where Context: CanCreateCrdPollTimer + CanReadCrd + CanLogCrdEstablished,
{
    // The source deliberately uses context.Background(), ignoring its caller's
    // cancellation. This owned timer is the independent 60-second deadline.
    let mut timer = context.create_crd_poll_timer();
    loop {
        // Immediate=true guarantees the first condition call. Each GET future
        // remains inside the owned deadline and is dropped on timeout.
        let crd = tokio::time::timeout_at(timer.deadline, context.read_crd(name)).await
            .map_err(|_| Box::new(CrdDeadlineExceeded) as BoxError)??;
        if let Some(status) = &crd.status {
            for condition in status.conditions.iter().flatten() {
                if condition.condition_type.as_deref() == Some("Established") && condition.status.as_deref() == Some("True") {
                    let name = crd.metadata.as_ref().and_then(|metadata| metadata.name.as_deref()).unwrap_or_default();
                    context.log_crd_established(logger, name);
                    return Ok(());
                }
            }
        }
        tokio::select! {
            _ = tokio::time::sleep_until(timer.deadline) => return Err(Box::new(CrdDeadlineExceeded)),
            _ = timer.interval.tick() => (),
        }
        // Go wait checks ctx.Err again after select so a ready tick cannot
        // repeatedly win against an expired deadline.
        if context.crd_poll_now() >= timer.deadline { return Err(Box::new(CrdDeadlineExceeded)); }
    }
}

#[cgp_component(CrdEstablishmentWaiter)]
pub trait CanWaitForEstablishedCrd: Sync {
    fn wait_for_established_crd<'a>(&'a self, name: &'a str, logger: &'a LogEntry) -> BoxFuture<'a, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl<Context> CrdEstablishmentWaiter<Context> for WaitForSourceCrdEstablishment
where Context: CanCreateCrdPollTimer + CanReadCrd + CanLogCrdEstablished + Sync,
{
    fn wait_for_established_crd<'a>(context: &'a Context, name: &'a str, logger: &'a LogEntry) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(WaitForEstablishedCRD(context, None, name, logger))
    }
}
