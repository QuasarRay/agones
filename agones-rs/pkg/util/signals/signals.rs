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
    pub fn stop(&self) -> Result<(), BoxError> { self.context.cancel(); self.registration.stop() }
}
pub fn NewSigKillContext<Context>(context: &Context) -> Result<(CancellationToken, SignalCancel), BoxError>
where Context: CanSubscribeSourceSignals + CanSpawnUtilityTask,
{
    // The source comment mentions os.Kill, but its actual set is SIGINT and
    // SIGTERM. The returned context is independent until a signal or stop.
    let mut subscription = context.subscribe_source_signals()?;
    let signal_context = CancellationToken::new();
    let cancel = SignalCancel { context: signal_context.clone(), registration: subscription.registration.clone() };
    let child_context = signal_context.clone();
    let spawn = context.spawn_utility_task("signal NotifyContext", Box::pin(async move {
        tokio::select! {
            _ = child_context.cancelled() => (),
            _ = subscription.owner_stop.cancelled() => (),
            _ = subscription.receiver.recv() => (),
        }
        child_context.cancel();
        // Cancellation alone does not unregister the signal notification.
        Ok(())
    }));
    if let Err(error) = spawn { cancel.stop()?; return Err(error); }
    Ok((signal_context, cancel))
}
pub type SignalHandler = Arc<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync>;
pub fn NewSigTermHandler<Context>(context: &Context, handle: SignalHandler) -> Result<(), BoxError>
where Context: CanSubscribeSourceSignals + CanSpawnUtilityTask,
{
    let mut subscription = context.subscribe_source_signals()?;
    let registration = subscription.registration.clone();
    let spawn = context.spawn_utility_task("one-shot SIGTERM or SIGINT callback", Box::pin(async move {
        tokio::select! {
            _ = subscription.owner_stop.cancelled() => (),
            signal = subscription.receiver.recv() => if signal.is_some() { handle().await; },
        }
        // Source signal.Notify keeps its channel registered after this
        // one-shot callback. The enclosing owner still performs final cleanup.
        Ok(())
    }));
    if let Err(error) = spawn { registration.stop()?; return Err(error); }
    Ok(())
}
