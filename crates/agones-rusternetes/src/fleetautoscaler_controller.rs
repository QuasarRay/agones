use std::{collections::HashMap, sync::Arc, time::Duration};

use agones_core::{
    AutoscalerFeatures, BufferPolicy, CounterPolicy, Fleet,
    FleetAutoscaleRequest, FleetAutoscaleReview,
    FleetAutoscaler, FleetAutoscalerPolicy, FleetAutoscalerPolicyType,
    FleetAutoscalerScaleDecision, GameServer, GameServerState,
    ListPolicy, NodeCount, SchedulePolicy, UrlConfiguration,
    FLEET_NAME_LABEL,
    apply_buffer_autoscaler_policy, apply_counter_autoscaler_policy,
    apply_list_autoscaler_policy,
    is_fleet_autoscaler_schedule_active,
};
use cgp::prelude::*;
use futures::future::BoxFuture;
use chrono::{DateTime, Utc};
use rusternetes_storage::Storage;

use crate::{AdapterError, AgonesRepository};

/// FleetAutoscaler controller context.
///
/// The context starts with storage even though pure policy providers do not
/// require it. Subsequent reconciliation/runtime milestones can add providers
/// to this same context without threading storage through policy functions.
#[derive(HasField)]
pub struct FleetAutoscalerController<S> {
    pub storage: Arc<S>,
    pub autoscaler_features: AutoscalerFeatures,
    pub fleet_autoscale_request_metadata: bool,
}

impl<S> FleetAutoscalerController<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage: Arc::new(storage),
            autoscaler_features: AutoscalerFeatures::default(),
            fleet_autoscale_request_metadata: false,
        }
    }

    pub fn with_autoscaler_features(
        mut self,
        autoscaler_features: AutoscalerFeatures,
    ) -> Self {
        self.autoscaler_features = autoscaler_features;
        self
    }

    pub fn with_fleet_autoscale_request_metadata(
        mut self,
        enabled: bool,
    ) -> Self {
        self.fleet_autoscale_request_metadata = enabled;
        self
    }
}

#[cgp_type]
pub trait HasFleetAutoscalerStorageType {
    type FleetAutoscalerStorage: Storage + 'static;
}

#[cgp_getter]
pub trait HasFleetAutoscalerStorage: HasFleetAutoscalerStorageType {
    fn fleet_autoscaler_storage(
        &self,
    ) -> &Arc<Self::FleetAutoscalerStorage>;
}


#[cgp_getter]
pub trait HasFleetAutoscalerRuntimeFeatures {
    fn fleet_autoscaler_runtime_features(
        &self,
    ) -> &AutoscalerFeatures;
}

#[cgp_getter]
pub trait HasFleetAutoscaleRequestMetadata {
    fn fleet_autoscale_request_metadata(&self) -> &bool;
}



#[cgp_component(BufferFleetAutoscalerPolicyEvaluator)]
pub trait CanApplyBufferFleetAutoscalerPolicy {
    fn apply_buffer_fleet_autoscaler_policy(
        &self,
        policy: &BufferPolicy,
        fleet: &Fleet,
    ) -> Result<FleetAutoscalerScaleDecision, AdapterError>;
}

#[cgp_new_provider]
impl<Context> BufferFleetAutoscalerPolicyEvaluator<Context>
    for UseAgonesBufferFleetAutoscalerPolicy
{
    fn apply_buffer_fleet_autoscaler_policy(
        _context: &Context,
        policy: &BufferPolicy,
        fleet: &Fleet,
    ) -> Result<FleetAutoscalerScaleDecision, AdapterError> {
        apply_buffer_autoscaler_policy(policy, fleet)
            .map_err(AdapterError::Hook)
    }
}


#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FleetAutoscalerInventory {
    pub game_servers: Vec<GameServer>,
    pub node_counts: HashMap<String, NodeCount>,
}

#[async_trait]
#[cgp_component(FleetAutoscalerInventoryLoader)]
pub trait CanLoadFleetAutoscalerInventory {
    async fn load_fleet_autoscaler_inventory(
        &self,
        fleet: &Fleet,
    ) -> Result<FleetAutoscalerInventory, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerInventoryLoader<Context>
    for LoadFleetAutoscalerInventoryWithStorage
where
    Context: HasFleetAutoscalerStorage + Sync,
{
    async fn load_fleet_autoscaler_inventory(
        context: &Context,
        fleet: &Fleet,
    ) -> Result<FleetAutoscalerInventory, AdapterError> {
        let repository = AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ));
        let game_servers = repository
            .list::<GameServer>(&fleet.metadata.namespace)
            .await?
            .into_iter()
            .filter(|game_server| {
                game_server
                    .metadata
                    .labels
                    .get(FLEET_NAME_LABEL)
                    .is_some_and(|fleet_name| {
                        fleet_name == &fleet.metadata.name
                    })
            })
            .collect::<Vec<_>>();

        let mut node_counts = HashMap::new();
        for game_server in &game_servers {
            let count = node_counts
                .entry(game_server.status.node_name.clone())
                .or_insert_with(NodeCount::default);
            match game_server.status.state.as_ref() {
                Some(GameServerState::Ready) => {
                    count.ready = count.ready.wrapping_add(1);
                }
                Some(GameServerState::Allocated) => {
                    count.allocated =
                        count.allocated.wrapping_add(1);
                }
                _ => {}
            }
        }

        Ok(FleetAutoscalerInventory {
            game_servers,
            node_counts,
        })
    }
}

#[cgp_component(CounterFleetAutoscalerPolicyEvaluator)]
pub trait CanApplyCounterFleetAutoscalerPolicy {
    fn apply_counter_fleet_autoscaler_policy(
        &self,
        policy: &CounterPolicy,
        fleet: &Fleet,
        inventory: &FleetAutoscalerInventory,
    ) -> Result<FleetAutoscalerScaleDecision, AdapterError>;
}

#[cgp_new_provider]
impl<Context> CounterFleetAutoscalerPolicyEvaluator<Context>
    for UseAgonesCounterFleetAutoscalerPolicy
{
    fn apply_counter_fleet_autoscaler_policy(
        _context: &Context,
        policy: &CounterPolicy,
        fleet: &Fleet,
        inventory: &FleetAutoscalerInventory,
    ) -> Result<FleetAutoscalerScaleDecision, AdapterError> {
        apply_counter_autoscaler_policy(
            policy,
            fleet,
            &inventory.game_servers,
            &inventory.node_counts,
        )
        .map_err(AdapterError::Hook)
    }
}

#[cgp_component(ListFleetAutoscalerPolicyEvaluator)]
pub trait CanApplyListFleetAutoscalerPolicy {
    fn apply_list_fleet_autoscaler_policy(
        &self,
        policy: &ListPolicy,
        fleet: &Fleet,
        inventory: &FleetAutoscalerInventory,
    ) -> Result<FleetAutoscalerScaleDecision, AdapterError>;
}

#[cgp_new_provider]
impl<Context> ListFleetAutoscalerPolicyEvaluator<Context>
    for UseAgonesListFleetAutoscalerPolicy
{
    fn apply_list_fleet_autoscaler_policy(
        _context: &Context,
        policy: &ListPolicy,
        fleet: &Fleet,
        inventory: &FleetAutoscalerInventory,
    ) -> Result<FleetAutoscalerScaleDecision, AdapterError> {
        apply_list_autoscaler_policy(
            policy,
            fleet,
            &inventory.game_servers,
            &inventory.node_counts,
        )
        .map_err(AdapterError::Hook)
    }
}


/// Resolves the URLConfiguration exactly at the FleetAutoscaler transport
/// boundary. Service references use Agones' default port 8000 and select HTTPS
/// when a CA bundle is present.
pub fn fleet_autoscaler_webhook_url(
    configuration: &UrlConfiguration,
) -> Result<String, AdapterError> {
    if configuration.url.is_some() && configuration.service.is_some() {
        return Err(AdapterError::Hook(
            "service and URL cannot be used simultaneously".into(),
        ));
    }

    if let Some(url) = configuration.url.as_deref() {
        if url.is_empty() {
            return Err(AdapterError::Hook(
                "URL was not provided".into(),
            ));
        }
        return reqwest::Url::parse(url)
            .map(|url| url.to_string())
            .map_err(|error| AdapterError::Hook(error.to_string()));
    }

    let service = configuration.service.as_ref().ok_or_else(|| {
        AdapterError::Hook(
            "service was not provided, either URL or Service must be provided"
                .into(),
        )
    })?;
    if service.name.is_empty() {
        return Err(AdapterError::Hook(
            "service name was not provided".into(),
        ));
    }

    let namespace = if service.namespace.is_empty() {
        "default"
    } else {
        service.namespace.as_str()
    };
    let port = service.port.unwrap_or(8000);
    let scheme = if configuration.ca_bundle.is_empty() {
        "http"
    } else {
        "https"
    };
    let path = service.path.as_deref().unwrap_or("");
    let path = if path.is_empty() || path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{path}")
    };

    Ok(format!(
        "{scheme}://{}.{namespace}.svc:{port}{path}",
        service.name
    ))
}

