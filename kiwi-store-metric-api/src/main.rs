use std::{env, net::SocketAddr};

use axum::{Router, routing::get};
use data::get_metrics;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

mod data;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new().route(
        "/metrics",
        get({
            let pool = pool.clone();
            move || get_metrics(pool)
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 5556));
    println!("API running at http://{}/metrics", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Server started on {}", addr);
}
