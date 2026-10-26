// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/webhooks/webhooks.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_camel_case_types)]
use std::{collections::BTreeMap, sync::{Arc, RwLock}};
use derive_deftly::Deftly;
use crate::{errors::{BoxError, Errors}, logfields::LogEntry, runtime,
    utility_http::{HttpRequest, ResponseWriter, SourceMux}, utility_runtime::UtilityRuntimeContext,
    utility_logging::{CanEmitSourceLog, LogLevel},
    utility_admission::{CanCallAdmissionHandler, Status, StatusDetails, StatusCause},
    utility_declarations::derive_deftly_template_SourceErrorOrigin};
pub use crate::utility_admission::{AdmissionReview, AdmissionRequest, AdmissionResponse, GroupKind, Handler};

#[derive(Clone)]
struct operationHandler { handler: Handler, groupKind: GroupKind, operation: String }
struct WebHookState { logger: LogEntry, errs: Errors, handlers: RwLock<BTreeMap<String, Vec<operationHandler>>> }

#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "agones.dev/agones/pkg/util/webhooks", source_struct = "WebHook", source_log_type = "*webhooks.WebHook")]
pub struct WebHook { state: Arc<WebHookState>, mux: SourceMux, runtime: UtilityRuntimeContext }

pub fn NewWebHook(context: &UtilityRuntimeContext, mux: SourceMux) -> WebHook {
    WebHook { state: Arc::new(WebHookState { logger: runtime::NewLoggerWithType::<_, WebHook>(context, None),
        errs: Errors::FromStruct::<WebHook>(None, None), handlers: RwLock::default() }), mux, runtime: context.clone() }
}
impl WebHook {
    pub fn AddHandler(&self, path: &str, gk: GroupKind, op: String, handler: Handler) {
        let mut handlers = self.state.handlers.write().unwrap_or_else(|error| error.into_inner());
        if handlers.get(path).is_none_or(Vec::is_empty) {
            // A closure retains the shared data without retaining its mux.
            let (state, registered_path) = (Arc::clone(&self.state), path.to_owned());
            self.mux.handle(path, Arc::new(move |context, writer, request| {
                let (state, path) = (Arc::clone(&state), registered_path.clone());
                Box::pin(async move {
                    if let Err(error) = Self::handle(context, &state, &path, writer, request).await {
                        runtime::HandleError(context, Some(&state.logger.with_field("url", request.url_log_value.clone())), Some(error.as_ref())).await;
                        let _ = writer.write_header(500).await;
                    }
                })
            }));
        }
        self.runtime.emit_source_log(&self.state.logger.with_field("path", path)
            .with_field("groupKind", serde_json::to_value(&gk).expect("string group-kind"))
            .with_field("op", op.clone()), LogLevel::Debug, "Added webhook handler");
        handlers.entry(path.to_owned()).or_default().push(operationHandler { groupKind: gk, operation: op, handler });
    }
    async fn handle(context: &UtilityRuntimeContext, state: &WebHookState, path: &str, writer: &mut ResponseWriter, request: &mut HttpRequest) -> Result<(), BoxError> {
        context.emit_source_log(&state.logger.with_field("path", path), LogLevel::Debug, "running webhook");
        let mut review: AdmissionReview = request.decode_one_json::<Option<AdmissionReview>>().await.map_err(|error|
            state.errs.Wrap(Some(error), &format!("error decoding decoding json for path {path}")).expect("non-nil decode error"))?.unwrap_or_default();
        if review.response.is_none() { review.response = Some(AdmissionResponse { allowed: true, ..Default::default() }); }
        // Source dereferences a missing Request; it does not invent an allowed
        // empty admission request or turn this branch into a normal response.
        let incoming = review.request.as_ref().expect("nil AdmissionReview.Request");
        review.response.as_mut().expect("initialized response").uid = incoming.uid.clone();
        context.emit_source_log(&state.logger.with_field("name", incoming.name.clone()).with_field("path", path)
            .with_field("kind", incoming.kind.kind.clone()).with_field("group", incoming.kind.group.clone()), LogLevel::Debug, "handling webhook request");
        let handlers = state.handlers.read().unwrap_or_else(|error| error.into_inner()).get(path).cloned().unwrap_or_default();
        for operation in handlers {
            let incoming = review.request.as_ref().expect("nil AdmissionReview.Request returned by handler");
            if operation.operation == incoming.operation && operation.groupKind.kind == incoming.kind.kind && operation.groupKind.group == incoming.kind.group {
                let (returned, error) = context.call_admission_handler(&operation.handler, review).await;
                review = returned;
                if let Some(error) = error {
                    let response = review.response.as_mut().expect("nil AdmissionReview.Response returned by handler");
                    response.allowed = false;
                    let incoming = review.request.as_ref().expect("nil AdmissionReview.Request returned by handler");
                    response.result = Some(Status { status: "Failure".into(), message: error.to_string(), reason: "InternalError".into(),
                        details: Some(StatusDetails { name: incoming.name.clone(), group: incoming.kind.group.clone(), kind: incoming.kind.kind.clone(),
                            causes: Some(vec![StatusCause { cause_type: "InternalError".into(), message: error.to_string(), ..Default::default() }]), ..Default::default() }),
                        ..Default::default() });
                }
            }
        }
        writer.encode_json(&review).await.map_err(|error|
            state.errs.Wrap(Some(error), &format!("error decoding encoding json for path {path}")).expect("non-nil encode error"))
    }
}