pub(crate) fn fleet_autoscaler_http_client(
    configuration: &UrlConfiguration,
) -> Result<reqwest::Client, AdapterError> {
    let mut client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15));

    if !configuration.ca_bundle.is_empty() {
        let certificates =
            reqwest::Certificate::from_pem_bundle(
                &configuration.ca_bundle,
            )
            .map_err(|_| {
                AdapterError::Hook(
                    "no certs were appended from caBundle".into(),
                )
            })?;
        if certificates.is_empty() {
            return Err(AdapterError::Hook(
                "no certs were appended from caBundle".into(),
            ));
        }
        for certificate in certificates {
            client = client.add_root_certificate(certificate);
        }
    }

    client
        .build()
        .map_err(|error| AdapterError::Hook(error.to_string()))
}

pub(crate) fn fleet_autoscale_review_request(
    fleet: &Fleet,
    include_metadata: bool,
) -> FleetAutoscaleReview {
    FleetAutoscaleReview {
        request: Some(FleetAutoscaleRequest {
            uid: uuid::Uuid::new_v4().to_string(),
            name: fleet.metadata.name.clone(),
            namespace: fleet.metadata.namespace.clone(),
            status: fleet.status.clone(),
            labels: if include_metadata {
                fleet.metadata.labels.clone()
            } else {
                Default::default()
            },
            annotations: if include_metadata {
                fleet.metadata.annotations.clone()
            } else {
                Default::default()
            },
        }),
        response: None,
    }
}

#[cgp_component(FleetAutoscalerWebhookCaller)]
pub trait CanCallFleetAutoscalerWebhook {
    fn call_fleet_autoscaler_webhook<'a>(
        &'a self,
        configuration: &'a UrlConfiguration,
        review: &'a FleetAutoscaleReview,
    ) -> BoxFuture<'a, Result<FleetAutoscaleReview, AdapterError>>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerWebhookCaller<Context>
    for CallFleetAutoscalerWebhookWithReqwest
where
    Context: Sync,
{
    fn call_fleet_autoscaler_webhook<'a>(
        _context: &'a Context,
        configuration: &'a UrlConfiguration,
        review: &'a FleetAutoscaleReview,
    ) -> BoxFuture<'a, Result<FleetAutoscaleReview, AdapterError>> {
        Box::pin(async move {
            let url = fleet_autoscaler_webhook_url(configuration)?;
            let client =
                fleet_autoscaler_http_client(configuration)?;
            let response = client
                .post(url.as_str())
                .json(review)
                .send()
                .await
                .map_err(|error| AdapterError::Hook(error.to_string()))?;

            if response.status() != reqwest::StatusCode::OK {
                return Err(AdapterError::Hook(format!(
                    "bad status code {} from the server: {url}",
                    response.status().as_u16(),
                )));
            }

            response
                .json::<FleetAutoscaleReview>()
                .await
                .map_err(|error| AdapterError::Hook(error.to_string()))
        })
    }
}

#[cgp_component(WebhookFleetAutoscalerPolicyEvaluator)]
pub trait CanApplyWebhookFleetAutoscalerPolicy {
    fn apply_webhook_fleet_autoscaler_policy<'a>(
        &'a self,
        configuration: &'a UrlConfiguration,
        fleet: &'a Fleet,
    ) -> BoxFuture<'a, Result<FleetAutoscalerScaleDecision, AdapterError>>;
}

#[cgp_new_provider]
impl<Context> WebhookFleetAutoscalerPolicyEvaluator<Context>
    for ApplyWebhookFleetAutoscalerPolicyWithCaller
where
    Context: CanCallFleetAutoscalerWebhook
        + HasFleetAutoscaleRequestMetadata
        + Sync,
{
    fn apply_webhook_fleet_autoscaler_policy<'a>(
        context: &'a Context,
        configuration: &'a UrlConfiguration,
        fleet: &'a Fleet,
    ) -> BoxFuture<'a, Result<FleetAutoscalerScaleDecision, AdapterError>> {
        Box::pin(async move {
            let review = fleet_autoscale_review_request(
                fleet,
                *context.fleet_autoscale_request_metadata(),
            );
            let response = context
                .call_fleet_autoscaler_webhook(
                    configuration,
                    &review,
                )
                .await?;
            let response = response.response.ok_or_else(|| {
                AdapterError::Hook(
                    "webhook response missing required 'response' field"
                        .into(),
                )
            })?;

            Ok(FleetAutoscalerScaleDecision {
                replicas: if response.scale {
                    response.replicas
                } else {
                    fleet.status.replicas
                },
                limited: false,
            })
        })
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetAutoscalerPolicyDecision {
    pub replicas: i32,
    pub limited: bool,
    pub applied_policy: FleetAutoscalerPolicyType,
}

