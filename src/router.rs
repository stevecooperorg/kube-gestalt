use crate::store;
use crate::webserver::Router;
use askama::Template;
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

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {}

async fn home() -> impl IntoResponse {
    HomeTemplate {}
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
        .collect::<Vec<String>>();
    let html: String = list_items.join("\n");

    (StatusCode::OK, [("content-type", "text/html")], html)
}

#[derive(Template)]
#[template(path = "pod_node_summary_list.html", escape = "none")]
struct PodNodeSummaryListTemplate {
    items: Vec<PodNodeSummaryTemplate>,
}

#[derive(Template)]
#[template(path = "pod_node_summary.html")]
struct PodNodeSummaryTemplate {
    pod_name: String,
    pod_namespace: String,
    pod_status: String,
    node_name: String,
    allocatable_mem: String,
}

fn pod_node_summary(pod: &Arc<Pod>, node: &Arc<Node>) -> PodNodeSummaryTemplate {
    let pod_name = pod.name_unchecked();
    let pod_namespace = pod.namespace().unwrap_or_default();
    let pod_status = pod
        .status
        .as_ref()
        .and_then(|s| s.phase.clone())
        .unwrap_or_default();
    let node_name = node.name_unchecked();
    let allocatable_mem = node
        .status
        .as_ref()
        .and_then(|s| s.allocatable.as_ref())
        .and_then(|alloc| alloc.get("memory"))
        .map(|q| q.0.clone())
        .unwrap_or_default();

    PodNodeSummaryTemplate {
        pod_name,
        pod_namespace,
        node_name,
        allocatable_mem,
        pod_status,
    }
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

    let items: Vec<PodNodeSummaryTemplate> = pod_list
        .state()
        .iter()
        .map(|p| {
            let node_name: String = p
                .spec
                .as_ref()
                .and_then(|s| s.node_name.clone())
                .unwrap_or_else(|| "<no node>".to_string());
            let node = node_dict.get(&node_name).unwrap();
            pod_node_summary(p, node)
        })
        .collect();

    (
        StatusCode::OK,
        [("content-type", "text/html")],
        PodNodeSummaryListTemplate { items },
    )
}

#[derive(Template)]
#[template(path = "node_summary.html")]
struct NodeSummaryTemplate {
    name: String,
    allocatable_mem: String,
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

    NodeSummaryTemplate {
        name,
        allocatable_mem,
    }
    .render()
    .unwrap()
}

#[derive(Template)]
#[template(path = "pod_summary.html")]
struct PodSummaryTemplate {
    name: String,
    namespace: String,
    status: String,
}

fn pod_summary(pod: &Arc<Pod>) -> String {
    let name = pod.name_unchecked();
    let namespace = pod.namespace().unwrap_or_default();
    let status = pod
        .status
        .as_ref()
        .and_then(|s| s.phase.clone())
        .unwrap_or_default();

    PodSummaryTemplate {
        name,
        namespace,
        status,
    }
    .render()
    .unwrap()
}

async fn random() -> impl IntoResponse {
    // a paragraph with a random number. Used to show refreshing
    let num = next_u32();
    let html: String = format!("<p>{num}</p>");
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
