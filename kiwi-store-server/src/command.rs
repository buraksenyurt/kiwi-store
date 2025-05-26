/// Commands for the Kiwi Store server

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
    /// let cmd = Command::parse("INVALID COMMAND");
    /// assert_eq!(cmd, Command::Invalid("INVALID COMMAND".to_string()));
    /// ```
    pub fn parse(input: &str) -> Self {
        let mut parts = input.trim().splitn(3, ' ');
        let cmd = parts.next().unwrap_or("").to_uppercase();

        match cmd.as_str() {
            "SET" => {
                let key = parts.next().unwrap_or("").to_string();
                let value = parts.next().unwrap_or("").to_string();
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
            _ => Command::Invalid(cmd),
        }
    }
}