#[cgp_component(FleetAutoscalerScheduleChecker)]
pub trait CanCheckFleetAutoscalerSchedule {
    fn is_fleet_autoscaler_schedule_active(
        &self,
        schedule: &SchedulePolicy,
        current_time: DateTime<Utc>,
    ) -> Result<bool, AdapterError>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerScheduleChecker<Context>
    for UseAgonesFleetAutoscalerScheduleChecker
{
    fn is_fleet_autoscaler_schedule_active(
        _context: &Context,
        schedule: &SchedulePolicy,
        current_time: DateTime<Utc>,
    ) -> Result<bool, AdapterError> {
        is_fleet_autoscaler_schedule_active(schedule, current_time)
            .map_err(AdapterError::Hook)
    }
}

#[cgp_component(FleetAutoscalerClock)]
pub trait CanReadFleetAutoscalerTime {
    fn fleet_autoscaler_now(&self) -> DateTime<Utc>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerClock<Context>
    for UseUtcFleetAutoscalerClock
{
    fn fleet_autoscaler_now(_context: &Context) -> DateTime<Utc> {
        Utc::now()
    }
}

fn fleet_autoscaler_policy_needs_inventory(
    policy: &FleetAutoscalerPolicy,
) -> bool {
    match policy.policy_type {
        FleetAutoscalerPolicyType::Counter
        | FleetAutoscalerPolicyType::List => true,
        FleetAutoscalerPolicyType::Schedule => policy
            .schedule
            .as_deref()
            .is_some_and(|schedule| {
                fleet_autoscaler_policy_needs_inventory(
                    &schedule.policy,
                )
            }),
        FleetAutoscalerPolicyType::Chain => policy
            .chain
            .iter()
            .any(|entry| {
                fleet_autoscaler_policy_needs_inventory(
                    &entry.policy,
                )
            }),
        _ => false,
    }
}

fn compute_fleet_autoscaler_desired_size_at<'a, Context>(
    context: &'a Context,
    policy: &'a FleetAutoscalerPolicy,
    fleet: &'a Fleet,
    inventory: &'a FleetAutoscalerInventory,
    current_time: &'a DateTime<Utc>,
) -> BoxFuture<'a, Result<FleetAutoscalerPolicyDecision, AdapterError>>
where
    Context: CanApplyBufferFleetAutoscalerPolicy
        + CanApplyCounterFleetAutoscalerPolicy
        + CanApplyListFleetAutoscalerPolicy
        + CanApplyWebhookFleetAutoscalerPolicy
        + CanCheckFleetAutoscalerSchedule
        + HasFleetAutoscalerRuntimeFeatures
        + Sync,
{
    Box::pin(async move {
    match &policy.policy_type {
        FleetAutoscalerPolicyType::Buffer => {
            let buffer = policy.buffer.as_ref().ok_or_else(|| {
                AdapterError::Hook(
                    "buffer policy config params are missing".into(),
                )
            })?;
            let decision = context
                .apply_buffer_fleet_autoscaler_policy(
                    buffer,
                    fleet,
                )?;
            Ok(FleetAutoscalerPolicyDecision {
                replicas: decision.replicas,
                limited: decision.limited,
                applied_policy: FleetAutoscalerPolicyType::Buffer,
            })
        }
        FleetAutoscalerPolicyType::Counter => {
            let counter = policy.counter.as_ref().ok_or_else(|| {
                AdapterError::Hook(
                    "counter policy config params are missing".into(),
                )
            })?;
            let decision = context
                .apply_counter_fleet_autoscaler_policy(
                    counter,
                    fleet,
                    inventory,
                )?;
            Ok(FleetAutoscalerPolicyDecision {
                replicas: decision.replicas,
                limited: decision.limited,
                applied_policy: FleetAutoscalerPolicyType::Counter,
            })
        }
        FleetAutoscalerPolicyType::List => {
            let list = policy.list.as_ref().ok_or_else(|| {
                AdapterError::Hook(
                    "list policy config params are missing".into(),
                )
            })?;
            let decision = context
                .apply_list_fleet_autoscaler_policy(
                    list,
                    fleet,
                    inventory,
                )?;
            Ok(FleetAutoscalerPolicyDecision {
                replicas: decision.replicas,
                limited: decision.limited,
                applied_policy: FleetAutoscalerPolicyType::List,
            })
        }
        FleetAutoscalerPolicyType::Webhook => {
            let webhook = policy.webhook.as_ref().ok_or_else(|| {
                AdapterError::Hook(
                    "webhookPolicy parameter must not be nil".into(),
                )
            })?;
            let decision = context
                .apply_webhook_fleet_autoscaler_policy(
                    webhook,
                    fleet,
                )
                .await?;
            Ok(FleetAutoscalerPolicyDecision {
                replicas: decision.replicas,
                limited: decision.limited,
                applied_policy: FleetAutoscalerPolicyType::Webhook,
            })
        }
        FleetAutoscalerPolicyType::Schedule => {
            if !context
                .fleet_autoscaler_runtime_features()
                .scheduled_autoscaler
            {
                return Err(AdapterError::Hook(
                    "cannot apply SchedulePolicy unless feature flag ScheduledAutoscaler is enabled"
                        .into(),
                ));
            }

            let schedule = policy
                .schedule
                .as_deref()
                .ok_or_else(|| {
                    AdapterError::Hook(
                        "schedule policy config params are missing".into(),
                    )
                })?;
            if !context.is_fleet_autoscaler_schedule_active(
                schedule,
                current_time.clone(),
            )? {
                return Err(
                    AdapterError::InactiveFleetAutoscalerSchedule,
                );
            }

            compute_fleet_autoscaler_desired_size_at(
                context,
                &schedule.policy,
                fleet,
                inventory,
                current_time,
            )
            .await
        }
        FleetAutoscalerPolicyType::Chain => {
            if !context
                .fleet_autoscaler_runtime_features()
                .scheduled_autoscaler
            {
                return Err(AdapterError::Hook(
                    "cannot apply ChainPolicy unless feature flag ScheduledAutoscaler is enabled"
                        .into(),
                ));
            }

            let mut last_error = None;
            for entry in &policy.chain {
                match compute_fleet_autoscaler_desired_size_at(
                    context,
                    &entry.policy,
                    fleet,
                    inventory,
                    current_time,
                )
                .await
                {
                    Ok(mut decision) => {
                        decision.applied_policy =
                            FleetAutoscalerPolicyType::Unknown(
                                format!(
                                    "Chain:{}:{}",
                                    entry.id,
                                    entry.policy.policy_type.as_str(),
                                ),
                            );
                        return Ok(decision);
                    }
                    Err(error) => {
                        last_error = Some(error);
                    }
                }
            }

            Err(last_error.unwrap_or_else(|| {
                AdapterError::Hook(
                    "Failed to apply ChainPolicy: no valid policy applied"
                        .into(),
                )
            }))
        }
        _ => Err(AdapterError::Hook(
            "wrong policy type, should be one of: Buffer, Webhook, Counter, List, Schedule, Chain"
                .into(),
        )),
    }
    })
}

#[cgp_component(FleetAutoscalerPolicyEvaluatorAt)]
pub trait CanComputeFleetAutoscalerDesiredSizeAt {
    fn compute_fleet_autoscaler_desired_size_at<'a>(
        &'a self,
        policy: &'a FleetAutoscalerPolicy,
        fleet: &'a Fleet,
        inventory: &'a FleetAutoscalerInventory,
        current_time: DateTime<Utc>,
    ) -> BoxFuture<'a, Result<FleetAutoscalerPolicyDecision, AdapterError>>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerPolicyEvaluatorAt<Context>
    for ComputeFleetAutoscalerDesiredSizeAtWithComponents
where
    Context: CanApplyBufferFleetAutoscalerPolicy
        + CanApplyCounterFleetAutoscalerPolicy
        + CanApplyListFleetAutoscalerPolicy
        + CanApplyWebhookFleetAutoscalerPolicy
        + CanCheckFleetAutoscalerSchedule
        + HasFleetAutoscalerRuntimeFeatures
        + Sync,
{
    fn compute_fleet_autoscaler_desired_size_at<'a>(
        context: &'a Context,
        policy: &'a FleetAutoscalerPolicy,
        fleet: &'a Fleet,
        inventory: &'a FleetAutoscalerInventory,
        current_time: DateTime<Utc>,
    ) -> BoxFuture<'a, Result<FleetAutoscalerPolicyDecision, AdapterError>> {
        Box::pin(async move {
            compute_fleet_autoscaler_desired_size_at(
                context,
                policy,
                fleet,
                inventory,
                &current_time,
            )
            .await
        })
    }
}

#[cgp_component(FleetAutoscalerPolicyEvaluator)]
pub trait CanComputeFleetAutoscalerDesiredSize {
    fn compute_fleet_autoscaler_desired_size<'a>(
        &'a self,
        policy: &'a FleetAutoscalerPolicy,
        fleet: &'a Fleet,
        inventory: &'a FleetAutoscalerInventory,
    ) -> BoxFuture<'a, Result<FleetAutoscalerPolicyDecision, AdapterError>>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerPolicyEvaluator<Context>
    for ComputeFleetAutoscalerDesiredSizeWithClock
where
    Context: CanComputeFleetAutoscalerDesiredSizeAt
        + CanReadFleetAutoscalerTime
        + Sync,
{
    fn compute_fleet_autoscaler_desired_size<'a>(
        context: &'a Context,
        policy: &'a FleetAutoscalerPolicy,
        fleet: &'a Fleet,
        inventory: &'a FleetAutoscalerInventory,
    ) -> BoxFuture<'a, Result<FleetAutoscalerPolicyDecision, AdapterError>> {
        let current_time = context.fleet_autoscaler_now();
        Box::pin(async move {
            context
                .compute_fleet_autoscaler_desired_size_at(
                    policy,
                    fleet,
                    inventory,
                    current_time,
                )
                .await
        })
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetAutoscalerReconcileResult {
    pub fleet_autoscaler: FleetAutoscaler,
    pub fleet: Fleet,
    pub decision: FleetAutoscalerPolicyDecision,
    pub scaled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FleetAutoscalerSyncOutcome {
    NotFound,
    FleetNotFound(FleetAutoscaler),
    FleetDeleting {
        fleet_autoscaler: FleetAutoscaler,
        fleet: Fleet,
    },
    Reconciled(FleetAutoscalerReconcileResult),
}

#[async_trait]
#[cgp_component(FleetAutoscalerFleetScaler)]
pub trait CanScaleFleetForAutoscaler {
    async fn scale_fleet_for_autoscaler(
        &self,
        fleet: &Fleet,
        replicas: i32,
    ) -> Result<Fleet, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerFleetScaler<Context>
    for ScaleFleetForAutoscalerWithStorage
where
    Context: HasFleetAutoscalerStorage + Sync,
{
    async fn scale_fleet_for_autoscaler(
        context: &Context,
        fleet: &Fleet,
        replicas: i32,
    ) -> Result<Fleet, AdapterError> {
        if replicas == fleet.spec.replicas {
            return Ok(fleet.clone());
        }

        let mut updated = fleet.clone();
        updated.spec.replicas = replicas;
        AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ))
        .update(&updated)
        .await
        .map_err(|error| {
            AdapterError::Hook(format!(
                "error updating replicas for fleet {}: {error}",
                fleet.metadata.name
            ))
        })
    }
}

