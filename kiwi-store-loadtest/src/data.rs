use std::collections::HashMap;

#[allow(dead_code)]
/// Represents a dataset containing valid store entries and invalid commands for testing purposes.
pub struct DataSet {
    pub valid_store: HashMap<&'static str, &'static str>,
    pub invalid_commands: Vec<&'static str>,
}

impl DataSet {
    #[allow(dead_code)]
    /// Creates a new instance of `DataSet` with predefined valid store entries and invalid commands.
    pub fn new() -> DataSet {
        let valid_store = HashMap::from([
            (
                "dbConn",
                "data source=localhost;Initial Catalog=kiwi;User ID=sa;Password",
            ),
            ("logLevel", "Verbose"),
            ("theme", "dark"),
            ("maxConnections", "100"),
            ("timeout", "30"),
            ("cacheSize", "256MB"),
            ("retry", "3"),
            ("caching", "off"),
            ("loginapi", "https://api.azon.com/v1/login"),
            ("apiKey", "12345-abcde-67890-fghij"),
            ("apiSec", "s3cr"),
            ("lang", "tr-TR"),
            ("timezone", "Europe/Istanbul"),
            ("dateFormat", "dd/MM/yyyy"),
            ("currency", "TRY"),
            ("smtpSrv", "smtp.azon.com"),
            ("smtpPrt", "587"),
            ("smtpUsr", ""),
            ("mntcMode", "false"),
            ("backup", "daily"),
            ("compr", "enabled"),
            ("encrKey", "s3cr3tK3y123!"),
            ("maxUploadSize", "50MB"),
            ("sessionTimeout", "15m"),
        ]);

        let invalid_commands = vec![
            "INSERT username john password secret",
            "UPDATE userSettings theme light",
            "SELECT * FROM users WHERE id = 1",
            "RETREIVE username",
            "DELETE username",
            "CREATE TABLE newTable (id INT, name VARCHAR(100))",
            "CHECK health",
        ];

        Self {
            valid_store,
            invalid_commands,
        }
    }
}
