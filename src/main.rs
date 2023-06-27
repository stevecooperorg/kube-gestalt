mod router;
mod store;
mod util;
mod webserver;

use anyhow::Result;
use kube::Client;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    // connect to the current context and list nodes;
    let client = Client::try_default().await?;

    // let addr = SocketAddrHelper::find_open_port();
    let addr = SocketAddr::V4(SocketAddrV4::from_str("0.0.0.0:3001").unwrap());
    println!("Start the server: http://{}", addr);
    let routes = router::routes(client.clone());
    let server = webserver::GestaltServer::new().with(routes);
    server.serve(addr).await?;

    Ok(())
}
