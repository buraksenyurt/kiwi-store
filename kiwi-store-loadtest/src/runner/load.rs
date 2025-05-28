use crate::{
    data::DataSet,
    measurement::{Metrics, TestType},
};
use log::info;
use rand::{SeedableRng, rngs::StdRng, seq::IndexedMutRandom};

use super::run_test;

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
#[allow(dead_code)]
pub async fn execute(
    address: &str,
    data_set: &DataSet,
    client_count: usize,
    commands_per_client: usize,
) -> Metrics {
    let mut keys: Vec<_> = data_set.valid_store.keys().cloned().collect();
    let values = data_set.valid_store.clone();

    let factory = {
        let mut rng = StdRng::from_os_rng();
        move || {
            let key = keys.choose_mut(&mut rng).unwrap();
            let value = values.get(key).unwrap();
            info!("Executing Command: SET {} {}", key, value);
            format!("SET {} {}\n", key, value)
        }
    };

    run_test(
        address,
        TestType::Fuzz,
        client_count,
        commands_per_client,
        factory,
    )
    .await
}
