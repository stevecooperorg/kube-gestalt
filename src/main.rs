mod router;
mod webserver;

use crate::webserver::SocketAddrHelper;
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

    let addr = SocketAddrHelper::find_open_port();
    println!("Start the server: http://{}", addr);
    let server = webserver::GestaltServer::new().with(router::routes());
    server.serve(addr).await?;

    Ok(())
}
