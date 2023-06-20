use crate::webserver::Router;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use http::StatusCode;
use k8s_openapi::api::core::v1::{Node, Pod};
use kube::api::ListParams;
use kube::{Api, Client, ResourceExt};
use rand::RngCore;

async fn home() -> impl IntoResponse {
    // site homepage
    Html(
        r##"
<html>
    <head>
        <script src="https://unpkg.com/htmx.org@1.9.2"></script>
        <title>kube-gestalt</title>
    </head>
    <body>
        <h1>kube-gestalt homepage</h1>
        <h2>Nodes</h2>
        <ol id="node-list" hx-get="/nodes" hx-trigger="every 2s">
            <li>node list.</li>
        </ol>
        <h2>Pods</h2>
        <ol id="pod-list" hx-get="/pods" hx-trigger="every 2s">
            <li>pod list.</li>
        </ol>
    </body>
</html>
    "##,
    )
}

async fn nodes() -> impl IntoResponse {
    // connect to the current context and list nodes;
    let client = match Client::try_default().await {
        Ok(client) => client,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("content-type", "text/plain")],
                "sorry, cannot create a client".to_string(),
            );
        }
    };

    let api: Api<Node> = Api::all(client);
    let nodes = match api.list(&ListParams::default()).await {
        Ok(nodes) => nodes,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("content-type", "text/plain")],
                "sorry, cannot list nodes".to_string(),
            );
        }
    };

    let list_items: Vec<String> = nodes
        .iter()
        .map(node_summary)
        .map(|n| format!("<li>{}</li>", n))
        .collect::<Vec<String>>();
    let html: String = list_items.join("\n");

    (StatusCode::OK, [("content-type", "text/html")], html)
}

async fn pods() -> impl IntoResponse {
    // connect to the current context and list nodes;
    let client = match Client::try_default().await {
        Ok(client) => client,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("content-type", "text/plain")],
                "sorry, cannot create a client".to_string(),
            );
        }
    };

    let api: Api<Pod> = Api::all(client);
    let pods = match api.list(&ListParams::default()).await {
        Ok(nodes) => nodes,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("content-type", "text/plain")],
                "sorry, cannot list pods".to_string(),
            );
        }
    };

    let list_items: Vec<String> = pods
        .iter()
        .map(pod_summary)
        .map(|n| format!("<li>{}</li>", n))
        .collect::<Vec<String>>();
    let html: String = list_items.join("\n");

    (StatusCode::OK, [("content-type", "text/html")], html)
}

fn node_summary(node: &Node) -> String {
    let name = node.name_unchecked();
    let allocatable_mem = node
        .status
        .as_ref()
        .and_then(|s| s.allocatable.as_ref())
        .and_then(|alloc| alloc.get("memory"))
        .map(|q| q.0.clone())
        .unwrap_or_default();

    format!("{}, {} allocatable", name, allocatable_mem)
}

fn pod_summary(pod: &Pod) -> String {
    let name = pod.name_unchecked();
    let namespace = pod.namespace().unwrap_or_default();
    let status = pod
        .status
        .as_ref()
        .and_then(|s| s.phase.clone())
        .unwrap_or_default();
    format!("{}.{}, {}", name, namespace, status)
}

async fn random() -> impl IntoResponse {
    // a paragraph with a random number. Used to show refreshing
    let num = next_u32();
    let html: String = format!("<p>{num}</h1>");
    Html(html)
}

fn next_u32() -> u32 {
    let mut rnd = rand::thread_rng();
    rnd.next_u32()
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/random", get(random))
        .route("/nodes", get(nodes))
        .route("/pods", get(pods))
}
