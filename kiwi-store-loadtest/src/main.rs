mod data;
mod measurement;
mod runner;

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:5555";
    let data_set = data::DataSet::new();
    let client_count = 10;
    let commands_per_client = 50;
    // let metrics = runner::fuzz::execute(address, &data_set, client_count, commands_per_client).await;
    let metrics =
        runner::load::execute(address, &data_set, client_count, commands_per_client).await;
    let file_name = "metrics.json";
    measurement::export(&metrics, file_name).expect("Failed to export metrics");
    println!("Metrics exported to {}", file_name);
    println!("Test completed successfully.");
    println!("Metrics: {:?}", metrics);
}
