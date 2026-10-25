use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use agones_core::GameServerAllocation;
use async_trait::async_trait;
use futures::future::join_all;
use summer::app::AppBuilder;
use summer_grpc::{GrpcConfigurator, tonic as grpc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, warn};

use crate::{
    allocation::converters::{allocation_request_to_gsa, gsa_to_allocation_response},
    allocationpb::{
        processor_message::Payload, processor_server::{Processor, ProcessorServer},
        response_wrapper::Result as WrapperResult, AllocationRequest, AllocationResponse,
        BatchResponse, ProcessorMessage, PullRequest, RequestWrapper, ResponseWrapper,
    },
    google::rpc::Status as RpcStatus,
};

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;
type GrpcResult<T> = std::result::Result<T, grpc::Status>;
type ClientTx = mpsc::Sender<GrpcResult<ProcessorMessage>>;

#[derive(Debug)]
pub enum AllocationObject {
    Status { code: u16, message: String },
    GameServerAllocation(GameServerAllocation),
    Other(String),
}

#[async_trait]
pub trait GameServerAllocator: Send + Sync {
    async fn allocate(
        &self,
        cancel: CancellationToken,
        gsa: GameServerAllocation,
    ) -> Result<AllocationObject, BoxError>;
}

enum AllocationResult {
    Response(AllocationResponse),
    Error(RpcStatus),
}

#[derive(Clone)]
pub struct Handler {
    allocator: Arc<dyn GameServerAllocator>,
    clients: Arc<RwLock<HashMap<String, ClientTx>>>,
    pull_interval: Duration,
    grpc_unallocated_status_code: grpc::Code,
}

impl Handler {
    pub fn new(
        allocator: Arc<dyn GameServerAllocator>,
        pull_interval: Duration,
        grpc_unallocated_status_code: grpc::Code,
    ) -> Self {
        Self {
            allocator,
            clients: Default::default(),
            pull_interval,
            grpc_unallocated_status_code,
        }
    }

    pub fn register(self, app: &mut AppBuilder) -> &mut AppBuilder {
        app.add_service(ProcessorServer::new(self))
    }

    pub fn start_pull_request_ticker(&self, cancel: CancellationToken) {
        let this = self.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(this.pull_interval);
            loop {
                tokio::select! {
                    _ = cancel.cancelled() => break,
                    _ = ticker.tick() => this.send_pull_requests_to_clients().await,
                }
            }
        });
    }

    async fn send_pull_requests_to_clients(&self) {
        let clients = self.clients.read().await.clone();
        for (client_id, stream) in clients {
            let msg = ProcessorMessage {
                client_id: client_id.clone(),
                payload: Some(Payload::Pull(PullRequest { message: "pull".into() })),
            };
            if stream.send(Ok(msg)).await.is_err() {
                warn!(%client_id, "failed to send pull request; removing client");
                self.remove_client(&client_id).await;
            }
        }
    }

    async fn process_allocations(
        &self,
        cancel: CancellationToken,
        requests: &[RequestWrapper],
    ) -> Vec<AllocationResult> {
        join_all(requests.iter().map(|wrapper| {
            let request = wrapper.request.clone().unwrap_or_default();
            self.process_allocation(cancel.clone(), request)
        })).await
    }

    async fn process_allocation(
        &self,
        cancel: CancellationToken,
        request: AllocationRequest,
    ) -> AllocationResult {
        let mut gsa = allocation_request_to_gsa(&request);
        gsa.apply_defaults();

        let result = match self.allocator.allocate(cancel, gsa).await {
            Ok(result) => result,
            Err(err) => return AllocationResult::Error(rpc_error(err.as_ref(), grpc::Code::Internal)),
        };

        let gsa = match result {
            AllocationObject::Status { code, message } => {
                return AllocationResult::Error(RpcStatus {
                    code: grpc_code_from_http_status(code) as i32,
                    message,
                    details: vec![],
                });
            }
            AllocationObject::GameServerAllocation(gsa) => gsa,
            AllocationObject::Other(value) => {
                return AllocationResult::Error(rpc_status(
                    grpc::Code::Internal,
                    format!("internal server error - Bad GSA format {value}"),
                ));
            }
        };

        match gsa_to_allocation_response(&gsa, self.grpc_unallocated_status_code) {
            Ok(response) => AllocationResult::Response(response),
            Err(err) => AllocationResult::Error(rpc_error(err.as_ref(), grpc::Code::Internal)),
        }
    }

    async fn submit_batch(
        &self,
        cancel: CancellationToken,
        batch_id: String,
        requests: Vec<RequestWrapper>,
    ) -> BatchResponse {
        let results = self.process_allocations(cancel, &requests).await;
        let responses = requests.into_iter().zip(results).map(|(request, result)| {
            ResponseWrapper {
                request_id: request.request_id,
                result: Some(match result {
                    AllocationResult::Response(response) => WrapperResult::Response(response),
                    AllocationResult::Error(error) => WrapperResult::Error(error),
                }),
            }
        }).collect();
        BatchResponse { batch_id, responses }
    }

    async fn add_client(&self, id: String, tx: ClientTx) {
        self.clients.write().await.insert(id, tx);
    }

    async fn remove_client(&self, id: &str) {
        self.clients.write().await.remove(id);
    }
}

