use anyhow::{Context, Result};
pub use axum;
pub use axum::async_trait;
use axum::body::BoxBody;
#[doc(inline)]
pub use axum::response::Response;
pub use http;
use std::net::SocketAddr;
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

    pub async fn serve(self, port: u16) -> Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
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
