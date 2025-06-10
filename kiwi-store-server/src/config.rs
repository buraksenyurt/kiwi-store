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
    pub fn from_env() -> Self {
        let listen_address = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.0.1".to_string());
        let listen_port = std::env::var("PORT")
            .unwrap_or_else(|_| "5544".to_string())
            .parse::<u16>()
            .unwrap_or(5544);
        let mode = std::env::var("MODE").unwrap_or_else(|_| "MINIMAL".to_string());

        let (max_key_length, max_value_length) = match mode.as_str() {
            "CACHE_MODE" => (20, 255),
            "VAULT_MODE" => (20, 40),
            _ => (20, 100),
        };
        Self {
            host: listen_address,
            port: listen_port,
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
