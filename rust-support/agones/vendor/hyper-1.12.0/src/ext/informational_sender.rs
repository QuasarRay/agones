//! Agones compatibility extension: owned server interim response commands.
use std::fmt;
use futures_channel::{mpsc, oneshot};
use http::Response;

/// Per-request server capability attached by the H1/H2 driver.
///
/// Each mutable sender submits a bounded command and awaits its acknowledgement.
/// The connection/stream driver owns the receiver; dropping that driver wakes
/// pending sends with an error. This extension spawns no tasks.
#[derive(Clone)]
pub struct InformationalSender { sender: mpsc::Sender<Command>, http2: bool }
impl fmt::Debug for InformationalSender {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.debug_struct("InformationalSender").finish_non_exhaustive() }
}
/// A rejected or disconnected informational response operation.
#[derive(Clone, Debug)]
pub struct InformationalError(&'static str);
impl InformationalError {
    pub(crate) fn disconnected() -> Self { Self("HTTP informational response driver closed") }
}
impl fmt::Display for InformationalError { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(self.0) } }
impl std::error::Error for InformationalError {}
pub(crate) type Acknowledgement = oneshot::Sender<Result<(), InformationalError>>;
#[derive(Debug)]
pub(crate) enum Command {
    Response { head: Response<()>, done: Acknowledgement },
    DisableContinue { done: Acknowledgement },
}
pub(crate) type Receiver = mpsc::Receiver<Command>;
#[derive(Clone, Debug)]
pub(crate) struct ServerInterim;
pub(crate) fn channel(http2: bool) -> (InformationalSender, Receiver) {
    // futures mpsc reserves one slot per sender. A source ResponseWriter has
    // one mutable sender and at most one outstanding header operation.
    let (send, receive) = mpsc::channel(0);
    (InformationalSender { sender: send, http2 }, receive)
}
impl InformationalSender {
    /// Send a non-terminal 1xx response. HTTP/1 101 uses the normal upgrade path.
    pub async fn send(&mut self, response: Response<()>) -> Result<(), InformationalError> {
        if !response.status().is_informational() || !self.http2 && response.status() == http::StatusCode::SWITCHING_PROTOCOLS {
            return Err(InformationalError("not a non-terminal informational response"));
        }
        let (done, result) = oneshot::channel();
        self.submit(Command::Response { head: response, done }).await?;
        result.await.map_err(|_| InformationalError::disconnected())?
    }
    /// Disable a deferred, automatically generated HTTP/1 100 Continue.
    pub async fn disable_continue(&mut self) -> Result<(), InformationalError> {
        let (done, result) = oneshot::channel();
        self.submit(Command::DisableContinue { done }).await?;
        result.await.map_err(|_| InformationalError::disconnected())?
    }
    async fn submit(&mut self, command: Command) -> Result<(), InformationalError> {
        crate::common::future::poll_fn(|cx| self.sender.poll_ready(cx)).await.map_err(|_| InformationalError::disconnected())?;
        self.sender.start_send(command).map_err(|_| InformationalError::disconnected())
    }
}