#[async_trait]
#[cgp_component(FleetAutoscalerStatusUpdater)]
pub trait CanUpdateFleetAutoscalerStatus {
    async fn update_fleet_autoscaler_status(
        &self,
        fleet_autoscaler: &FleetAutoscaler,
        current_replicas: i32,
        desired_replicas: i32,
        scaled: bool,
        scaling_limited: bool,
        applied_policy: FleetAutoscalerPolicyType,
    ) -> Result<FleetAutoscaler, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerStatusUpdater<Context>
    for UpdateFleetAutoscalerStatusWithStorage
where
    Context: HasFleetAutoscalerStorage + Sync,
{
    async fn update_fleet_autoscaler_status(
        context: &Context,
        fleet_autoscaler: &FleetAutoscaler,
        current_replicas: i32,
        desired_replicas: i32,
        scaled: bool,
        scaling_limited: bool,
        applied_policy: FleetAutoscalerPolicyType,
    ) -> Result<FleetAutoscaler, AdapterError> {
        let mut updated = fleet_autoscaler.clone();
        updated.status.able_to_scale = true;
        updated.status.scaling_limited = scaling_limited;
        updated.status.current_replicas = current_replicas;
        updated.status.desired_replicas = desired_replicas;
        updated.status.last_applied_policy = applied_policy;
        if scaled {
            updated.status.last_scale_time =
                Some(Utc::now().to_rfc3339());
        }

        if updated.status == fleet_autoscaler.status {
            return Ok(updated);
        }

        AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ))
        .update(&updated)
        .await
        .map_err(|error| {
            AdapterError::Hook(format!(
                "error updating status for fleetautoscaler {}: {error}",
                fleet_autoscaler.metadata.name
            ))
        })
    }
}

#[async_trait]
#[cgp_component(FleetAutoscalerUnableStatusUpdater)]
pub trait CanMarkFleetAutoscalerUnableToScale {
    async fn mark_fleet_autoscaler_unable_to_scale(
        &self,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<FleetAutoscaler, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerUnableStatusUpdater<Context>
    for MarkFleetAutoscalerUnableToScaleWithStorage
where
    Context: HasFleetAutoscalerStorage + Sync,
{
    async fn mark_fleet_autoscaler_unable_to_scale(
        context: &Context,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<FleetAutoscaler, AdapterError> {
        let mut updated = fleet_autoscaler.clone();
        updated.status.able_to_scale = false;
        updated.status.scaling_limited = false;
        updated.status.current_replicas = 0;
        updated.status.desired_replicas = 0;
        updated.status.last_applied_policy =
            FleetAutoscalerPolicyType::Unknown(String::new());

        if updated.status == fleet_autoscaler.status {
            return Ok(updated);
        }

        AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ))
        .update(&updated)
        .await
        .map_err(|error| {
            AdapterError::Hook(format!(
                "error updating status for fleetautoscaler {}: {error}",
                fleet_autoscaler.metadata.name
            ))
        })
    }
}

#[async_trait]
#[cgp_component(FleetAutoscalerReconciler)]
pub trait CanReconcileFleetAutoscaler {
    async fn reconcile_fleet_autoscaler(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<FleetAutoscalerSyncOutcome, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerReconciler<Context>
    for ReconcileFleetAutoscalerWithComponents
where
    Context: HasFleetAutoscalerStorage
        + CanComputeFleetAutoscalerDesiredSize
        + CanLoadFleetAutoscalerInventory
        + CanScaleFleetForAutoscaler
        + CanUpdateFleetAutoscalerStatus
        + CanMarkFleetAutoscalerUnableToScale
        + Sync,
{
    async fn reconcile_fleet_autoscaler(
        context: &Context,
        namespace: &str,
        name: &str,
    ) -> Result<FleetAutoscalerSyncOutcome, AdapterError> {
        let repository = AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ));

        let fleet_autoscaler = match repository
            .get::<FleetAutoscaler>(namespace, name)
            .await
        {
            Ok(fleet_autoscaler) => fleet_autoscaler,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_),
            )) => return Ok(FleetAutoscalerSyncOutcome::NotFound),
            Err(error) => return Err(error),
        };

        let fleet = match repository
            .get::<Fleet>(
                namespace,
                fleet_autoscaler.spec.fleet_name.as_str(),
            )
            .await
        {
            Ok(fleet) => fleet,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_),
            )) => {
                let fleet_autoscaler = context
                    .mark_fleet_autoscaler_unable_to_scale(
                        &fleet_autoscaler,
                    )
                    .await?;
                return Ok(
                    FleetAutoscalerSyncOutcome::FleetNotFound(
                        fleet_autoscaler,
                    ),
                );
            }
            Err(error) => return Err(error),
        };

        if fleet.metadata.deletion_timestamp.is_some() {
            return Ok(
                FleetAutoscalerSyncOutcome::FleetDeleting {
                    fleet_autoscaler,
                    fleet,
                },
            );
        }

        let inventory = if fleet_autoscaler_policy_needs_inventory(
            &fleet_autoscaler.spec.policy,
        ) {
            context
                .load_fleet_autoscaler_inventory(&fleet)
                .await?
        } else {
            FleetAutoscalerInventory::default()
        };

        let decision = match context
            .compute_fleet_autoscaler_desired_size(
                &fleet_autoscaler.spec.policy,
                &fleet,
                &inventory,
            )
            .await
        {
            Ok(decision) => decision,
            Err(AdapterError::InactiveFleetAutoscalerSchedule) => {
                return Err(AdapterError::Hook(format!(
                    "error calculating autoscaling fleet: {}: inactive schedule, policy not applicable",
                    fleet.metadata.name
                )));
            }
            Err(error) => {
                context
                    .mark_fleet_autoscaler_unable_to_scale(
                        &fleet_autoscaler,
                    )
                    .await?;
                return Err(AdapterError::Hook(format!(
                    "error calculating autoscaling fleet: {}: {error}",
                    fleet.metadata.name
                )));
            }
        };

        let current_replicas = fleet.status.replicas;
        let scaled = decision.replicas != fleet.spec.replicas;
        let fleet = context
            .scale_fleet_for_autoscaler(
                &fleet,
                decision.replicas,
            )
            .await?;

        let fleet_autoscaler = context
            .update_fleet_autoscaler_status(
                &fleet_autoscaler,
                current_replicas,
                decision.replicas,
                scaled,
                decision.limited,
                decision.applied_policy.clone(),
            )
            .await?;

        Ok(FleetAutoscalerSyncOutcome::Reconciled(
            FleetAutoscalerReconcileResult {
                fleet_autoscaler,
                fleet,
                decision,
                scaled,
            },
        ))
    }
}

