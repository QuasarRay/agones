use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};

use agones_core::{
    ClusterConnectionInfo, ConnectionInfoIterator, GameServer, GameServerAllocation,
    GameServerAllocationPolicy, GameServerAllocationState, GameServerMetadata, GameServerState,
    SchedulingStrategy, ValidationError,
};
use async_trait::async_trait;
use rand::Rng;
use rusternetes_storage::Storage;
use serde_json::{json, Value};
use summer_grpc::tonic as grpc;
use tokio::{
    sync::{mpsc, oneshot, watch, Mutex, Semaphore},
    time::{sleep, timeout},
};
use tracing::{debug, error, warn};

use crate::{AdapterError, AgonesRepository};
use crate::allocationpb::{
    allocation_service_client::AllocationServiceClient, AllocationRequest, AllocationResponse,
};
use crate::allocation::converters::{
    allocation_response_to_gsa, gsa_to_allocation_request,
};

const LAST_ALLOCATED_ANNOTATION: &str = "agones.dev/last-allocated";
const ALLOCATOR_PORT: &str = "443";
const MAX_BATCH_QUEUE: usize = 100;
const MAX_BATCH_BEFORE_REFRESH: usize = 100;

pub type Result<T> = std::result::Result<T, AllocatorError>;

#[derive(Debug)]
pub enum AllocatorError {
    NoGameServer,
    ConflictInGameServerSelection,
    TotalTimeoutExceeded,
    GameServerUpdateConflict,
    Adapter(AdapterError),
    Grpc(grpc::Status),
    Message(String),
}

impl fmt::Display for AllocatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoGameServer => f.write_str("Could not find an Allocatable GameServer"),
            Self::ConflictInGameServerSelection =>
                f.write_str("The Gameserver was already allocated"),
            Self::TotalTimeoutExceeded =>
                f.write_str("remote allocation total timeout exceeded"),
            Self::GameServerUpdateConflict =>
                f.write_str("could not update the selected GameServer"),
            Self::Adapter(e) => e.fmt(f),
            Self::Grpc(e) => e.fmt(f),
            Self::Message(e) => f.write_str(e),
        }
    }
}

impl Error for AllocatorError {}
impl From<AdapterError> for AllocatorError {
    fn from(value: AdapterError) -> Self { Self::Adapter(value) }
}
impl From<grpc::Status> for AllocatorError {
    fn from(value: grpc::Status) -> Self { Self::Grpc(value) }
}

#[derive(Debug)]
pub enum AllocationOutput {
    Status { code: u16, value: Value },
    Allocation(GameServerAllocation),
}

#[async_trait]
pub trait AllocationCache: Send + Sync {
    async fn run(&self, cancel: watch::Receiver<bool>) -> Result<()>;
    fn resync(&self);
    fn list_sorted_game_servers(&self, gsa: &GameServerAllocation) -> Result<Vec<Arc<GameServer>>>;
    fn list_sorted_game_servers_priorities(
        &self,
        gsa: &GameServerAllocation,
    ) -> Result<Vec<Arc<GameServer>>>;
    fn remove_game_server(&self, gs: &GameServer) -> Result<()>;
    fn add_game_server(&self, gs: &GameServer);
}

#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn get_secret(&self, namespace: &str, name: &str) -> Result<SecretData>;
}

#[derive(Clone, Default)]
pub struct SecretData {
    pub data: BTreeMap<String, Vec<u8>>,
}

pub trait EventRecorder: Send + Sync {
    fn warning(&self, gs: &GameServer, reason: &str, message: &str);
    fn normal(&self, gs: &GameServer, reason: &str, message: &str);
}

pub trait AllocationMetrics: Send + Sync {
    fn allocation(
        &self,
        request: &GameServerAllocation,
        response: Option<&AllocationOutput>,
        error: Option<&AllocatorError>,
        elapsed: Duration,
    );
    fn retry_success(&self, retries: usize);
}

