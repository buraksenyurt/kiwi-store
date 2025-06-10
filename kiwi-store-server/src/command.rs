//! Commands for the Kiwi Store server

use log::{error, warn};

use crate::config::Configuration;

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
    pub fn parse(input: &str) -> Command {
        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap_or("").to_uppercase();

        match cmd.as_str() {
            "SET" => {
                let key = parts.next().unwrap_or("").to_string();
                let value = parts.collect::<Vec<&str>>().join(" ");
                Command::Set { key, value }
            }
            "GET" => {
                let key = parts.next().unwrap_or("").to_string();
                Command::Get { key }
            }
            "REMOVE" => {
                let key = parts.next().unwrap_or("").to_string();
                Command::Remove { key }
            }
            "LIST" => Command::List,
            "STATS" => Command::Stats,
            "PING" => Command::Ping,
            _ => Command::Invalid(cmd),
        }
    }

    /// Validates the command against the provided configuration
    /// 
    /// # Arguments
    /// * `config` - The configuration to validate against
    /// 
    /// # Returns
    /// Returns `Ok(Self)` if the command is valid, or an `Err(String)` with an error message if it is not.
    /// 
    /// # Example
    /// ```rust
    /// use kiwi_store_server::command::Command;
    /// use kiwi_store_server::config::Configuration;
    /// let config = Configuration::from_env();
    /// let cmd = Command::parse("SET UseHttps Off");
    /// let validated_cmd = cmd.validate(&config);
    /// assert!(validated_cmd.is_ok());
    /// let cmd = Command::parse("SET");
    /// let invalid_cmd = cmd.validate(&config);
    /// assert!(invalid_cmd.is_err());
    /// assert_eq!(invalid_cmd.unwrap_err(), "Key or value cannot be empty");
    /// ```
    pub fn validate(self, config: &Configuration) -> Result<Self, String> {
        match self {
            Command::Set { ref key, ref value } => {
                if key.is_empty() || value.is_empty() {
                    error!("Key or value is empty");
                    return Err("Key or value cannot be empty".to_string());
                }
                if key.len() > config.max_key_length {
                    error!("Key exceeds maximum length: {}", key.len());
                    return Err(format!("Key exceeds maximum length: {}", key.len()));
                }
                if value.len() > config.max_value_length {
                    error!("Value exceeds maximum length: {}", value.len());
                    return Err(format!("Value exceeds maximum length: {}", value.len()));
                }
                if key.chars().any(|c| config.forbidden_keys.contains(&c)) {
                    warn!("Key contains forbidden characters: {}", key);
                    return Err(format!("Key contains forbidden characters: {}", key));
                }
                Ok(self)
            }
            Command::Get { ref key } | Command::Remove { ref key } => {
                if key.len() > config.max_key_length {
                    error!("Key exceeds maximum length: {}", key.len());
                    return Err(format!("Key exceeds maximum length: {}", key.len()));
                }
                Ok(self)
            }
            Command::List | Command::Stats | Command::Ping => Ok(self),
            Command::Invalid(cmd) => {
                error!("Invalid command: {}", cmd);
                Err(format!("Invalid command: {}", cmd))
            }
        }
    }
}
