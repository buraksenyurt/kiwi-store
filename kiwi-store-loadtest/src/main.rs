use std::env;

use log::info;

mod data;
mod measurement;
mod runner;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:5555".to_string());
    let data_set = data::DataSet::new();
    let client_count = 10;
    let commands_per_client = 50;
    info!("Starting load/fuzz test against server at {}", address);
    // let metrics = runner::fuzz::execute(address, &data_set, client_count, commands_per_client).await;
    let metrics =
        runner::load::execute(&address, &data_set, client_count, commands_per_client).await;
    // let file_name = "metrics.json";
    let file_name = "metrics.dat";
    measurement::export(&metrics, file_name).expect("Failed to export metrics");
    info!("Metrics exported to {}", file_name);
    info!("Test completed successfully.");
    info!("Metrics: {:?}", metrics);
}
