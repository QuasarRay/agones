use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use agones_core::{
    DEFAULT_INTERVAL_SYNC_SECONDS, FleetAutoscaler, FleetAutoscalerSyncType,
};
use agones_macros::agones_invariant;
use cgp::prelude::*;
use futures::stream::{BoxStream, FuturesUnordered};
use futures::StreamExt;
use rusternetes_storage::Storage;

use crate::controller_work_queue::{
    ControllerWorkItem, ControllerWorkQueue, FastSlowRetryPolicy,
};
use crate::{
    AdapterError, AgonesRepository, AgonesWatchEvent,
    CanEvictFleetAutoscalerWasmState, CanReconcileFleetAutoscaler,
    FleetAutoscalerController,
    FleetAutoscalerSyncOutcome, HasFleetAutoscalerStorage,
};

pub const FLEET_AUTOSCALER_FAST_RETRIES: u32 = 5;
pub const FLEET_AUTOSCALER_FAST_RETRY_DELAY_MS: u64 = 200;
pub const FLEET_AUTOSCALER_SLOW_RETRY_DELAY_MS: u64 = 3_000;

pub const fn fleet_autoscaler_retry_delay_ms(retries: u32) -> u64 {
    if retries < FLEET_AUTOSCALER_FAST_RETRIES {
        FLEET_AUTOSCALER_FAST_RETRY_DELAY_MS
    } else {
        FLEET_AUTOSCALER_SLOW_RETRY_DELAY_MS
    }
}

#[agones_invariant]
fn fleet_autoscaler_retry_policy_matches_upstream(retries: u8) -> bool {
    let retries = retries as u32;
    fleet_autoscaler_retry_delay_ms(retries)
        == if retries < FLEET_AUTOSCALER_FAST_RETRIES {
            FLEET_AUTOSCALER_FAST_RETRY_DELAY_MS
        } else {
            FLEET_AUTOSCALER_SLOW_RETRY_DELAY_MS
        }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetAutoscalerWorkItem {
    pub namespace: String,
    pub name: String,
}

impl FleetAutoscalerWorkItem {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }
}

impl ControllerWorkItem for FleetAutoscalerWorkItem {
    fn work_key(&self) -> String {
        self.key()
    }
}

type FleetAutoscalerWorkQueue =
    ControllerWorkQueue<FleetAutoscalerWorkItem, FastSlowRetryPolicy>;

fn fleet_autoscaler_retry_policy() -> FastSlowRetryPolicy {
    FastSlowRetryPolicy {
        fast_retries: FLEET_AUTOSCALER_FAST_RETRIES,
        fast_delay: Duration::from_millis(
            FLEET_AUTOSCALER_FAST_RETRY_DELAY_MS,
        ),
        slow_delay: Duration::from_millis(
            FLEET_AUTOSCALER_SLOW_RETRY_DELAY_MS,
        ),
    }
}

pub type FleetAutoscalerControllerWatchStream =
    BoxStream<'static, Result<AgonesWatchEvent<FleetAutoscaler>, AdapterError>>;

pub async fn watch_fleet_autoscaler_controller_events<S>(
    storage: Arc<S>,
) -> Result<FleetAutoscalerControllerWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    AgonesRepository::new(storage)
        .watch_all::<FleetAutoscaler>()
        .await
}

pub fn fleet_autoscaler_generation(
    fleet_autoscaler: &FleetAutoscaler,
) -> i64 {
    fleet_autoscaler
        .metadata
        .extensions
        .get("generation")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(0)
}

pub fn fleet_autoscaler_fixed_interval_seconds(
    fleet_autoscaler: &FleetAutoscaler,
) -> Result<u64, AdapterError> {
    let Some(sync) = fleet_autoscaler.spec.sync.as_ref() else {
        return Ok(DEFAULT_INTERVAL_SYNC_SECONDS as u64);
    };

    if !matches!(
        sync.sync_type,
        FleetAutoscalerSyncType::FixedInterval
    ) {
        return Err(AdapterError::Hook(
            "unsupported FleetAutoscaler sync type".into(),
        ));
    }

    if sync.fixed_interval.seconds <= 0 {
        return Err(AdapterError::Hook(format!(
            "FleetAutoscaler fixed interval must be positive, got {}",
            sync.fixed_interval.seconds
        )));
    }

    Ok(sync.fixed_interval.seconds as u64)
}

