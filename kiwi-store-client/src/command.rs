#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
    Ping,
    List,
    Stats,
}

#[allow(dead_code)]
impl Command {
    pub fn to_message(&self) -> String {
        match self {
            Command::Set { key, value } => format!("SET {} {}", key, value),
            Command::Get { key } => format!("GET {}", key),
            Command::Remove { key } => format!("REMOVE {}", key),
            Command::Ping => "PING".to_string(),
            Command::List => "LIST".to_string(),
            Command::Stats => "STATS".to_string(),
        }
    }
}
