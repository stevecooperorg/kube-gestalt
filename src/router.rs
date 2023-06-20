use crate::webserver::Router;
use axum::response::Html;
use axum::routing::get;

async fn home() -> Html<&'static str> {
    Html("<h1>kube-gestalt homepage</h1>")
}

pub fn routes() -> Router {
    Router::new().route("/", get(home))
}
