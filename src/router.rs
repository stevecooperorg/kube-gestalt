use crate::webserver::Router;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use rand::RngCore;

async fn home() -> impl IntoResponse {
    // site homepage
    Html(
        r#"
<html>
    <head>
        <script src="https://unpkg.com/htmx.org@1.9.2"></script>
    </head>
    <body>
        <h1>kube-gestalt homepage</h1>
        <button hx-get="/random" hx-swap="outerHTML">Random</button>
    </body>
</html>
    "#,
    )
}

async fn random() -> impl IntoResponse {
    // a paragraph with a random number. Used to show refreshing
    let mut rnd = rand::thread_rng();
    let num = rnd.next_u32();
    let html = format!("<p>{num}</h1>");
    Html(html)
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/random", get(random))
}
