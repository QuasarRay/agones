use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use agones_core::{
    Fleet, FleetAutoscaleReview, FleetAutoscaler, FleetAutoscalerScaleDecision,
    UrlConfiguration, WasmPolicy,
};
use cgp::prelude::*;
use extism::{Manifest, Plugin, Wasm};
use futures::future::BoxFuture;
use rusternetes_storage::Storage;

use crate::fleetautoscaler_controller::{
    fleet_autoscale_review_request, fleet_autoscaler_http_client,
    fleet_autoscaler_webhook_url,
};
use crate::{
    AdapterError, FleetAutoscalerController,
    HasFleetAutoscaleRequestMetadata, HasFleetAutoscalerRuntimeFeatures,
};

type CachedWasmPlugin = Arc<Mutex<Plugin>>;

/// Per-FleetAutoscaler-generation plugin state.
///
/// Upstream stores the Extism plugin in the UID-keyed fasState. The Rust
/// controller keeps the same lifetime semantics while allowing the state store
/// itself to be substituted through a CGP getter capability.
#[derive(Clone, Default)]
pub struct FleetAutoscalerWasmCache {
    plugins: Arc<RwLock<HashMap<String, CachedWasmPlugin>>>,
}

impl FleetAutoscalerWasmCache {
    fn state_owner(state_key: &str) -> &str {
        state_key
            .rsplit_once('@')
            .map_or(state_key, |(owner, _)| owner)
    }

    fn get(
        &self,
        state_key: &str,
    ) -> Result<Option<CachedWasmPlugin>, AdapterError> {
        Ok(self
            .plugins
            .read()
            .map_err(|_| {
                AdapterError::Hook(
                    "FleetAutoscaler Wasm cache lock poisoned".into(),
                )
            })?
            .get(state_key)
            .cloned())
    }

    fn insert_or_get(
        &self,
        state_key: String,
        plugin: Plugin,
    ) -> Result<CachedWasmPlugin, AdapterError> {
        let mut plugins = self.plugins.write().map_err(|_| {
            AdapterError::Hook(
                "FleetAutoscaler Wasm cache lock poisoned".into(),
            )
        })?;

        Ok(plugins
            .entry(state_key)
            .or_insert_with(|| Arc::new(Mutex::new(plugin)))
            .clone())
    }

    /// Keep only the current generation for this FleetAutoscaler UID/name.
    pub fn retain_state_key(
        &self,
        state_key: &str,
    ) -> Result<(), AdapterError> {
        if state_key.is_empty() {
            return Ok(());
        }

        let owner = Self::state_owner(state_key).to_owned();
        self.plugins
            .write()
            .map_err(|_| {
                AdapterError::Hook(
                    "FleetAutoscaler Wasm cache lock poisoned".into(),
                )
            })?
            .retain(|key, _| {
                key == state_key || Self::state_owner(key) != owner
            });
        Ok(())
    }

    pub fn remove_owner(
        &self,
        owner: &str,
    ) -> Result<(), AdapterError> {
        self.plugins
            .write()
            .map_err(|_| {
                AdapterError::Hook(
                    "FleetAutoscaler Wasm cache lock poisoned".into(),
                )
            })?
            .retain(|key, _| Self::state_owner(key) != owner);
        Ok(())
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.plugins
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .len()
    }
}

pub fn fleet_autoscaler_state_owner_key(
    fleet_autoscaler: &FleetAutoscaler,
) -> String {
    if fleet_autoscaler.metadata.uid.is_empty() {
        format!(
            "{}/{}",
            fleet_autoscaler.metadata.namespace,
            fleet_autoscaler.metadata.name,
        )
    } else {
        fleet_autoscaler.metadata.uid.clone()
    }
}

pub fn fleet_autoscaler_policy_state_key(
    fleet_autoscaler: &FleetAutoscaler,
) -> String {
    let generation = fleet_autoscaler
        .metadata
        .extensions
        .get("generation")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(0);
    format!(
        "{}@{generation}",
        fleet_autoscaler_state_owner_key(fleet_autoscaler),
    )
}

#[cgp_getter]
pub trait HasFleetAutoscalerWasmCache {
    fn fleet_autoscaler_wasm_cache(
        &self,
    ) -> &FleetAutoscalerWasmCache;
}

#[cgp_component(FleetAutoscalerWasmModuleFetcher)]
pub trait CanFetchFleetAutoscalerWasmModule {
    fn fetch_fleet_autoscaler_wasm_module<'a>(
        &'a self,
        configuration: &'a UrlConfiguration,
    ) -> BoxFuture<'a, Result<Vec<u8>, AdapterError>>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerWasmModuleFetcher<Context>
    for FetchFleetAutoscalerWasmModuleWithReqwest