struct FleetAutoscalerTimerEntry {
    generation: i64,
    handle: tokio::task::JoinHandle<()>,
}

/// Runtime-owned equivalent of upstream's UID-keyed fasThreads map.
///
/// The queue is embedded so scheduling capabilities can remain expressed in
/// terms of one public state object rather than leaking queue internals through
/// CGP trait signatures.
pub struct FleetAutoscalerScheduleRegistry {
    queue: FleetAutoscalerWorkQueue,
    timers: HashMap<String, FleetAutoscalerTimerEntry>,
}

impl FleetAutoscalerScheduleRegistry {
    fn new(queue: FleetAutoscalerWorkQueue) -> Self {
        Self {
            queue,
            timers: HashMap::new(),
        }
    }

    fn uid_key(fleet_autoscaler: &FleetAutoscaler) -> String {
        if fleet_autoscaler.metadata.uid.is_empty() {
            format!(
                "{}/{}",
                fleet_autoscaler.metadata.namespace,
                fleet_autoscaler.metadata.name
            )
        } else {
            fleet_autoscaler.metadata.uid.clone()
        }
    }

    fn upsert(
        &mut self,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<bool, AdapterError> {
        let uid = Self::uid_key(fleet_autoscaler);
        let generation = fleet_autoscaler_generation(fleet_autoscaler);

        if self
            .timers
            .get(uid.as_str())
            .is_some_and(|entry| entry.generation == generation)
        {
            return Ok(false);
        }

        if let Some(previous) = self.timers.remove(uid.as_str()) {
            previous.handle.abort();
        }

        let seconds =
            fleet_autoscaler_fixed_interval_seconds(fleet_autoscaler)?;
        let item = FleetAutoscalerWorkItem::new(
            fleet_autoscaler.metadata.namespace.clone(),
            fleet_autoscaler.metadata.name.clone(),
        );

        // Upstream performs an immediate enqueue on Add/recreated generation.
        self.queue.enqueue(item.clone());

        let queue = self.queue.clone();
        let handle = tokio::spawn(async move {
            let interval = Duration::from_secs(seconds);
            loop {
                tokio::time::sleep(interval).await;
                queue.enqueue(item.clone());
            }
        });

        self.timers.insert(
            uid,
            FleetAutoscalerTimerEntry {
                generation,
                handle,
            },
        );
        Ok(true)
    }

    fn remove(
        &mut self,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> bool {
        let uid = Self::uid_key(fleet_autoscaler);
        self.timers.remove(uid.as_str()).is_some_and(|entry| {
            entry.handle.abort();
            true
        })
    }

    fn clear(&mut self) {
        for (_, entry) in self.timers.drain() {
            entry.handle.abort();
        }
    }

    #[cfg(test)]
    fn contains(&self, fleet_autoscaler: &FleetAutoscaler) -> bool {
        self.timers
            .contains_key(Self::uid_key(fleet_autoscaler).as_str())
    }

    #[cfg(test)]
    fn generation_for(
        &self,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Option<i64> {
        self.timers
            .get(Self::uid_key(fleet_autoscaler).as_str())
            .map(|entry| entry.generation)
    }
}

impl Drop for FleetAutoscalerScheduleRegistry {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cgp_component(FleetAutoscalerScheduler)]
pub trait CanScheduleFleetAutoscaler {
    fn upsert_fleet_autoscaler_schedule(
        &self,
        registry: &mut FleetAutoscalerScheduleRegistry,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<bool, AdapterError>;

    fn remove_fleet_autoscaler_schedule(
        &self,
        registry: &mut FleetAutoscalerScheduleRegistry,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> bool;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerScheduler<Context>
    for ScheduleFleetAutoscalerWithTokio
{
    fn upsert_fleet_autoscaler_schedule(
        _context: &Context,
        registry: &mut FleetAutoscalerScheduleRegistry,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<bool, AdapterError> {
        registry.upsert(fleet_autoscaler)
    }

    fn remove_fleet_autoscaler_schedule(
        _context: &Context,
        registry: &mut FleetAutoscalerScheduleRegistry,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> bool {
        registry.remove(fleet_autoscaler)
    }
}

#[async_trait]
#[cgp_component(FleetAutoscalerScheduleRebuilder)]
pub trait CanRebuildFleetAutoscalerSchedules {
    async fn rebuild_fleet_autoscaler_schedules(
        &self,
        registry: &mut FleetAutoscalerScheduleRegistry,
    ) -> Result<(), AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerScheduleRebuilder<Context>
    for RebuildFleetAutoscalerSchedulesWithStorage
where
    Context: HasFleetAutoscalerStorage
        + CanScheduleFleetAutoscaler
        + Sync,
{
    async fn rebuild_fleet_autoscaler_schedules(
        context: &Context,
        registry: &mut FleetAutoscalerScheduleRegistry,
    ) -> Result<(), AdapterError> {
        let values = AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ))
        .list_all::<FleetAutoscaler>()
        .await?;

        for fleet_autoscaler in values {
            context.upsert_fleet_autoscaler_schedule(
                registry,
                &fleet_autoscaler,
            )?;
        }

        Ok(())
    }
}

#[cgp_component(FleetAutoscalerEventRouter)]
pub trait CanRouteFleetAutoscalerEvent {
    fn route_fleet_autoscaler_event(
        &self,
        registry: &mut FleetAutoscalerScheduleRegistry,
        event: AgonesWatchEvent<FleetAutoscaler>,
    ) -> Result<(), AdapterError>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerEventRouter<Context>
    for RouteFleetAutoscalerEventWithScheduler
where
    Context: CanScheduleFleetAutoscaler
        + CanEvictFleetAutoscalerWasmState,
{
    fn route_fleet_autoscaler_event(
        context: &Context,
        registry: &mut FleetAutoscalerScheduleRegistry,
        event: AgonesWatchEvent<FleetAutoscaler>,
    ) -> Result<(), AdapterError> {
        match event {
            AgonesWatchEvent::Added { resource, .. }
            | AgonesWatchEvent::Modified { resource, .. } => {
                context.upsert_fleet_autoscaler_schedule(
                    registry,
                    &resource,
                )?;
            }
            AgonesWatchEvent::Deleted { resource, .. } => {
                context.remove_fleet_autoscaler_schedule(
                    registry,
                    &resource,
                );
                context.evict_fleet_autoscaler_wasm_state(
                    &resource,
                )?;
            }
        }

        Ok(())
    }
}

async fn run_fleet_autoscaler_worker<C>(
    controller: &C,
    queue: FleetAutoscalerWorkQueue,
) -> Result<(), AdapterError>
where
    C: CanReconcileFleetAutoscaler + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .reconcile_fleet_autoscaler(
                item.namespace.as_str(),
                item.name.as_str(),
            )
            .await
        {
            Ok(
                FleetAutoscalerSyncOutcome::NotFound
                | FleetAutoscalerSyncOutcome::FleetNotFound(_)
                | FleetAutoscalerSyncOutcome::FleetDeleting { .. }
                | FleetAutoscalerSyncOutcome::Reconciled(_),
            ) => queue.complete_success(&item),
            Err(_) => {
                let delay = queue.record_failure(&item);
                tokio::time::sleep(delay).await;
                queue.complete_retry(&item);
            }
        }
    }

    Ok(())
}

#[async_trait]
#[cgp_component(FleetAutoscalerControllerRunner)]
pub trait CanRunFleetAutoscalerController {
    async fn run_fleet_autoscaler_controller(
        &self,
        workers: usize,
    ) -> Result<(), AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerControllerRunner<Context>
    for RunFleetAutoscalerControllerWithRusternetes
where
    Context: HasFleetAutoscalerStorage
        + CanScheduleFleetAutoscaler
        + CanRebuildFleetAutoscalerSchedules
        + CanRouteFleetAutoscalerEvent
        + CanReconcileFleetAutoscaler
        + Sync,
{
    async fn run_fleet_autoscaler_controller(
        context: &Context,
        workers: usize,
    ) -> Result<(), AdapterError> {
        if workers == 0 {
            return Err(AdapterError::Hook(
                "FleetAutoscaler controller requires at least one worker"
                    .into(),
            ));
        }

        // Subscribe before LIST so events racing startup are retained. The
        // UID/generation registry and keyed queue collapse duplicate work.
        let mut events =
            watch_fleet_autoscaler_controller_events(Arc::clone(
                context.fleet_autoscaler_storage(),
            ))
            .await?;

        let queue =
            FleetAutoscalerWorkQueue::new(fleet_autoscaler_retry_policy());
        let mut registry =
            FleetAutoscalerScheduleRegistry::new(queue.clone());
        context
            .rebuild_fleet_autoscaler_schedules(&mut registry)
            .await?;

        let mut workers_set = FuturesUnordered::new();
        for _ in 0..workers {
            workers_set.push(run_fleet_autoscaler_worker(
                context,
                queue.clone(),
            ));
        }

        loop {
            tokio::select! {
                event = events.next() => {
                    let Some(event) = event else {
                        registry.clear();
                        return Ok(());
                    };
                    context.route_fleet_autoscaler_event(
                        &mut registry,
                        event?,
                    )?;
                }
                worker = workers_set.next() => {
                    match worker {
                        Some(Ok(())) => {
                            return Err(AdapterError::Hook(
                                "FleetAutoscaler worker queue terminated unexpectedly".into(),
                            ));
                        }
                        Some(Err(error)) => return Err(error),
                        None => {
                            return Err(AdapterError::Hook(
                                "FleetAutoscaler controller has no active workers".into(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

delegate_and_check_components! {
    <S: Storage + 'static>
    FleetAutoscalerController<S> {
        FleetAutoscalerSchedulerComponent:
            ScheduleFleetAutoscalerWithTokio,
        FleetAutoscalerScheduleRebuilderComponent:
            RebuildFleetAutoscalerSchedulesWithStorage,
        FleetAutoscalerEventRouterComponent:
            RouteFleetAutoscalerEventWithScheduler,
        FleetAutoscalerControllerRunnerComponent:
            RunFleetAutoscalerControllerWithRusternetes,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        BufferPolicy, Fleet, FleetAutoscalerPolicy,
        FleetAutoscalerPolicyType, FleetAutoscalerSpec, FleetSpec,
        FleetStatus, IntOrPercent, ObjectMeta,
    };
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn autoscaler(generation: i64, interval_seconds: i32) -> FleetAutoscaler {
        let mut fleet_autoscaler = FleetAutoscaler {
            metadata: ObjectMeta {
                name: "fas".into(),
                namespace: "default".into(),
                uid: "fas-uid".into(),
                extensions: BTreeMap::from([(
                    "generation".into(),
                    serde_json::json!(generation),
                )]),
                ..Default::default()
            },
            spec: FleetAutoscalerSpec {
                fleet_name: "fleet".into(),
                policy: FleetAutoscalerPolicy {
                    policy_type: FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        min_replicas: 0,
                        max_replicas: 100,
                        buffer_size: IntOrPercent::Int(7),
                    }),
                    ..Default::default()
                },
                sync: None,
            },
            ..Default::default()
        };
        fleet_autoscaler.apply_defaults();
        fleet_autoscaler
            .spec
            .sync
            .as_mut()
            .unwrap()
            .fixed_interval
            .seconds = interval_seconds;
        fleet_autoscaler
    }

    fn fleet() -> Fleet {
        Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                uid: "fleet-uid".into(),
                ..Default::default()
            },
            spec: FleetSpec {
                replicas: 5,
                ..Default::default()
            },
            status: FleetStatus {
                replicas: 5,
                allocated_replicas: 5,
                ..Default::default()
            },
        }
    }

    #[test]
    fn retry_schedule_matches_upstream_fast_rate_limiter() {
        for retries in 0..FLEET_AUTOSCALER_FAST_RETRIES {
            assert_eq!(
                fleet_autoscaler_retry_delay_ms(retries),
                FLEET_AUTOSCALER_FAST_RETRY_DELAY_MS
            );
        }
        assert_eq!(
            fleet_autoscaler_retry_delay_ms(
                FLEET_AUTOSCALER_FAST_RETRIES
            ),
            FLEET_AUTOSCALER_SLOW_RETRY_DELAY_MS
        );
    }

    #[test]
    fn generation_and_interval_are_read_from_resource_state() {
        let fleet_autoscaler = autoscaler(7, 11);
        assert_eq!(
            fleet_autoscaler_generation(&fleet_autoscaler),
            7
        );
        assert_eq!(
            fleet_autoscaler_fixed_interval_seconds(
                &fleet_autoscaler
            )
            .unwrap(),
            11
        );
    }

    #[tokio::test]
    async fn scheduler_enqueues_immediately_and_restarts_on_generation() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let queue =
            FleetAutoscalerWorkQueue::new(fleet_autoscaler_retry_policy());
        let mut registry =
            FleetAutoscalerScheduleRegistry::new(queue.clone());
        let first = autoscaler(1, 30);

        assert!(
            controller
                .upsert_fleet_autoscaler_schedule(
                    &mut registry,
                    &first,
                )
                .unwrap()
        );
        assert!(registry.contains(&first));
        assert_eq!(queue.next().await.unwrap().key(), "default/fas");
        queue.complete_success(
            &FleetAutoscalerWorkItem::new("default", "fas"),
        );

        assert!(
            !controller
                .upsert_fleet_autoscaler_schedule(
                    &mut registry,
                    &first,
                )
                .unwrap()
        );
        assert!(
            tokio::time::timeout(
                Duration::from_millis(10),
                queue.next(),
            )
            .await
            .is_err()
        );

        let second = autoscaler(2, 30);
        assert!(
            controller
                .upsert_fleet_autoscaler_schedule(
                    &mut registry,
                    &second,
                )
                .unwrap()
        );
        assert_eq!(registry.generation_for(&second), Some(2));
        assert_eq!(queue.next().await.unwrap().key(), "default/fas");
    }

    #[tokio::test]
    async fn event_router_adds_updates_and_deletes_uid_schedule() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let queue =
            FleetAutoscalerWorkQueue::new(fleet_autoscaler_retry_policy());
        let mut registry =
            FleetAutoscalerScheduleRegistry::new(queue);
        let first = autoscaler(1, 30);

        controller
            .route_fleet_autoscaler_event(
                &mut registry,
                AgonesWatchEvent::Added {
                    resource: first.clone(),
                    key: "default/fas".into(),
                },
            )
            .unwrap();
        assert!(registry.contains(&first));

        let second = autoscaler(2, 30);
        controller
            .route_fleet_autoscaler_event(
                &mut registry,
                AgonesWatchEvent::Modified {
                    resource: second.clone(),
                    key: "default/fas".into(),
                },
            )
            .unwrap();
        assert_eq!(registry.generation_for(&second), Some(2));

        controller
            .route_fleet_autoscaler_event(
                &mut registry,
                AgonesWatchEvent::Deleted {
                    resource: second.clone(),
                    key: "default/fas".into(),
                },
            )
            .unwrap();
        assert!(!registry.contains(&second));
    }

    #[tokio::test]
    async fn rebuild_schedules_existing_autoscalers() {
        let storage = MemoryStorage::new();
        AgonesRepository::new(storage.clone())
            .create(&autoscaler(1, 30))
            .await
            .unwrap();

        let controller =
            FleetAutoscalerController::new(storage);
        let queue =
            FleetAutoscalerWorkQueue::new(fleet_autoscaler_retry_policy());
        let mut registry =
            FleetAutoscalerScheduleRegistry::new(queue.clone());

        controller
            .rebuild_fleet_autoscaler_schedules(&mut registry)
            .await
            .unwrap();

        assert!(registry.contains(&autoscaler(1, 30)));
        assert_eq!(queue.next().await.unwrap().key(), "default/fas");
    }

    #[tokio::test]
    async fn runtime_reconciles_initial_snapshot_immediately() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        repository.create(&fleet()).await.unwrap();
        repository
            .create(&autoscaler(1, 30))
            .await
            .unwrap();

        let runtime_storage = storage.clone();
        let runtime = tokio::spawn(async move {
            FleetAutoscalerController::new(runtime_storage)
                .run_fleet_autoscaler_controller(1)
                .await
        });

        let repository = AgonesRepository::new(storage);
        let mut scaled = false;
        for _ in 0..100 {
            if repository
                .get::<Fleet>("default", "fleet")
                .await
                .unwrap()
                .spec
                .replicas
                == 12
            {
                scaled = true;
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        runtime.abort();
        assert!(scaled, "initial FleetAutoscaler was not reconciled");
    }

    #[tokio::test]
    async fn zero_workers_is_rejected() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let error = controller
            .run_fleet_autoscaler_controller(0)
            .await
            .unwrap_err();
        assert!(error.to_string().contains(
            "requires at least one worker"
        ));
    }
}