delegate_and_check_components! {
    <S: Storage + 'static>
    FleetAutoscalerController<S> {
        FleetAutoscalerStorageTypeProviderComponent: UseType<S>,
        FleetAutoscalerStorageGetterComponent:
            UseField<Symbol!("storage")>,
        FleetAutoscalerRuntimeFeaturesGetterComponent:
            UseField<Symbol!("autoscaler_features")>,
        FleetAutoscaleRequestMetadataGetterComponent:
            UseField<Symbol!("fleet_autoscale_request_metadata")>,
        FleetAutoscalerWebhookCallerComponent:
            CallFleetAutoscalerWebhookWithReqwest,
        WebhookFleetAutoscalerPolicyEvaluatorComponent:
            ApplyWebhookFleetAutoscalerPolicyWithCaller,
        BufferFleetAutoscalerPolicyEvaluatorComponent:
            UseAgonesBufferFleetAutoscalerPolicy,
        CounterFleetAutoscalerPolicyEvaluatorComponent:
            UseAgonesCounterFleetAutoscalerPolicy,
        ListFleetAutoscalerPolicyEvaluatorComponent:
            UseAgonesListFleetAutoscalerPolicy,
        FleetAutoscalerScheduleCheckerComponent:
            UseAgonesFleetAutoscalerScheduleChecker,
        FleetAutoscalerClockComponent:
            UseUtcFleetAutoscalerClock,
        FleetAutoscalerInventoryLoaderComponent:
            LoadFleetAutoscalerInventoryWithStorage,
        FleetAutoscalerPolicyEvaluatorAtComponent:
            ComputeFleetAutoscalerDesiredSizeAtWithComponents,
        FleetAutoscalerPolicyEvaluatorComponent:
            ComputeFleetAutoscalerDesiredSizeWithClock,
        FleetAutoscalerFleetScalerComponent:
            ScaleFleetForAutoscalerWithStorage,
        FleetAutoscalerStatusUpdaterComponent:
            UpdateFleetAutoscalerStatusWithStorage,
        FleetAutoscalerUnableStatusUpdaterComponent:
            MarkFleetAutoscalerUnableToScaleWithStorage,
        FleetAutoscalerReconcilerComponent:
            ReconcileFleetAutoscalerWithComponents,
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        AggregatedCounterStatus, AggregatedListStatus, BufferPolicy,
        ActivePeriod, AutoscalerFeatures, Between, ChainEntry,
        CounterPolicy, CounterStatus, FleetAutoscalerPolicy,
        FleetAutoscaleResponse, GameServerStatus, IntOrPercent,
        ListPolicy, ListStatus, ObjectMeta, SchedulePolicy,
        ServiceReference, UrlConfiguration,
    };
    use std::collections::BTreeMap;
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn fleet(allocated: i32) -> Fleet {
        let mut fleet = Fleet::default();
        fleet.status.allocated_replicas = allocated;
        fleet
    }

    #[test]
    fn cgp_buffer_provider_matches_upstream_integer_policy() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let decision = controller
            .apply_buffer_fleet_autoscaler_policy(
                &BufferPolicy {
                    buffer_size: IntOrPercent::Int(20),
                    min_replicas: 0,
                    max_replicas: 100,
                },
                &fleet(40),
            )
            .unwrap();

        assert_eq!(decision.replicas, 60);
        assert!(!decision.limited);
    }

    #[tokio::test]
    async fn cgp_dispatch_uses_buffer_provider_and_preserves_policy_tag() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Buffer,
            buffer: Some(BufferPolicy {
                buffer_size: IntOrPercent::String("20%".into()),
                min_replicas: 0,
                max_replicas: 100,
            }),
            ..Default::default()
        };

        let decision = controller
            .compute_fleet_autoscaler_desired_size(
                &policy,
                &fleet(50),
                &FleetAutoscalerInventory::default(),
            )
            .await
            .unwrap();

        assert_eq!(decision.replicas, 63);
        assert!(!decision.limited);
        assert_eq!(
            decision.applied_policy,
            FleetAutoscalerPolicyType::Buffer
        );
    }


    #[tokio::test]
    async fn cgp_dispatch_uses_counter_and_list_providers() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());

        let mut counter_fleet = Fleet::default();
        counter_fleet.status.replicas = 10;
        counter_fleet.spec.template.spec.counters =
            Some(BTreeMap::from([(
                "rooms".into(),
                CounterStatus {
                    count: 0,
                    capacity: 7,
                },
            )]));
        counter_fleet.status.counters.insert(
            "rooms".into(),
            AggregatedCounterStatus {
                count: 68,
                capacity: 70,
                ..Default::default()
            },
        );
        let counter = controller
            .compute_fleet_autoscaler_desired_size(
                &FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Counter,
                    counter: Some(CounterPolicy {
                        key: "rooms".into(),
                        min_capacity: 10,
                        max_capacity: 100,
                        buffer_size: IntOrPercent::Int(10),
                    }),
                    ..Default::default()
                },
                &counter_fleet,
                &FleetAutoscalerInventory::default(),
            )
            .await
            .unwrap();
        assert_eq!(counter.replicas, 12);
        assert_eq!(
            counter.applied_policy,
            FleetAutoscalerPolicyType::Counter
        );

        let mut list_fleet = Fleet::default();
        list_fleet.status.replicas = 10;
        list_fleet.spec.template.spec.lists =
            Some(BTreeMap::from([(
                "gamers".into(),
                ListStatus {
                    capacity: 3,
                    values: vec!["a".into(), "b".into()],
                },
            )]));
        list_fleet.status.lists.insert(
            "gamers".into(),
            AggregatedListStatus {
                count: 29,
                capacity: 30,
                ..Default::default()
            },
        );
        let list = controller
            .compute_fleet_autoscaler_desired_size(
                &FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::List,
                    list: Some(ListPolicy {
                        key: "gamers".into(),
                        min_capacity: 10,
                        max_capacity: 100,
                        buffer_size: IntOrPercent::Int(5),
                    }),
                    ..Default::default()
                },
                &list_fleet,
                &FleetAutoscalerInventory::default(),
            )
            .await
            .unwrap();
        assert_eq!(list.replicas, 14);
        assert_eq!(
            list.applied_policy,
            FleetAutoscalerPolicyType::List
        );
    }

    #[tokio::test]
    async fn inventory_loader_filters_fleet_and_rebuilds_node_counts() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let fleet = Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            ..Default::default()
        };

        let game_server = |name: &str,
                           fleet_name: &str,
                           state: GameServerState| GameServer {
            metadata: ObjectMeta {
                name: name.into(),
                namespace: "default".into(),
                uid: format!("{name}-uid"),
                labels: BTreeMap::from([(
                    FLEET_NAME_LABEL.into(),
                    fleet_name.into(),
                )]),
                ..Default::default()
            },
            status: GameServerStatus {
                state: Some(state),
                node_name: "n1".into(),
                ..Default::default()
            },
            ..Default::default()
        };

        repository
            .create(&game_server(
                "ready",
                "fleet",
                GameServerState::Ready,
            ))
            .await
            .unwrap();
        repository
            .create(&game_server(
                "allocated",
                "fleet",
                GameServerState::Allocated,
            ))
            .await
            .unwrap();
        repository
            .create(&game_server(
                "other",
                "other-fleet",
                GameServerState::Ready,
            ))
            .await
            .unwrap();

        let inventory =
            FleetAutoscalerController::new(storage)
                .load_fleet_autoscaler_inventory(&fleet)
                .await
                .unwrap();

        assert_eq!(inventory.game_servers.len(), 2);
        assert_eq!(
            inventory.node_counts["n1"],
            NodeCount {
                ready: 1,
                allocated: 1,
            }
        );
    }



    #[test]
    fn webhook_url_matches_upstream_url_and_service_resolution() {
        assert_eq!(
            fleet_autoscaler_webhook_url(&UrlConfiguration {
                url: Some("http://127.0.0.1:8080/scale".into()),
                ..Default::default()
            })
            .unwrap(),
            "http://127.0.0.1:8080/scale"
        );

        assert_eq!(
            fleet_autoscaler_webhook_url(&UrlConfiguration {
                service: Some(ServiceReference {
                    name: "autoscaler".into(),
                    namespace: String::new(),
                    path: Some("scale".into()),
                    port: None,
                }),
                ..Default::default()
            })
            .unwrap(),
            "http://autoscaler.default.svc:8000/scale"
        );

        assert_eq!(
            fleet_autoscaler_webhook_url(&UrlConfiguration {
                service: Some(ServiceReference {
                    name: "autoscaler".into(),
                    namespace: "games".into(),
                    path: Some("/scale".into()),
                    port: Some(9443),
                }),
                ca_bundle: b"non-empty".to_vec(),
                ..Default::default()
            })
            .unwrap(),
            "https://autoscaler.games.svc:9443/scale"
        );
    }

    fn serve_one_webhook_response(
        response: FleetAutoscaleReview,
    ) -> (
        String,
        std::sync::mpsc::Receiver<FleetAutoscaleReview>,
        std::thread::JoinHandle<()>,
    ) {
        use std::io::{Read, Write};
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut bytes = Vec::new();
            let mut scratch = [0u8; 4096];

            let header_end = loop {
                let read = stream.read(&mut scratch).unwrap();
                if read == 0 {
                    panic!("webhook request ended before headers");
                }
                bytes.extend_from_slice(&scratch[..read]);
                if let Some(position) = bytes
                    .windows(4)
                    .position(|window| window == b"\r\n\r\n")
                {
                    break position + 4;
                }
            };

            let headers =
                String::from_utf8(bytes[..header_end].to_vec())
                    .unwrap();
            let content_length = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().unwrap())
                })
                .unwrap_or(0);

            while bytes.len() < header_end + content_length {
                let read = stream.read(&mut scratch).unwrap();
                if read == 0 {
                    break;
                }
                bytes.extend_from_slice(&scratch[..read]);
            }

            let review: FleetAutoscaleReview =
                serde_json::from_slice(
                    &bytes[header_end..header_end + content_length],
                )
                .unwrap();
            sender.send(review).unwrap();

            let body = serde_json::to_vec(&response).unwrap();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            )
            .unwrap();
            stream.write_all(&body).unwrap();
            stream.flush().unwrap();
        });

        (format!("http://{address}/scale"), receiver, handle)
    }

    #[tokio::test]
    async fn cgp_webhook_provider_posts_review_metadata_and_scales() {
        let response = FleetAutoscaleReview {
            request: None,
            response: Some(FleetAutoscaleResponse {
                uid: "response".into(),
                scale: true,
                replicas: 17,
            }),
        };
        let (url, received, handle) =
            serve_one_webhook_response(response);

        let mut target = fleet(5);
        target.metadata.name = "fleet".into();
        target.metadata.namespace = "default".into();
        target
            .metadata
            .labels
            .insert("region".into(), "test".into());
        target
            .metadata
            .annotations
            .insert("owner".into(), "agones".into());
        target.status.replicas = 5;

        let controller =
            FleetAutoscalerController::new(MemoryStorage::new())
                .with_fleet_autoscale_request_metadata(true);
        let decision = controller
            .apply_webhook_fleet_autoscaler_policy(
                &UrlConfiguration {
                    url: Some(url),
                    ..Default::default()
                },
                &target,
            )
            .await
            .unwrap();

        assert_eq!(
            decision,
            FleetAutoscalerScaleDecision {
                replicas: 17,
                limited: false,
            }
        );

        let review = received
            .recv_timeout(std::time::Duration::from_secs(2))
            .unwrap();
        let request = review.request.unwrap();
        assert_eq!(request.name, "fleet");
        assert_eq!(request.namespace, "default");
        assert_eq!(request.status, target.status);
        assert_eq!(request.labels["region"], "test");
        assert_eq!(request.annotations["owner"], "agones");
        assert!(!request.uid.is_empty());
        handle.join().unwrap();
    }

    #[tokio::test]
    async fn cgp_webhook_dispatch_honors_scale_false() {
        let response = FleetAutoscaleReview {
            request: None,
            response: Some(FleetAutoscaleResponse {
                uid: "response".into(),
                scale: false,
                replicas: 99,
            }),
        };
        let (url, _received, handle) =
            serve_one_webhook_response(response);

        let mut target = fleet(4);
        target.status.replicas = 9;
        let decision =
            FleetAutoscalerController::new(MemoryStorage::new())
                .compute_fleet_autoscaler_desired_size(
                    &FleetAutoscalerPolicy {
                        policy_type:
                            FleetAutoscalerPolicyType::Webhook,
                        webhook: Some(UrlConfiguration {
                            url: Some(url),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    &target,
                    &FleetAutoscalerInventory::default(),
                )
                .await
                .unwrap();

        assert_eq!(decision.replicas, 9);
        assert!(!decision.limited);
        assert_eq!(
            decision.applied_policy,
            FleetAutoscalerPolicyType::Webhook
        );
        handle.join().unwrap();
    }

    #[tokio::test]
    async fn webhook_missing_response_is_rejected() {
        let (url, _received, handle) =
            serve_one_webhook_response(FleetAutoscaleReview::default());
        let error =
            FleetAutoscalerController::new(MemoryStorage::new())
                .apply_webhook_fleet_autoscaler_policy(
                    &UrlConfiguration {
                        url: Some(url),
                        ..Default::default()
                    },
                    &fleet(1),
                )
                .await
                .unwrap_err();

        assert!(error.to_string().contains(
            "webhook response missing required 'response' field"
        ));
        handle.join().unwrap();
    }

    fn scheduled_buffer_policy(
        start: Option<&str>,
        end: Option<&str>,
    ) -> FleetAutoscalerPolicy {
        FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Schedule,
            schedule: Some(Box::new(SchedulePolicy {
                between: Between {
                    start: start.map(str::to_owned),
                    end: end.map(str::to_owned),
                },
                active_period: ActivePeriod::default(),
                policy: FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        buffer_size: IntOrPercent::Int(1),
                        min_replicas: 3,
                        max_replicas: 10,
                    }),
                    ..Default::default()
                },
            })),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn cgp_schedule_dispatch_uses_explicit_clock_and_nested_provider() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new())
                .with_autoscaler_features(AutoscalerFeatures {
                    scheduled_autoscaler: true,
                    wasm_autoscaler: false,
                });
        let now = DateTime::parse_from_rfc3339(
            "2021-01-01T00:30:00Z",
        )
        .unwrap()
        .with_timezone(&Utc);

        let decision = controller
            .compute_fleet_autoscaler_desired_size_at(
                &scheduled_buffer_policy(
                    Some("2021-01-01T00:00:00Z"),
                    Some("2021-01-01T01:00:00Z"),
                ),
                &fleet(2),
                &FleetAutoscalerInventory::default(),
                now,
            )
            .await
            .unwrap();

        assert_eq!(decision.replicas, 3);
        assert!(!decision.limited);
        assert_eq!(
            decision.applied_policy,
            FleetAutoscalerPolicyType::Buffer
        );
    }

    #[tokio::test]
    async fn cgp_schedule_dispatch_enforces_feature_gate_and_inactive_error() {
        let now = DateTime::parse_from_rfc3339(
            "2021-01-01T00:30:00Z",
        )
        .unwrap()
        .with_timezone(&Utc);
        let policy = scheduled_buffer_policy(
            Some("2021-01-01T00:00:00Z"),
            Some("2021-01-01T00:15:00Z"),
        );

        let disabled =
            FleetAutoscalerController::new(MemoryStorage::new())
                .compute_fleet_autoscaler_desired_size_at(
                    &policy,
                    &fleet(2),
                    &FleetAutoscalerInventory::default(),
                    now,
                )
                .await
                .unwrap_err();
        assert!(disabled.to_string().contains(
            "feature flag ScheduledAutoscaler is enabled"
        ));

        let enabled =
            FleetAutoscalerController::new(MemoryStorage::new())
                .with_autoscaler_features(AutoscalerFeatures {
                    scheduled_autoscaler: true,
                    wasm_autoscaler: false,
                })
                .compute_fleet_autoscaler_desired_size_at(
                    &policy,
                    &fleet(2),
                    &FleetAutoscalerInventory::default(),
                    now,
                )
                .await
                .unwrap_err();
        assert!(matches!(
            enabled,
            AdapterError::InactiveFleetAutoscalerSchedule
        ));
    }


    fn chain_controller() -> FleetAutoscalerController<MemoryStorage> {
        FleetAutoscalerController::new(MemoryStorage::new())
            .with_autoscaler_features(AutoscalerFeatures {
                scheduled_autoscaler: true,
                wasm_autoscaler: false,
            })
    }

    fn buffer_chain_entry(id: &str, min_replicas: i32) -> ChainEntry {
        ChainEntry {
            id: id.into(),
            policy: FleetAutoscalerPolicy {
                policy_type: FleetAutoscalerPolicyType::Buffer,
                buffer: Some(BufferPolicy {
                    buffer_size: IntOrPercent::Int(1),
                    min_replicas,
                    max_replicas: 10,
                }),
                ..Default::default()
            },
        }
    }

    #[tokio::test]
    async fn cgp_chain_applies_first_success_and_records_entry_identity() {
        let now = DateTime::parse_from_rfc3339(
            "2021-11-05T05:30:00Z",
        )
        .unwrap()
        .with_timezone(&Utc);
        let inactive_schedule = ChainEntry {
            id: "schedule-1".into(),
            policy: scheduled_buffer_policy(
                Some("2024-08-01T16:07:36Z"),
                None,
            ),
        };
        let unsupported_webhook = ChainEntry {
            id: "webhook".into(),
            policy: FleetAutoscalerPolicy {
                policy_type:
                    FleetAutoscalerPolicyType::Webhook,
                ..Default::default()
            },
        };
        let policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Chain,
            chain: vec![
                inactive_schedule,
                unsupported_webhook,
                buffer_chain_entry("default", 6),
            ],
            ..Default::default()
        };

        let decision = chain_controller()
            .compute_fleet_autoscaler_desired_size_at(
                &policy,
                &fleet(2),
                &FleetAutoscalerInventory::default(),
                now,
            )
            .await
            .unwrap();

        assert_eq!(decision.replicas, 6);
        assert!(decision.limited);
        assert_eq!(
            decision.applied_policy.as_str(),
            "Chain:default:Buffer"
        );
    }

    #[tokio::test]
    async fn cgp_chain_uses_first_overlapping_active_schedule() {
        let now = DateTime::parse_from_rfc3339(
            "2024-08-01T16:07:36Z",
        )
        .unwrap()
        .with_timezone(&Utc);
        let first = ChainEntry {
            id: "schedule-1".into(),
            policy: FleetAutoscalerPolicy {
                policy_type: FleetAutoscalerPolicyType::Schedule,
                schedule: Some(Box::new(SchedulePolicy {
                    between: Between {
                        start: Some(
                            "2024-08-01T16:07:36Z".into(),
                        ),
                        end: None,
                    },
                    active_period: ActivePeriod {
                        timezone: "America/Chicago".into(),
                        start_cron: "* * * * *".into(),
                        duration: String::new(),
                    },
                    policy: FleetAutoscalerPolicy {
                        policy_type:
                            FleetAutoscalerPolicyType::Buffer,
                        buffer: Some(BufferPolicy {
                            buffer_size: IntOrPercent::Int(1),
                            min_replicas: 10,
                            max_replicas: 10,
                        }),
                        ..Default::default()
                    },
                })),
                ..Default::default()
            },
        };
        let second = buffer_chain_entry("default", 6);
        let policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Chain,
            chain: vec![first, second],
            ..Default::default()
        };

        let decision = chain_controller()
            .compute_fleet_autoscaler_desired_size_at(
                &policy,
                &fleet(2),
                &FleetAutoscalerInventory::default(),
                now,
            )
            .await
            .unwrap();

        assert_eq!(decision.replicas, 10);
        assert!(decision.limited);
        assert_eq!(
            decision.applied_policy.as_str(),
            "Chain:schedule-1:Schedule"
        );
    }

    #[tokio::test]
    async fn cgp_chain_all_inactive_schedules_fall_off_with_inactive_error() {
        let now = DateTime::parse_from_rfc3339(
            "2021-01-01T00:30:00Z",
        )
        .unwrap()
        .with_timezone(&Utc);
        let inactive = || ChainEntry {
            id: "schedule".into(),
            policy: scheduled_buffer_policy(
                Some("2024-08-01T16:07:36Z"),
                None,
            ),
        };
        let policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Chain,
            chain: vec![inactive(), inactive()],
            ..Default::default()
        };

        let error = chain_controller()
            .compute_fleet_autoscaler_desired_size_at(
                &policy,
                &fleet(2),
                &FleetAutoscalerInventory::default(),
                now,
            )
            .await
            .unwrap_err();
        assert!(matches!(
            error,
            AdapterError::InactiveFleetAutoscalerSchedule
        ));
    }

    #[tokio::test]
    async fn cgp_chain_enforces_scheduled_autoscaler_feature_gate() {
        let error =
            FleetAutoscalerController::new(MemoryStorage::new())
                .compute_fleet_autoscaler_desired_size_at(
                    &FleetAutoscalerPolicy {
                        policy_type:
                            FleetAutoscalerPolicyType::Chain,
                        chain: vec![
                            buffer_chain_entry("default", 6),
                        ],
                        ..Default::default()
                    },
                    &fleet(2),
                    &FleetAutoscalerInventory::default(),
                    Utc::now(),
                )
                .await
                .unwrap_err();

        assert!(error.to_string().contains(
            "cannot apply ChainPolicy unless feature flag ScheduledAutoscaler is enabled"
        ));
    }

    #[tokio::test]
    async fn missing_buffer_configuration_is_reported_without_panic() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let error = controller
            .compute_fleet_autoscaler_desired_size(
                &FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Buffer,
                    ..Default::default()
                },
                &fleet(1),
                &FleetAutoscalerInventory::default(),
            )
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("buffer policy config params are missing")
        );
    }

    #[tokio::test]
    async fn unsupported_policy_preserves_upstream_dispatch_error() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let error = controller
            .compute_fleet_autoscaler_desired_size(
                &FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Wasm,
                    ..Default::default()
                },
                &fleet(1),
                &FleetAutoscalerInventory::default(),
            )
            .await
            .unwrap_err();

        assert!(error.to_string().contains(
            "wrong policy type, should be one of: Buffer, Webhook, Counter, List, Schedule, Chain"
        ));
    }
}


