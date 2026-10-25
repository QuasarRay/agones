use std::{error::Error, io, sync::{Arc, Once}};

use agones_core::{
    GameServerAllocation, GameServerAllocationState, GameServerAllocationStatus, ValidationError,
};
use async_trait::async_trait;
use chrono::Utc;
use futures::future::BoxFuture;
use serde::Serialize;
use serde_json::{json, Value};
use tokio::sync::watch;

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, BoxError>;

const API_VERSION: &str = "allocation.agones.dev/v1";
const KIND: &str = "GameServerAllocation";
static VIEWS: Once = Once::new();

fn error(msg: impl Into<String>) -> BoxError { io::Error::other(msg.into()).into() }

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub content_type: String,
    pub accept: String,
    pub body: Vec<u8>,
}

pub struct Response {
    pub status: u16,
    pub content_type: String,
    pub body: Vec<u8>,
}

impl Response {
    fn text(status: u16, body: impl Into<Vec<u8>>) -> Self {
        Self { status, content_type: "text/plain; charset=utf-8".into(), body: body.into() }
    }
}

pub struct ApiResource {
    pub name: &'static str,
    pub singular: &'static str,
    pub namespaced: bool,
    pub kind: &'static str,
    pub verbs: &'static [&'static str],
    pub short_names: &'static [&'static str],
}

type ApiHandler =
    Arc<dyn Fn(Request, String) -> BoxFuture<'static, Result<Response>> + Send + Sync>;

pub trait ApiServer: Send + Sync {
    fn add_api_resource(&self, group_version: &str, resource: ApiResource, handler: ApiHandler);
}

pub trait Codec: Send + Sync {
    fn decode_gsa(&self, content_type: &str, body: &[u8]) -> Result<GameServerAllocation>;
    fn encode(&self, accept: &str, value: &Value) -> Result<(String, Vec<u8>)>;
}

pub enum AllocationResult {
    Status(Value, u16),
    Allocation(GameServerAllocation),
    Other(Value),
}

#[async_trait]
pub trait Allocator: Send + Sync {
    async fn run(&self, stop: watch::Receiver<bool>) -> Result<()>;
    async fn allocate(
        &self,
        stop: watch::Receiver<bool>,
        gsa: GameServerAllocation,
    ) -> Result<AllocationResult>;
}

#[derive(Clone, Copy)]
pub enum RpcCode {
    Canceled, Unknown, InvalidArgument, DeadlineExceeded, NotFound, AlreadyExists,
    PermissionDenied, ResourceExhausted, FailedPrecondition, Aborted, OutOfRange,
    Unimplemented, Internal, Unavailable, DataLoss, Unauthenticated,
}

pub enum ProcessorError {
    Rpc(RpcCode, String),
    Other(String),
}

pub struct ProcessorResponse {
    pub status: GameServerAllocationStatus,
    pub game_server_name: String,
    pub source: String,
}

#[async_trait]
pub trait ProcessorClient: Send + Sync {
    async fn allocate(
        &self,
        stop: watch::Receiver<bool>,
        gsa: &GameServerAllocation,
    ) -> std::result::Result<ProcessorResponse, ProcessorError>;
}

pub struct Extensions {
    api: Arc<dyn ApiServer>,
    codec: Arc<dyn Codec>,
    allocator: Option<Arc<dyn Allocator>>,
    processor: Option<Arc<dyn ProcessorClient>>,
}

impl Extensions {
    pub fn new(
        api: Arc<dyn ApiServer>,
        codec: Arc<dyn Codec>,
        allocator: Arc<dyn Allocator>,
    ) -> Arc<Self> {
        Self::init();
        Arc::new(Self { api, codec, allocator: Some(allocator), processor: None })
    }

    pub fn new_processor(
        api: Arc<dyn ApiServer>,
        codec: Arc<dyn Codec>,
        processor: Arc<dyn ProcessorClient>,
    ) -> Arc<Self> {
        Self::init();
        Arc::new(Self { api, codec, allocator: None, processor: Some(processor) })
    }

    fn init() {
        VIEWS.call_once(|| {});
    }

    pub async fn run(self: &Arc<Self>, stop: watch::Receiver<bool>) -> Result<()> {
        if let Some(allocator) = &self.allocator {
            allocator.run(stop.clone()).await?;
        }
        self.register_api_resource(stop);
        Ok(())
    }

    fn register_api_resource(self: &Arc<Self>, stop: watch::Receiver<bool>) {
        let this = self.clone();
        self.api.add_api_resource(
            API_VERSION,
            ApiResource {
                name: "gameserverallocations",
                singular: "gameserverallocation",
                namespaced: true,
                kind: KIND,
                verbs: &["create"],
                short_names: &["gsa"],
            },
            Arc::new(move |request, namespace| {
                let (this, stop) = (this.clone(), stop.clone());
                Box::pin(async move { this.process_allocation_request(stop, request, namespace).await })
            }),
        );
    }

