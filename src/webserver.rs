use anyhow::{Context, Result};
pub use axum;
pub use axum::async_trait;
use axum::body::BoxBody;
#[doc(inline)]
pub use axum::response::Response;
pub use http;
use std::net::{SocketAddr, TcpListener};
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

pub type Router<B = BoxBody> = axum::Router<(), B>;

pub struct GestaltRouter {
    router: Router,
}

impl GestaltRouter {
    pub fn new() -> Self {
        GestaltRouter {
            router: Router::default(),
        }
    }

    pub fn with<T>(mut self, router: T) -> Self
    where
        T: Into<axum::Router<(), BoxBody>>,
    {
        self.router = self.router.merge(router);
        self
    }

    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let make_svc = self
            .into_service()
            .into_make_service_with_connect_info::<SocketAddr>();

        axum::Server::bind(&addr)
            .serve(make_svc)
            .await
            .context("serve")
    }

    pub fn into_service(self) -> axum::Router<(), axum::body::Body> {
        self.router
            // these middleware are called for all routes
            .layer(ServiceBuilder::new().map_request_body(axum::body::boxed))
    }
}

pub struct SocketAddrHelper {}

impl SocketAddrHelper {
    /// Return Ok(SocketAddr) if the port is available on 127.0.0.1. Returns Err(_) if the port is
    /// not bindable. If you set port=0, a random address will be found and returned.
    pub fn checked_on_port(port: u16) -> Result<SocketAddr> {
        // we'll attempt to bind to a local port - if port=0, a random, available port will be found
        // note
        Ok(TcpListener::bind(format!("127.0.0.1:{}", port))?.local_addr()?)
    }

    /// Find a random open port on localhost, or panic.
    pub fn find_open_port() -> SocketAddr {
        #[allow(clippy::expect_used)]
        SocketAddrHelper::checked_on_port(0).expect("cannot find open port")
    }
}