where
    Context: Sync,
{
    fn fetch_fleet_autoscaler_wasm_module<'a>(
        _context: &'a Context,
        configuration: &'a UrlConfiguration,
    ) -> BoxFuture<'a, Result<Vec<u8>, AdapterError>> {
        Box::pin(async move {
            let url = fleet_autoscaler_webhook_url(configuration)?;
            let client = fleet_autoscaler_http_client(configuration)?;
            let response = client
                .get(url.as_str())
                .send()
                .await
                .map_err(|error| {
                    AdapterError::Hook(format!(
                        "failed to fetch Wasm module from {url}: {error}"
                    ))
                })?;

            if response.status() != reqwest::StatusCode::OK {
                return Err(AdapterError::Hook(format!(
                    "bad status code {} from the server: {url}",
                    response.status().as_u16(),
                )));
            }

            response
                .bytes()
                .await
                .map(|bytes| bytes.to_vec())
                .map_err(|error| {
                    AdapterError::Hook(format!(
                        "failed to read Wasm module from {url}: {error}"
                    ))
                })
        })
    }
}

#[cgp_component(FleetAutoscalerWasmExecutor)]
pub trait CanExecuteFleetAutoscalerWasm {
    fn execute_fleet_autoscaler_wasm<'a>(
        &'a self,
        state_key: &'a str,
        policy: &'a WasmPolicy,
        input: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<u8>, AdapterError>>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerWasmExecutor<Context>
    for ExecuteFleetAutoscalerWasmWithExtism
where
    Context: HasFleetAutoscalerWasmCache
        + CanFetchFleetAutoscalerWasmModule
        + Sync,
{
    fn execute_fleet_autoscaler_wasm<'a>(
        context: &'a Context,
        state_key: &'a str,
        policy: &'a WasmPolicy,
        input: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<u8>, AdapterError>> {
        Box::pin(async move {
            let cache = context.fleet_autoscaler_wasm_cache();
            cache.retain_state_key(state_key)?;

            let plugin = if let Some(plugin) = cache.get(state_key)? {
                plugin
            } else {
                let source =
                    policy.from.url.as_ref().ok_or_else(|| {
                        AdapterError::Hook(
                            "wasm from configuration is missing".into(),
                        )
                    })?;
                let source_url =
                    fleet_autoscaler_webhook_url(source)?;
                let bytes = context
                    .fetch_fleet_autoscaler_wasm_module(source)
                    .await?;

                let mut wasm = Wasm::data(bytes);
                if !policy.hash.is_empty() {
                    wasm = wasm.with_hash(policy.hash.clone());
                }
                let manifest = Manifest::new([wasm])
                    .with_config(policy.config.clone().into_iter());
                let plugin = Plugin::new(&manifest, [], true)
                    .map_err(|error| {
                        AdapterError::Hook(format!(
                            "failed to create Wasm plugin from {source_url}: {error}"
                        ))
                    })?;

                cache.insert_or_get(state_key.to_owned(), plugin)?
            };

            let mut plugin = plugin.lock().map_err(|_| {
                AdapterError::Hook(
                    "FleetAutoscaler Wasm plugin lock poisoned".into(),
                )
            })?;
            plugin
                .call::<&[u8], Vec<u8>>(
                    policy.function.as_str(),
                    input,
                )
                .map_err(|error| {
                    AdapterError::Hook(format!(
                        "failed to call Wasm plugin function {}: {error}",
                        policy.function,
                    ))
                })
        })
    }
}

#[cgp_component(WasmFleetAutoscalerPolicyEvaluator)]
pub trait CanApplyWasmFleetAutoscalerPolicy {
    fn apply_wasm_fleet_autoscaler_policy<'a>(
        &'a self,
        state_key: &'a str,
        policy: &'a WasmPolicy,
        fleet: &'a Fleet,
    ) -> BoxFuture<'a, Result<FleetAutoscalerScaleDecision, AdapterError>>;
}

#[cgp_new_provider]
impl<Context> WasmFleetAutoscalerPolicyEvaluator<Context>
    for ApplyWasmFleetAutoscalerPolicyWithExecutor
