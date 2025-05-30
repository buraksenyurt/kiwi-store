use std::{env, net::SocketAddr};

use axum::{Router, http::Method, routing::get};
use data::get_metrics;
use dotenvy::dotenv;
use log::info;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

mod data;
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers(Any);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route(
            "/metrics",
            get({
                let pool = pool.clone();
                move || get_metrics(pool)
            }),
        )
        .layer(cors);

    let address = SocketAddr::from(([127, 0, 0, 1], 5556));
    info!("API running at http://{}/metrics", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
