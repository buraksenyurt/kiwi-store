use std::env;

use clap::Parser;
use cli::Cli;
use log::info;
use measurement::{Metrics, TestType};
use sqlx::postgres::PgPoolOptions;

mod cli;
mod data;
mod measurement;
mod runner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:5555".to_string());
    let db_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let data_set = data::DataSet::new();
    info!("Starting load/fuzz test against server at {}", address);
    let cli = Cli::parse();
    let mut metrics = Metrics::default();

    let test_type = match cli.kind.as_str() {
        "fuzz" => TestType::Fuzz,
        "load" => TestType::Load,
        _ => panic!("Invalid test kind specified. Use 'fuzz' or 'load'."),
    };

    let client_count = cli.client_count;
    let commands_per_client = cli.sample_count;

    match test_type {
        TestType::Fuzz => {
            info!(
                "Running fuzz test with {} clients, each executing {} commands",
                client_count, commands_per_client
            );
            metrics.test_type = TestType::Fuzz;
            metrics =
                runner::fuzz::execute(&address, &data_set, client_count, commands_per_client).await;
        }
        TestType::Load => {
            info!(
                "Running load test with {} clients, each executing {} commands",
                client_count, commands_per_client
            );
            metrics.test_type = TestType::Load;
            metrics =
                runner::load::execute(&address, &data_set, client_count, commands_per_client).await;
        }
    }
    info!("Test completed. Metrics collected: {:?}", metrics);
    metrics
        .save_to_db(&pool)
        .await
        .expect("Failed to save metrics to database");
    info!("Metrics saved to database");

    // let file_name = "metrics.dat";
    // measurement::export(&metrics, file_name).expect("Failed to export metrics");
    // info!("Metrics exported to {}", file_name);

    Ok(())
}
