pub mod fuzz;
pub mod load;

use crate::measurement::{Metrics, TestType};
use chrono::Utc;
use std::time::Instant;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn run_test<F>(
    address: &str,
    test_type: TestType,
    client_count: usize,
    commands_per_client: usize,
    command_factory: F,
) -> Metrics
where
    F: FnMut() -> String + Send + Clone + 'static,
{
    let mut success = 0;
    let mut failure = 0;
    let mut durations = vec![];
    let mut handles = vec![];

    for _ in 0..client_count {
        let address = address.to_string();
        let mut factory = command_factory.clone();

        let handle = tokio::spawn(async move {
            let mut local_success = 0;
            let mut local_failure = 0;
            let mut local_durations = vec![];

            for _ in 0..commands_per_client {
                let cmd = factory();
                if let Ok(mut stream) = TcpStream::connect(&address).await {
                    let start = Instant::now();
                    if stream.write_all(cmd.as_bytes()).await.is_ok() {
                        let mut buf = vec![0; 512];
                        if stream.read(&mut buf).await.is_ok() {
                            local_success += 1;
                        } else {
                            local_failure += 1;
                        }
                    } else {
                        local_failure += 1;
                    }
                    local_durations.push(start.elapsed().as_millis());
                } else {
                    local_failure += 1;
                }
            }

            (local_success, local_failure, local_durations)
        });

        handles.push(handle);
    }

    for h in handles {
        let (s, f, d) = h.await.unwrap();
        success += s;
        failure += f;
        durations.extend(d);
    }

    let avg = if durations.is_empty() {
        0.0
    } else {
        durations.iter().sum::<u128>() as f64 / durations.len() as f64
    };

    Metrics {
        time_stamp: Utc::now(),
        test_type,
        total_commands: success + failure,
        successful_commands: success,
        failed_commands: failure,
        average_latency_ms: avg,
    }
}
