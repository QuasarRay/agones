//! Concrete native certificate and TLS-listener effects.
use std::{fmt, io::Cursor, path::Path, sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};
use cgp::prelude::*;
use futures::future::BoxFuture;
use rustls::{ServerConfig, server::{ClientHello, ResolvesServerCert}, sign::CertifiedKey};
use tokio::task::JoinSet;
use tokio_rustls::TlsAcceptor;
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, https::certServer, utility_http::{self, HttpProtocol, SourceMux, ServerClosed},
    utility_runtime::UtilityRuntimeContext, utility_scope::{CanSpawnUtilityTask, UtilityTaskScope}};

#[derive(Clone)]
pub struct LoadedCertificate { key: Arc<CertifiedKey> }
impl fmt::Debug for LoadedCertificate {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("LoadedCertificate { private material omitted }") }
}
#[derive(Default)]
pub struct TlsListenerState { closed: AtomicBool }

#[cgp_component(SourceCertificateLoader)]
pub trait CanLoadSourceCertificate: Sync {
    fn load_source_certificate<'a>(&'a self, cert: &'a Path, key: &'a Path) -> BoxFuture<'a, Result<LoadedCertificate, BoxError>>;
}
#[cgp_new_provider]
impl<Context: Sync> SourceCertificateLoader<Context> for LoadRustlsCertificate {
    fn load_source_certificate<'a>(_context: &'a Context, cert: &'a Path, key: &'a Path) -> BoxFuture<'a, Result<LoadedCertificate, BoxError>> {
        Box::pin(async move {
            // Go LoadX509KeyPair reads both files, in this order, before parsing.
            let certificate_bytes = tokio::fs::read(cert).await?;
            let key_bytes = tokio::fs::read(key).await?;
            let certificates = rustls_pemfile::certs(&mut Cursor::new(&certificate_bytes)).collect::<Result<Vec<_>, _>>()?;
            if certificates.is_empty() { return Err(Errors::default().New("tls: failed to find any PEM data in certificate input")); }
            let private_key = rustls_pemfile::private_key(&mut Cursor::new(&key_bytes))?
                .ok_or_else(|| Errors::default().New("tls: failed to find any PEM data in key input"))?;
            let provider = rustls::crypto::ring::default_provider();
            let key = CertifiedKey::from_der(certificates, private_key, &provider)?;
            Ok(LoadedCertificate { key: Arc::new(key) })
        })
    }
}

struct SourceCertificateResolver { current: Arc<certServer>, fallback: Option<LoadedCertificate> }
impl fmt::Debug for SourceCertificateResolver {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("SourceCertificateResolver") }
}
impl ResolvesServerCert for SourceCertificateResolver {
    fn resolve(&self, hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        // crypto/tls consults GetCertificate when no fallback Certificates
        // exist or SNI is present; a nil callback result falls back afterwards.
        if self.fallback.is_none() || hello.server_name().is_some() {
            if let Some(certificate) = self.current.current() { return Some(certificate.key); }
        }
        self.fallback.as_ref().map(|certificate| Arc::clone(&certificate.key))
    }
}

#[cgp_component(HttpsServerRunner)]
pub trait CanRunHttpsServer: Sync {
    fn run_https_server<'a>(&'a self, state: Arc<TlsListenerState>, certificates: Arc<certServer>, mux: &'a SourceMux,
        port: &'a str, cert: &'a Path, key: &'a Path, stop: CancellationToken, read_header_timeout: Duration) -> BoxFuture<'a, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl HttpsServerRunner<UtilityRuntimeContext> for RunNativeHttpsServer {
    fn run_https_server<'a>(context: &'a UtilityRuntimeContext, state: Arc<TlsListenerState>, certificates: Arc<certServer>, mux: &'a SourceMux,
        port: &'a str, cert: &'a Path, key: &'a Path, stop: CancellationToken, read_header_timeout: Duration) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            if state.closed.load(Ordering::Acquire) || stop.is_cancelled() {
                state.closed.store(true, Ordering::Release); return Err(Box::new(ServerClosed) as BoxError);
            }
            // ListenAndServeTLS binds before ServeTLS loads its fallback files.
            let owner_stop = context.utility_child_token()?;
            let listener = utility_http::bind_source_listener(port).await?;
            let fallback = if !cert.as_os_str().is_empty() || !key.as_os_str().is_empty() {
                Some(context.load_source_certificate(cert, key).await?)
            } else { None };
            let provider = Arc::new(rustls::crypto::ring::default_provider());
            let mut config = ServerConfig::builder_with_provider(provider).with_safe_default_protocol_versions()?
                .with_no_client_auth().with_cert_resolver(Arc::new(SourceCertificateResolver { current: certificates, fallback }));
            config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
            let acceptor = TlsAcceptor::from(Arc::new(config));
            let handlers = Arc::new(UtilityTaskScope::default());
            let mut connections = JoinSet::new();
            let result = loop {
                tokio::select! {
                    _ = stop.cancelled() => { state.closed.store(true, Ordering::Release); break Err(Box::new(ServerClosed) as BoxError); }
                    _ = owner_stop.cancelled() => { state.closed.store(true, Ordering::Release); break Err(Box::new(ServerClosed) as BoxError); }
                    completed = connections.join_next(), if !connections.is_empty() => { let _ = completed; }
                    accepted = listener.accept() => match accepted {
                        Err(error) => break Err(Box::new(error) as BoxError),
                        Ok((socket, peer)) => {
                            let (runtime, mux, stop, owner_stop, handlers, acceptor) = (context.clone(), mux.clone(), stop.clone(), owner_stop.clone(), Arc::clone(&handlers), acceptor.clone());
                            connections.spawn(async move {
                                let handshake = tokio::time::timeout(read_header_timeout, acceptor.accept(socket));
                                tokio::pin!(handshake);
                                let handshake = tokio::select! {
                                    result = &mut handshake => Some(result),
                                    _ = async { tokio::select! { _ = stop.cancelled() => (), _ = owner_stop.cancelled() => () } } => {
                                        // A new connection gets the source shutdown grace
                                        // for handshakes, independently of the cancelled ctx.
                                        tokio::time::timeout(Duration::from_secs(5), &mut handshake).await.ok()
                                    }
                                };
                                match handshake {
                                    Some(Ok(Ok(stream))) => {
                                        let protocol = if stream.get_ref().1.alpn_protocol() == Some(b"h2") { HttpProtocol::Http2 } else { HttpProtocol::Http1 };
                                        utility_http::serve_connection(stream, runtime, mux, stop, owner_stop, handlers, read_header_timeout, protocol).await
                                    }
                                    Some(Ok(Err(error))) => { eprintln!("http: TLS handshake error from {peer}: {error}"); Ok(()) }
                                    Some(Err(_)) => { eprintln!("http: TLS handshake error from {peer}: i/o timeout"); Ok(()) }
                                    None => Ok(()),
                                }
                            });
                        }
                    }
                }
            };
            drop(listener);
            utility_http::retain_connection_drain(context, connections, handlers, owner_stop).await;
            result
        })
    }
}

#[cgp_component(SourceProcessExit)]
pub trait CanExitSourceProcess { fn exit_source_process(&self, status: i32); }
#[cgp_new_provider]
impl<Context> SourceProcessExit<Context> for ExitNativeSourceProcess {
    fn exit_source_process(_context: &Context, status: i32) { std::process::exit(status); }
}
