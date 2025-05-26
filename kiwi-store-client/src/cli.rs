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
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
    Ping,
    List,
}
