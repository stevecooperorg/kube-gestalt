mod router;
mod store;
mod webserver;

use crate::webserver::SocketAddrHelper;
use anyhow::Result;
use kube::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // connect to the current context and list nodes;
    let client = Client::try_default().await?;

    let addr = SocketAddrHelper::find_open_port();
    println!("Start the server: http://{}", addr);
    let routes = router::routes(client.clone());
    let server = webserver::GestaltServer::new().with(routes);
    server.serve(addr).await?;

    Ok(())
}