pub struct NoopRecorder;
impl EventRecorder for NoopRecorder {
    fn warning(&self, _: &GameServer, _: &str, _: &str) {}
    fn normal(&self, _: &GameServer, _: &str, _: &str) {}
}

pub struct NoopMetrics;
impl AllocationMetrics for NoopMetrics {
    fn allocation(
        &self,
        _: &GameServerAllocation,
        _: Option<&AllocationOutput>,
        _: Option<&AllocatorError>,
        _: Duration,
    ) {}
    fn retry_success(&self, _: usize) {}
}

#[derive(Clone)]
pub struct RemoteTls {
    pub client_cert: Vec<u8>,
    pub client_key: Vec<u8>,
    pub ca_bundle: Vec<u8>,
}

#[async_trait]
pub trait RemoteAllocator: Send + Sync {
    async fn allocate(
        &self,
        endpoint: &str,
        tls: &RemoteTls,
        request: AllocationRequest,
        call_timeout: Duration,
    ) -> Result<AllocationResponse>;
}

pub struct SummerGrpcRemoteAllocator;

#[async_trait]
impl RemoteAllocator for SummerGrpcRemoteAllocator {
    async fn allocate(
        &self,
        endpoint: &str,
        tls: &RemoteTls,
        request: AllocationRequest,
        call_timeout: Duration,
    ) -> Result<AllocationResponse> {
        let uri = if endpoint.contains("://") {
            endpoint.to_owned()
        } else {
            format!("https://{endpoint}")
        };

        let mut cfg = grpc::transport::ClientTlsConfig::new().identity(
            grpc::transport::Identity::from_pem(&tls.client_cert, &tls.client_key),
        );
        if !tls.ca_bundle.is_empty() {
            cfg = cfg.ca_certificate(grpc::transport::Certificate::from_pem(&tls.ca_bundle));
        }

        let endpoint = grpc::transport::Endpoint::from_shared(uri)
            .map_err(|e| AllocatorError::Message(e.to_string()))?
            .tls_config(cfg)
            .map_err(|e| AllocatorError::Message(e.to_string()))?;

        let channel = timeout(call_timeout, endpoint.connect())
            .await
            .map_err(|_| grpc::Status::deadline_exceeded("remote allocation timeout"))?
            .map_err(|e| AllocatorError::Message(e.to_string()))?;

        let response = timeout(
            call_timeout,
            AllocationServiceClient::new(channel).allocate(grpc::Request::new(request)),
        )
        .await
        .map_err(|_| grpc::Status::deadline_exceeded("remote allocation timeout"))??;

        Ok(response.into_inner())
    }
}

struct Request {
    gsa: GameServerAllocation,
    reply: oneshot::Sender<Result<GameServer>>,
    cancel: watch::Receiver<bool>,
}

pub struct Allocator<S: Storage + 'static> {
    storage: Arc<S>,
    cache: Arc<dyn AllocationCache>,
    secrets: Arc<dyn SecretStore>,
    recorder: Arc<dyn EventRecorder>,
    metrics: Arc<dyn AllocationMetrics>,
    remote: Arc<dyn RemoteAllocator>,
    pending_tx: mpsc::Sender<Request>,
    pending_rx: Mutex<mpsc::Receiver<Request>>,
    remote_timeout: Duration,
    total_remote_timeout: Duration,
    batch_wait: Duration,
    list_max_capacity: i64,
    is_conflict: fn(&AdapterError) -> bool,
}

