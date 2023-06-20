mod router;
mod webserver;

use anyhow::Result;
use k8s_openapi::api::core::v1::Node;
use kube::api::{Api, ListParams};
use kube::Client;

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

    const PORT: u16 = 14324;
    println!("Start the server: http://localhost:{}", PORT);
    let server = webserver::GestaltRouter::new().with(router::routes());
    server.serve(PORT).await?;

    Ok(())
}
