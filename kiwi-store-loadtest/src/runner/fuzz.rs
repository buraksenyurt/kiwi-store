use crate::{
    data::DataSet,
    measurement::{Metrics, TestType},
};
use log::info;
use rand::seq::IndexedMutRandom;
use rand::{SeedableRng, rngs::StdRng};

use super::run_test;

/// Runs a fuzz test against a server at the specified address using the provided data set.
///
/// # Arguments
///
/// * `address` - The address of the server to connect to.
/// * `data_set` - The data set containing invalid commands for the test.
/// * `client_count` - The number of concurrent clients to simulate.
/// * `commands_per_client` - The number of commands each client will execute.
///
/// # Returns
///
/// A `Metrics` struct containing the results of the fuzz test, including the total number of commands executed,
/// the number of successful commands, the number of failed commands, and the average latency in milliseconds.
#[allow(dead_code)]
pub async fn execute(
    address: &str,
    data_set: &DataSet,
    client_count: usize,
    commands_per_client: usize,
) -> Metrics {
    let mut commands = data_set.invalid_commands.clone();

    let factory = {
        let mut rng = StdRng::from_os_rng();
        move || {
            let cmd = commands.choose_mut(&mut rng).unwrap();
            info!("Executing command: {}", cmd);
            format!("{}\n", cmd)
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
