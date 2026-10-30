//! HTTP bindings from the pinned Agones SDK and allocation protobuf files.
//! These are application routes, not grpc-gateway bytecode or a route language.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyBinding { None, All, Field(&'static str) }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RpcMode { Unary, ClientStream, ServerStream }
#[derive(Clone, Copy, Debug)]
pub struct PathBinding {
    pub prefix: &'static str,
    pub field: &'static str,
    pub suffix: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct GatewayRouteSpec {
    pub service: &'static str,
    pub rpc: &'static str,
    pub method: &'static str,
    pub path: &'static str,
    pub body: BodyBinding,
    pub mode: RpcMode,
    pub capture: Option<PathBinding>,
    pub infer_list_mask: bool,
}
impl GatewayRouteSpec {
    const fn new(service: &'static str, rpc: &'static str, method: &'static str,
        path: &'static str, body: BodyBinding, mode: RpcMode) -> Self {
        Self { service, rpc, method, path, body, mode, capture: None, infer_list_mask: false }
    }
    const fn capture(mut self, prefix: &'static str, field: &'static str, suffix: &'static str) -> Self {
        self.capture = Some(PathBinding { prefix, field, suffix }); self
    }
    const fn list_mask(mut self) -> Self { self.infer_list_mask = true; self }
    pub fn rpc_path(&self) -> String { format!("/{}/{}", self.service, self.rpc) }
}
use BodyBinding::{All, Field, None as NoBody};
use RpcMode::{ClientStream, ServerStream, Unary};

// proto/sdk/sdk.proto, service SDK.
const SDK: &str = "agones.dev.sdk.SDK";
pub static SDK_ROUTES: [GatewayRouteSpec; 9] = [
    GatewayRouteSpec::new(SDK, "Ready", "POST", "/ready", All, Unary),
    GatewayRouteSpec::new(SDK, "Allocate", "POST", "/allocate", All, Unary),
    GatewayRouteSpec::new(SDK, "Shutdown", "POST", "/shutdown", All, Unary),
    GatewayRouteSpec::new(SDK, "Health", "POST", "/health", All, ClientStream),
    GatewayRouteSpec::new(SDK, "GetGameServer", "GET", "/gameserver", NoBody, Unary),
    GatewayRouteSpec::new(SDK, "WatchGameServer", "GET", "/watch/gameserver", NoBody, ServerStream),
    GatewayRouteSpec::new(SDK, "SetLabel", "PUT", "/metadata/label", All, Unary),
    GatewayRouteSpec::new(SDK, "SetAnnotation", "PUT", "/metadata/annotation", All, Unary),
    GatewayRouteSpec::new(SDK, "Reserve", "POST", "/reserve", All, Unary),
];
// proto/sdk/alpha/alpha.proto, service SDK.
const ALPHA: &str = "agones.dev.sdk.alpha.SDK";
pub static ALPHA_ROUTES: [GatewayRouteSpec; 7] = [
    GatewayRouteSpec::new(ALPHA, "PlayerConnect", "POST", "/alpha/player/connect", All, Unary),
    GatewayRouteSpec::new(ALPHA, "PlayerDisconnect", "POST", "/alpha/player/disconnect", All, Unary),
    GatewayRouteSpec::new(ALPHA, "SetPlayerCapacity", "PUT", "/alpha/player/capacity", All, Unary),
    GatewayRouteSpec::new(ALPHA, "GetPlayerCapacity", "GET", "/alpha/player/capacity", NoBody, Unary),
    GatewayRouteSpec::new(ALPHA, "GetPlayerCount", "GET", "/alpha/player/count", NoBody, Unary),
    GatewayRouteSpec::new(ALPHA, "IsPlayerConnected", "GET", "/alpha/player/connected/{playerID}", NoBody, Unary)
        .capture("/alpha/player/connected/", "playerID", ""),
    GatewayRouteSpec::new(ALPHA, "GetConnectedPlayers", "GET", "/alpha/player/connected", NoBody, Unary),
];
// proto/sdk/beta/beta.proto, service SDK. Path names override body names.
const BETA: &str = "agones.dev.sdk.beta.SDK";
pub static BETA_ROUTES: [GatewayRouteSpec; 6] = [
    GatewayRouteSpec::new(BETA, "GetCounter", "GET", "/v1beta1/counters/{name}", NoBody, Unary)
        .capture("/v1beta1/counters/", "name", ""),
    GatewayRouteSpec::new(BETA, "UpdateCounter", "PATCH", "/v1beta1/counters/{counterUpdateRequest.name}", Field("counterUpdateRequest"), Unary)
        .capture("/v1beta1/counters/", "counterUpdateRequest.name", ""),
    GatewayRouteSpec::new(BETA, "GetList", "GET", "/v1beta1/lists/{name}", NoBody, Unary)
        .capture("/v1beta1/lists/", "name", ""),
    GatewayRouteSpec::new(BETA, "UpdateList", "PATCH", "/v1beta1/lists/{list.name}", Field("list"), Unary)
        .capture("/v1beta1/lists/", "list.name", "").list_mask(),
    GatewayRouteSpec::new(BETA, "AddListValue", "POST", "/v1beta1/lists/{name}:addValue", All, Unary)
        .capture("/v1beta1/lists/", "name", ":addValue"),
    GatewayRouteSpec::new(BETA, "RemoveListValue", "POST", "/v1beta1/lists/{name}:removeValue", All, Unary)
        .capture("/v1beta1/lists/", "name", ":removeValue"),
];
// proto/allocation/allocation.proto, service AllocationService.
pub static ALLOCATION_ROUTES: [GatewayRouteSpec; 1] = [
    GatewayRouteSpec::new("allocation.AllocationService", "Allocate", "POST", "/gameserverallocation", All, Unary),
];
