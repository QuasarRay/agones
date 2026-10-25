use std::{
    collections::HashMap,
    error::Error,
    io,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc, Mutex, MutexGuard,
    },
    time::Duration,
};

use async_trait::async_trait;
use summer_grpc::tonic as grpc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{
    allocationpb::{
        processor_client::ProcessorClient as RpcProcessorClient,
        processor_message::Payload,
        response_wrapper::Result as WrapperResult,
        AllocationRequest, AllocationResponse, BatchRequest, BatchResponse,
        ProcessorMessage, PullRequest, RequestWrapper,
    },
    grpc_health::{
        health_check_response::ServingStatus,
        health_client::HealthClient,
        HealthCheckRequest,
    },
};

const MAX_SEND_RETRIES: u8 = 3;

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, BoxError>;
type AllocationResult = std::result::Result<AllocationResponse, grpc::Status>;
type Outbound = mpsc::Sender<ProcessorMessage>;

#[derive(Clone, Debug)]
pub struct Config {
    pub client_id: String,
    pub processor_address: String,
    pub max_batch_size: usize,
    pub allocation_timeout: Duration,
    pub reconnect_interval: Duration,
}

impl Config {
    fn normalize(mut self) -> Self {
        if self.client_id.is_empty() {
            self.client_id = Uuid::new_v4().to_string();
        }
        self
    }
}

struct Pending {
    id: String,
    request: AllocationRequest,
    result: mpsc::Sender<AllocationResult>,
    send_retries: AtomicU8,
}

impl Pending {
    fn wrapper(&self) -> RequestWrapper {
        RequestWrapper {
            request_id: self.id.clone(),
            request: Some(self.request.clone()),
        }
    }
}

#[derive(Default)]
struct BatchState {
    hot: Vec<Arc<Pending>>,
    requests: HashMap<String, Arc<Pending>>,
}

struct Inner {
    config: Config,
    state: Mutex<BatchState>,
}

#[derive(Clone)]
pub struct Client(Arc<Inner>);

#[async_trait]
pub trait ProcessorClient: Send + Sync {
    async fn run(&self, cancel: CancellationToken) -> Result<()>;
    async fn allocate(
        &self,
        cancel: CancellationToken,
        request: AllocationRequest,
    ) -> AllocationResult;
}

impl Client {
    pub fn new(config: Config) -> Self {
        let config = config.normalize();
        Self(Arc::new(Inner {
            state: Mutex::new(BatchState {
                hot: Vec::with_capacity(config.max_batch_size),
                requests: HashMap::new(),
            }),
            config,
        }))
    }

    fn state(&self) -> MutexGuard<'_, BatchState> {
        self.0.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    async fn run_inner(&self, cancel: CancellationToken) -> Result<()> {
        info!(client_id = %self.0.config.client_id, "starting processor client");

        loop {
            if cancel.is_cancelled() {
                return Err(grpc::Status::cancelled("processor client stopping").into());
            }

            if let Err(err) = self.connect_and_run(cancel.clone()).await {
                if cancel.is_cancelled() {
                    return Err(grpc::Status::cancelled("processor client stopping").into());
                }

                error!(%err, "processor connection failed; retrying");
                tokio::select! {
                    _ = cancel.cancelled() =>
                        return Err(grpc::Status::cancelled("processor client stopping").into()),
                    _ = tokio::time::sleep(self.0.config.reconnect_interval) => {}
                }
            }
        }
    }

    async fn allocate_inner(
        &self,
        cancel: CancellationToken,
        request: AllocationRequest,
    ) -> AllocationResult {
        let id = Uuid::new_v4().to_string();
        let (tx, mut rx) = mpsc::channel(1);
        let pending = Arc::new(Pending {
            id: id.clone(),
            request,
            result: tx,
            send_retries: AtomicU8::new(0),
        });

        {
            let mut state = self.state();
            state.requests.insert(id.clone(), pending.clone());
            state.hot.push(pending);
        }

        let outcome = tokio::select! {
            result = rx.recv() => result.unwrap_or_else(||
                Err(grpc::Status::unavailable("processor response channel closed"))),
            _ = cancel.cancelled() =>
                Err(grpc::Status::cancelled("allocation cancelled")),
            _ = tokio::time::sleep(self.0.config.allocation_timeout) =>
                Err(grpc::Status::deadline_exceeded(format!(
                    "allocation timeout after {:?}", self.0.config.allocation_timeout
                ))),
        };

        if matches!(
            outcome,
            Err(ref e) if matches!(e.code(), grpc::Code::Cancelled | grpc::Code::DeadlineExceeded)
        ) {
            self.state().requests.remove(&id);
        }

        outcome
    }

