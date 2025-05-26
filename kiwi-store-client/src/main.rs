mod cli;
mod command;
mod handler;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    handler::handle_command().await
}
