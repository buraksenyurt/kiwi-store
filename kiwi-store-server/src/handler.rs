use std::sync::Arc;

/// Handlers module for the Kiwi Store Server
use crate::command::Command;
use crate::config::Configuration;
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
/// * `config` - The configuration for the server, used to validate key and value lengths.
pub async fn handle_request(
    mut stream: TcpStream,
    data_store: DataStore,
    config: Arc<Configuration>,
) {
    let mut buffer = [0; 1024];
    let size = match stream.read(&mut buffer).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("Read {}(bytes)", size);

    let request = String::from_utf8_lossy(&buffer[..size]);
    let raw_cmd = Command::parse(&request);
    let response = match raw_cmd.validate(&config) {
        Ok(cmd) => match cmd {
            Command::Ping => respond("PONG"),
            Command::Set { key, value } => {
                info!("Setting key: {}, value: {}", key, value);
                data_store.set(&key, &value).await;
                respond("OK")
            }
            Command::Get { key } => data_store
                .get(&key)
                .await
                .unwrap_or_else(|| respond("NOT FOUND")),
            Command::Remove { key } => {
                if data_store.remove(&key).await {
                    respond("OK")
                } else {
                    warn!("Key not found: {}", key);
                    respond("NOT FOUND")
                }
            }
            Command::List => {
                if data_store.is_empty().await {
                    warn!("Data store is empty");
                    respond("EMPTY STORE")
                } else {
                    data_store.keys().await.join("\n")
                }
            }
            Command::Stats => {
                let stats = data_store.stats().await;
                format!("STATS: {}\n", stats)
            }
            _ => unreachable!(),
        },
        Err(err) => format!("ERROR: {}\n", err),
    };

    if let Err(e) = stream.write_all(response.as_bytes()).await {
        error!("{}", e);
    }
}

fn respond(message: &str) -> String {
    format!("{}\n", message)
}