    async fn connect_and_run(&self, cancel: CancellationToken) -> Result<()> {
        let channel = self.connect(cancel.clone()).await?;
        let mut client = RpcProcessorClient::new(channel);
        let (outbound, requests) = mpsc::channel(8);
        let inbound = client
            .stream_batches(grpc::Request::new(ReceiverStream::new(requests)))
            .await
            .map_err(|e| boxed("failed to create stream", e))?
            .into_inner();

        self.register_client(&outbound).await?;
        info!(client_id = %self.0.config.client_id, "connected to processor");
        self.handle_stream(cancel, inbound, outbound).await
    }

    async fn connect(&self, cancel: CancellationToken) -> Result<grpc::transport::Channel> {
        info!("attempting processor connection");
        let address = if self.0.config.processor_address.contains("://") {
            self.0.config.processor_address.clone()
        } else {
            format!("http://{}", self.0.config.processor_address)
        };

        let endpoint = grpc::transport::Endpoint::from_shared(address)?;
        let channel = tokio::select! {
            _ = cancel.cancelled() =>
                return Err(grpc::Status::cancelled("connection cancelled").into()),
            result = endpoint.connect() => result?,
        };

        self.health_check(cancel, channel.clone()).await?;
        info!("successfully connected to processor");
        Ok(channel)
    }

    async fn health_check(
        &self,
        cancel: CancellationToken,
        channel: grpc::transport::Channel,
    ) -> Result<()> {
        let mut client = HealthClient::new(channel);
        let request = grpc::Request::new(HealthCheckRequest {
            service: "processor".into(),
        });

        let response = tokio::select! {
            _ = cancel.cancelled() =>
                return Err(grpc::Status::cancelled("health check cancelled").into()),
            result = tokio::time::timeout(Duration::from_secs(5), client.check(request)) =>
                result.map_err(|_| grpc::Status::deadline_exceeded("processor health check timeout"))??,
        };

        let status = ServingStatus::try_from(response.into_inner().status)
            .unwrap_or(ServingStatus::Unknown);
        if status != ServingStatus::Serving {
            return Err(io::Error::other(format!("processor not serving: {status:?}")).into());
        }
        Ok(())
    }

    async fn register_client(&self, outbound: &Outbound) -> Result<()> {
        info!(client_id = %self.0.config.client_id, "registering processor client");
        outbound
            .send(ProcessorMessage {
                client_id: self.0.config.client_id.clone(),
                payload: None,
            })
            .await
            .map_err(|e| io::Error::other(format!("failed to register client: {e}")))?;
        Ok(())
    }

    async fn handle_stream(
        &self,
        cancel: CancellationToken,
        mut inbound: grpc::Streaming<ProcessorMessage>,
        outbound: Outbound,
    ) -> Result<()> {
        info!("starting processor stream handling");
        let (pull_tx, pull_rx) = mpsc::channel(1);
        let pull_cancel = cancel.child_token();
        let pull_task = tokio::spawn(
            self.clone().pull_request_handler(pull_cancel.clone(), outbound, pull_rx),
        );

        let result = loop {
            let message = tokio::select! {
                _ = cancel.cancelled() =>
                    break Err(grpc::Status::cancelled("stream handling cancelled").into()),
                message = inbound.message() => match message {
                    Ok(Some(message)) => message,
                    Ok(None) => break Err(grpc::Status::unavailable("processor stream closed").into()),
                    Err(error) => break Err(boxed("stream recv error", error)),
                }
            };

            match message.payload {
                Some(Payload::Pull(_)) => match pull_tx.try_send(()) {
                    Ok(()) => debug!("pull request queued"),
                    Err(mpsc::error::TrySendError::Full(_)) =>
                        warn!("pull request queue full; dropping request"),
                    Err(mpsc::error::TrySendError::Closed(_)) =>
                        break Err(grpc::Status::unavailable("pull request handler stopped").into()),
                },
                Some(Payload::BatchResponse(response)) => self.handle_batch_response(response),
                _ => warn!("received unknown processor message"),
            }
        };

        pull_cancel.cancel();
        let _ = pull_task.await;
        result
    }