where
    Context: CanExecuteFleetAutoscalerWasm
        + HasFleetAutoscaleRequestMetadata
        + HasFleetAutoscalerRuntimeFeatures
        + Sync,
{
    fn apply_wasm_fleet_autoscaler_policy<'a>(
        context: &'a Context,
        state_key: &'a str,
        policy: &'a WasmPolicy,
        fleet: &'a Fleet,
    ) -> BoxFuture<'a, Result<FleetAutoscalerScaleDecision, AdapterError>> {
        Box::pin(async move {
            if !context
                .fleet_autoscaler_runtime_features()
                .wasm_autoscaler
            {
                return Err(AdapterError::Hook(
                    "cannot apply WasmPolicy unless feature flag WasmAutoscaler is enabled"
                        .into(),
                ));
            }

            let review = fleet_autoscale_review_request(
                fleet,
                *context.fleet_autoscale_request_metadata(),
            );
            let input = serde_json::to_vec(&review).map_err(|error| {
                AdapterError::Hook(format!(
                    "failed to marshal autoscaling request: {error}"
                ))
            })?;

            let output = context
                .execute_fleet_autoscaler_wasm(
                    state_key,
                    policy,
                    &input,
                )
                .await?;

            let review: FleetAutoscaleReview =
                serde_json::from_slice(&output).map_err(|error| {
                    AdapterError::Hook(format!(
                        "failed to unmarshal autoscaling response: {error}"
                    ))
                })?;
            let response = review.response.ok_or_else(|| {
                AdapterError::Hook(
                    "wasm response missing required 'response' field"
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

#[cgp_component(FleetAutoscalerWasmStateEvicter)]
pub trait CanEvictFleetAutoscalerWasmState {
    fn evict_fleet_autoscaler_wasm_state(
        &self,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<(), AdapterError>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerWasmStateEvicter<Context>
    for EvictFleetAutoscalerWasmStateFromCache
where
    Context: HasFleetAutoscalerWasmCache,
{
    fn evict_fleet_autoscaler_wasm_state(
        context: &Context,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<(), AdapterError> {
        context
            .fleet_autoscaler_wasm_cache()
            .remove_owner(
                fleet_autoscaler_state_owner_key(fleet_autoscaler)
                    .as_str(),
            )
    }
}

delegate_and_check_components! {
    <S: Storage + 'static>
    FleetAutoscalerController<S> {
        FleetAutoscalerWasmCacheGetterComponent:
            UseField<Symbol!("wasm_cache")>,
        FleetAutoscalerWasmModuleFetcherComponent:
            FetchFleetAutoscalerWasmModuleWithReqwest,
        FleetAutoscalerWasmExecutorComponent:
            ExecuteFleetAutoscalerWasmWithExtism,
        WasmFleetAutoscalerPolicyEvaluatorComponent:
            ApplyWasmFleetAutoscalerPolicyWithExecutor,
        FleetAutoscalerWasmStateEvicterComponent:
            EvictFleetAutoscalerWasmStateFromCache,
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    use agones_core::{
        AutoscalerFeatures, FleetStatus, ObjectMeta, WasmFrom,
    };
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn fleet(replicas: i32, allocated: i32) -> Fleet {
        Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            status: FleetStatus {
                replicas,
                allocated_replicas: allocated,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn wasm_policy(url: String, function: &str) -> WasmPolicy {
        WasmPolicy {
            function: function.into(),
            config: HashMap::from([
                ("buffer_size".into(), "5".into()),
            ])
            .into_iter()
            .collect(),
            from: WasmFrom {
                url: Some(UrlConfiguration {
                    url: Some(url),
                    ..Default::default()
                }),
            },
            hash: String::new(),
        }
    }

    fn serve_wasm_once(
        bytes: &'static [u8],
    ) -> (String, std::thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0u8; 2048];
            let _ = stream.read(&mut request).unwrap();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/wasm\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                bytes.len()
            )
            .unwrap();
            stream.write_all(bytes).unwrap();
            stream.flush().unwrap();
        });
        (format!("http://{address}/plugin.wasm"), handle)
    }

    #[test]
    fn state_key_tracks_uid_and_generation() {
        let mut autoscaler = FleetAutoscaler::default();
        autoscaler.metadata.uid = "uid".into();
        autoscaler
            .metadata
            .extensions
            .insert("generation".into(), serde_json::json!(7));
        assert_eq!(
            fleet_autoscaler_policy_state_key(&autoscaler),
            "uid@7"
        );
        assert_eq!(
            fleet_autoscaler_state_owner_key(&autoscaler),
            "uid"
        );
    }

    #[tokio::test]
    async fn wasm_policy_executes_upstream_example_and_caches_plugin() {
        static PLUGIN: &[u8] =
            include_bytes!("../../../examples/autoscaler-wasm/plugin.wasm");
        let (url, server) = serve_wasm_once(PLUGIN);
        let policy = wasm_policy(url, "scale");
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new())
                .with_autoscaler_features(AutoscalerFeatures {
                    scheduled_autoscaler: false,
                    wasm_autoscaler: true,
                });

        let decision = controller
            .apply_wasm_fleet_autoscaler_policy(
                "fas@1",
                &policy,
                &fleet(10, 8),
            )
            .await
            .unwrap();
        assert_eq!(
            decision,
            FleetAutoscalerScaleDecision {
                replicas: 13,
                limited: false,
            }
        );
        server.join().unwrap();

        let second = controller
            .apply_wasm_fleet_autoscaler_policy(
                "fas@1",
                &policy,
                &fleet(15, 10),
            )
            .await
            .unwrap();
        assert_eq!(second.replicas, 15);
        assert_eq!(controller.wasm_cache.len(), 1);
    }


    #[tokio::test]
    async fn wasm_policy_verifies_sha256_hash_like_upstream() {
        use sha2::{Digest, Sha256};

        static PLUGIN: &[u8] =
            include_bytes!("../../../examples/autoscaler-wasm/plugin.wasm");
        let digest = Sha256::digest(PLUGIN);
        let good_hash: String = digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();

        let (good_url, good_server) = serve_wasm_once(PLUGIN);
        let mut good = wasm_policy(good_url, "scale");
        good.hash = good_hash.clone();
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new())
                .with_autoscaler_features(AutoscalerFeatures {
                    scheduled_autoscaler: false,
                    wasm_autoscaler: true,
                });
        let decision = controller
            .apply_wasm_fleet_autoscaler_policy(
                "good@1",
                &good,
                &fleet(10, 8),
            )
            .await
            .unwrap();
        assert_eq!(decision.replicas, 13);
        good_server.join().unwrap();

        let (bad_url, bad_server) = serve_wasm_once(PLUGIN);
        let mut bad = wasm_policy(bad_url, "scale");
        let mut bad_hash = good_hash;
        bad_hash.replace_range(
            0..2,
            if &bad_hash[..2] == "00" { "ff" } else { "00" },
        );
        bad.hash = bad_hash;
        let error = controller
            .apply_wasm_fleet_autoscaler_policy(
                "bad@1",
                &bad,
                &fleet(10, 8),
            )
            .await
            .unwrap_err();
        assert!(
            error.to_string().contains("hash mismatch for module"),
            "unexpected hash error: {error}",
        );
        bad_server.join().unwrap();
    }

    #[tokio::test]
    async fn wasm_policy_scale_none_and_empty_response_match_upstream() {
        static PLUGIN: &[u8] =
            include_bytes!("../../../examples/autoscaler-wasm/plugin.wasm");

        let (url, server) = serve_wasm_once(PLUGIN);
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new())
                .with_autoscaler_features(AutoscalerFeatures {
                    scheduled_autoscaler: false,
                    wasm_autoscaler: true,
                });
        let none = controller
            .apply_wasm_fleet_autoscaler_policy(
                "fas@1",
                &wasm_policy(url, "scaleNone"),
                &fleet(10, 8),
            )
            .await
            .unwrap();
        assert_eq!(none.replicas, 10);
        server.join().unwrap();

        let error = controller
            .apply_wasm_fleet_autoscaler_policy(
                "fas@1",
                &wasm_policy(
                    "http://127.0.0.1:1/plugin.wasm".into(),
                    "scaleEmpty",
                ),
                &fleet(10, 8),
            )
            .await
            .unwrap_err();
        assert!(error.to_string().contains(
            "wasm response missing required 'response' field"
        ));
    }

    #[tokio::test]
    async fn wasm_policy_enforces_feature_gate() {
        let error =
            FleetAutoscalerController::new(MemoryStorage::new())
                .apply_wasm_fleet_autoscaler_policy(
                    "fas@1",
                    &WasmPolicy::default(),
                    &fleet(10, 8),
                )
                .await
                .unwrap_err();
        assert!(error.to_string().contains(
            "cannot apply WasmPolicy unless feature flag WasmAutoscaler is enabled"
        ));
    }

    #[test]
    fn cache_generation_cleanup_and_owner_eviction_are_idempotent() {
        let cache = FleetAutoscalerWasmCache::default();
        cache.retain_state_key("uid@1").unwrap();
        cache.retain_state_key("uid@2").unwrap();
        cache.remove_owner("uid").unwrap();
        assert_eq!(cache.len(), 0);
    }
}