impl<S: Storage + 'static> Allocator<S> {
    pub fn new(
        storage: Arc<S>,
        cache: Arc<dyn AllocationCache>,
        secrets: Arc<dyn SecretStore>,
        recorder: Arc<dyn EventRecorder>,
        metrics: Arc<dyn AllocationMetrics>,
        remote: Arc<dyn RemoteAllocator>,
        remote_timeout: Duration,
        total_remote_timeout: Duration,
        batch_wait: Duration,
        list_max_capacity: i64,
        is_conflict: fn(&AdapterError) -> bool,
    ) -> Arc<Self> {
        let (pending_tx, pending_rx) = mpsc::channel(MAX_BATCH_QUEUE);
        Arc::new(Self {
            storage,
            cache,
            secrets,
            recorder,
            metrics,
            remote,
            pending_tx,
            pending_rx: Mutex::new(pending_rx),
            remote_timeout,
            total_remote_timeout,
            batch_wait,
            list_max_capacity,
            is_conflict,
        })
    }

    pub async fn run(self: &Arc<Self>, cancel: watch::Receiver<bool>) -> Result<()> {
        self.sync().await?;
        self.cache.run(cancel.clone()).await?;
        let this = self.clone();
        tokio::spawn(async move { this.listen_and_allocate(cancel, MAX_BATCH_QUEUE).await });
        Ok(())
    }

    pub async fn sync(&self) -> Result<()> {
        // Rusternetes uses direct storage reads here; there are no client-go informer
        // caches to wait for. AllocationCache::run performs its own initial sync.
        Ok(())
    }

    pub async fn allocate(
        &self,
        cancel: watch::Receiver<bool>,
        mut gsa: GameServerAllocation,
    ) -> Result<AllocationOutput> {
        let started = Instant::now();
        let request_copy = gsa.clone();

        let output = async {
            let validation = gsa.validate();
            if !validation.is_empty() {
                return Ok(AllocationOutput::Status {
                    code: 422,
                    value: invalid_status(&gsa, validation),
                });
            }

            gsa.converter();
            let result = if gsa.spec.multi_cluster_setting.enabled {
                self.apply_multi_cluster_allocation(cancel, gsa).await?
            } else {
                self.allocate_from_local_cluster(cancel, gsa).await?
            };
            Ok(AllocationOutput::Allocation(result))
        }
        .await;

        match &output {
            Ok(response) =>
                self.metrics.allocation(&request_copy, Some(response), None, started.elapsed()),
            Err(err) =>
                self.metrics.allocation(&request_copy, None, Some(err), started.elapsed()),
        }
        output
    }

    async fn allocate_from_local_cluster(
        &self,
        cancel: watch::Receiver<bool>,
        mut gsa: GameServerAllocation,
    ) -> Result<GameServerAllocation> {
        let mut retries = 0usize;
        let result = retry(5, Duration::from_millis(10), 1.0, 0.1, || {
            retries += 1;
            self.allocate_one(cancel.clone(), gsa.clone())
        }).await;

        let gs = match result {
            Ok(gs) => {
                self.metrics.retry_success(retries);
                Some(gs)
            }
            Err(AllocatorError::NoGameServer) => None,
            Err(AllocatorError::ConflictInGameServerSelection) => {
                gsa.status.state = GameServerAllocationState::Contention;
                return Ok(gsa);
            }
            Err(err) => {
                self.cache.resync();
                return Err(err);
            }
        };

        let Some(gs) = gs else {
            gsa.status.state = GameServerAllocationState::Unallocated;
            return Ok(gsa);
        };

        gsa.metadata.name = gs.metadata.name.clone();
        gsa.status.state = GameServerAllocationState::Allocated;
        gsa.status.game_server_name = gs.metadata.name.clone();
        gsa.status.ports = gs.status.ports.clone().into_iter().collect();
        gsa.status.address = gs.status.address.clone();
        gsa.status.addresses = gs.status.addresses.clone().into_iter().collect();
        gsa.status.node_name = gs.status.node_name.clone();
        gsa.status.source = "local".into();
        gsa.status.metadata = Some(GameServerMetadata {
            labels: gs.metadata.labels.clone(),
            annotations: gs.metadata.annotations.clone(),
        });
        gsa.status.counters = gs.status.counters.clone().unwrap_or_default();
        gsa.status.lists = gs.status.lists.clone().unwrap_or_default();
        Ok(gsa)
    }

    async fn apply_multi_cluster_allocation(
        &self,
        cancel: watch::Receiver<bool>,
        gsa: GameServerAllocation,
    ) -> Result<GameServerAllocation> {
        let repo = AgonesRepository::new(self.storage.clone());
        let selector = &gsa.spec.multi_cluster_setting.policy_selector;
        let policies = repo
            .list::<GameServerAllocationPolicy>(&gsa.metadata.namespace)
            .await?
            .into_iter()
            .filter(|p| selector.matches(&p.metadata.labels))
            .collect::<Vec<_>>();

        if policies.is_empty() {
            return Err(AllocatorError::Message(
                "no multi-cluster allocation policy is specified".into(),
            ));
        }

        let mut last = None;
        for connection in ConnectionInfoIterator::new(&policies) {
            let result = if connection.allocation_endpoints.is_empty() {
                let mut local = gsa.clone();
                local.metadata.namespace = connection.namespace.clone();
                self.allocate_from_local_cluster(cancel.clone(), local).await
            } else {
                self.allocate_from_remote_cluster(&gsa, &connection, &gsa.metadata.namespace)
                    .await
            };

            match result {
                Ok(result) => {
                    if result.status.state == GameServerAllocationState::Allocated {
                        return Ok(result);
                    }
                    last = Some(Ok(result));
                }
                Err(err) => {
                    error!(cluster = %connection.cluster_name, %err, "multi-cluster allocation failed");
                    last = Some(Err(err));
                }
            }
        }

        last.unwrap_or_else(|| Err(AllocatorError::NoGameServer))
    }

    async fn allocate_from_remote_cluster(
        &self,
        gsa: &GameServerAllocation,
        connection: &ClusterConnectionInfo,
        namespace: &str,
    ) -> Result<GameServerAllocation> {
        let tls = self.remote_tls(namespace, connection).await?;
        let mut request = gsa_to_allocation_request(gsa);
        if let Some(setting) = request.multi_cluster_setting.as_mut() {
            setting.enabled = false;
        }
        request.namespace = connection.namespace.clone();

        let endpoints = connection.allocation_endpoints.clone();
        let remote = self.remote.clone();
        let call_timeout = self.remote_timeout;

        let attempt = async move {
            retry(7, Duration::from_millis(100), 2.0, 0.0, || {
                let endpoints = endpoints.clone();
                let remote = remote.clone();
                let tls = tls.clone();
                let request = request.clone();
                async move {
                    let mut last = None;
                    for endpoint in endpoints {
                        let endpoint = add_port(&endpoint);
                        debug!(%endpoint, "forwarding allocation request");
                        match remote.allocate(&endpoint, &tls, request.clone(), call_timeout).await {
                            Ok(response) => return Ok((response, endpoint)),
                            Err(err) => {
                                error!(%endpoint, %err, "remote allocation failed");
                                last = Some(err);
                            }
                        }
                    }
                    Err(last.unwrap_or(AllocatorError::NoGameServer))
                }
            }).await
        };

        let (response, endpoint) = timeout(self.total_remote_timeout, attempt)
            .await
            .map_err(|_| AllocatorError::TotalTimeoutExceeded)??;

        Ok(allocation_response_to_gsa(&response, &endpoint))
    }

    async fn remote_tls(
        &self,
        namespace: &str,
        connection: &ClusterConnectionInfo,
    ) -> Result<RemoteTls> {
        let secret = self
            .secrets
            .get_secret(namespace, &connection.secret_name)
            .await?;

        let client_cert = secret.data.get("tls.crt").cloned().unwrap_or_default();
        let client_key = secret.data.get("tls.key").cloned().unwrap_or_default();
        if client_cert.is_empty() || client_key.is_empty() {
            return Err(AllocatorError::Message(format!(
                "missing client certificate key pair in secret {}",
                connection.secret_name
            )));
        }

        let mut ca_bundle = connection.server_ca.clone();
        if let Some(ca) = secret.data.get("ca.crt") {
            if !ca_bundle.is_empty() && !ca_bundle.ends_with(b"\n") {
                ca_bundle.push(b'\n');
            }
            ca_bundle.extend_from_slice(ca);
        }

        Ok(RemoteTls { client_cert, client_key, ca_bundle })
    }

    async fn allocate_one(
        &self,
        mut cancel: watch::Receiver<bool>,
        gsa: GameServerAllocation,
    ) -> Result<GameServer> {
        if *cancel.borrow() {
            return Err(AllocatorError::TotalTimeoutExceeded);
        }

        let (reply, response) = oneshot::channel();
        let request = Request { gsa, reply, cancel: cancel.clone() };

        tokio::select! {
            _ = cancelled(&mut cancel) => return Err(AllocatorError::TotalTimeoutExceeded),
            result = self.pending_tx.send(request) =>
                result.map_err(|_| AllocatorError::Message("allocation queue closed".into()))?,
        }

        tokio::select! {
            _ = cancelled(&mut cancel) => Err(AllocatorError::TotalTimeoutExceeded),
            result = response => result
                .map_err(|_| AllocatorError::Message("allocation response dropped".into()))?,
        }
    }

    async fn listen_and_allocate(
        self: Arc<Self>,
        mut cancel: watch::Receiver<bool>,
        workers: usize,
    ) {
        let semaphore = Arc::new(Semaphore::new(workers));
        let mut list: Option<Vec<Arc<GameServer>>> = None;
        let mut sort_key = 0u64;
        let mut request_count = 0usize;
        let mut rx = self.pending_rx.lock().await;

        loop {
            if *cancel.borrow() { return; }

            let req = match rx.try_recv() {
                Ok(req) => req,
                Err(mpsc::error::TryRecvError::Disconnected) => return,
                Err(mpsc::error::TryRecvError::Empty) => {
                    list = None;
                    request_count = 0;
                    tokio::select! {
                        _ = cancelled(&mut cancel) => return,
                        _ = sleep(self.batch_wait) => continue,
                    }
                }
            };

            if *req.cancel.borrow() {
                let _ = req.reply.send(Err(AllocatorError::TotalTimeoutExceeded));
                continue;
            }

            if request_count >= MAX_BATCH_BEFORE_REFRESH {
                list = None;
                request_count = 0;
            }

            let new_sort_key = req.gsa.sort_key().unwrap_or(0);
            if sort_key == 0 { sort_key = new_sort_key; }
            if new_sort_key != sort_key {
                sort_key = new_sort_key;
                list = None;
                request_count = 0;
            }
            request_count += 1;

            if list.is_none() {
                let sorted = match req.gsa.spec.scheduling {
                    Some(SchedulingStrategy::Packed) =>
                        self.cache.list_sorted_game_servers(&req.gsa),
                    _ => self.cache.list_sorted_game_servers_priorities(&req.gsa),
                };
                match sorted {
                    Ok(sorted) => list = Some(sorted),
                    Err(err) => {
                        let _ = req.reply.send(Err(err));
                        continue;
                    }
                }
            }

            let candidates = list.as_mut().unwrap();
            let (gs, index) = match find_game_server_for_allocation(&req.gsa, candidates) {
                Ok(value) => value,
                Err(err) => {
                    let _ = req.reply.send(Err(err));
                    continue;
                }
            };
            candidates.remove(index);

            if let Err(err) = self.cache.remove_game_server(&gs) {
                let _ = req.reply.send(Err(err));
                continue;
            }

            let permit = tokio::select! {
                _ = cancelled(&mut cancel) => return,
                permit = semaphore.clone().acquire_owned() => match permit {
                    Ok(permit) => permit,
                    Err(_) => return,
                }
            };

            let this = self.clone();
            tokio::spawn(async move {
                let _permit = permit;
                let original = (*gs).clone();
                let result = this
                    .apply_allocation_to_game_server(&req.gsa.spec.metadata, original.clone(), &req.gsa)
                    .await;

                let result = match result {
                    Ok(updated) => {
                        this.cache.add_game_server(&updated);
                        Ok(updated)
                    }
                    Err(AllocatorError::Adapter(ref e)) if !(this.is_conflict)(e) => {
                        this.cache.add_game_server(&original);
                        Err(AllocatorError::GameServerUpdateConflict)
                    }
                    Err(_) => Err(AllocatorError::GameServerUpdateConflict),
                };
                let _ = req.reply.send(result);
            });
        }
    }

    async fn apply_allocation_to_game_server(
        &self,
        patch: &agones_core::MetaPatch,
        mut gs: GameServer,
        gsa: &GameServerAllocation,
    ) -> Result<GameServer> {
        gs.metadata.labels.extend(patch.labels.clone());
        gs.metadata.annotations.extend(patch.annotations.clone());
        gs.metadata.annotations.insert(
            LAST_ALLOCATED_ANNOTATION.into(),
            chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true),
        );
        gs.status.state = Some(GameServerState::Allocated);

        let mut counter_errors = Vec::new();
        for (name, action) in &gsa.spec.counters {
            if let Err(err) = action.counter_actions(name, &mut gs) {
                counter_errors.push(err);
            }
        }

        let mut list_errors = Vec::new();
        for (name, action) in &gsa.spec.lists {
            if let Err(err) = action.list_actions(name, &mut gs, self.list_max_capacity) {
                list_errors.push(err);
            }
        }

        let updated = AgonesRepository::new(self.storage.clone()).update(&gs).await?;

        if !counter_errors.is_empty() {
            self.recorder.warning(&updated, "CounterActionError", &counter_errors.join("\n"));
        }
        if !list_errors.is_empty() {
            self.recorder.warning(&updated, "ListActionError", &list_errors.join("\n"));
        }
        self.recorder.normal(&updated, "Allocated", "Allocated");
        Ok(updated)
    }
}

