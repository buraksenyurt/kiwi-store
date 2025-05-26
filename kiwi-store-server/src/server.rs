/// Server module for the Kiwi Store application
use crate::handler::handle_request;
use crate::store::DataStore;
use log::info;
use tokio::net::TcpListener;

/// Runs the server, listening for incoming TCP connections on the specified address.
///
/// # Arguments
///
/// * `address` - The address to bind the server to, in the format "IP:port".
///
/// # Returns
///
/// Returns a `tokio::io::Result<()>` indicating success or failure of the operation.
pub async fn run(address: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(address).await?;
    let store = DataStore::new();

    info!("Server running at {}", address);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {} connected", addr);
        let store = store.clone();
        tokio::spawn(async move {
            handle_request(stream, store).await;
        });
    }
}
