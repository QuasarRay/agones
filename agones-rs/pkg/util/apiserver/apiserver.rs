// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/apiserver/apiserver.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::{collections::BTreeMap, sync::{Arc, RwLock, LazyLock}};
use futures::future::BoxFuture;
use bytes::Bytes;
use derive_deftly::Deftly;
use better_comprehension::vector;
use crate::{errors::{BoxError, Errors}, logfields::LogEntry, runtime, https,
    utility_web::{SourceMux, HttpRequest, ResponseWriter, header, set_header},
    utility_runtime::UtilityRuntimeContext, utility_logging::{CanEmitSourceLog, LogLevel},
    utility_discovery::{CanEncodeApiObject, SourceScheme, CodecFactory, SerializerInfo, MetaObject},
    utility_declarations::derive_deftly_template_SourceErrorOrigin};
pub use crate::utility_discovery::{APIResource, APIResourceList};
pub static Scheme: SourceScheme = SourceScheme { unversioned_version: "v1", unversioned_kinds: &["Status", "APIResourceList"] };
pub static Codecs: LazyLock<CodecFactory> = LazyLock::new(CodecFactory::default);
const errs: Errors = Errors::from_package("agones.dev/agones/pkg/util/apiserver");
pub const ContentTypeHeader: &str = "Content-Type";
pub const AcceptHeader: &str = "Accept";
pub type CRDHandler = Arc<dyn for<'a> Fn(&'a UtilityRuntimeContext, &'a mut ResponseWriter, &'a mut HttpRequest, Bytes) -> BoxFuture<'a, Result<(), BoxError>> + Send + Sync>;

struct ApiState {
    logger: LogEntry, errs: Errors,
    resourceList: RwLock<BTreeMap<String, Arc<RwLock<APIResourceList>>>>,
    delegates: RwLock<BTreeMap<Bytes, CRDHandler>>,
}
#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "agones.dev/agones/pkg/util/apiserver", source_struct = "APIServer", source_log_type = "*apiserver.APIServer")]
pub struct APIServer { state: Arc<ApiState>, mux: SourceMux, runtime: UtilityRuntimeContext }

pub fn NewAPIServer(context: &UtilityRuntimeContext, mux: SourceMux) -> APIServer {
    let state = Arc::new(ApiState { logger: runtime::NewLoggerWithType::<_, APIServer>(context, None),
        errs: Errors::FromStruct::<APIServer>(None, None), resourceList: RwLock::default(), delegates: RwLock::default() });
    context.emit_source_log(&state.logger, LogLevel::Debug, "API Server Started");
    for (path, value, error_message) in [
        ("/openapi/v3", serde_json::json!({"paths":{}}), "error encoding openapi/v3"),
        ("/openapi/v2", serde_json::json!({"info":{"title":"allocation.agones.dev"},"paths":null}), "error encoding openapi/v2"),
    ] {
        let (logger, state) = (state.logger.clone(), Arc::clone(&state));
        mux.handle(path, https::ErrorHTTPHandler(logger, Arc::new(move |_context, writer, _request| {
            let (value, state) = (value.clone(), Arc::clone(&state));
            Box::pin(async move {
                set_header(&mut writer.headers, ContentTypeHeader, "application/json")?;
                writer.encode_json(&value).await.map_err(|error| state.errs.Wrap(Some(error), error_message).expect("non-nil encoding error"))
            })
        })));
    }
    mux.handle("/apis", Arc::new(|_context, writer, _request| Box::pin(async move {
        let _ = writer.write_header(406).await;
        // Deliberately after WriteHeader: this change is not on the wire.
        let _ = set_header(&mut writer.headers, ContentTypeHeader, "application/json");
    })));
    APIServer { state, mux, runtime: context.clone() }
}
impl APIServer {
    pub fn AddAPIResource(&self, groupVersion: &str, resource: APIResource, handler: CRDHandler) {
        let mut lists = self.state.resourceList.write().unwrap_or_else(|error| error.into_inner());
        let list = match lists.get(groupVersion) {
            Some(list) => Arc::clone(list),
            None => {
                let list = Arc::new(RwLock::new(APIResourceList { group_version: groupVersion.into(), resources: Some(Arc::default()), ..Default::default() }));
                lists.insert(groupVersion.into(), Arc::clone(&list));
                let pattern = format!("/apis/{groupVersion}");
                self.addSerializedHandler(&pattern, Arc::clone(&list));
                self.runtime.emit_source_log(&self.state.logger.with_field("groupversion", groupVersion).with_field("pattern", pattern), LogLevel::Debug, "Adding Discovery Handler");
                let pattern = format!("/apis/{groupVersion}/namespaces/");
                self.mux.handle(&pattern, https::ErrorHTTPHandler(self.state.logger.clone(), self.resourceHandler(groupVersion)));
                self.runtime.emit_source_log(&self.state.logger.with_field("groupversion", groupVersion).with_field("pattern", pattern), LogLevel::Debug, "Adding Resource Handler");
                list
            }
        };
        {
            let mut list = list.write().unwrap_or_else(|error| error.into_inner());
            Arc::make_mut(list.resources.get_or_insert_with(Arc::default)).push(resource.clone());
        }
        self.state.delegates.write().unwrap_or_else(|error| error.into_inner()).insert(Bytes::from(format!("{groupVersion}/{}", resource.name)), handler);
        self.runtime.emit_source_log(&self.state.logger.with_field("groupversion", groupVersion)
            .with_field("apiresource", serde_json::to_value(&resource).expect("APIResource wire value")), LogLevel::Debug, "Adding APIResource");
    }
    fn resourceHandler(&self, gv: &str) -> https::ErrorHandlerFunc {
        let (state, gv) = (Arc::clone(&self.state), gv.to_owned());
        Arc::new(move |context, writer, request| {
            let (state, gv) = (Arc::clone(&state), gv.clone());
            Box::pin(async move {
                let (namespace, resource) = match splitNameSpaceResource(request.path()) {
                    Ok(value) => value,
                    Err(error) => {
                        let logger = state.logger.with_error(Some(error.as_ref()));
                        https::FourZeroFour(context, &logger, writer, request).await; return Ok(());
                    }
                };
                let mut key = gv.as_bytes().to_vec(); key.push(b'/'); key.extend_from_slice(&resource);
                let delegate = state.delegates.read().unwrap_or_else(|error| error.into_inner()).get(key.as_slice()).cloned();
                if let Some(delegate) = delegate { delegate(context, writer, request, namespace).await }
                else { https::FourZeroFour(context, &state.logger, writer, request).await; Ok(()) }
            })
        })
    }
    fn addSerializedHandler(&self, pattern: &str, object: Arc<RwLock<APIResourceList>>) {
        let state = Arc::clone(&self.state);
        self.mux.handle(pattern, https::ErrorHTTPHandler(state.logger.clone(), Arc::new(move |context, writer, request| {
            let (state, object) = (Arc::clone(&state), Arc::clone(&object));
            Box::pin(async move {
                if request.method() == hyper::Method::GET {
                    let info = AcceptedSerializer(request, &Codecs)?;
                    let object = MetaObject::APIResourceList(object.read().unwrap_or_else(|error| error.into_inner()).clone());
                    let mut copy = shallowCopyObjectForTargetKind(&object); copy.target_v1();
                    set_header(&mut writer.headers, ContentTypeHeader, info.media_type())?;
                    context.encode_api_object(info, &copy, writer).await.map_err(|_error| state.errs.New("error marshalling"))
                } else { https::FourZeroFour(context, &state.logger, writer, request).await; Ok(()) }
            })
        })));
    }
}

