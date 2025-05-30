use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::{
    fmt::{Display, Formatter},
    fs::OpenOptions,
    io::Write,
};

#[derive(Serialize, Debug, Default)]
#[allow(dead_code)]
/// Represents the type of test being conducted.
pub enum TestType {
    #[default]
    Load,
    Fuzz,
}

impl Display for TestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::Load => write!(f, "Load"),
            TestType::Fuzz => write!(f, "Fuzz"),
        }
    }
}

#[derive(Serialize, Debug, Default)]
#[allow(dead_code)]
/// Represents the metrics collected during a test run.
pub struct Metrics {
    /// The timestamp when the test was conducted.
    pub time_stamp: DateTime<Utc>,
    /// The type of test being conducted.
    pub test_type: TestType,
    /// The total number of commands executed during the test.
    pub total_commands: usize,
    /// The number of successful commands executed.
    pub successful_commands: usize,
    /// The number of commands that failed during the test.
    pub failed_commands: usize,
    /// The average latency in milliseconds for the commands executed.
    pub average_latency_ms: f64,
}

impl Display for Metrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{:?}|{}|{}|{}|{}",
            self.time_stamp,
            self.test_type,
            self.total_commands,
            self.successful_commands,
            self.failed_commands,
            self.average_latency_ms
        )
    }
}

impl Metrics {
    /// Append metrics to csv file.
    ///
    /// # Arguments
    ///
    /// * `metrics` - A reference to the `Metrics` struct containing the test results.
    /// * `file_name` - The name of the file to which the metrics will be written.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the file write operation.
    pub async fn append_to_csv(&self, file_name: &str) -> Result<(), std::io::Error> {
        // let content = serde_json::to_string_pretty(metrics)?;
        let content = self.to_string();
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_name)?;
        file.write_all(format!("{}\n", content).as_bytes())?;
        Ok(())
    }

    /// Saves the metrics to a PostgreSQL database.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to the PostgreSQL connection pool.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the database operation.
    pub async fn save_to_db(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                INSERT INTO metrics (
                    time_stamp,
                    test_type,
                    total_commands,
                    successful_commands,
                    failed_commands,
                    average_latency_ms
                ) VALUES ($1, $2, $3, $4, $5, $6)
                "#,
        )
        .bind(self.time_stamp)
        .bind(self.test_type.to_string())
        .bind(self.total_commands as i32)
        .bind(self.successful_commands as i32)
        .bind(self.failed_commands as i32)
        .bind(self.average_latency_ms)
        .execute(pool)
        .await?;

        Ok(())
    }
}
