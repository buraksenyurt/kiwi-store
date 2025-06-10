mod command;
mod config;
mod handler;
mod server;
mod store;
mod tests;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    server::run().await
}
