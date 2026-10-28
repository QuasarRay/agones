// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/https/server.go. Copyright Contributors to Agones a Series of LF
// Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
use std::{path::{Path, PathBuf}, sync::{Arc, Mutex}, time::Duration};
use derive_deftly::Deftly;
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, fswatch::{self, ProcessEvent, WatchHandle}, logfields::LogEntry, runtime,
    utility_http::{HttpRequest, ResponseWriter, SourceMux, is_server_closed}, utility_runtime::UtilityRuntimeContext,
    utility_logging::{CanEmitSourceLog, LogLevel}, utility_tls::{CanLoadSourceCertificate, CanRunHttpsServer, CanExitSourceProcess, LoadedCertificate, TlsListenerState},
    utility_declarations::derive_deftly_template_SourceErrorOrigin};

pub const tlsDir: &str = "/certs/";
pub const readHeaderTimeout: Duration = Duration::from_secs(60);

#[derive(Default)]
pub struct certServer { pub(crate) certs: Mutex<Option<LoadedCertificate>> }
impl certServer {
    pub(crate) fn current(&self) -> Option<LoadedCertificate> { self.certs.lock().unwrap_or_else(|error| error.into_inner()).clone() }
}

#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "agones.dev/agones/pkg/util/https", source_struct = "Server", source_log_type = "*https.Server")]
pub struct Server {
    pub Mux: SourceMux,
    certServer: Arc<certServer>,
    logger: LogEntry,
    errs: Errors,
    tls: Arc<TlsListenerState>,
    certFile: PathBuf,
    keyFile: PathBuf,
    port: String,
    runtime: UtilityRuntimeContext,
}
pub async fn NewServer(context: &UtilityRuntimeContext, certFile: PathBuf, keyFile: PathBuf, port: String) -> Server {
    let mut server = Server { Mux: SourceMux::default(), certServer: Arc::default(),
        logger: runtime::NewLoggerWithType::<_, Server>(context, None), errs: Errors::FromStruct::<Server>(None, None),
        tls: Arc::default(), certFile, keyFile, port, runtime: context.clone() };
    server.setupServer().await;
    // Capture only the fields used by this method, avoiding a native Arc cycle
    // from mux -> bound Server method -> mux.
    let logger = server.logger.clone();
    server.Mux.handle("/", Arc::new(move |context, writer, request| {
        let logger = logger.clone();
        Box::pin(async move { Server::defaultHandler(context, &logger, writer, request).await; })
    }));
    server
}
impl Server {
    async fn setupServer(&mut self) {
        self.tls = Arc::default();
        match self.runtime.load_source_certificate(Path::new("/certs/server.crt"), Path::new("/certs/server.key")).await {
            Err(error) => self.runtime.emit_source_log(&self.logger.with_error(Some(error.as_ref())), LogLevel::Error, "could not load Initial TLS certs; keeping old one"),
            Ok(certificate) => *self.certServer.certs.lock().unwrap_or_else(|error| error.into_inner()) = Some(certificate),
        }
    }
    pub fn getCertificate(&self) -> Result<Option<LoadedCertificate>, BoxError> { Ok(self.certServer.current()) }

    pub fn WatchForCertificateChanges(&self) -> Result<WatchHandle, BoxError> {
        let (context, certificates, logger) = (self.runtime.clone(), Arc::clone(&self.certServer), self.logger.clone());
        let process: ProcessEvent = Arc::new(move || {
            let (context, certificates, logger) = (context.clone(), Arc::clone(&certificates), logger.clone());
            Box::pin(async move {
                context.emit_source_log(&logger, LogLevel::Info, "TLS certs changed, reloading");
                match context.load_source_certificate(Path::new("/certs/server.crt"), Path::new("/certs/server.key")).await {
                    Err(error) => context.emit_source_log(&logger.with_error(Some(error.as_ref())), LogLevel::Error, "could not load TLS certs; keeping old one"),
                    Ok(certificate) => {
                        let mut certs = certificates.certs.lock().unwrap_or_else(|error| error.into_inner());
                        *certs = Some(certificate);
                        // The source logs the successful update while holding certMu.
                        context.emit_source_log(&logger, LogLevel::Info, "TLS certs updated");
                    }
                }
            })
        });
        match fswatch::Watch(&self.runtime, &self.logger, Path::new(tlsDir), 1_000_000_000, process) {
            Ok(handle) => Ok(handle),
            Err(error) => {
                self.runtime.emit_source_log(&self.logger.with_error(Some(error.as_ref())), LogLevel::Fatal, "could not create watcher for TLS certs");
                self.runtime.exit_source_process(1);
                Err(error) // retained for an explicitly substituted source ExitFunc
            }
        }
    }
    pub async fn Run(&self, stop: CancellationToken, _workers: i32) -> Result<(), BoxError> {
        self.runtime.emit_source_log(&self.logger.with_field("server", serde_json::json!({"Mux":{}})), LogLevel::Info,
            &format!("https server started on port :{}", self.port));
        match self.runtime.run_https_server(Arc::clone(&self.tls), Arc::clone(&self.certServer), &self.Mux,
            &self.port, &self.certFile, &self.keyFile, stop, readHeaderTimeout).await {
            Err(error) if is_server_closed(error.as_ref()) => {
                self.runtime.emit_source_log(&self.logger.with_error(Some(error.as_ref())), LogLevel::Info, "https server closed"); Ok(())
            }
            Err(error) => Err(self.errs.Wrap(Some(error), &format!("Could not listen on :{}", self.port)).expect("source non-nil error")),
            Ok(()) => Ok(()),
        }
    }
    async fn defaultHandler(context: &UtilityRuntimeContext, logger: &LogEntry, writer: &mut ResponseWriter, request: &mut HttpRequest) {
        if request.path().as_ref() == b"/" { let _ = writer.write_header(200).await; }
        else { super::FourZeroFour(context, logger, writer, request).await; }
    }
}
