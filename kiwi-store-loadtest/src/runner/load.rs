use crate::{
    data::DataSet,
    measurement::{Metrics, TestType},
};
use rand::seq::IndexedMutRandom;
use std::time::Instant;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

/// Runs a load test against a server at the specified address using the provided data set.
///
/// # Arguments
///
/// * `address` - The address of the server to connect to.
/// * `data_set` - The data set containing valid keys and values for the test.
/// * `client_count` - The number of concurrent clients to simulate.
/// * `commands_per_client` - The number of commands each client will execute.
///
/// # Returns
///
/// A `Metrics` struct containing the results of the load test, including the total number of commands executed,
/// the number of successful commands, the number of failed commands, and the average latency in milliseconds.
pub async fn run(
    address: &str,
    data_set: &DataSet,
    client_count: usize,
    commands_per_client: usize,
) -> Metrics {
    let keys: Vec<_> = data_set.valid_store.keys().cloned().collect();
    let mut success = 0;
    let mut failure = 0;
    let mut durations = vec![];

    let mut handles = vec![];

    for _ in 0..client_count {
        let mut keys = keys.clone();
        let address = address.to_string();
        let ds = data_set.valid_store.clone();

        let handle = tokio::spawn(async move {
            let mut success_total = 0;
            let mut failure_total = 0;
            let mut durations = vec![];

            for _ in 0..commands_per_client {
                let key = keys.choose_mut(&mut rand::rng()).unwrap();
                let value = ds.get(key).unwrap();
                let cmd = format!("SET {} {}\n", key, value);

                if let Ok(mut stream) = TcpStream::connect(&address).await {
                    let start = Instant::now();
                    if stream.write_all(cmd.as_bytes()).await.is_ok() {
                        let mut buf = vec![0; 512];
                        if let Ok(_) = stream.read(&mut buf).await {
                            success_total += 1;
                        } else {
                            failure_total += 1;
                        }
                    } else {
                        failure_total += 1;
                    }
                    let elapsed = start.elapsed();
                    durations.push(elapsed.as_millis());
                } else {
                    failure_total += 1;
                }
            }

            (success_total, failure_total, durations)
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
        test_type: TestType::LoadTest,
        total_commands: success + failure,
        successful_commands: success,
        failed_commands: failure,
        average_latency_ms: avg,
    }
}