fn find_game_server_for_allocation(
    gsa: &GameServerAllocation,
    list: &[Arc<GameServer>],
) -> Result<(Arc<GameServer>, usize)> {
    for selector in &gsa.spec.selectors {
        if let Some((index, gs)) = list.iter().enumerate().find(|(_, gs)| selector.matches(gs)) {
            return Ok((gs.clone(), index));
        }
    }
    Err(AllocatorError::NoGameServer)
}

async fn retry<T, F, Fut>(
    steps: usize,
    initial: Duration,
    factor: f64,
    jitter: f64,
    mut f: F,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut delay = initial;
    let mut last = None;

    for attempt in 0..steps {
        match f().await {
            Ok(value) => return Ok(value),
            Err(err @ AllocatorError::NoGameServer)
            | Err(err @ AllocatorError::TotalTimeoutExceeded) => return Err(err),
            Err(AllocatorError::Grpc(status))
                if status.code() == grpc::Code::ResourceExhausted =>
            {
                return Err(AllocatorError::Grpc(status));
            }
            Err(err) => last = Some(err),
        }

        if attempt + 1 < steps {
            let multiplier = if jitter == 0.0 {
                1.0
            } else {
                rand::thread_rng().gen_range(1.0..=1.0 + jitter)
            };
            sleep(delay.mul_f64(multiplier)).await;
            delay = delay.mul_f64(factor);
        }
    }

    Err(last.unwrap_or_else(|| AllocatorError::Message("retry exhausted".into())))
}

fn invalid_status(gsa: &GameServerAllocation, errors: Vec<ValidationError>) -> Value {
    json!({
        "apiVersion": "v1",
        "kind": "Status",
        "status": "Failure",
        "reason": "Invalid",
        "message": format!("GameServerAllocation {:?} is invalid", gsa.metadata.name),
        "details": {
            "name": gsa.metadata.name,
            "group": "allocation.agones.dev",
            "kind": "GameServerAllocation",
            "causes": errors.into_iter().map(|e| json!({
                "reason": format!("{:?}", e.kind),
                "message": e.detail,
                "field": e.path,
            })).collect::<Vec<_>>()
        },
        "code": 422
    })
}

fn add_port(ip: &str) -> String {
    if ip.contains(':') { ip.into() } else { format!("{ip}:{ALLOCATOR_PORT}") }
}

async fn cancelled(cancel: &mut watch::Receiver<bool>) {
    if *cancel.borrow() { return; }
    while cancel.changed().await.is_ok() {
        if *cancel.borrow() { return; }
    }
}