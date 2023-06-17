use k8s_openapi::api::core::v1::Node;
use kube::{Client};
use kube::api::{Api, ListParams};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // connect to the current context and list nodes;
     let client = Client::try_default().await?;

    let api: Api<Node> = Api::all(client);
    let nodes = api.list(&ListParams::default()).await?;

    println!("Found {} nodes", nodes.items.len());
    for node in nodes.items {
        println!("Node: {}", node.metadata.name.unwrap());
    }
    Ok(())
}
