// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/signals/signals.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case)]
use std::sync::Arc;
use futures::future::BoxFuture;
use tokio_util::sync::CancellationToken;
use crate::{errors::BoxError, utility_scope::CanSpawnUtilityTask,
    utility_signals::{CanSubscribeSourceSignals, SignalRegistration}};

#[derive(Clone)]
pub struct SignalCancel { context: CancellationToken, registration: SignalRegistration }
impl SignalCancel {
    pub fn stop(&self) { self.context.cancel(); self.registration.stop(); }
}
pub fn NewSigKillContext<Context>(context: &Context) -> Result<(CancellationToken, SignalCancel), BoxError>
where Context: CanSubscribeSourceSignals + CanSpawnUtilityTask,
{
    // The Agones operation listens for interrupt and termination. On Unix
    // these are SIGINT and SIGTERM; SIGKILL cannot be subscribed to.
    let mut subscription = context.subscribe_source_signals()?;
    let signal_context = subscription.registration.child_token();
    let cancel = SignalCancel { context: signal_context.clone(), registration: subscription.registration.clone() };
    let child_context = signal_context.clone();
    let cancel_on_drop = child_context.clone().drop_guard();
    let spawn = context.spawn_utility_task("signal cancellation context", Box::pin(async move {
        // Capture the guard before spawning so even an unpolled, rejected or
        // aborted task cancels the returned context through native RAII.
        let _cancel_on_drop = cancel_on_drop;
        tokio::select! {
            biased;
            _ = child_context.cancelled() => (),
            _ = subscription.recv() => (),
        }
        child_context.cancel();
        Ok(())
    }));
    if let Err(error) = spawn { cancel.stop(); return Err(error); }
    Ok((signal_context, cancel))
}
pub type SignalHandler = Arc<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync>;
pub fn NewSigTermHandler<Context>(context: &Context, handle: SignalHandler) -> Result<(), BoxError>
where Context: CanSubscribeSourceSignals + CanSpawnUtilityTask,
{
    let mut subscription = context.subscribe_source_signals()?;
    context.spawn_utility_task("one-shot process signal callback", Box::pin(async move {
        if subscription.recv().await.is_some() { handle().await; }
        Ok(())
    }))
}
