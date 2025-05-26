use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::cli::{Arguments, Cli};
use crate::command::Command;

pub async fn handle_command() -> tokio::io::Result<()> {
    let cli = Cli::parse();

    let message: String = match cli.argument {
        Arguments::Set { key, value } => Command::Set { key, value }.to_message(),
        Arguments::Get { key } => Command::Get { key }.to_message(),
        Arguments::Remove { key } => Command::Remove { key }.to_message(),
        Arguments::Ping => Command::Ping.to_message(),
        Arguments::List => Command::List.to_message(),
    };

    let message = message + "\n";

    let mut stream = TcpStream::connect(cli.address).await?;
    stream.write_all(message.as_bytes()).await?;

    let mut buffer = vec![0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;

    if bytes_read == 0 {
        eprintln!("Connection closed by the server.");
        return Ok(());
    }
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    if response.trim() == "PONG" {
        println!("Server is alive!");
    } else {
        println!("{}", response);
    }

    Ok(())
}
