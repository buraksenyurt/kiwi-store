/// Handlers module for the Kiwi Store Server
use crate::command::Command;
use crate::store::DataStore;
use log::{error, info, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[allow(dead_code)]
/// Handles incoming requests from a TCP stream.
/// It reads the request, processes it on key-value store according to the command, and sends back the response.
///
/// # Arguments
///
/// * `stream` - The TCP stream to read from and write to.
/// * `data_store` - The data store to keep the key-value pairs.
pub async fn handle_request(mut stream: TcpStream, data_store: DataStore) {
    let mut buffer = [0; 1024];
    loop {
        let size = match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                error!("{}", e);
                break;
            }
        };

        info!("Read {}(bytes)", size);

        let request = String::from_utf8_lossy(&buffer[..size]);
        let cmd = Command::parse(&request);

        let response = match cmd {
            Command::Ping => {
                info!("PING");
                "PONG\n".to_string()
            }
            Command::Set { key, value } => {
                data_store.set(&key, &value).await;
                "OK\n".to_string()
            }
            Command::Get { key } => {
                info!("GET {}", key);
                data_store
                    .get(&key)
                    .await
                    .unwrap_or_else(|| "NOT FOUND\n".to_string())
            }
            Command::Remove { key } => {
                info!("REMOVE {}", key);
                if data_store.remove(&key).await {
                    "OK\n".to_string()
                } else {
                    warn!("Key '{}' not found for removal", key);
                    "NOT FOUND\n".to_string()
                }
            }
            Command::List => data_store.keys().await.join("\n").to_string(),
            Command::Stats => {
                let stats = data_store.stats().await;
                format!("STATS: {}\n", stats)
            }
            Command::Invalid(cmd) => format!("ERROR: Unknown command '{}'\n", cmd),
        };

        if let Err(e) = stream.write_all(response.as_bytes()).await {
            error!("{}", e);
            break;
        }
    }
}
