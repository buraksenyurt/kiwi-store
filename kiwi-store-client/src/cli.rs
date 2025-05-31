use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kiwi-store-client",
    version = "1.0",
    author = "Burak Selim Şenyurt",
    about = "A simple key-value store client"
)]
pub struct Cli {
    #[arg(long, short, default_value = "127.0.0.1:5555")]
    pub address: String,
    #[command(subcommand)]
    pub argument: Arguments,
}

#[derive(Subcommand)]
pub enum Arguments {
    #[command(name = "set", about = "Set a key-value pair")]
    Set { key: String, value: String },
    #[command(name = "get", about = "Get the value of a key")]
    Get { key: String },
    #[command(name = "remove", about = "Remove a key")]
    Remove { key: String },
    #[command(name = "ping", about = "Ping the server for health check")]
    Ping,
    #[command(name = "list", about = "List all keys in the store")]
    List,
    #[command(name = "stats", about = "Get store statistics")]
    Stats,
}
