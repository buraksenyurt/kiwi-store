use serde::Serialize;
use std::io::Write;

#[derive(Serialize, Debug, Default)]
#[allow(dead_code)]
/// Represents the type of test being conducted.
pub enum TestType {
    #[default]
    LoadTest,
    FuzzTest,
    StressTest,
}
#[derive(Serialize, Debug, Default)]
#[allow(dead_code)]
/// Represents the metrics collected during a test run.
pub struct Metrics {
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
    let json = serde_json::to_string_pretty(metrics)?;
    let mut file = std::fs::File::create(file_name)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
