use crate::store;
use crate::webserver::Router;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Extension;
use http::StatusCode;
use k8s_openapi::api::core::v1::{Node, Pod};
use kube::runtime::reflector::Store;
use kube::{Client, ResourceExt};
use rand::RngCore;
use std::collections::HashMap;
use std::sync::Arc;

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
        <h2>Pod-Node gestalts</h2>
        <ol id="pod-node-list" hx-get="/podnodes" hx-trigger="every 2s">
            <li>pod-node gestalt list.</li>
        </ol>
    </body>
</html>
    "##,
    )
}

async fn nodes(node_list: Extension<Store<Node>>) -> impl IntoResponse {
    let list_items: Vec<String> = node_list
        .state()
        .iter()
        .map(node_summary)
        .map(|n| format!("<li>{}</li>", n))
        .collect::<Vec<String>>();
    let html: String = list_items.join("\n");

    (StatusCode::OK, [("content-type", "text/html")], html)
}

async fn pods(pod_list: Extension<Store<Pod>>) -> impl IntoResponse {
    let list_items: Vec<String> = pod_list
        .state()
        .iter()
        .map(pod_summary)
        .map(|n| format!("<li>{}</li>", n))
        .collect::<Vec<String>>();
    let html: String = list_items.join("\n");

    (StatusCode::OK, [("content-type", "text/html")], html)
}

async fn podnodes(
    pod_list: Extension<Store<Pod>>,
    node_list: Extension<Store<Node>>,
) -> impl IntoResponse {
    let node_dict: HashMap<_, _> = node_list
        .state()
        .into_iter()
        .map(|n| (n.name_unchecked(), n))
        .collect();

    let list_items: Vec<String> = pod_list
        .state()
        .iter()
        .map(|p| {
            let pod_summary = pod_summary(p);
            let node_name: String = p
                .spec
                .as_ref()
                .and_then(|s| s.node_name.clone())
                .unwrap_or_else(|| "<no node>".to_string());
            let node = node_dict.get(&node_name).unwrap();
            let node_summary = node_summary(node);
            format!("{} on {}", pod_summary, node_summary)
        })
        .map(|n| format!("<li>{}</li>", n))
        .collect::<Vec<String>>();
    let html: String = list_items.join("\n");

    (StatusCode::OK, [("content-type", "text/html")], html)
}

fn node_summary(node: &Arc<Node>) -> String {
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

fn pod_summary(pod: &Arc<Pod>) -> String {
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

pub fn routes(client: Client) -> Router {
    let (_nodes, _node_handle) =
        store::cluster_store::<Node>(client.clone()).expect("could not start store");
    let (_pods, _pod_handle) =
        store::cluster_store::<Pod>(client.clone()).expect("could not start store");

    Router::new()
        .route("/", get(home))
        .route("/random", get(random))
        .route("/nodes", get(nodes))
        .route("/pods", get(pods))
        .route("/podnodes", get(podnodes))
        .layer(Extension(_nodes))
        .layer(Extension(_pods))
}
