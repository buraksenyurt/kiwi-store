use serde::Serialize;
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

#[derive(Serialize, Debug, Default)]
#[allow(dead_code)]
/// Represents the metrics collected during a test run.
pub struct Metrics {
    /// The timestamp when the test was conducted.
    pub time_stamp: String,
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

/// Exports the metrics as a JSON string.
///
/// # Arguments
///
/// * `metrics` - A reference to the `Metrics` struct containing the test results.
/// * `file_name` - The name of the file to which the metrics will be written.
///
/// # Returns
/// A `Result` indicating success or failure of the file write operation.
pub fn export(metrics: &Metrics, file_name: &str) -> std::io::Result<()> {
    // let content = serde_json::to_string_pretty(metrics)?;
    let content = metrics.to_string();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)?;
    file.write_all(format!("{}\n", content).as_bytes())?;
    Ok(())
}