#[async_trait]
impl Processor for Handler {
    type StreamBatchesStream = ReceiverStream<GrpcResult<ProcessorMessage>>;

    async fn stream_batches(
        &self,
        request: grpc::Request<grpc::Streaming<ProcessorMessage>>,
    ) -> GrpcResult<grpc::Response<Self::StreamBatchesStream>> {
        let mut inbound = request.into_inner();
        let first = inbound.message().await?
            .ok_or_else(|| grpc::Status::cancelled("stream closed before registration"))?;
        let client_id = first.client_id;
        if client_id.is_empty() {
            return Err(grpc::Status::invalid_argument("clientID is required"));
        }

        let (tx, rx) = mpsc::channel(32);
        self.add_client(client_id.clone(), tx.clone()).await;
        debug!(%client_id, "processor client registered");

        let this = self.clone();
        tokio::spawn(async move {
            let cancel = CancellationToken::new();
            loop {
                let msg = match inbound.message().await {
                    Ok(Some(msg)) => msg,
                    Ok(None) => {
                        debug!(%client_id, "processor stream closed by client");
                        break;
                    }
                    Err(err) => {
                        warn!(%client_id, %err, "processor stream receive error");
                        let _ = tx.send(Err(err)).await;
                        break;
                    }
                };

                let Some(Payload::BatchRequest(batch)) = msg.payload else {
                    warn!(%client_id, "processor received non-batch or empty payload");
                    continue;
                };

                let batch_id = batch.batch_id;
                let requests = batch.requests;
                debug!(%client_id, %batch_id, count = requests.len(), "received batch request");

                let response = this.submit_batch(cancel.clone(), batch_id.clone(), requests).await;
                let msg = ProcessorMessage {
                    client_id: client_id.clone(),
                    payload: Some(Payload::BatchResponse(response)),
                };
                if tx.send(Ok(msg)).await.is_err() {
                    error!(%client_id, %batch_id, "failed to send batch response");
                }
            }

            cancel.cancel();
            this.remove_client(&client_id).await;
        });

        Ok(grpc::Response::new(ReceiverStream::new(rx)))
    }
}

fn rpc_error(err: &(dyn Error + 'static), fallback: grpc::Code) -> RpcStatus {
    let mut current = Some(err);
    while let Some(error) = current {
        if let Some(status) = error.downcast_ref::<grpc::Status>() {
            return rpc_status(status.code(), status.message());
        }
        current = error.source();
    }
    rpc_status(fallback, err.to_string())
}

fn rpc_status(code: grpc::Code, message: impl Into<String>) -> RpcStatus {
    RpcStatus { code: code as i32, message: message.into(), details: vec![] }
}

pub fn grpc_code_from_http_status(code: u16) -> grpc::Code {
    use grpc::Code::*;
    match code {
        200 => Ok,
        499 => Cancelled,
        500 => Internal,
        400 | 422 => InvalidArgument,
        504 => DeadlineExceeded,
        404 => NotFound,
        409 => AlreadyExists,
        403 => PermissionDenied,
        401 => Unauthenticated,
        429 => ResourceExhausted,
        501 => Unimplemented,
        503 => Unavailable,
        _ => {
            warn!(http_status_code = code, "unknown HTTP status; defaulting to ResourceExhausted");
            ResourceExhausted
        }
    }
}

pub const GRPC_POLICY: GrpcPolicy = GrpcPolicy {
    min_ping_interval: Duration::from_secs(10),
    permit_without_stream: true,
    max_connection_idle: Duration::from_secs(60),
    keepalive_interval: Duration::from_secs(30),
    keepalive_timeout: Duration::from_secs(30),
};

pub struct GrpcPolicy {
    pub min_ping_interval: Duration,
    pub permit_without_stream: bool,
    pub max_connection_idle: Duration,
    pub keepalive_interval: Duration,
    pub keepalive_timeout: Duration,
}