    async fn process_allocation_request(
        &self,
        stop: watch::Receiver<bool>,
        request: Request,
        namespace: String,
    ) -> Result<Response> {
        if request.method != "POST" {
            return Ok(Response::text(405, "Method not supported\n"));
        }

        let mut gsa = self.deserialize(&request, namespace)?;

        if let Some(processor) = &self.processor {
            let errors = gsa.validate();
            if !errors.is_empty() {
                return self.serialize(&request, invalid_status(&gsa, &errors), 422);
            }

            return match processor.allocate(stop, &gsa).await {
                Ok(response) => {
                    gsa.status = response.status;
                    gsa.status.source = response.source;
                    gsa.metadata.name = response.game_server_name;
                    self.serialize(&request, typed_value(KIND, API_VERSION, &gsa)?, 201)
                }
                Err(error) => {
                    let (value, status) = processor_error(error, &mut gsa)?;
                    self.serialize(&request, value, status)
                }
            };
        }

        let allocator = self.allocator.as_ref()
            .ok_or_else(|| error("Extensions has neither allocator nor processor"))?;

        let (value, status) = match allocator.allocate(stop, gsa).await? {
            AllocationResult::Status(value, status) => (value, status),
            AllocationResult::Allocation(gsa) => (typed_value(KIND, API_VERSION, &gsa)?, 201),
            AllocationResult::Other(value) => (value, 200),
        };
        self.serialize(&request, value, status)
    }

    fn deserialize(&self, request: &Request, namespace: String) -> Result<GameServerAllocation> {
        let mut gsa = self.codec.decode_gsa(&request.content_type, &request.body)
            .map_err(|e| error(format!("error decoding body: {e}")))?;
        gsa.metadata.namespace = namespace;
        gsa.metadata.extensions.insert(
            "creationTimestamp".into(),
            Value::String(Utc::now().to_rfc3339()),
        );
        gsa.apply_defaults();
        Ok(gsa)
    }

    fn serialize(&self, request: &Request, value: Value, status: u16) -> Result<Response> {
        let (content_type, body) = self.codec.encode(&request.accept, &value)
            .map_err(|e| error(format!("error encoding response: {e}")))?;
        Ok(Response { status, content_type, body })
    }
}

fn typed_value<T: Serialize>(kind: &str, api_version: &str, value: &T) -> Result<Value> {
    let mut value = serde_json::to_value(value)?;
    let object = value.as_object_mut().ok_or_else(|| error("API object must serialize as object"))?;
    object.insert("apiVersion".into(), api_version.into());
    object.insert("kind".into(), kind.into());
    Ok(value)
}

fn invalid_status(gsa: &GameServerAllocation, errors: &[ValidationError]) -> Value {
    let causes = errors.iter().map(|e| json!({
        "reason": format!("{:?}", e.kind), "message": e.detail, "field": e.path
    })).collect::<Vec<_>>();
    json!({
        "apiVersion": "v1", "kind": "Status", "status": "Failure",
        "message": format!("{KIND} {:?} is invalid", gsa.metadata.name),
        "reason": "Invalid",
        "details": {
            "name": gsa.metadata.name, "group": "allocation.agones.dev",
            "kind": KIND, "causes": causes
        },
        "code": 422
    })
}

fn processor_error(error: ProcessorError, gsa: &mut GameServerAllocation) -> Result<(Value, u16)> {
    match error {
        ProcessorError::Rpc(RpcCode::ResourceExhausted, _) => {
            gsa.status.state = GameServerAllocationState::Unallocated;
            Ok((typed_value(KIND, API_VERSION, gsa)?, 201))
        }
        ProcessorError::Rpc(RpcCode::Aborted, _) => {
            gsa.status.state = GameServerAllocationState::Contention;
            Ok((typed_value(KIND, API_VERSION, gsa)?, 201))
        }
        ProcessorError::Rpc(code, message) => {
            let status = rpc_http_status(code);
            Ok((failure_status(message, status), status))
        }
        ProcessorError::Other(message) => Ok((failure_status(message, 500), 500)),
    }
}

fn failure_status(message: String, code: u16) -> Value {
    json!({
        "apiVersion": "v1", "kind": "Status", "status": "Failure",
        "message": message, "code": code
    })
}

fn rpc_http_status(code: RpcCode) -> u16 {
    use RpcCode::*;
    match code {
        Canceled => 499,
        InvalidArgument | FailedPrecondition | OutOfRange => 400,
        DeadlineExceeded => 504,
        NotFound => 404,
        AlreadyExists | Aborted => 409,
        PermissionDenied => 403,
        ResourceExhausted => 429,
        Unimplemented => 501,
        Unavailable => 503,
        Unauthenticated => 401,
        Unknown | Internal | DataLoss => 500,
    }
}