#[cfg(test)]
mod reconciliation_tests {
    use agones_core::{
        ActivePeriod, AggregatedCounterStatus, AutoscalerFeatures,
        Between, BufferPolicy, ChainEntry, CounterPolicy, CounterStatus,
        FleetAutoscalerSpec, FleetSpec, IntOrPercent, ObjectMeta,
        SchedulePolicy,
    };
    use std::collections::BTreeMap;
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn autoscaler(buffer: i32) -> FleetAutoscaler {
        let mut value = FleetAutoscaler {
            metadata: ObjectMeta {
                name: "fas".into(),
                namespace: "default".into(),
                uid: "fas-uid".into(),
                ..Default::default()
            },
            spec: FleetAutoscalerSpec {
                fleet_name: "fleet".into(),
                policy: FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        buffer_size: IntOrPercent::Int(buffer),
                        min_replicas: 0,
                        max_replicas: 100,
                    }),
                    ..Default::default()
                },
                sync: None,
            },
            ..Default::default()
        };
        value.apply_defaults();
        value
    }

    fn fleet(
        spec_replicas: i32,
        status_replicas: i32,
        allocated_replicas: i32,
    ) -> Fleet {
        Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                uid: "fleet-uid".into(),
                ..Default::default()
            },
            spec: FleetSpec {
                replicas: spec_replicas,
                ..Default::default()
            },
            status: agones_core::FleetStatus {
                replicas: status_replicas,
                allocated_replicas,
                ..Default::default()
            },
        }
    }

    async fn store_pair(
        storage: &MemoryStorage,
        fleet_autoscaler: &FleetAutoscaler,
        fleet: &Fleet,
    ) {
        let repository = AgonesRepository::new(storage.clone());
        repository.create(fleet_autoscaler).await.unwrap();
        repository.create(fleet).await.unwrap();
    }

    #[tokio::test]
    async fn reconcile_scales_up_and_persists_status() {
        let storage = MemoryStorage::new();
        store_pair(
            &storage,
            &autoscaler(7),
            &fleet(5, 5, 5),
        )
        .await;

        let controller =
            FleetAutoscalerController::new(storage.clone());
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("stored autoscaler should reconcile");
        };

        assert_eq!(result.decision.replicas, 12);
        assert!(result.scaled);
        assert_eq!(result.fleet.spec.replicas, 12);
        assert!(result.fleet_autoscaler.status.able_to_scale);
        assert!(!result.fleet_autoscaler.status.scaling_limited);
        assert_eq!(
            result.fleet_autoscaler.status.current_replicas,
            5
        );
        assert_eq!(
            result.fleet_autoscaler.status.desired_replicas,
            12
        );
        assert_eq!(
            result.fleet_autoscaler.status.last_applied_policy,
            FleetAutoscalerPolicyType::Buffer
        );
        assert!(
            result
                .fleet_autoscaler
                .status
                .last_scale_time
                .is_some()
        );

        let repository = AgonesRepository::new(storage);
        let stored: Fleet =
            repository.get("default", "fleet").await.unwrap();
        assert_eq!(stored.spec.replicas, 12);
    }


    #[tokio::test]
    async fn reconcile_scales_up_with_counter_policy() {
        let storage = MemoryStorage::new();
        let mut fleet_autoscaler = autoscaler(1);
        fleet_autoscaler.spec.policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Counter,
            counter: Some(CounterPolicy {
                key: "rooms".into(),
                min_capacity: 10,
                max_capacity: 100,
                buffer_size: IntOrPercent::Int(10),
            }),
            ..Default::default()
        };
        let mut target = fleet(10, 10, 10);
        target.spec.template.spec.counters =
            Some(BTreeMap::from([(
                "rooms".into(),
                CounterStatus {
                    count: 0,
                    capacity: 7,
                },
            )]));
        target.status.counters.insert(
            "rooms".into(),
            AggregatedCounterStatus {
                count: 68,
                capacity: 70,
                ..Default::default()
            },
        );
        store_pair(&storage, &fleet_autoscaler, &target).await;

        let outcome =
            FleetAutoscalerController::new(storage.clone())
                .reconcile_fleet_autoscaler("default", "fas")
                .await
                .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("counter autoscaler should reconcile");
        };

        assert_eq!(result.decision.replicas, 12);
        assert_eq!(
            result.decision.applied_policy,
            FleetAutoscalerPolicyType::Counter
        );
        assert_eq!(result.fleet.spec.replicas, 12);

        let stored: Fleet =
            AgonesRepository::new(storage)
                .get("default", "fleet")
                .await
                .unwrap();
        assert_eq!(stored.spec.replicas, 12);
    }


    #[tokio::test]
    async fn reconcile_active_schedule_delegates_to_nested_buffer() {
        let storage = MemoryStorage::new();
        let mut fleet_autoscaler = autoscaler(1);
        fleet_autoscaler.spec.policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Schedule,
            schedule: Some(Box::new(SchedulePolicy {
                between: Between::default(),
                active_period: ActivePeriod::default(),
                policy: FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        buffer_size: IntOrPercent::Int(7),
                        min_replicas: 0,
                        max_replicas: 100,
                    }),
                    ..Default::default()
                },
            })),
            ..Default::default()
        };
        store_pair(
            &storage,
            &fleet_autoscaler,
            &fleet(5, 5, 5),
        )
        .await;

        let outcome = FleetAutoscalerController::new(storage)
            .with_autoscaler_features(AutoscalerFeatures {
                scheduled_autoscaler: true,
                wasm_autoscaler: false,
            })
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("active schedule should reconcile");
        };

        assert_eq!(result.fleet.spec.replicas, 12);
        assert_eq!(
            result.decision.applied_policy,
            FleetAutoscalerPolicyType::Buffer
        );
    }

    #[tokio::test]
    async fn inactive_schedule_does_not_mark_autoscaler_unable() {
        let storage = MemoryStorage::new();
        let mut fleet_autoscaler = autoscaler(1);
        fleet_autoscaler.status.able_to_scale = true;
        fleet_autoscaler.status.current_replicas = 5;
        fleet_autoscaler.status.desired_replicas = 5;
        fleet_autoscaler.status.last_applied_policy =
            FleetAutoscalerPolicyType::Buffer;
        fleet_autoscaler.spec.policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Schedule,
            schedule: Some(Box::new(SchedulePolicy {
                between: Between {
                    start: None,
                    end: Some(
                        "2000-01-01T00:00:00Z".into(),
                    ),
                },
                active_period: ActivePeriod::default(),
                policy: FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        buffer_size: IntOrPercent::Int(7),
                        min_replicas: 0,
                        max_replicas: 100,
                    }),
                    ..Default::default()
                },
            })),
            ..Default::default()
        };
        store_pair(
            &storage,
            &fleet_autoscaler,
            &fleet(5, 5, 5),
        )
        .await;

        let error = FleetAutoscalerController::new(storage.clone())
            .with_autoscaler_features(AutoscalerFeatures {
                scheduled_autoscaler: true,
                wasm_autoscaler: false,
            })
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap_err();
        assert!(error.to_string().contains(
            "inactive schedule, policy not applicable"
        ));

        let stored: FleetAutoscaler =
            AgonesRepository::new(storage)
                .get("default", "fas")
                .await
                .unwrap();
        assert!(stored.status.able_to_scale);
        assert_eq!(stored.status.current_replicas, 5);
        assert_eq!(stored.status.desired_replicas, 5);
        assert_eq!(
            stored.status.last_applied_policy,
            FleetAutoscalerPolicyType::Buffer
        );
    }


    #[tokio::test]
    async fn reconcile_chain_persists_selected_policy_identity() {
        let storage = MemoryStorage::new();
        let mut fleet_autoscaler = autoscaler(1);
        fleet_autoscaler.spec.policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Chain,
            chain: vec![ChainEntry {
                id: "default".into(),
                policy: FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        buffer_size: IntOrPercent::Int(7),
                        min_replicas: 0,
                        max_replicas: 100,
                    }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        };
        store_pair(
            &storage,
            &fleet_autoscaler,
            &fleet(5, 5, 5),
        )
        .await;

        let outcome = FleetAutoscalerController::new(storage)
            .with_autoscaler_features(AutoscalerFeatures {
                scheduled_autoscaler: true,
                wasm_autoscaler: false,
            })
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("chain should reconcile");
        };

        assert_eq!(result.fleet.spec.replicas, 12);
        assert_eq!(
            result
                .fleet_autoscaler
                .status
                .last_applied_policy
                .as_str(),
            "Chain:default:Buffer"
        );
    }

    #[tokio::test]
    async fn reconcile_scales_down_with_buffer_policy() {
        let storage = MemoryStorage::new();
        store_pair(
            &storage,
            &autoscaler(8),
            &fleet(20, 20, 5),
        )
        .await;

        let controller =
            FleetAutoscalerController::new(storage);
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("stored autoscaler should reconcile");
        };

        assert_eq!(result.decision.replicas, 13);
        assert_eq!(result.fleet.spec.replicas, 13);
        assert_eq!(
            result.fleet_autoscaler.status.current_replicas,
            20
        );
        assert_eq!(
            result.fleet_autoscaler.status.desired_replicas,
            13
        );
    }

    #[tokio::test]
    async fn missing_fleet_marks_autoscaler_unable_to_scale() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut value = autoscaler(5);
        value.status.able_to_scale = true;
        value.status.current_replicas = 5;
        value.status.desired_replicas = 10;
        value.status.last_applied_policy =
            FleetAutoscalerPolicyType::Buffer;
        repository.create(&value).await.unwrap();

        let controller =
            FleetAutoscalerController::new(storage.clone());
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::FleetNotFound(updated) =
            outcome
        else {
            panic!("missing Fleet should be a successful sync");
        };

        assert!(!updated.status.able_to_scale);
        assert!(!updated.status.scaling_limited);
        assert_eq!(updated.status.current_replicas, 0);
        assert_eq!(updated.status.desired_replicas, 0);
        assert_eq!(
            updated.status.last_applied_policy.as_str(),
            ""
        );
    }

    #[tokio::test]
    async fn deleting_fleet_is_a_noop() {
        let storage = MemoryStorage::new();
        let fleet_autoscaler = autoscaler(7);
        let mut deleting = fleet(5, 5, 5);
        deleting.metadata.deletion_timestamp =
            Some("2026-10-18T00:00:00Z".into());
        store_pair(&storage, &fleet_autoscaler, &deleting).await;

        let controller =
            FleetAutoscalerController::new(storage.clone());
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        assert!(matches!(
            outcome,
            FleetAutoscalerSyncOutcome::FleetDeleting { .. }
        ));

        let repository = AgonesRepository::new(storage);
        let stored_fleet: Fleet =
            repository.get("default", "fleet").await.unwrap();
        let stored_fas: FleetAutoscaler =
            repository.get("default", "fas").await.unwrap();
        assert_eq!(stored_fleet.spec.replicas, 5);
        assert_eq!(
            stored_fas.status,
            fleet_autoscaler.status
        );
    }

    #[tokio::test]
    async fn wrong_policy_marks_unable_and_returns_calculation_error() {
        let storage = MemoryStorage::new();
        let mut value = autoscaler(5);
        value.spec.policy = FleetAutoscalerPolicy {
            policy_type:
                FleetAutoscalerPolicyType::Unknown(
                    "WRONG TYPE".into(),
                ),
            ..Default::default()
        };
        store_pair(&storage, &value, &fleet(5, 5, 1)).await;

        let controller =
            FleetAutoscalerController::new(storage.clone());
        let error = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap_err();
        assert!(error.to_string().contains(
            "error calculating autoscaling fleet: fleet"
        ));

        let stored: FleetAutoscaler =
            AgonesRepository::new(storage)
                .get("default", "fas")
                .await
                .unwrap();
        assert!(!stored.status.able_to_scale);
        assert_eq!(stored.status.current_replicas, 0);
        assert_eq!(stored.status.desired_replicas, 0);
    }

    #[tokio::test]
    async fn missing_autoscaler_is_a_successful_noop() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        assert_eq!(
            controller
                .reconcile_fleet_autoscaler(
                    "default",
                    "missing",
                )
                .await
                .unwrap(),
            FleetAutoscalerSyncOutcome::NotFound
        );
    }

    #[tokio::test]
    async fn no_spec_scale_preserves_last_scale_time() {
        let storage = MemoryStorage::new();
        let mut fleet_autoscaler = autoscaler(5);
        fleet_autoscaler.status.able_to_scale = true;
        fleet_autoscaler.status.current_replicas = 10;
        fleet_autoscaler.status.desired_replicas = 10;
        fleet_autoscaler.status.last_applied_policy =
            FleetAutoscalerPolicyType::Buffer;
        fleet_autoscaler.status.last_scale_time =
            Some("2026-10-17T00:00:00Z".into());

        let target = fleet(10, 10, 5);
        store_pair(&storage, &fleet_autoscaler, &target).await;

        let controller =
            FleetAutoscalerController::new(storage);
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("stored autoscaler should reconcile");
        };

        assert!(!result.scaled);
        assert_eq!(
            result
                .fleet_autoscaler
                .status
                .last_scale_time
                .as_deref(),
            Some("2026-10-17T00:00:00Z")
        );
    }
}
