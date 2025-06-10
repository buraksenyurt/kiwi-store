#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::store::DataStore;

    #[tokio::test]
    async fn test_set_and_get() {
        let data_store = DataStore::new();
        data_store.set("Resilience", "on").await;
        let expected = data_store.get("Resilience").await.unwrap();
        assert_eq!(expected, "on");
    }

    #[test]
    fn test_set_command_parse() {
        let cmd = Command::parse("SET H-Check On");
        match cmd {
            Command::Set { key, value } => {
                assert_eq!(key, "H-Check");
                assert_eq!(value, "On");
            }
            _ => panic!("Expected to parse SET command!"),
        }
    }

    #[test]
    fn test_long_value_command_parse() {
        let cmd = Command::parse("SET DbConn dataSource=localhost;database=MongoDb");
        match cmd {
            Command::Set { key, value } => {
                assert_eq!(key, "DbConn");
                assert_eq!(value, "dataSource=localhost;database=MongoDb");
            }
            _ => panic!("Expected to parse SET command!"),
        }
    }

    #[test]
    fn test_ping_command_parse() {
        let cmd = Command::parse("PING");
        match cmd {
            Command::Ping => {}
            _ => panic!("Expected to parse PING command!"),
        }
    }

    #[test]
    fn test_len_exceeded_command_parse() {
        let cmd =
            Command::parse("SET DbConnectionStringIsTooLong dataSource=localhost;database=MongoDb");
        match cmd {
            Command::Invalid(input) => {
                assert_eq!(
                    input,
                    format!("Key or value exceeds maximum length: {} / {}", 27, 37)
                );
            }
            _ => panic!("Expected to parse SET command!"),
        }
    }

    #[test]
    fn test_invalid_command() {
        let cmd = Command::parse("INPUT Connection dataSource=localhost;database=MongoDb");
        match cmd {
            Command::Invalid(input) => {
                assert_eq!(input, "INPUT")
            }
            _ => panic!("Expected to parse SET command!"),
        }
    }
}
