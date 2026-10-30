//! Agones process-signal subscriptions supplied by Tokio.
//! The CGP runtime owns cancellation and child tasks. Tokio owns native signal
//! registration, delivery and coalescing; subscriptions own their stream values.
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use cgp::prelude::*;
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, utility_scope::CanSpawnUtilityTask};

#[derive(Default)]
pub struct SourceSignalState { closing: AtomicBool, stop: CancellationToken }
impl SourceSignalState {
    pub fn close(&self) { self.closing.store(true, Ordering::Release); self.stop.cancel(); }
}
impl Drop for SourceSignalState { fn drop(&mut self) { self.stop.cancel(); } }

#[derive(Clone)]
pub struct SignalRegistration { stop: CancellationToken }
impl SignalRegistration {
    pub fn stop(&self) { self.stop.cancel(); }
    pub(crate) fn child_token(&self) -> CancellationToken { self.stop.child_token() }
}
pub struct SignalSubscription {
    receiver: NativeSignals,
    pub registration: SignalRegistration,
    pub owner_stop: CancellationToken,
}
impl SignalSubscription {
    /// Cancellation wins when shutdown and a signal are both ready. Returning
    /// releases the owned native streams once the consuming task finishes.
    pub async fn recv(&mut self) -> Option<()> {
        tokio::select! {
            biased;
            _ = self.registration.stop.cancelled() => None,
            _ = self.owner_stop.cancelled() => None,
            signal = self.receiver.recv() => signal,
        }
    }
}

struct NativeSignals {
    #[cfg(unix)] interrupt: tokio::signal::unix::Signal,
    #[cfg(unix)] terminate: tokio::signal::unix::Signal,
    #[cfg(windows)] interrupt: tokio::signal::windows::CtrlC,
    #[cfg(windows)] break_event: tokio::signal::windows::CtrlBreak,
    #[cfg(windows)] close: tokio::signal::windows::CtrlClose,
    #[cfg(windows)] logoff: tokio::signal::windows::CtrlLogoff,
    #[cfg(windows)] shutdown: tokio::signal::windows::CtrlShutdown,
}
impl NativeSignals {
    fn open() -> Result<Self, BoxError> {
        #[cfg(unix)] {
            use tokio::signal::unix::{signal, SignalKind};
            Ok(Self { interrupt: signal(SignalKind::interrupt())?, terminate: signal(SignalKind::terminate())? })
        }
        #[cfg(windows)] {
            use tokio::signal::windows;
            Ok(Self { interrupt: windows::ctrl_c()?, break_event: windows::ctrl_break()?,
                close: windows::ctrl_close()?, logoff: windows::ctrl_logoff()?, shutdown: windows::ctrl_shutdown()? })
        }
        #[cfg(not(any(unix, windows)))] {
            Err(Errors::default().New("process signal subscriptions are unsupported on this Rust target"))
        }
    }
    async fn recv(&mut self) -> Option<()> {
        #[cfg(unix)] {
            tokio::select! { signal = self.interrupt.recv() => signal, signal = self.terminate.recv() => signal }
        }
        #[cfg(windows)] {
            tokio::select! {
                signal = self.interrupt.recv() => signal,
                signal = self.break_event.recv() => signal,
                signal = self.close.recv() => signal,
                signal = self.logoff.recv() => signal,
                signal = self.shutdown.recv() => signal,
            }
        }
        #[cfg(not(any(unix, windows)))] { None }
    }
}

#[cgp_getter]
pub trait HasSourceSignalState { fn source_signal_state(&self) -> &Arc<SourceSignalState>; }
#[cgp_component(SourceSignalSubscriber)]
pub trait CanSubscribeSourceSignals { fn subscribe_source_signals(&self) -> Result<SignalSubscription, BoxError>; }
#[cgp_new_provider]
impl<Context> SourceSignalSubscriber<Context> for SubscribeTokioProcessSignals
where Context: HasSourceSignalState + CanSpawnUtilityTask,
{
    fn subscribe_source_signals(context: &Context) -> Result<SignalSubscription, BoxError> {
        let state = context.source_signal_state();
        if state.closing.load(Ordering::Acquire) { return Err(Errors::default().New("process signal owner is closing")); }
        let owner_stop = context.utility_child_token()?;
        let registration = SignalRegistration { stop: state.stop.child_token() };
        let receiver = NativeSignals::open()?;
        if registration.stop.is_cancelled() || owner_stop.is_cancelled() {
            return Err(Errors::default().New("process signal owner is closing"));
        }
        Ok(SignalSubscription { receiver, registration, owner_stop })
    }
}
