// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/httpserver/server.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::time::Duration;
use derive_deftly::Deftly;
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, logfields::LogEntry, utility_http::{CanRunHttpServer, SourceMux, is_direct_server_closed},
    utility_logging::{CanEmitSourceLog, CanHandleSourceRuntimeError, LogLevel}, utility_runtime::UtilityRuntimeContext,
    utility_declarations::derive_deftly_template_SourceErrorOrigin};

pub const readHeaderTimeout: Duration = Duration::from_secs(60);
const ERRORS: Errors = Errors::from_package("agones.dev/agones/pkg/util/httpserver");

#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "agones.dev/agones/pkg/util/httpserver", source_struct = "Server", source_log_type = "*httpserver.Server")]
pub struct Server<Context = UtilityRuntimeContext> {
    pub Mux: SourceMux,
    pub Port: String,
    pub Logger: Option<LogEntry>,
    pub runtime: Context,
}
impl<Context> Server<Context> {
    pub fn new(runtime: Context, logger: Option<LogEntry>, port: String) -> Self { Self { Mux: SourceMux::default(), Port: port, Logger: logger, runtime } }
}
impl<Context> Server<Context>
where Context: CanRunHttpServer + CanEmitSourceLog + CanHandleSourceRuntimeError,
{
    pub async fn Run(&mut self, stop: CancellationToken, _workers: i32) -> Result<(), BoxError> {
        let logger = self.Logger.as_ref().expect("nil httpserver.Server.Logger");
        self.runtime.emit_source_log(logger, LogLevel::Info, "Starting http server...");
        if self.Port.is_empty() { self.Port = "8080".into(); }
        if let Err(error) = self.runtime.run_http_server(&self.Mux, &self.Port, stop, readHeaderTimeout).await {
            if is_direct_server_closed(error.as_ref()) {
                self.runtime.emit_source_log(&logger.with_error(Some(error.as_ref())), LogLevel::Info, "http server closed");
            } else {
                let error = ERRORS.Wrap(Some(error), &format!("Could not listen on :{}", self.Port)).expect("source non-nil error");
                self.runtime.handle_source_runtime_error(Some(&logger.with_error(Some(error.as_ref()))), Some(error.as_ref())).await;
            }
        }
        // The source logs a non-ErrServerClosed listener error and returns nil.
        Ok(())
    }
}