/// Metadata is independent; the resource slice remains shared until append.
pub fn shallowCopyObjectForTargetKind(object: &MetaObject) -> MetaObject { object.clone() }

pub fn AcceptedSerializer(request: &HttpRequest, codecs: &CodecFactory) -> Result<SerializerInfo, BoxError> {
    let alternatives = vector![info.media_type() for info in &codecs.media_types];
    let accept = header(&request.headers, AcceptHeader).map_or_else(String::new, |value| String::from_utf8_lossy(&value).into_owned());
    let negotiated = negotiate(&accept, &alternatives).unwrap_or("application/json");
    codecs.media_types.iter().copied().find(|info| info.media_type() == negotiated)
        .ok_or_else(|| errs.New(format!("Could not find serializer for Accept: {accept}")))
}
struct AcceptClause { media: mime::Mime, quality: f32, specificity: u8 }
fn accept_parts(input: &str) -> Vec<&str> {
    let (mut quoted, mut escaped, mut start) = (false, false, 0);
    let mut parts = Vec::new();
    for (index, byte) in input.bytes().enumerate() {
        if escaped { escaped = false; continue; }
        match byte {
            b'\\' if quoted => escaped = true,
            b'"' => quoted = !quoted,
            b',' if !quoted => { parts.push(&input[start..index]); start = index + 1; },
            _ => (),
        }
    }
    parts.push(&input[start..]); parts
}
fn negotiate<'a>(header: &str, alternatives: &[&'a str]) -> Option<&'a str> {
    let mut clauses = Vec::new();
    for part in accept_parts(header) {
        let Ok(media) = part.trim().parse::<mime::Mime>() else { continue; };
        let quality = match media.get_param("q") {
            Some(value) => match value.as_str().parse::<f32>() { Ok(value) => value, Err(_) => continue },
            None => 1.0,
        };
        if !quality.is_finite() || quality <= 0.0 || quality > 1.0 { continue; }
        let specificity = u8::from(media.type_() != mime::STAR) + u8::from(media.subtype() != mime::STAR);
        clauses.push(AcceptClause { media, quality, specificity });
    }
    clauses.sort_by(|a, b| b.quality.total_cmp(&a.quality).then_with(|| b.specificity.cmp(&a.specificity)));
    for clause in clauses {
        for alternative in alternatives {
            let Ok(media) = alternative.parse::<mime::Mime>() else { continue; };
            if (clause.media.type_() == mime::STAR || clause.media.type_() == media.type_())
                && (clause.media.subtype() == mime::STAR || clause.media.subtype() == media.subtype()) { return Some(alternative); }
        }
    }
    None
}
pub fn splitNameSpaceResource(path: impl AsRef<[u8]>) -> Result<(Bytes, Bytes), BoxError> {
    let path = path.as_ref();
    let start = path.iter().position(|byte| *byte != b'/').unwrap_or(path.len());
    let end = path.iter().rposition(|byte| *byte != b'/').map_or(start, |index| index + 1);
    let list: Vec<_> = path[start..end].split(|byte| *byte == b'/').collect();
    if list.len() < 3 { return Err(errs.New(format!("could not find namespace and resource in path: {}", String::from_utf8_lossy(path)))); }
    let last = &list[list.len()-3..];
    if last[0] != b"namespaces" { return Err(errs.New(format!("wrong format in path: {}", String::from_utf8_lossy(path)))); }
    Ok((Bytes::copy_from_slice(last[1]), Bytes::copy_from_slice(last[2])))
}