    async fn pull_request_handler(
        self,
        cancel: CancellationToken,
        outbound: Outbound,
        mut pulls: mpsc::Receiver<()>,
    ) {
        debug!("starting async pull request handler");
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                pull = pulls.recv() => match pull {
                    Some(()) => self.handle_pull_request(&outbound).await,
                    None => break,
                }
            }
        }
    }

    async fn handle_pull_request(&self, outbound: &Outbound) {
        let ready = {
            let mut state = self.state();
            let hot = std::mem::replace(
                &mut state.hot,
                Vec::with_capacity(self.0.config.max_batch_size),
            );
            hot.into_iter()
                .filter(|request| {
                    let live = state.requests.contains_key(&request.id);
                    if !live {
                        debug!(request_id = %request.id, "dropping stale request from batch");
                    }
                    live
                })
                .collect::<Vec<_>>()
        };

        if ready.is_empty() {
            debug!("no requests to send in batch");
            return;
        }

        self.send_batch(outbound, ready).await;
    }

    async fn send_batch(&self, outbound: &Outbound, requests: Vec<Arc<Pending>>) {
        let batch_id = Uuid::new_v4().to_string();
        let batch = BatchRequest {
            batch_id: batch_id.clone(),
            requests: requests.iter().map(|r| r.wrapper()).collect(),
        };
        let message = ProcessorMessage {
            client_id: self.0.config.client_id.clone(),
            payload: Some(Payload::BatchRequest(batch)),
        };
        let started = std::time::Instant::now();

        if let Err(err) = outbound.send(message).await {
            error!(%err, %batch_id, "failed to send processor batch");
            let mut state = self.state();

            for request in requests {
                let retries = request.send_retries.fetch_add(1, Ordering::AcqRel) + 1;
                if retries > MAX_SEND_RETRIES {
                    warn!(request_id = %request.id, "request exceeded max send retries");
                    state.requests.remove(&request.id);
                    let _ = request.result.try_send(Err(grpc::Status::unavailable(format!(
                        "failed to send after {MAX_SEND_RETRIES} retries"
                    ))));
                } else {
                    state.hot.push(request);
                }
            }
            return;
        }

        debug!(
            %batch_id,
            request_count = requests.len(),
            send_duration = ?started.elapsed(),
            "processor batch sent"
        );
    }

    fn handle_batch_response(&self, response: BatchResponse) {
        debug!(
            batch_id = %response.batch_id,
            response_count = response.responses.len(),
            "processing processor batch response"
        );

        let mut success = 0usize;
        let mut errors = 0usize;
        let mut missing = 0usize;

        for wrapper in response.responses {
            let id = wrapper.request_id;
            let pending = self.state().requests.get(&id).cloned();

            let Some(pending) = pending else {
                missing += 1;
                warn!(request_id = %id, batch_id = %response.batch_id,
                    "no pending request found for response");
                continue;
            };

            let result = match wrapper.result {
                Some(WrapperResult::Response(response)) => {
                    success += 1;
                    Ok(response)
                }
                Some(WrapperResult::Error(error)) => {
                    errors += 1;
                    Err(grpc::Status::new(
                        grpc::Code::from_i32(error.code),
                        error.message,
                    ))
                }
                None => {
                    errors += 1;
                    Err(grpc::Status::internal("empty response from processor"))
                }
            };

            if pending.result.try_send(result).is_ok() {
                self.state().requests.remove(&id);
                debug!(request_id = %id, "processor request cleaned up");
            } else {
                warn!(request_id = %id, "keeping request after failed response delivery");
            }
        }

        debug!(
            batch_id = %response.batch_id,
            success_count = success,
            error_count = errors,
            not_found_count = missing,
            "batch response processing completed"
        );
    }

    fn drain_pending_requests(&self) {
        let mut state = self.state();
        for request in state.requests.values() {
            let _ = request
                .result
                .try_send(Err(grpc::Status::unavailable("processor client shutting down")));
        }
        state.requests.clear();
        state.hot.clear();
        info!("drained processor requests on shutdown");
    }
}

#[async_trait]
impl ProcessorClient for Client {
    async fn run(&self, cancel: CancellationToken) -> Result<()> {
        let result = self.run_inner(cancel).await;
        self.drain_pending_requests();
        result
    }

    async fn allocate(
        &self,
        cancel: CancellationToken,
        request: AllocationRequest,
    ) -> AllocationResult {
        self.allocate_inner(cancel, request).await
    }
}

fn boxed(context: &str, error: impl Error) -> BoxError {
    io::Error::other(format!("{context}: {error}")).into()
}