/// Configuration for the Kiwi Store Server
/// This configuration is loaded from environment variables.
/// It can also be used to set default values.
///
/// # Example:
/// ```rust
/// use kiwi_store_server::config::Configuration;
/// let config = Configuration::from_env();
/// println!("Server will run on: {}", config.get_listen_address());
/// ```
#[derive(Debug, Clone)]
pub struct Configuration {
    pub host: String,
    pub port: u16,
    // pub max_connections: usize,
    pub max_key_length: usize,
    pub max_value_length: usize,
    pub forbidden_keys: Vec<char>,
}

impl Configuration {
    /// Creates a new `Configuration` instance from environment variables.
    /// If the environment variables are not set, it uses default values.
    ///
    /// # Environment Variables:
    /// - `HOST`: The host address (default: "127.0.0.01")
    /// - `PORT`: The port number (default: 5544)
    /// - `MODE`: The mode of operation, which affects the maximum key and value lengths.
    ///   - `CACHE_MODE`: max key length 20, max value length 255
    ///   - `VAULT_MODE`: max key length 20, max value length 40
    ///   - Any other value defaults to max key length 20, max value length 100
    ///
    /// # Returns:
    /// A `Configuration` instance with the values from the environment variables or defaults.
    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.0.1".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "5544".to_string())
            .parse::<u16>()
            .unwrap_or(5544);
        let mode = std::env::var("MODE").unwrap_or_else(|_| "DEFAULT_MODE".to_string());

        let (max_key_length, max_value_length) = match mode.as_str() {
            "CACHE_MODE" => (20, 255),
            "VAULT_MODE" => (20, 40),
            _ => (20, 100),
        };
        Self {
            host,
            port,
            // max_connections: 1000,
            max_key_length,
            max_value_length,
            forbidden_keys: ['\n', '\r', '\0'].to_vec(),
        }
    }
    pub fn get_listen_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Configuration {
    /// Returns a default `Configuration` instance with predefined values.
    /// - Host: "127.0.0.1"
    /// - Port: 5544
    /// - Max Key Length: 20
    /// - Max Value Length: 100
    /// - Forbidden Keys: ['\n', '\r', '\0']
    ///
    /// # Returns:
    /// A `Configuration` instance with default values.
    fn default() -> Self {
        let host = "127.0.0.1".to_string();
        let port = 5544;
        Self {
            host,
            port,
            max_key_length: 20,
            max_value_length: 100,
            forbidden_keys: ['\n', '\r', '\0'].to_vec(),
        }
    }
}
