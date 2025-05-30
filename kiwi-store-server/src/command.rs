//! Commands for the Kiwi Store server

use log::{error, warn};
const MAX_KEY_LENGTH: usize = 20;
const MAX_VALUE_LENGTH: usize = 100;
const FORBIDDEN_KEY_CHARS: [char; 3] = ['\n', '\r', '\0'];

#[derive(Debug)]
/// Represents the key-value store commands
pub enum Command {
    /// Set command with a key and value
    Set { key: String, value: String },
    /// Get command with a key
    Get { key: String },
    /// Remove command with a key
    Remove { key: String },
    /// List command to list all keys
    List,
    /// Ping command to check server status
    Ping,
    /// Stats command to get server statistics
    Stats,
    /// Invalid command with the command string
    Invalid(String),
}

impl Command {
    /// Parses a command string into a `Command` enum
    ///
    /// # Arguments
    ///
    /// * `input` - The command string to parse
    ///
    /// # Returns
    /// Returns a `Command` enum representing the parsed command.
    ///
    /// # Example
    /// ```rust
    /// use kiwi_store_server::command::Command;
    ///
    /// let cmd = Command::parse("SET UseHttps Off");
    /// assert_eq!(cmd, Command::Set { key: "UseHttps".to_string(), value: "Off".to_string() });
    ///
    /// let cmd = Command::parse("GET UseHttps");
    /// assert_eq!(cmd, Command::Get { key: "UseHttps".to_string() });
    ///
    /// let cmd = Command::parse("REMOVE UseHttps");
    /// assert_eq!(cmd, Command::Remove { key : "UseHttps".to_string() });
    ///
    /// let cmd = Command::parse("LIST");
    /// assert_eq!(cmd, Command::List);
    ///
    /// let cmd = Command::parse("PING");
    /// assert_eq!(cmd, Command::Ping);
    ///
    /// let cmd = Command::parse("STATS");
    /// assert_eq!(cmd, Command::Stats);
    ///
    /// let cmd = Command::parse("INVALID COMMAND");
    /// assert_eq!(cmd, Command::Invalid("INVALID COMMAND".to_string()));
    /// ```
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap_or("").to_uppercase();

        match cmd.as_str() {
            "SET" => {
                let key = parts.next().unwrap_or("").to_string();
                let value = parts.collect::<Vec<&str>>().join(" ");

                if key.is_empty() || value.is_empty() {
                    error!("Key or value is empty");
                    return Command::Invalid("Key or value cannot be empty".to_string());
                }

                if key.len() > MAX_KEY_LENGTH || value.len() > MAX_VALUE_LENGTH {
                    error!(
                        "Key or value exceeds maximum length: {} / {}",
                        key.len(),
                        value.len()
                    );
                    return Command::Invalid(format!(
                        "Key or value exceeds maximum length: {} / {}",
                        key.len(),
                        value.len()
                    ));
                }
                
                if !key.chars().any(|c| !FORBIDDEN_KEY_CHARS.contains(&c)) {
                    error!("Key contains forbidden characters: {}", key);
                    return Command::Invalid(format!("Key contains forbidden characters: {}", key));
                }

                Command::Set { key, value }
            }
            "GET" | "REMOVE" => {
                let key = parts.next().unwrap_or("").to_string();

                if key.len() > MAX_KEY_LENGTH {
                    error!("Key exceeds maximum length: {}", key.len());
                    return Command::Invalid(format!("Key exceeds maximum length: {}", key.len()));
                }

                if cmd == "GET" {
                    Command::Get { key }
                } else {
                    Command::Remove { key }
                }
            }
            "LIST" => Command::List,
            "STATS" => {
                warn!("Received STATS command");
                Command::Stats
            }
            "PING" => {
                warn!("Received PING command");
                Command::Ping
            }
            _ => {
                error!("Invalid command: {}", cmd);
                Command::Invalid(cmd)
            }
        }
    }
}
